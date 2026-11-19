// Low-Level Windows Keyboard & Mouse Hook (WH_KEYBOARD_LL + WH_MOUSE_LL)
// Intercepts keys globally across all Windows applications and routes them to the Matra Bangla engine.
// Includes Smart Coding Mode to automatically switch to English when editing code in IDEs.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};
use crate::config::{AppConfig, ShortcutKey};
use crate::engine::{
    classify_window_context, normalize_window_title, Engine, KeyboardMode, WindowContext,
};
use crate::win32::*;

static SUPPRESSED_KEYS: [AtomicBool; 256] = [const { AtomicBool::new(false) }; 256];
static IS_BANGLA_ACTIVE: AtomicBool = AtomicBool::new(false);

// Tracks the most recent mouse click anywhere so when user clicks AI chat and presses Ctrl+\,
// we know the exact coordinates of the AI chat input box.
static LAST_MOUSE_CLICK_X: AtomicI32 = AtomicI32::new(0);
static LAST_MOUSE_CLICK_Y: AtomicI32 = AtomicI32::new(0);
static LAST_MOUSE_CLICK_HWND: AtomicUsize = AtomicUsize::new(0);

// Tracks the remembered AI Chat / Bangla input zone inside a Coding IDE window.
// Clicking outside this zone (into the Code Editor) auto-switches to English (EN);
// clicking back inside this zone (into the AI Chat box) auto-switches back to Bangla (BN)!
static CHAT_ZONE_ACTIVE: AtomicBool = AtomicBool::new(false);
static CHAT_ZONE_HWND: AtomicUsize = AtomicUsize::new(0);
static CHAT_ZONE_X: AtomicI32 = AtomicI32::new(0);
static CHAT_ZONE_Y: AtomicI32 = AtomicI32::new(0);

const CHAT_ZONE_HALF_WIDTH: i32 = 220;
const CHAT_ZONE_HALF_HEIGHT: i32 = 140;

pub fn is_point_in_chat_zone(pt_x: i32, pt_y: i32, zone_x: i32, zone_y: i32) -> bool {
    (pt_x - zone_x).abs() <= CHAT_ZONE_HALF_WIDTH && (pt_y - zone_y).abs() <= CHAT_ZONE_HALF_HEIGHT
}

pub struct WindowStateManager {
    pub enabled: bool,
    pub last_active_hwnd: usize,
    pub last_normalized_title: String,
    pub window_modes: HashMap<usize, KeyboardMode>,
}

impl WindowStateManager {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            last_active_hwnd: 0,
            last_normalized_title: String::new(),
            window_modes: HashMap::new(),
        }
    }

    /// Sync active mode with currently focused window.
    /// Returns Some(target_mode) if the active window changed mode.
    #[allow(dead_code)]
    pub fn sync_active_window(&mut self, current_hwnd: usize) -> Option<KeyboardMode> {
        self.sync_active_window_with_context(current_hwnd, "", WindowContext::Normal, false)
    }

    /// Sync active mode with currently focused window and IDE file/tab context.
    /// Returns Some(target_mode) if the active window or active IDE tab changed mode.
    pub fn sync_active_window_with_context(
        &mut self,
        current_hwnd: usize,
        raw_title: &str,
        context: WindowContext,
        smart_coding_mode: bool,
    ) -> Option<KeyboardMode> {
        if current_hwnd == 0 {
            return None;
        }

        let norm_title = normalize_window_title(raw_title);

        if current_hwnd != self.last_active_hwnd {
            self.last_active_hwnd = current_hwnd;
            self.last_normalized_title = norm_title;

            if !self.enabled {
                if smart_coding_mode && context == WindowContext::Terminal {
                    return Some(KeyboardMode::English);
                }
                return None;
            }

            // Retain valid windows if map gets large
            if self.window_modes.len() > 100 {
                self.window_modes.retain(|&hwnd, _| unsafe { IsWindow(hwnd as HWND) != 0 });
            }

            // Terminals always default to English in Smart Coding Mode
            if smart_coding_mode && context == WindowContext::Terminal {
                self.window_modes.insert(current_hwnd, KeyboardMode::English);
                return Some(KeyboardMode::English);
            }

            // If window not yet in map, default to English for maximum safety
            let mode = *self.window_modes.entry(current_hwnd).or_insert(KeyboardMode::English);
            Some(mode)
        } else {
            // Same top-level window: check if user switched file tabs inside a Coding IDE
            if smart_coding_mode
                && context == WindowContext::CodingIde
                && !norm_title.is_empty()
                && !self.last_normalized_title.is_empty()
                && norm_title != self.last_normalized_title
            {
                self.last_normalized_title = norm_title;
                if self.enabled {
                    self.window_modes.insert(current_hwnd, KeyboardMode::English);
                }
                return Some(KeyboardMode::English);
            }
            if !norm_title.is_empty() {
                self.last_normalized_title = norm_title;
            }
            None
        }
    }

    /// Update mode for current window
    pub fn set_mode_for_window(&mut self, current_hwnd: usize, mode: KeyboardMode) {
        if self.enabled && current_hwnd != 0 {
            self.last_active_hwnd = current_hwnd;
            self.window_modes.insert(current_hwnd, mode);
        }
    }
}

pub static ENGINE: Mutex<Engine> = Mutex::new(Engine::new());
pub static CONFIG: Mutex<Option<AppConfig>> = Mutex::new(None);
pub static WINDOW_STATE_MGR: LazyLock<Mutex<WindowStateManager>> = LazyLock::new(|| {
    Mutex::new(WindowStateManager::new(true))
});
static mut HOOK_HANDLE: HHOOK = std::ptr::null_mut();
static mut MOUSE_HOOK_HANDLE: HHOOK = std::ptr::null_mut();

/// Record the AI Chat / Bangla zone when the user manually turns Bangla ON
pub fn record_manual_bangla_zone(current_hwnd: usize, mode: KeyboardMode) {
    if mode == KeyboardMode::Bangla {
        let (x, y) = if current_hwnd != 0
            && LAST_MOUSE_CLICK_HWND.load(Ordering::Relaxed) == current_hwnd
        {
            (
                LAST_MOUSE_CLICK_X.load(Ordering::Relaxed),
                LAST_MOUSE_CLICK_Y.load(Ordering::Relaxed),
            )
        } else {
            unsafe {
                let mut pt: POINT = std::mem::zeroed();
                if GetCursorPos(&mut pt) != 0 {
                    (pt.x, pt.y)
                } else {
                    (0, 0)
                }
            }
        };
        CHAT_ZONE_X.store(x, Ordering::Relaxed);
        CHAT_ZONE_Y.store(y, Ordering::Relaxed);
        CHAT_ZONE_HWND.store(current_hwnd, Ordering::Relaxed);
        CHAT_ZONE_ACTIVE.store(true, Ordering::Relaxed);
    } else {
        // User manually turned Bangla OFF -> disable auto-Bangla zone until they turn Bangla ON again
        CHAT_ZONE_ACTIVE.store(false, Ordering::Relaxed);
    }
}

/// Update atomic flag, per-window state, OSD badge, and tray icon after a mode change
pub fn apply_mode_change_side_effects(current_hwnd: usize, new_mode: KeyboardMode) {
    IS_BANGLA_ACTIVE.store(new_mode == KeyboardMode::Bangla, Ordering::Relaxed);

    if current_hwnd != 0 {
        let mut wsm = WINDOW_STATE_MGR.lock().unwrap();
        wsm.set_mode_for_window(current_hwnd, new_mode);
    }

    let show_osd = {
        let cfg_guard = CONFIG.lock().unwrap();
        cfg_guard.as_ref().map_or(true, |c| c.osd_badge)
    };
    if show_osd {
        crate::osd::show_mode_badge(new_mode);
    }

    let tray_hwnd = crate::tray::get_tray_hwnd();
    if !tray_hwnd.is_null() {
        crate::tray::update_tray_tooltip(tray_hwnd, new_mode);
    }
}

/// Low-level keyboard hook callback procedure
pub unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code < 0 {
        return unsafe { CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param) };
    }

    let kbd = unsafe { *(l_param as *const KBDLLHOOKSTRUCT) };

    // Ignore injected keys from our own SendInput to prevent infinite feedback loops
    if (kbd.flags & LLKHF_INJECTED != 0) || kbd.extra_info == CUSTOM_INJECTED_SIGNATURE {
        return unsafe { CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param) };
    }

    let is_key_down = w_param as u32 == WM_KEYDOWN || w_param as u32 == WM_SYSKEYDOWN;
    let is_key_up = w_param as u32 == WM_KEYUP || w_param as u32 == WM_SYSKEYUP;

    let is_ctrl = is_key_pressed(VK_CONTROL);
    let is_alt = is_key_pressed(VK_MENU);
    let is_win = is_key_pressed(VK_LWIN) || is_key_pressed(VK_RWIN);

    // Sync active mode with currently focused window & detect IDE / Terminal context
    let current_hwnd = get_active_window_handle();
    let title = get_window_title(current_hwnd);
    let class_name = get_window_class(current_hwnd);
    let context = classify_window_context(&title, &class_name);
    let smart_coding = {
        let cfg_guard = CONFIG.lock().unwrap();
        cfg_guard.as_ref().map_or(true, |c| c.smart_coding_mode)
    };

    {
        let mut engine = ENGINE.lock().unwrap();
        engine.window_context = context;
        engine.smart_coding_mode = smart_coding;
    }

    {
        let mut wsm = WINDOW_STATE_MGR.lock().unwrap();
        if let Some(target_mode) =
            wsm.sync_active_window_with_context(current_hwnd, &title, context, smart_coding)
        {
            drop(wsm);
            let changed = {
                let mut engine = ENGINE.lock().unwrap();
                if engine.mode != target_mode {
                    engine.set_mode(target_mode);
                    true
                } else {
                    false
                }
            };
            if changed {
                apply_mode_change_side_effects(current_hwnd, target_mode);
            }
        }
    }

    let engine_mode = {
        let engine = ENGINE.lock().unwrap();
        engine.mode
    };

    // In Bangla mode: prevent accidental Caps Lock toggle from disrupting typing
    if engine_mode == KeyboardMode::Bangla && kbd.vk_code == VK_CAPITAL as u32 {
        // Swallow Caps Lock completely so Windows NEVER locks the keyboard into ALL CAPS
        return 1;
    }

    // In Bangla mode, if user holds Caps Lock or Shift, treat as Shift!
    let is_caps_down = is_key_pressed(VK_CAPITAL);
    let is_shift = is_key_pressed(VK_SHIFT) || (engine_mode == KeyboardMode::Bangla && is_caps_down);

    // Check for Language Switch Hotkey on KeyDown
    let is_toggle_key = if is_win {
        false
    } else {
        let cfg_guard = CONFIG.lock().unwrap();
        if let Some(cfg) = cfg_guard.as_ref() {
            match cfg.shortcut {
                ShortcutKey::CtrlBackslash => is_ctrl && !is_alt && kbd.vk_code == VK_OEM_5 as u32,
                ShortcutKey::CtrlF12 => is_ctrl && !is_alt && kbd.vk_code == VK_F12 as u32,
                ShortcutKey::F12 => !is_ctrl && !is_alt && kbd.vk_code == VK_F12 as u32,
                ShortcutKey::AltBackslash => !is_ctrl && is_alt && kbd.vk_code == VK_OEM_5 as u32,
                ShortcutKey::CtrlSpace => is_ctrl && !is_alt && kbd.vk_code == VK_SPACE as u32,
            }
        } else {
            // Default fallback: Ctrl + \ or Ctrl + F12
            (is_ctrl && kbd.vk_code == VK_OEM_5 as u32) || (is_ctrl && kbd.vk_code == VK_F12 as u32)
        }
    };

    if is_toggle_key {
        if is_key_down {
            let new_mode = {
                let mut engine = ENGINE.lock().unwrap();
                engine.toggle_mode()
            };
            print_mode_banner(new_mode);
            record_manual_bangla_zone(current_hwnd, new_mode);
            apply_mode_change_side_effects(current_hwnd, new_mode);
        }
        return 1; // Swallow the toggle keypress completely
    }

    // Process normal keystrokes
    if is_key_down {
        let (handled, mode_changed, new_mode) = {
            let mut engine = ENGINE.lock().unwrap();
            let prev_mode = engine.mode;
            let handled = engine.handle_key_down_with_modifiers(
                kbd.vk_code,
                is_shift,
                is_alt,
                is_ctrl,
                is_win,
            );
            let current_mode = engine.mode;
            (handled, prev_mode != current_mode, current_mode)
        };

        if mode_changed {
            print_mode_banner(new_mode);
            apply_mode_change_side_effects(current_hwnd, new_mode);
        }

        if handled {
            if (kbd.vk_code as usize) < 256 {
                SUPPRESSED_KEYS[kbd.vk_code as usize].store(true, Ordering::Relaxed);
            }
            return 1; // Suppress the physical key from reaching the foreground application
        } else if (kbd.vk_code as usize) < 256 {
            SUPPRESSED_KEYS[kbd.vk_code as usize].store(false, Ordering::Relaxed);
        }
    } else if is_key_up {
        if (kbd.vk_code as usize) < 256
            && SUPPRESSED_KEYS[kbd.vk_code as usize].swap(false, Ordering::Relaxed)
        {
            return 1;
        }
    }

    unsafe { CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param) }
}

/// Low-level mouse hook callback procedure for Smart Coding Mode.
/// Supports two-way automatic language switching in Coding IDEs:
/// - Clicking outside the AI Chat box (into the Code Editor) auto-switches to English (EN).
/// - Clicking back inside the AI Chat box auto-switches back to Bangla (BN)!
pub unsafe extern "system" fn low_level_mouse_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= 0 && (w_param as u32 == WM_LBUTTONDOWN || w_param as u32 == WM_RBUTTONDOWN) {
        let ms = unsafe { *(l_param as *const MSLLHOOKSTRUCT) };
        let current_hwnd = get_active_window_handle();

        // Always record the latest click coordinates for this window
        LAST_MOUSE_CLICK_X.store(ms.pt.x, Ordering::Relaxed);
        LAST_MOUSE_CLICK_Y.store(ms.pt.y, Ordering::Relaxed);
        LAST_MOUSE_CLICK_HWND.store(current_hwnd, Ordering::Relaxed);

        if IS_BANGLA_ACTIVE.load(Ordering::Relaxed) || CHAT_ZONE_ACTIVE.load(Ordering::Relaxed) {
            let title = get_window_title(current_hwnd);
            let class_name = get_window_class(current_hwnd);
            let context = classify_window_context(&title, &class_name);

            if context == WindowContext::CodingIde {
                let target_switch: Option<KeyboardMode> = {
                    let mut engine = ENGINE.lock().unwrap();
                    engine.window_context = context;

                    if !engine.smart_coding_mode {
                        None
                    } else if engine.mode == KeyboardMode::Bangla {
                        if engine.bangla_chars_typed == 0 {
                            // User pressed Ctrl+\ first and then clicked the AI chat input box to start typing
                            CHAT_ZONE_X.store(ms.pt.x, Ordering::Relaxed);
                            CHAT_ZONE_Y.store(ms.pt.y, Ordering::Relaxed);
                            CHAT_ZONE_HWND.store(current_hwnd, Ordering::Relaxed);
                            CHAT_ZONE_ACTIVE.store(true, Ordering::Relaxed);
                            None
                        } else {
                            let zone_x = CHAT_ZONE_X.load(Ordering::Relaxed);
                            let zone_y = CHAT_ZONE_Y.load(Ordering::Relaxed);

                            if !is_point_in_chat_zone(ms.pt.x, ms.pt.y, zone_x, zone_y) {
                                // Clicked outside the AI Chat box (e.g. into Code Editor / Explorer / Terminal)
                                engine.set_mode(KeyboardMode::English);
                                Some(KeyboardMode::English)
                            } else {
                                // Clicked inside the AI Chat box -> refine chat zone center and stay in Bangla
                                CHAT_ZONE_X.store(ms.pt.x, Ordering::Relaxed);
                                CHAT_ZONE_Y.store(ms.pt.y, Ordering::Relaxed);
                                None
                            }
                        }
                    } else {
                        // Currently in English mode inside the Coding IDE:
                        // Check if user clicked BACK into the remembered AI Chat Zone!
                        let zone_active = CHAT_ZONE_ACTIVE.load(Ordering::Relaxed);
                        let zone_hwnd = CHAT_ZONE_HWND.load(Ordering::Relaxed);
                        let zone_x = CHAT_ZONE_X.load(Ordering::Relaxed);
                        let zone_y = CHAT_ZONE_Y.load(Ordering::Relaxed);

                        if zone_active
                            && current_hwnd != 0
                            && current_hwnd == zone_hwnd
                            && is_point_in_chat_zone(ms.pt.x, ms.pt.y, zone_x, zone_y)
                        {
                            engine.set_mode(KeyboardMode::Bangla);
                            // Mark bangla_chars_typed = 1 so clicking back out to the code editor
                            // will still switch back to English even if no new key was pressed yet.
                            engine.bangla_chars_typed = 1;
                            Some(KeyboardMode::Bangla)
                        } else {
                            None
                        }
                    }
                };

                if let Some(new_mode) = target_switch {
                    print_mode_banner(new_mode);
                    apply_mode_change_side_effects(current_hwnd, new_mode);
                }
            }
        }
    }

    unsafe { CallNextHookEx(MOUSE_HOOK_HANDLE, n_code, w_param, l_param) }
}

pub fn print_mode_banner(mode: KeyboardMode) {
    match mode {
        KeyboardMode::English => {
            println!("\n  [\x1b[1;36mMODE\x1b[0m] \x1b[1;32mENGLISH\x1b[0m (Direct passthrough)");
        }
        KeyboardMode::Bangla => {
            println!("\n  [\x1b[1;36mMODE\x1b[0m] \x1b[1;33mBANGLA (বাংলা)\x1b[0m | Bengali Unicode Active!");
        }
    }
}

/// Install the global keyboard and mouse hooks
pub fn install_hook() -> Result<(), &'static str> {
    unsafe {
        let h_instance = GetModuleHandleW(std::ptr::null());
        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc),
            h_instance,
            0,
        );
        if hook.is_null() {
            return Err("Failed to install low-level keyboard hook! Check permissions.");
        }
        HOOK_HANDLE = hook;

        // Install lightweight mouse click hook for Smart Coding Mode
        let mouse_hook = SetWindowsHookExW(
            WH_MOUSE_LL,
            Some(low_level_mouse_proc),
            h_instance,
            0,
        );
        if !mouse_hook.is_null() {
            MOUSE_HOOK_HANDLE = mouse_hook;
        }
    }
    Ok(())
}

/// Uninstall the global keyboard and mouse hooks
pub fn uninstall_hook() {
    unsafe {
        if !HOOK_HANDLE.is_null() {
            UnhookWindowsHookEx(HOOK_HANDLE);
            HOOK_HANDLE = std::ptr::null_mut();
        }
        if !MOUSE_HOOK_HANDLE.is_null() {
            UnhookWindowsHookEx(MOUSE_HOOK_HANDLE);
            MOUSE_HOOK_HANDLE = std::ptr::null_mut();
        }
    }
}

// Low-Level Windows Keyboard Hook (WH_KEYBOARD_LL)
// Intercepts keys globally across all Windows applications and routes them to the UniBijoy engine.

use std::sync::Mutex;
use crate::config::{AppConfig, ShortcutKey};
use crate::engine::{Engine, KeyboardMode};
use crate::win32::*;

pub static ENGINE: Mutex<Engine> = Mutex::new(Engine::new());
pub static CONFIG: Mutex<Option<AppConfig>> = Mutex::new(None);
static mut HOOK_HANDLE: HHOOK = std::ptr::null_mut();

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

    let engine_mode = {
        let engine = ENGINE.lock().unwrap();
        engine.mode
    };

    // In UniBijoy mode: prevent accidental Caps Lock toggle from disrupting typing
    if engine_mode == KeyboardMode::UniBijoy && kbd.vk_code == VK_CAPITAL as u32 {
        // Swallow Caps Lock completely so Windows NEVER locks the keyboard into ALL CAPS
        return 1;
    }

    // In UniBijoy mode, if user holds Caps Lock or Shift, treat as Shift!
    let is_caps_down = is_key_pressed(VK_CAPITAL);
    let is_shift = is_key_pressed(VK_SHIFT) || (engine_mode == KeyboardMode::UniBijoy && is_caps_down);

    // Check for Language Switch Hotkey on KeyDown
    let is_toggle_key = {
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
            let mut engine = ENGINE.lock().unwrap();
            let new_mode = engine.toggle_mode();
            print_mode_banner(new_mode);
        }
        return 1; // Swallow the toggle keypress completely
    }

    // Process normal keystrokes
    if is_key_down {
        let mut engine = ENGINE.lock().unwrap();
        if engine.handle_key_down(kbd.vk_code, is_shift, is_alt, is_ctrl) {
            return 1; // Suppress the physical key from reaching the foreground application
        }
    } else if is_key_up {
        let engine = ENGINE.lock().unwrap();
        if engine.mode == KeyboardMode::UniBijoy {
            // If in UniBijoy mode and not holding Ctrl/Alt, suppress the character keyup
            let is_alt_gr = is_ctrl && is_alt;
            if (!is_ctrl && !is_alt) || is_alt_gr {
                // If it's a typing key that UniBijoy intercepts, suppress KeyUp too
                if is_typing_key(kbd.vk_code) {
                    return 1;
                }
            }
        }
    }

    unsafe { CallNextHookEx(HOOK_HANDLE, n_code, w_param, l_param) }
}

fn is_typing_key(vk: u32) -> bool {
    (vk >= 0x30 && vk <= 0x39) // 0-9
        || (vk >= 0x41 && vk <= 0x5A) // A-Z
        || (vk >= 0x60 && vk <= 0x6F) // Numpad
        || vk == VK_OEM_1 as u32
        || vk == VK_OEM_PLUS as u32
        || vk == VK_OEM_COMMA as u32
        || vk == VK_OEM_MINUS as u32
        || vk == VK_OEM_PERIOD as u32
        || vk == VK_OEM_2 as u32
        || vk == VK_OEM_3 as u32
        || vk == VK_OEM_4 as u32
        || vk == VK_OEM_5 as u32
        || vk == VK_OEM_6 as u32
        || vk == VK_OEM_7 as u32
}

pub fn print_mode_banner(mode: KeyboardMode) {
    match mode {
        KeyboardMode::English => {
            println!("\n  [\x1b[1;36mMODE\x1b[0m] \x1b[1;32mENGLISH\x1b[0m (Direct passthrough)");
        }
        KeyboardMode::UniBijoy => {
            println!("\n  [\x1b[1;36mMODE\x1b[0m] \x1b[1;33mUNIBIJOY (ইউনিবিজয়)\x1b[0m | Bengali Unicode Active!");
        }
    }
}

/// Install the global keyboard hook
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
    }
    Ok(())
}

/// Uninstall the global keyboard hook
pub fn uninstall_hook() {
    unsafe {
        if !HOOK_HANDLE.is_null() {
            UnhookWindowsHookEx(HOOK_HANDLE);
            HOOK_HANDLE = std::ptr::null_mut();
        }
    }
}

// System Tray Integration for Borno (বর্ণ)
// Creates a native Windows Notification Area (System Tray) icon with right-click menu,
// tooltips, and click-to-toggle capability without needing any external crates.

use std::sync::atomic::{AtomicIsize, Ordering};
use crate::engine::KeyboardMode;
use crate::hook::{print_mode_banner, ENGINE};
use crate::win32::*;

pub const WM_TRAYICON: u32 = 0x0400 + 1; // WM_USER + 1
pub const ID_TRAY_TOGGLE: usize = 1001;
pub const ID_TRAY_CONFIG: usize = 1002;
pub const ID_TRAY_README: usize = 1003;
pub const ID_TRAY_EXIT: usize = 1004;

static TRAY_HWND: AtomicIsize = AtomicIsize::new(0);

pub fn get_tray_hwnd() -> HWND {
    TRAY_HWND.load(Ordering::SeqCst) as HWND
}

pub fn set_tray_hwnd(hwnd: HWND) {
    TRAY_HWND.store(hwnd as isize, Ordering::SeqCst);
}

pub unsafe extern "system" fn tray_window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            let event = l_param as u32;
            match event {
                // Left click or double click: toggle mode!
                0x0202 | 0x0203 => { // WM_LBUTTONUP = 0x0202, WM_LBUTTONDBLCLK = 0x0203
                    let mut engine = ENGINE.lock().unwrap();
                    let new_mode = engine.toggle_mode();
                    print_mode_banner(new_mode);
                    update_tray_tooltip(hwnd, new_mode);

                    let active = get_active_window_handle();
                    if active != 0 {
                        let mut wsm = crate::hook::WINDOW_STATE_MGR.lock().unwrap();
                        wsm.set_mode_for_window(active, new_mode);
                    }
                }
                // Right click or Context Menu: show popup menu
                0x0205 | 0x007B => { // WM_RBUTTONUP = 0x0205, WM_CONTEXTMENU = 0x007B
                    unsafe {
                        show_tray_menu(hwnd);
                    }
                }
                _ => {}
            }
            0
        }
        0x0111 => { // WM_COMMAND
            let cmd_id = (w_param & 0xFFFF) as usize;
            match cmd_id {
                ID_TRAY_TOGGLE => {
                    let mut engine = ENGINE.lock().unwrap();
                    let new_mode = engine.toggle_mode();
                    print_mode_banner(new_mode);
                    update_tray_tooltip(hwnd, new_mode);

                    let active = get_active_window_handle();
                    if active != 0 {
                        let mut wsm = crate::hook::WINDOW_STATE_MGR.lock().unwrap();
                        wsm.set_mode_for_window(active, new_mode);
                    }
                }
                ID_TRAY_CONFIG => {
                    open_file_in_default_app("config.ini");
                }
                ID_TRAY_README => {
                    open_file_in_default_app("README.md");
                }
                ID_TRAY_EXIT => {
                    remove_tray_icon(hwnd);
                    unsafe {
                        PostQuitMessage(0);
                    }
                }
                _ => {}
            }
            0
        }
        0x0002 => { // WM_DESTROY
            remove_tray_icon(hwnd);
            unsafe {
                PostQuitMessage(0);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
    }
}

pub fn init_tray(mode: KeyboardMode) -> HWND {
    unsafe {
        let class_name = to_wide_string("BornoTrayClass");
        let window_name = to_wide_string("Borno Tray Window");
        let h_instance = GetModuleHandleW(std::ptr::null());

        let mut wnd_class: WNDCLASSEXW = std::mem::zeroed();
        wnd_class.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wnd_class.lpfnWndProc = Some(tray_window_proc);
        wnd_class.hInstance = h_instance;
        wnd_class.lpszClassName = class_name.as_ptr();

        RegisterClassExW(&wnd_class);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        if !hwnd.is_null() {
            set_tray_hwnd(hwnd);
            add_tray_icon(hwnd, mode);
        }

        hwnd
    }
}

pub fn add_tray_icon(hwnd: HWND, mode: KeyboardMode) {
    unsafe {
        let h_icon = load_app_icon();
        let mut nid = make_nid(hwnd, h_icon);
        nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_INFO;
        nid.uCallbackMessage = WM_TRAYICON;

        let tip = format!("Borno (বর্ণ) - {}", mode_name(mode));
        copy_to_wide_array(&tip, &mut nid.szTip);

        // Friendly startup balloon notification
        let title = "Borno (বর্ণ) Keyboard Active";
        let info = "Press Ctrl + \\ to switch language anytime.\nRight-click icon for options.";
        copy_to_wide_array(title, &mut nid.szInfoTitle);
        copy_to_wide_array(info, &mut nid.szInfo);
        nid.dwInfoFlags = 0x00000001; // NIIF_INFO

        Shell_NotifyIconW(NIM_ADD, &mut nid);
    }
}

pub fn update_tray_tooltip(hwnd: HWND, mode: KeyboardMode) {
    unsafe {
        if hwnd.is_null() {
            return;
        }
        let h_icon = load_app_icon();
        let mut nid = make_nid(hwnd, h_icon);
        nid.uFlags = NIF_TIP | NIF_ICON;

        let tip = format!("Borno (বর্ণ) - {}", mode_name(mode));
        copy_to_wide_array(&tip, &mut nid.szTip);

        Shell_NotifyIconW(NIM_MODIFY, &mut nid);
    }
}

pub fn remove_tray_icon(hwnd: HWND) {
    unsafe {
        if hwnd.is_null() {
            return;
        }
        let mut nid = make_nid(hwnd, std::ptr::null_mut());
        Shell_NotifyIconW(NIM_DELETE, &mut nid);
    }
}

unsafe fn show_tray_menu(hwnd: HWND) {
    let mut pt: POINT = unsafe { std::mem::zeroed() };
    unsafe {
        GetCursorPos(&mut pt);
    }

    let h_menu = unsafe { CreatePopupMenu() };
    if h_menu.is_null() {
        return;
    }

    let mode = {
        let engine = ENGINE.lock().unwrap();
        engine.mode
    };

    let toggle_label = match mode {
        KeyboardMode::English => "Switch to বাংলা (UniBijoy)    Ctrl+\\",
        KeyboardMode::UniBijoy => "Switch to English                Ctrl+\\",
    };

    let w_toggle = to_wide_string(toggle_label);
    let w_config = to_wide_string("⚙ Settings (config.ini)");
    let w_about = to_wide_string("📖 Keyboard Cheat-Sheet (README)");
    let w_exit = to_wide_string("❌ Exit Borno");

    unsafe {
        AppendMenuW(h_menu, 0x0000, ID_TRAY_TOGGLE, w_toggle.as_ptr());
        AppendMenuW(h_menu, 0x0800, 0, std::ptr::null()); // MF_SEPARATOR
        AppendMenuW(h_menu, 0x0000, ID_TRAY_CONFIG, w_config.as_ptr());
        AppendMenuW(h_menu, 0x0000, ID_TRAY_README, w_about.as_ptr());
        AppendMenuW(h_menu, 0x0800, 0, std::ptr::null()); // MF_SEPARATOR
        AppendMenuW(h_menu, 0x0000, ID_TRAY_EXIT, w_exit.as_ptr());

        SetForegroundWindow(hwnd);
        TrackPopupMenu(
            h_menu,
            0x0008 | 0x0002, // TPM_RIGHTBUTTON | TPM_BOTTOMALIGN
            pt.x,
            pt.y,
            0,
            hwnd,
            std::ptr::null(),
        );
        DestroyMenu(h_menu);
    }
}

fn mode_name(mode: KeyboardMode) -> &'static str {
    match mode {
        KeyboardMode::English => "English (EN)",
        KeyboardMode::UniBijoy => "UniBijoy (বাংলা)",
    }
}

unsafe fn load_app_icon() -> *mut std::ffi::c_void {
    let h_inst = unsafe { GetModuleHandleW(std::ptr::null()) };
    // Try to load embedded resource icon 1 (from borno.rc)
    let icon = unsafe {
        LoadImageW(
            h_inst,
            1 as *const u16,
            1, // IMAGE_ICON
            16, // Tray icon standard size 16x16
            16,
            0,
        )
    };
    if !icon.is_null() {
        return icon;
    }
    // Fallback: standard application icon
    unsafe { LoadIconW(std::ptr::null_mut(), 32512 as *const u16) } // IDI_APPLICATION
}

unsafe fn make_nid(hwnd: HWND, h_icon: *mut std::ffi::c_void) -> NOTIFYICONDATAW {
    let mut nid: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.hIcon = h_icon;
    nid
}

fn copy_to_wide_array(src: &str, dst: &mut [u16]) {
    let wide = to_wide_string(src);
    let len = wide.len().min(dst.len() - 1);
    dst[..len].copy_from_slice(&wide[..len]);
    dst[len] = 0;
}

fn open_file_in_default_app(file_name: &str) {
    unsafe {
        let op = to_wide_string("open");
        let file = to_wide_string(file_name);
        ShellExecuteW(
            std::ptr::null_mut(),
            op.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1, // SW_SHOWNORMAL
        );
    }
}

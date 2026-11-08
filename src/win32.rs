// Win32 FFI bindings for low-level keyboard hook, input injection, and audio.
// Clean, zero external dependencies, 100% pure Rust.

#![allow(non_snake_case)]
#![allow(dead_code)]

pub type HWND = *mut std::ffi::c_void;
pub type HINSTANCE = *mut std::ffi::c_void;
pub type HHOOK = *mut std::ffi::c_void;
pub type LRESULT = isize;
pub type BOOL = i32;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type HOOKPROC = Option<unsafe extern "system" fn(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT>;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub w_param: WPARAM,
    pub l_param: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBDLLHOOKSTRUCT {
    pub vk_code: u32,
    pub scan_code: u32,
    pub flags: u32,
    pub time: u32,
    pub extra_info: usize,
}

pub const WH_KEYBOARD_LL: i32 = 13;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;

pub const LLKHF_EXTENDED: u32 = 0x01;
pub const LLKHF_INJECTED: u32 = 0x10;
pub const LLKHF_ALTDOWN: u32 = 0x20;
pub const LLKHF_UP: u32 = 0x80;

pub const INPUT_KEYBOARD: u32 = 1;
pub const KEYEVENTF_EXTENDEDKEY: u32 = 0x0001;
pub const KEYEVENTF_KEYUP: u32 = 0x0002;
pub const KEYEVENTF_UNICODE: u32 = 0x0004;

// Standard Virtual Key Codes
pub const VK_BACK: i32 = 0x08;
pub const VK_TAB: i32 = 0x09;
pub const VK_RETURN: i32 = 0x0D;
pub const VK_SHIFT: i32 = 0x10;
pub const VK_CONTROL: i32 = 0x11;
pub const VK_MENU: i32 = 0x12; // Alt
pub const VK_CAPITAL: i32 = 0x14; // Caps Lock
pub const VK_ESCAPE: i32 = 0x1B;
pub const VK_SPACE: i32 = 0x20;
pub const VK_LWIN: i32 = 0x5B;
pub const VK_RWIN: i32 = 0x5C;
pub const VK_F1: i32 = 0x70;
pub const VK_F12: i32 = 0x7B;

// OEM keys
pub const VK_OEM_1: i32 = 0xBA; // ';:'
pub const VK_OEM_PLUS: i32 = 0xBB; // '=+'
pub const VK_OEM_COMMA: i32 = 0xBC; // ',<'
pub const VK_OEM_MINUS: i32 = 0xBD; // '-_'
pub const VK_OEM_PERIOD: i32 = 0xBE; // '.>'
pub const VK_OEM_2: i32 = 0xBF; // '/?'
pub const VK_OEM_3: i32 = 0xC0; // '`~'
pub const VK_OEM_4: i32 = 0xDB; // '[{'
pub const VK_OEM_5: i32 = 0xDC; // '\|'
pub const VK_OEM_6: i32 = 0xDD; // ']}'
pub const VK_OEM_7: i32 = 0xDE; // ''"'

// Numpad
pub const VK_NUMPAD0: i32 = 0x60;
pub const VK_NUMPAD1: i32 = 0x61;
pub const VK_NUMPAD2: i32 = 0x62;
pub const VK_NUMPAD3: i32 = 0x63;
pub const VK_NUMPAD4: i32 = 0x64;
pub const VK_NUMPAD5: i32 = 0x65;
pub const VK_NUMPAD6: i32 = 0x66;
pub const VK_NUMPAD7: i32 = 0x67;
pub const VK_NUMPAD8: i32 = 0x68;
pub const VK_NUMPAD9: i32 = 0x69;
pub const VK_MULTIPLY: i32 = 0x6A;
pub const VK_ADD: i32 = 0x6B;
pub const VK_SUBTRACT: i32 = 0x6D;
pub const VK_DECIMAL: i32 = 0x6E;
pub const VK_DIVIDE: i32 = 0x6F;

// Custom signature used in dwExtraInfo to identify our injected inputs
pub const CUSTOM_INJECTED_SIGNATURE: usize = 0x554E4942; // "UNIB"

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEYBDINPUT {
    pub w_vk: u16,
    pub w_scan: u16,
    pub dw_flags: u32,
    pub time: u32,
    pub dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT {
    pub r#type: u32,
    pub _pad: u32,
    pub ki: KEYBDINPUT,
    pub _union_pad: [u8; 8],
}

impl Default for INPUT {
    fn default() -> Self {
        Self {
            r#type: INPUT_KEYBOARD,
            _pad: 0,
            ki: KEYBDINPUT {
                w_vk: 0,
                w_scan: 0,
                dw_flags: 0,
                time: 0,
                dw_extra_info: CUSTOM_INJECTED_SIGNATURE,
            },
            _union_pad: [0; 8],
        }
    }
}

#[link(name = "user32")]
unsafe extern "system" {
    pub fn SetWindowsHookExW(
        idHook: i32,
        lpfn: HOOKPROC,
        hmod: HINSTANCE,
        dwThreadId: u32,
    ) -> HHOOK;
    pub fn UnhookWindowsHookEx(hhk: HHOOK) -> BOOL;
    pub fn CallNextHookEx(hhk: HHOOK, nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn GetMessageW(
        lpMsg: *mut MSG,
        hWnd: HWND,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn SendInput(cInputs: u32, pInputs: *const INPUT, cbSize: i32) -> u32;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;
    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn keybd_event(bVk: u8, bScan: u8, dwFlags: u32, dwExtraInfo: usize);
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> BOOL;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn GetModuleHandleW(lpModuleName: *const u16) -> HINSTANCE;
    pub fn Beep(dwFreq: u32, dwDuration: u32) -> BOOL;
    pub fn GetStdHandle(nStdHandle: u32) -> *mut std::ffi::c_void;
    pub fn GetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, lpMode: *mut u32) -> BOOL;
    pub fn SetConsoleMode(hConsoleHandle: *mut std::ffi::c_void, dwMode: u32) -> BOOL;
    pub fn GetConsoleWindow() -> HWND;
    pub fn SetConsoleCtrlHandler(
        HandlerRoutine: Option<unsafe extern "system" fn(dwCtrlType: u32) -> BOOL>,
        Add: BOOL,
    ) -> BOOL;
}

pub const SW_HIDE: i32 = 0;
pub const SW_SHOW: i32 = 5;

pub fn hide_console_window() {
    unsafe {
        let hwnd = GetConsoleWindow();
        if !hwnd.is_null() {
            ShowWindow(hwnd, SW_HIDE);
        }
    }
}

pub const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5;
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

pub fn enable_ansi_colors() {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if !handle.is_null() {
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}

/// Send a unicode string to the active window using SendInput
pub fn send_unicode_string(s: &str) {
    let mut inputs: Vec<INPUT> = Vec::new();
    for c in s.encode_utf16() {
        // Key down
        let mut down = INPUT::default();
        down.ki.w_vk = 0;
        down.ki.w_scan = c;
        down.ki.dw_flags = KEYEVENTF_UNICODE;
        down.ki.dw_extra_info = CUSTOM_INJECTED_SIGNATURE;
        inputs.push(down);

        // Key up
        let mut up = INPUT::default();
        up.ki.w_vk = 0;
        up.ki.w_scan = c;
        up.ki.dw_flags = KEYEVENTF_UNICODE | KEYEVENTF_KEYUP;
        up.ki.dw_extra_info = CUSTOM_INJECTED_SIGNATURE;
        inputs.push(up);
    }

    if !inputs.is_empty() {
        unsafe {
            SendInput(
                inputs.len() as u32,
                inputs.as_ptr(),
                std::mem::size_of::<INPUT>() as i32,
            );
        }
    }
}

/// Send backspace keystrokes to erase N characters.
/// If Shift is physically held down by the user, Shift is released
/// so the application receives a pure Backspace, NOT Shift+Backspace.
pub fn send_backspaces(count: usize) {
    if count == 0 {
        return;
    }
    let shift_pressed = is_key_pressed(VK_SHIFT);

    let mut inputs: Vec<INPUT> = Vec::new();

    // 1. Release Shift so apps don't treat this as Shift+Backspace
    if shift_pressed {
        let mut shift_up = INPUT::default();
        shift_up.ki.w_vk = VK_SHIFT as u16;
        shift_up.ki.w_scan = 0x2A;
        shift_up.ki.dw_flags = KEYEVENTF_KEYUP;
        shift_up.ki.dw_extra_info = CUSTOM_INJECTED_SIGNATURE;
        inputs.push(shift_up);
    }

    // 2. Send Backspace down and up with scan code 0x0E
    for _ in 0..count {
        let mut down = INPUT::default();
        down.ki.w_vk = VK_BACK as u16;
        down.ki.w_scan = 0x0E;
        down.ki.dw_flags = 0;
        down.ki.dw_extra_info = CUSTOM_INJECTED_SIGNATURE;
        inputs.push(down);

        let mut up = INPUT::default();
        up.ki.w_vk = VK_BACK as u16;
        up.ki.w_scan = 0x0E;
        up.ki.dw_flags = KEYEVENTF_KEYUP;
        up.ki.dw_extra_info = CUSTOM_INJECTED_SIGNATURE;
        inputs.push(up);
    }

    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

/// Check if a key is currently pressed
pub fn is_key_pressed(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
}

/// Check if Caps Lock is currently toggled ON
pub fn is_caps_lock_active() -> bool {
    unsafe { (GetKeyState(VK_CAPITAL) & 0x0001) != 0 }
}

/// Turn Caps Lock OFF safely if it is currently ON
pub fn turn_caps_lock_off() {
    unsafe {
        if (GetKeyState(VK_CAPITAL) & 0x0001) != 0 {
            keybd_event(VK_CAPITAL as u8, 0x45, KEYEVENTF_EXTENDEDKEY, CUSTOM_INJECTED_SIGNATURE);
            keybd_event(VK_CAPITAL as u8, 0x45, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, CUSTOM_INJECTED_SIGNATURE);
        }
    }
}

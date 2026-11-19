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
pub type WNDPROC = Option<unsafe extern "system" fn(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT>;

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: *mut std::ffi::c_void,
    pub hCursor: *mut std::ffi::c_void,
    pub hbrBackground: *mut std::ffi::c_void,
    pub lpszMenuName: *const u16,
    pub lpszClassName: *const u16,
    pub hIconSm: *mut std::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: *mut std::ffi::c_void,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [u8; 32],
}

#[repr(C)]
pub struct GUITHREADINFO {
    pub cbSize: u32,
    pub flags: u32,
    pub hwndActive: HWND,
    pub hwndFocus: HWND,
    pub hwndCapture: HWND,
    pub hwndMenuOwner: HWND,
    pub hwndMoveSize: HWND,
    pub hwndCaret: HWND,
    pub rcCaret: RECT,
}

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub mouse_data: u32,
    pub flags: u32,
    pub time: u32,
    pub extra_info: usize,
}

pub const WH_KEYBOARD_LL: i32 = 13;
pub const WH_MOUSE_LL: i32 = 14;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_RBUTTONDOWN: u32 = 0x0204;

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
    pub fn RegisterClassExW(lpwcx: *const WNDCLASSEXW) -> u16;
    pub fn CreateWindowExW(
        dwExStyle: u32,
        lpClassName: *const u16,
        lpWindowName: *const u16,
        dwStyle: u32,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: *mut std::ffi::c_void,
        hInstance: HINSTANCE,
        lpParam: *mut std::ffi::c_void,
    ) -> HWND;
    pub fn DefWindowProcW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn CreatePopupMenu() -> *mut std::ffi::c_void;
    pub fn AppendMenuW(hMenu: *mut std::ffi::c_void, uFlags: u32, uIDNewItem: usize, lpNewItem: *const u16) -> BOOL;
    pub fn TrackPopupMenu(
        hMenu: *mut std::ffi::c_void,
        uFlags: u32,
        x: i32,
        y: i32,
        nReserved: i32,
        hWnd: HWND,
        prcRect: *const std::ffi::c_void,
    ) -> BOOL;
    pub fn DestroyMenu(hMenu: *mut std::ffi::c_void) -> BOOL;
    pub fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    pub fn GetCursorPos(lpPoint: *mut POINT) -> BOOL;
    pub fn LoadImageW(
        hInst: HINSTANCE,
        name: *const u16,
        r#type: u32,
        cx: i32,
        cy: i32,
        fuLoad: u32,
    ) -> *mut std::ffi::c_void;
    pub fn LoadIconW(hInstance: HINSTANCE, lpIconName: *const u16) -> *mut std::ffi::c_void;
    pub fn GetForegroundWindow() -> HWND;
    pub fn GetAncestor(hwnd: HWND, gaFlags: u32) -> HWND;
    pub fn IsWindow(hWnd: HWND) -> BOOL;
    pub fn GetWindowTextW(hWnd: HWND, lpString: *mut u16, nMaxCount: i32) -> i32;
    pub fn GetClassNameW(hWnd: HWND, lpClassName: *mut u16, nMaxCount: i32) -> i32;
    pub fn GetGUIThreadInfo(idThread: u32, pgui: *mut GUITHREADINFO) -> BOOL;
    pub fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: *mut u32) -> u32;
    pub fn ClientToScreen(hWnd: HWND, lpPoint: *mut POINT) -> BOOL;
    pub fn BeginPaint(hWnd: HWND, lpPaint: *mut PAINTSTRUCT) -> *mut std::ffi::c_void;
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;
    pub fn SetWindowPos(hWnd: HWND, hWndInsertAfter: HWND, X: i32, Y: i32, cx: i32, cy: i32, uFlags: u32) -> BOOL;
    pub fn SetTimer(hWnd: HWND, nIDEvent: usize, uElapse: u32, lpTimerFunc: *const std::ffi::c_void) -> usize;
    pub fn KillTimer(hWnd: HWND, uIDEvent: usize) -> BOOL;
    pub fn InvalidateRect(hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
    pub fn FillRect(hDC: *mut std::ffi::c_void, lprc: *const RECT, hbr: *mut std::ffi::c_void) -> i32;
    pub fn DrawTextW(hdc: *mut std::ffi::c_void, lpchText: *const u16, cchText: i32, lprc: *mut RECT, format: u32) -> i32;
    pub fn SetWindowRgn(hWnd: HWND, hRgn: *mut std::ffi::c_void, bRedraw: BOOL) -> i32;
    pub fn GetSystemMetrics(nIndex: i32) -> i32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    pub fn CreateSolidBrush(color: u32) -> *mut std::ffi::c_void;
    pub fn DeleteObject(ho: *mut std::ffi::c_void) -> BOOL;
    pub fn CreateFontW(
        cHeight: i32,
        cWidth: i32,
        cEscapement: i32,
        cOrientation: i32,
        cWeight: i32,
        bItalic: u32,
        bUnderline: u32,
        bStrikeOut: u32,
        iCharSet: u32,
        iOutPrecision: u32,
        iClipPrecision: u32,
        iQuality: u32,
        iPitchAndFamily: u32,
        pszFaceName: *const u16,
    ) -> *mut std::ffi::c_void;
    pub fn SelectObject(hdc: *mut std::ffi::c_void, h: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    pub fn SetBkMode(hdc: *mut std::ffi::c_void, mode: i32) -> i32;
    pub fn SetTextColor(hdc: *mut std::ffi::c_void, color: u32) -> u32;
    pub fn CreateRoundRectRgn(x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) -> *mut std::ffi::c_void;
}

pub const WS_EX_TOPMOST: u32 = 0x00000008;
pub const WS_EX_TOOLWINDOW: u32 = 0x00000080;
pub const WS_EX_NOACTIVATE: u32 = 0x08000000;
pub const WS_POPUP: u32 = 0x80000000;

pub const HWND_TOPMOST: HWND = -1 as isize as HWND;
pub const SWP_NOSIZE: u32 = 0x0001;
pub const SWP_NOMOVE: u32 = 0x0002;
pub const SWP_NOACTIVATE: u32 = 0x0010;
pub const SWP_SHOWWINDOW: u32 = 0x0040;

pub const DT_CENTER: u32 = 0x00000001;
pub const DT_VCENTER: u32 = 0x00000004;
pub const DT_SINGLELINE: u32 = 0x00000020;

pub const TRANSPARENT: i32 = 1;
pub const SM_CXSCREEN: i32 = 0;
pub const SM_CYSCREEN: i32 = 1;

pub const GA_ROOT: u32 = 2;

/// Get top-level window handle of current active foreground application
pub fn get_active_window_handle() -> usize {
    unsafe {
        let fg = GetForegroundWindow();
        if fg.is_null() {
            return 0;
        }
        let root = GetAncestor(fg, GA_ROOT);
        if root.is_null() {
            fg as usize
        } else {
            root as usize
        }
    }
}

/// Get window title of the given HWND
pub fn get_window_title(hwnd: usize) -> String {
    if hwnd == 0 {
        return String::new();
    }
    unsafe {
        let mut buf = [0u16; 256];
        let len = GetWindowTextW(hwnd as HWND, buf.as_mut_ptr(), buf.len() as i32);
        if len > 0 {
            String::from_utf16_lossy(&buf[..len as usize])
        } else {
            String::new()
        }
    }
}

/// Get window class name of the given HWND
pub fn get_window_class(hwnd: usize) -> String {
    if hwnd == 0 {
        return String::new();
    }
    unsafe {
        let mut buf = [0u16; 128];
        let len = GetClassNameW(hwnd as HWND, buf.as_mut_ptr(), buf.len() as i32);
        if len > 0 {
            String::from_utf16_lossy(&buf[..len as usize])
        } else {
            String::new()
        }
    }
}

#[repr(C)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: *mut std::ffi::c_void,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub uTimeoutOrVersion: u32,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: [u8; 16],
    pub hBalloonIcon: *mut std::ffi::c_void,
}

pub const NIM_ADD: u32 = 0x00000000;
pub const NIM_MODIFY: u32 = 0x00000001;
pub const NIM_DELETE: u32 = 0x00000002;

pub const NIF_MESSAGE: u32 = 0x00000001;
pub const NIF_ICON: u32 = 0x00000002;
pub const NIF_TIP: u32 = 0x00000004;
pub const NIF_INFO: u32 = 0x00000010;

#[link(name = "shell32")]
unsafe extern "system" {
    pub fn Shell_NotifyIconW(dwMessage: u32, lpData: *mut NOTIFYICONDATAW) -> BOOL;
    pub fn ShellExecuteW(
        hwnd: HWND,
        lpOperation: *const u16,
        lpFile: *const u16,
        lpParameters: *const u16,
        lpDirectory: *const u16,
        nShowCmd: i32,
    ) -> HINSTANCE;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn AttachConsole(dwProcessId: u32) -> BOOL;
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

pub fn to_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
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

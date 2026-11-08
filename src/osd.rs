// Modern Floating On-Screen Display (OSD) Badge for Borno (বর্ণ)
// Displays a sleek, minimalist "BN" / "EN" pill next to the text cursor
// without taking focus or disrupting typing. Zero external dependencies.

use std::sync::atomic::{AtomicIsize, Ordering};
use crate::engine::KeyboardMode;
use crate::win32::*;

const BADGE_WIDTH: i32 = 52;
const BADGE_HEIGHT: i32 = 28;
const HIDE_TIMER_ID: usize = 9001;
const HIDE_DELAY_MS: u32 = 600;

pub const WM_SHOW_BADGE: u32 = 0x0400 + 2; // WM_USER + 2

static OSD_HWND: AtomicIsize = AtomicIsize::new(0);
static CURRENT_MODE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

pub fn get_osd_hwnd() -> HWND {
    OSD_HWND.load(Ordering::SeqCst) as HWND
}

pub fn set_osd_hwnd(hwnd: HWND) {
    OSD_HWND.store(hwnd as isize, Ordering::SeqCst);
}

pub unsafe extern "system" fn osd_window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_SHOW_BADGE => {
            let mode_val = w_param as u8;
            CURRENT_MODE.store(mode_val, Ordering::SeqCst);

            // Compute ideal position near text caret or mouse cursor
            let (x, y) = unsafe { compute_badge_position() };

            unsafe {
                // Ensure rounded region (smooth pill shape)
                let rgn = CreateRoundRectRgn(0, 0, BADGE_WIDTH, BADGE_HEIGHT, 12, 12);
                SetWindowRgn(hwnd, rgn, 1);

                // Position and show window without activating
                SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    x,
                    y,
                    BADGE_WIDTH,
                    BADGE_HEIGHT,
                    SWP_NOACTIVATE | SWP_SHOWWINDOW,
                );

                InvalidateRect(hwnd, std::ptr::null(), 1);

                // Auto-hide after delay
                SetTimer(hwnd, HIDE_TIMER_ID, HIDE_DELAY_MS, std::ptr::null());
            }
            0
        }
        0x0113 => { // WM_TIMER
            if w_param == HIDE_TIMER_ID {
                unsafe {
                    KillTimer(hwnd, HIDE_TIMER_ID);
                    ShowWindow(hwnd, SW_HIDE);
                }
            }
            0
        }
        0x000F => { // WM_PAINT
            unsafe {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);

                let mode_val = CURRENT_MODE.load(Ordering::SeqCst);
                let is_bangla = mode_val == 1;

                // 1. Sleek Dark Obsidian Background: RGB(26, 26, 32) -> 0x00201A1A
                let bg_brush = CreateSolidBrush(0x00201A1A);
                let rect = RECT { left: 0, top: 0, right: BADGE_WIDTH, bottom: BADGE_HEIGHT };
                FillRect(hdc, &rect, bg_brush);
                DeleteObject(bg_brush);

                // 2. Select font (Segoe UI for ultra-crisp modern Latin typography)
                let font_name = to_wide_string("Segoe UI");
                let h_font = CreateFontW(
                    17, // Height
                    0, 0, 0,
                    700, // FW_BOLD
                    0, 0, 0,
                    1, // DEFAULT_CHARSET
                    0, 0, 5, 0, // CLEARTYPE_QUALITY
                    font_name.as_ptr(),
                );
                let old_font = SelectObject(hdc, h_font);
                SetBkMode(hdc, TRANSPARENT);

                // 3. Text & Color
                // BN = Emerald/Mint (0x0099D334), EN = Sky Blue (0x00F8BD38)
                let text_color = if is_bangla { 0x0099D334 } else { 0x00F8BD38 };
                SetTextColor(hdc, text_color);

                let badge_text = if is_bangla { "BN" } else { "EN" };
                let w_text = to_wide_string(badge_text);
                let mut draw_rc = rect;

                DrawTextW(
                    hdc,
                    w_text.as_ptr(),
                    -1,
                    &mut draw_rc,
                    DT_CENTER | DT_VCENTER | DT_SINGLELINE,
                );

                // Cleanup
                SelectObject(hdc, old_font);
                DeleteObject(h_font);

                EndPaint(hwnd, &ps);
            }
            0
        }
        0x0002 => { // WM_DESTROY
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
    }
}

pub fn init_osd() -> HWND {
    unsafe {
        let class_name = to_wide_string("BornoOSDClass");
        let window_name = to_wide_string("Borno OSD Window");
        let h_instance = GetModuleHandleW(std::ptr::null());

        let mut wnd_class: WNDCLASSEXW = std::mem::zeroed();
        wnd_class.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wnd_class.lpfnWndProc = Some(osd_window_proc);
        wnd_class.hInstance = h_instance;
        wnd_class.lpszClassName = class_name.as_ptr();

        RegisterClassExW(&wnd_class);

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_POPUP,
            0,
            0,
            BADGE_WIDTH,
            BADGE_HEIGHT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        if !hwnd.is_null() {
            set_osd_hwnd(hwnd);
        }

        hwnd
    }
}

/// Request OSD badge display for given mode
pub fn show_mode_badge(mode: KeyboardMode) {
    let hwnd = get_osd_hwnd();
    if !hwnd.is_null() {
        let mode_val = match mode {
            KeyboardMode::English => 0usize,
            KeyboardMode::Bangla => 1usize,
        };
        unsafe {
            PostMessageW(hwnd, WM_SHOW_BADGE, mode_val, 0);
        }
    }
}

#[link(name = "user32")]
unsafe extern "system" {
    fn PostMessageW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> BOOL;
}

unsafe fn compute_badge_position() -> (i32, i32) {
    unsafe {
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);

        // 1. Try to get caret position from foreground window's GUI thread
        let fg = GetForegroundWindow();
        if !fg.is_null() {
            let thread_id = GetWindowThreadProcessId(fg, std::ptr::null_mut());
            let mut gui: GUITHREADINFO = std::mem::zeroed();
            gui.cbSize = std::mem::size_of::<GUITHREADINFO>() as u32;

            if GetGUIThreadInfo(thread_id, &mut gui) != 0 && !gui.hwndCaret.is_null() {
                let mut pt = POINT {
                    x: gui.rcCaret.left,
                    y: gui.rcCaret.bottom + 6,
                };
                if ClientToScreen(gui.hwndCaret, &mut pt) != 0 && pt.x > 0 && pt.y > 0 && pt.x < screen_w && pt.y < screen_h {
                    let x = pt.x.clamp(10, screen_w - BADGE_WIDTH - 10);
                    let y = pt.y.clamp(10, screen_h - BADGE_HEIGHT - 10);
                    return (x, y);
                }
            }
        }

        // 2. Fallback: Position slightly below-right of mouse cursor
        let mut cursor: POINT = std::mem::zeroed();
        GetCursorPos(&mut cursor);

        let x = (cursor.x + 14).clamp(10, screen_w - BADGE_WIDTH - 10);
        let y = (cursor.y + 18).clamp(10, screen_h - BADGE_HEIGHT - 10);
        (x, y)
    }
}

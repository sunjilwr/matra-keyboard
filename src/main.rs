// Matra Bangla & English System-wide Keyboard
// Cross-platform Pure Rust implementation for Windows, Linux & macOS

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod keys;
mod config;
mod layout;
mod engine;

#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
mod hook;
#[cfg(target_os = "windows")]
mod tray;
#[cfg(target_os = "windows")]
mod osd;

#[cfg(target_os = "linux")]
mod platform_linux;

#[cfg(target_os = "macos")]
mod platform_macos;

use config::{AppConfig, ShortcutKey, TypingStyle};

fn main() {
    let config = AppConfig::load_or_create();

    #[cfg(target_os = "windows")]
    run_windows(config);

    #[cfg(target_os = "linux")]
    platform_linux::run_linux(config);

    #[cfg(target_os = "macos")]
    platform_macos::run_macos(config);
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn ctrl_handler(_ctrl_type: u32) -> win32::BOOL {
    hook::uninstall_hook();
    1 // TRUE (handled)
}

#[cfg(target_os = "windows")]
fn run_windows(config: AppConfig) {
    use win32::*;
    use hook::{install_hook, uninstall_hook, print_mode_banner, ENGINE, CONFIG};
    use tray::init_tray;

    // If run from a command prompt or PowerShell, attach to its console for logging
    unsafe {
        AttachConsole(0xFFFFFFFF); // ATTACH_PARENT_PROCESS
    }

    // Enable ANSI colors if running in a console
    enable_ansi_colors();

    // Apply config to typing engine and window manager
    {
        let mut engine = ENGINE.lock().unwrap();
        engine.sound_enabled = config.sound_enabled;
        engine.typing_style = config.typing_style;
    }
    {
        let mut wsm = hook::WINDOW_STATE_MGR.lock().unwrap();
        wsm.enabled = config.per_window_mode;
    }

    print_welcome_banner(&config);

    // Save active config in hook
    {
        let mut cfg_guard = CONFIG.lock().unwrap();
        *cfg_guard = Some(config);
    }

    unsafe {
        SetConsoleCtrlHandler(Some(ctrl_handler), 1);
    }

    // Install global low-level keyboard hook
    if let Err(e) = install_hook() {
        eprintln!("\x1b[1;31m[ERROR]\x1b[0m {}", e);
        return;
    }

    let initial_mode = {
        let engine = ENGINE.lock().unwrap();
        engine.mode
    };
    print_mode_banner(initial_mode);

    // Initialize On-Screen Display (OSD) Floating Badge
    osd::init_osd();

    // Initialize System Tray Icon (notification area)
    let tray_hwnd = init_tray(initial_mode);

    // Enter standard Windows message dispatch loop
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    // Cleanup tray and hook
    if !tray_hwnd.is_null() {
        tray::remove_tray_icon(tray_hwnd);
    }
    uninstall_hook();
}

fn print_welcome_banner(config: &AppConfig) {
    let shortcut_display = match config.shortcut {
        ShortcutKey::CtrlBackslash => "Ctrl + \\ (Recommended)",
        ShortcutKey::CtrlF12 => "Ctrl + F12",
        ShortcutKey::F12 => "F12 (Avro legacy)",
        ShortcutKey::AltBackslash => "Alt + \\",
        ShortcutKey::CtrlSpace => "Ctrl + Space",
    };

    let typing_style_display = match config.typing_style {
        TypingStyle::Classic => "Classic Visual (ি+ক = কি & ক+ি = কি)",
        TypingStyle::Modern => "Modern Unicode (ক+ি = কি)",
    };

    println!("\x1b[1;36m====================================================================\x1b[0m");
    println!("\x1b[1;32m  ⌨️   Matra (মাত্রা) - Native Bangla & English Keyboard (Rust)\x1b[0m");
    println!("\x1b[1;36m====================================================================\x1b[0m");
    println!("  \x1b[1mStatus:\x1b[0m System-wide Keyboard Active across ALL applications!");
    println!("  \x1b[1mActive Shortcut:\x1b[0m \x1b[1;33m{}\x1b[0m (Change anytime in config.ini)", shortcut_display);
    println!("  \x1b[1mTyping Style:\x1b[0m    \x1b[1;36m{}\x1b[0m", typing_style_display);
    println!("  \x1b[1mAudio Feedback:\x1b[0m  {}", if config.sound_enabled { "\x1b[1;32mEnabled (Beep)\x1b[0m" } else { "\x1b[90mDisabled\x1b[0m" });
    println!("  \x1b[1mOn-Screen Badge:\x1b[0m {}", if config.osd_badge { "\x1b[1;32mEnabled (Floating BN/EN near cursor)\x1b[0m" } else { "\x1b[90mDisabled\x1b[0m" });
    println!("  \x1b[1mPer-App Language:\x1b[0m {}", if config.per_window_mode { "\x1b[1;32mEnabled (Auto-remembers per app)\x1b[0m" } else { "\x1b[90mDisabled (Global)\x1b[0m" });
    println!();
    println!("  \x1b[1mBangla Quick Typing Cheat-Sheet:\x1b[0m");
    println!("    • \x1b[36mস্বরবর্ণ:\x1b[0m Shift+F=অ, g+f=আ, g+d=ই, g+D=ঈ, g+s=উ, g+S=ঊ, g+a=ঋ, g+c=এ, g+C=ঐ, g+x=ও, g+X=ঔ");
    println!("    • \x1b[36mকার চিহ্ন:\x1b[0m f=া, d=ি, D=ী, s=ু, S=ূ, a=ৃ, c=ে, C=ৈ, x=ো, X=ৌ");
    println!("    • \x1b[36mযুক্তাক্ষর:\x1b[0m ক + g + ক = ক্ক, ত + g + ত = ত্ত, ব + g + ব = ব্ব, ক + z = ক্র, ক + Z = ক্য");
    println!("    • \x1b[36mরেফ (র্):\x1b[0m   Shift+A (যেমন: j+m+Shift+A = কর্ম)");
    println!("    • \x1b[36mদাঁড়ি (।):\x1b[0m  Shift+G");
    println!("    • \x1b[36mটাকা (৳):\x1b[0m  Shift+4");
    println!("    • \x1b[36mখন্ড-ত (ৎ):\x1b[0m Shift+7");
    println!("    • \x1b[36mZWNJ / ZWJ:\x1b[0m ` (tilde key) / ~ (Shift + tilde key)");
    println!("\x1b[1;36m====================================================================\x1b[0m");
    println!("  \x1b[90mSettings can be modified in 'config.ini'. Press Ctrl+C here to exit.\x1b[0m");
}

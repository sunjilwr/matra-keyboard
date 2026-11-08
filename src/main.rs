// UniBijoy & English System-wide Keyboard for Windows
// Written in Pure Rust with zero external dependencies.

mod win32;
mod config;
mod layout;
mod engine;
mod hook;

use config::{AppConfig, ShortcutKey, TypingStyle};
use win32::*;
use hook::{install_hook, uninstall_hook, print_mode_banner, ENGINE, CONFIG};

unsafe extern "system" fn ctrl_handler(_ctrl_type: u32) -> BOOL {
    uninstall_hook();
    1 // TRUE (handled)
}

fn main() {
    // Enable ANSI colors in Windows console
    enable_ansi_colors();

    // Load or create user configuration (config.ini)
    let config = AppConfig::load_or_create();

    // Apply config to typing engine
    {
        let mut engine = ENGINE.lock().unwrap();
        engine.sound_enabled = config.sound_enabled;
        engine.typing_style = config.typing_style;
    }

    let start_hidden = config.start_hidden;

    print_welcome_banner(&config);

    // Save active config in hook
    {
        let mut cfg_guard = CONFIG.lock().unwrap();
        *cfg_guard = Some(config);
    }

    let args: Vec<String> = std::env::args().collect();
    let is_hidden_flag = args.iter().any(|a| a == "--hidden" || a == "--tray" || a == "-h");

    if start_hidden || is_hidden_flag {
        hide_console_window();
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

    // Enter standard Windows message dispatch loop
    unsafe {
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
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
        TypingStyle::Classic => "Classic Bijoy (ি+ক = কি & ক+ি = কি)",
        TypingStyle::Modern => "Modern Unicode (ক+ি = কি)",
    };

    println!("\x1b[1;35m====================================================================\x1b[0m");
    println!("\x1b[1;32m  ⌨️   UniBijoy & English Native Keyboard (Pure Rust)\x1b[0m");
    println!("\x1b[1;35m====================================================================\x1b[0m");
    println!("  \x1b[1mStatus:\x1b[0m System-wide Keyboard Active across ALL applications!");
    println!("  \x1b[1mActive Shortcut:\x1b[0m \x1b[1;33m{}\x1b[0m (Change anytime in config.ini)", shortcut_display);
    println!("  \x1b[1mTyping Style:\x1b[0m    \x1b[1;36m{}\x1b[0m", typing_style_display);
    println!("  \x1b[1mAudio Feedback:\x1b[0m  {}", if config.sound_enabled { "\x1b[1;32mEnabled (Beep)\x1b[0m" } else { "\x1b[90mDisabled\x1b[0m" });
    println!();
    println!("  \x1b[1mUniBijoy Quick Typing Cheat-Sheet:\x1b[0m");
    println!("    • \x1b[36mস্বরবর্ণ:\x1b[0m Shift+F=অ, g+f=আ, g+d=ই, g+D=ঈ, g+s=উ, g+S=ঊ, g+a=ঋ, g+c=এ, g+C=ঐ, g+x=ও, g+X=ঔ");
    println!("    • \x1b[36mকার চিহ্ন:\x1b[0m f=া, d=ি, D=ী, s=ু, S=ূ, a=ৃ, c=ে, C=ৈ, x=ো, X=ৌ");
    println!("    • \x1b[36mযুক্তাক্ষর:\x1b[0m ক + g + ক = ক্ক, ত + g + ত = ত্ত, ব + g + ব = ব্ব, ক + z = ক্র, ক + Z = ক্য");
    println!("    • \x1b[36mরেফ (র্):\x1b[0m   Shift+A (যেমন: Shift+A + m = র্ম)");
    println!("    • \x1b[36mদাঁড়ি (।):\x1b[0m  Shift+G");
    println!("    • \x1b[36mটাকা (৳):\x1b[0m  Shift+4");
    println!("    • \x1b[36mখন্ড-ত (ৎ):\x1b[0m Shift+7");
    println!("    • \x1b[36mZWNJ / ZWJ:\x1b[0m ` (tilde key) / ~ (Shift + tilde key)");
    println!("\x1b[1;35m====================================================================\x1b[0m");
    println!("  \x1b[90mSettings can be modified in 'config.ini'. Press Ctrl+C here to exit.\x1b[0m");
}

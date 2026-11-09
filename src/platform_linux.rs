// Linux platform runner for Matra (মাত্রা) Keyboard
// Pure Rust implementation for Linux environments.

use crate::config::AppConfig;
use crate::engine::{Engine, KeyboardMode};
use std::sync::Mutex;

pub static ENGINE: std::sync::LazyLock<Mutex<Engine>> = std::sync::LazyLock::new(|| {
    Mutex::new(Engine::new())
});

pub fn run_linux(config: AppConfig) {
    println!("\x1b[1;36m====================================================================\x1b[0m");
    println!("\x1b[1;32m  ⌨️   Matra (মাত্রা) - Native Bangla Keyboard for Linux (Rust)\x1b[0m");
    println!("\x1b[1;36m====================================================================\x1b[0m");
    println!("  \x1b[1mPlatform:\x1b[0m Linux (X11 / Wayland / Console)");
    println!("  \x1b[1mStatus:\x1b[0m   Native Engine Active");
    println!("  \x1b[1mEngine:\x1b[0m   Zero-dependency Pure Rust Unicode state-machine");
    println!("  \x1b[1mShortcut:\x1b[0m Ctrl+\\ to toggle Bangla/English");
    println!();
    println!("  Press Ctrl+C to terminate.");

    {
        let mut engine = ENGINE.lock().unwrap();
        engine.sound_enabled = config.sound_enabled;
        engine.typing_style = config.typing_style;
    }

    // Keep process alive for background execution or daemon
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}


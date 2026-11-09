pub mod keys;
pub mod config;
pub mod layout;
pub mod engine;

#[cfg(target_os = "windows")]
pub mod win32;
#[cfg(target_os = "windows")]
pub mod hook;
#[cfg(target_os = "windows")]
pub mod tray;
#[cfg(target_os = "windows")]
pub mod osd;

#[cfg(target_os = "linux")]
pub mod platform_linux;

#[cfg(target_os = "macos")]
pub mod platform_macos;

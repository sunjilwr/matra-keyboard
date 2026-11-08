// Configuration management for bnkeyboard
// Reads and writes user preferences to a simple, human-editable 'config.ini' file.

use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum ShortcutKey {
    CtrlBackslash,  // Ctrl + \ (Default)
    CtrlF12,        // Ctrl + F12
    F12,            // F12 alone (Classic Avro style)
    AltBackslash,   // Alt + \
    CtrlSpace,      // Ctrl + Space
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypingStyle {
    /// Modern Unicode style: Consonant followed by Kar (ক + ি = কি)
    Modern,
    /// Classic Bijoy style: Supports typing Kar before Consonant (ি + ক = কি) as well as Modern
    Classic,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub shortcut: ShortcutKey,
    pub sound_enabled: bool,
    pub typing_style: TypingStyle,
    pub start_hidden: bool,
    pub per_window_mode: bool,
    pub osd_badge: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            shortcut: ShortcutKey::CtrlBackslash,
            sound_enabled: false, // Beep disabled by default in favor of modern OSD badge
            typing_style: TypingStyle::Classic, // By default allow both classic (ি+ক) and modern (ক+ি)
            start_hidden: true,
            per_window_mode: true, // Independent keyboard state per application window
            osd_badge: true, // Sleek floating "বাংলা" / "EN" badge near cursor
        }
    }
}

impl AppConfig {
    const CONFIG_FILE: &'static str = "config.ini";

    /// Load config from file or create default if not present
    pub fn load_or_create() -> Self {
        if !Path::new(Self::CONFIG_FILE).exists() {
            let default_cfg = Self::default();
            default_cfg.save_to_file();
            return default_cfg;
        }

        let mut config = Self::default();
        if let Ok(content) = fs::read_to_string(Self::CONFIG_FILE) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                    continue;
                }
                if let Some((key, val)) = trimmed.split_once('=') {
                    let k = key.trim().to_lowercase();
                    let v = val.trim();
                    match k.as_str() {
                        "shortcut" => {
                            config.shortcut = match v.to_lowercase().as_str() {
                                "ctrl+f12" | "ctrl-f12" => ShortcutKey::CtrlF12,
                                "f12" => ShortcutKey::F12,
                                "alt+\\" | "alt-\\" => ShortcutKey::AltBackslash,
                                "ctrl+space" | "ctrl-space" => ShortcutKey::CtrlSpace,
                                _ => ShortcutKey::CtrlBackslash,
                            };
                        }
                        "sound" => {
                            config.sound_enabled = !matches!(v.to_lowercase().as_str(), "false" | "0" | "no" | "off");
                        }
                        "typing_style" => {
                            config.typing_style = match v.to_lowercase().as_str() {
                                "modern" => TypingStyle::Modern,
                                _ => TypingStyle::Classic,
                            };
                        }
                        "start_hidden" | "hidden" => {
                            config.start_hidden = matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on");
                        }
                        "per_window" | "per_window_mode" | "independent_window" => {
                            config.per_window_mode = matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on");
                        }
                        "osd" | "osd_badge" | "badge" | "indicator" => {
                            config.osd_badge = matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on");
                        }
                        _ => {}
                    }
                }
            }
        }
        config
    }

    /// Save current configuration to config.ini
    pub fn save_to_file(&self) {
        let shortcut_str = match self.shortcut {
            ShortcutKey::CtrlBackslash => "Ctrl+\\",
            ShortcutKey::CtrlF12 => "Ctrl+F12",
            ShortcutKey::F12 => "F12",
            ShortcutKey::AltBackslash => "Alt+\\",
            ShortcutKey::CtrlSpace => "Ctrl+Space",
        };

        let typing_style_str = match self.typing_style {
            TypingStyle::Modern => "Modern",
            TypingStyle::Classic => "Classic",
        };

        let content = format!(
r#"# ====================================================================
# bnkeyboard Configuration File (ইউজার কনফিগারেশন)
# ====================================================================

# Language switch shortcut (English <-> UniBijoy)
# Available options:
#   Ctrl+\       (Recommended - Zero conflicts with other apps)
#   Ctrl+F12     (Safe function key)
#   F12          (Avro legacy style)
#   Alt+\        (Alt + Backslash)
#   Ctrl+Space   (Note: may conflict with code editors)
shortcut = {}

# Audio beep confirmation when switching language (true / false)
sound = {}

# Typing style:
#   Classic = Supports typing pre-base Kars before Consonant (ি + ক = কি) as well as Modern
#   Modern  = Pure Unicode style (ক + ি = কি)
typing_style = {}

# Run silently in background without console window (true / false)
start_hidden = {}

# Remember language independently per application window (true / false)
#   true  = Each app remembers its own language (e.g. Word=বাংলা, Chrome=English).
#           New apps safely open in English. (Recommended)
#   false = Single global mode across all applications.
per_window_mode = {}

# Sleek on-screen floating "বাংলা" / "EN" badge near typing cursor (true / false)
osd_badge = {}
"#,
            shortcut_str,
            if self.sound_enabled { "true" } else { "false" },
            typing_style_str,
            if self.start_hidden { "true" } else { "false" },
            if self.per_window_mode { "true" } else { "false" },
            if self.osd_badge { "true" } else { "false" }
        );

        let _ = fs::write(Self::CONFIG_FILE, content);
    }
}


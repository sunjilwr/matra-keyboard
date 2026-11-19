// Typing Engine for Matra Bangla
// Handles stateful typing: Link key ('g'), conjuncts, vowel forming, Classic/Modern Kar typing,
// Reph after consonant, and key interception.

use crate::config::TypingStyle;
use crate::keys::*;
use crate::layout::{
    get_pre_base_kar, is_bengali_consonant, is_bengali_kar, map_link_vowel, map_bangla_key,
    KeyOutput, KeyShiftState,
};
#[cfg(target_os = "windows")]
use crate::win32::{Beep, send_backspaces, send_unicode_string, turn_caps_lock_off};

#[cfg(not(target_os = "windows"))]
pub fn send_unicode_string(text: &str) {
    print!("{}", text);
}

#[cfg(not(target_os = "windows"))]
pub fn send_backspaces(count: usize) {
    for _ in 0..count {
        print!("\x08 \x08");
    }
}

#[cfg(not(target_os = "windows"))]
pub fn turn_caps_lock_off() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardMode {
    English,
    Bangla,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowContext {
    /// Standard applications (Browser, Word, Messenger, Notepad, etc.)
    Normal,
    /// Code editors & IDEs (Antigravity IDE, VS Code, Cursor, Android Studio, etc.)
    CodingIde,
    /// Command-line terminals (Windows Terminal, PowerShell, CMD, Git Bash, etc.)
    Terminal,
}

/// Normalize a window title by stripping dirty file markers ('●', '*', '•') and outer whitespace
/// so that modifying an unsaved file doesn't trigger a false tab-switch event.
pub fn normalize_window_title(title: &str) -> String {
    title
        .chars()
        .filter(|&c| c != '●' && c != '•' && c != '*')
        .collect::<String>()
        .trim()
        .to_string()
}

/// Classify a window as Normal, CodingIde, or Terminal based on its title and Win32 class name.
pub fn classify_window_context(title: &str, class_name: &str) -> WindowContext {
    let t = title.to_lowercase();
    let c = class_name.to_lowercase();

    // 1. Check for Terminal emulators
    if c.contains("cascadia_hosting_window_class")
        || c == "consolewindowclass"
        || c == "mintty"
        || c.contains("wezterm")
        || c.contains("alacritty")
        || c == "putty"
        || t.contains("windows powershell")
        || t.contains("command prompt")
        || t.contains("cmd.exe")
        || t.contains("pwsh.exe")
        || t.contains("mingw64")
        || t.contains("git bash")
    {
        return WindowContext::Terminal;
    }

    // 2. Check for Coding IDEs and Code Editors
    if c.contains("sunawtframe") // JetBrains IDEs (Android Studio, IntelliJ, PyCharm, WebStorm, etc.)
        || c == "notepad++"
        || c == "px_window_class" // Sublime Text
        || t.contains("antigravity")
        || t.contains("visual studio code")
        || t.contains("vs code")
        || t.contains("vscodium")
        || t.contains(" - cursor")
        || t.starts_with("cursor - ")
        || t.contains("windsurf")
        || t.contains("android studio")
        || t.contains("intellij idea")
        || t.contains("pycharm")
        || t.contains("webstorm")
        || t.contains("phpstorm")
        || t.contains("goland")
        || t.contains("clion")
        || t.contains("rustrover")
        || t.contains("microsoft visual studio")
        || t.contains("sublime text")
        || t.contains("notepad++")
        || t.contains(" - zed")
        || t.contains("neovim")
        || has_code_file_in_title(&t)
    {
        return WindowContext::CodingIde;
    }

    WindowContext::Normal
}

fn has_code_file_in_title(lower_title: &str) -> bool {
    const CODE_EXTS: &[&str] = &[
        ".dart", ".yaml", ".yml", ".json", ".toml", ".rs", ".py", ".js", ".ts", ".tsx", ".jsx",
        ".java", ".kt", ".kts", ".gradle", ".xml", ".html", ".css", ".scss", ".c", ".cpp", ".h",
        ".hpp", ".cs", ".go", ".php", ".rb", ".sql", ".sh", ".bat", ".ps1", ".vue", ".svelte",
        ".astro", ".lock", ".ini", ".iss",
    ];
    for ext in CODE_EXTS {
        if let Some(idx) = lower_title.find(ext) {
            let after = &lower_title[idx + ext.len()..];
            if after.is_empty()
                || after.starts_with(' ')
                || after.starts_with('-')
                || after.starts_with('—')
                || after.starts_with('•')
                || after.starts_with('●')
                || after.starts_with('*')
                || after.starts_with(')')
                || after.starts_with(']')
            {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastTyped {
    None,
    /// Single consonant (e.g. "ম", "ক", "ফ")
    Consonant(&'static str),
    /// Consonant cluster formed by fola (ro-fola, jo-fola) or conjunct (e.g. "ফ্র", "ক্র")
    ConsonantCluster,
    /// A conjunct ending in 'ট' or 'ড' (e.g. স্ট, ন্ট, ল্ট, প্ট, ন্ড, ল্ড) where Windows'
    /// Nirmala UI font breaks the conjunct ligature if ু/ূ is attached without ZWJ (\u{200D}).
    ConjunctNeedingZwjBeforeUKar,
    /// A consonant where a pre-base Kar was attached via Classic visual typing (e.g. "ফে" from "c" + "Shift+R")
    ConsonantWithPendingKar {
        cons: &'static str,
        kar: &'static str,
    },
    /// Vowel sign / Kar (e.g. ি, ী, ু, ূ, ৃ, ে, ৈ, ো, ৌ, া)
    Kar,
    /// A ু/ূ Kar that was preceded by an automatic ZWJ (\u{200D}) to preserve conjunct shaping
    KarWithZwj,
    Other,
}

impl LastTyped {
    pub fn is_consonant_base(&self) -> bool {
        matches!(
            self,
            LastTyped::Consonant(_)
                | LastTyped::ConsonantCluster
                | LastTyped::ConjunctNeedingZwjBeforeUKar
                | LastTyped::ConsonantWithPendingKar { .. }
        )
    }
}

pub struct Engine {
    pub mode: KeyboardMode,
    pub link_active: bool,
    pub pending_kar: Option<&'static str>,
    pub last_typed: LastTyped,
    pub sound_enabled: bool,
    pub typing_style: TypingStyle,
    pub smart_coding_mode: bool,
    pub window_context: WindowContext,
    pub bangla_chars_typed: usize,
}

impl Engine {
    pub const fn new() -> Self {
        Self {
            mode: KeyboardMode::English,
            link_active: false,
            pending_kar: None,
            last_typed: LastTyped::None,
            sound_enabled: false, // Default off as user requested (OSD badge used instead)
            typing_style: TypingStyle::Classic,
            smart_coding_mode: true,
            window_context: WindowContext::Normal,
            bangla_chars_typed: 0,
        }
    }

    /// Toggle between English and Bangla modes
    pub fn toggle_mode(&mut self) -> KeyboardMode {
        self.reset_state();
        self.bangla_chars_typed = 0;

        self.mode = match self.mode {
            KeyboardMode::English => {
                turn_caps_lock_off();
                KeyboardMode::Bangla
            }
            KeyboardMode::Bangla => KeyboardMode::English,
        };

        // Play crisp, subtle confirmation beep
        #[cfg(target_os = "windows")]
        if self.sound_enabled {
            match self.mode {
                KeyboardMode::Bangla => {
                    // High pleasant tone (880 Hz, 80 ms)
                    unsafe { Beep(880, 80); }
                }
                KeyboardMode::English => {
                    // Lower pleasant tone (440 Hz, 80 ms)
                    unsafe { Beep(440, 80); }
                }
            }
        }

        self.mode
    }

    /// Set mode explicitly (e.g. when switching between applications).
    /// Does not play audio beep, making window switches smooth and silent.
    pub fn set_mode(&mut self, mode: KeyboardMode) {
        if self.mode != mode {
            self.reset_state();
            self.bangla_chars_typed = 0;
            self.mode = mode;
            if mode == KeyboardMode::Bangla {
                turn_caps_lock_off();
            }
        }
    }

    pub fn reset_state(&mut self) {
        self.link_active = false;
        self.pending_kar = None;
        self.last_typed = LastTyped::None;
    }

    /// Handle a key down event.
    /// Returns `true` if the event was handled and should be suppressed from the OS.
    #[allow(dead_code)]
    pub fn handle_key_down(&mut self, vk: u32, is_shift: bool, is_alt: bool, is_ctrl: bool) -> bool {
        self.handle_key_down_with_modifiers(vk, is_shift, is_alt, is_ctrl, false)
    }

    /// Handle a key down event including Windows/Super key state.
    /// Returns `true` if the event was handled and should be suppressed from the OS.
    pub fn handle_key_down_with_modifiers(
        &mut self,
        vk: u32,
        is_shift: bool,
        is_alt: bool,
        is_ctrl: bool,
        is_win: bool,
    ) -> bool {
        // If in English mode, do nothing - 100% passthrough
        if self.mode == KeyboardMode::English {
            return false;
        }

        // Ignore modifier keys alone (Shift, Ctrl, Alt, CapsLock, Win).
        // Crucial: They must NEVER reset last_typed, because user presses Shift BEFORE pressing 'A'!
        if is_modifier_key(vk) {
            return false;
        }

        // Allow system shortcuts (Ctrl+C, Ctrl+V, Win+V, Win+Key, Alt+Tab, etc.) to pass through untouched
        let is_alt_gr = is_ctrl && is_alt;
        if is_win || (is_ctrl && !is_alt_gr) || (is_alt && !is_alt_gr) {
            if is_win {
                self.reset_state();
            } else {
                self.flush_pending_states();
                self.last_typed = LastTyped::Other;

                // In Smart Coding Mode: IDE navigation/file shortcuts (Ctrl+S, Ctrl+P, Ctrl+F, Ctrl+G,
                // Ctrl+Tab, Ctrl+/, Ctrl+B, Ctrl+J, Ctrl+~) auto-switch back to English,
                // while clipboard/edit shortcuts (Ctrl+C, Ctrl+V, Ctrl+X, Ctrl+Z, Ctrl+Y, Ctrl+A) keep Bangla.
                if self.smart_coding_mode
                    && self.window_context != WindowContext::Normal
                    && is_ctrl
                    && !is_alt
                    && is_ide_navigation_shortcut(vk)
                {
                    self.set_mode(KeyboardMode::English);
                }
            }
            return false;
        }

        // Backspace handling
        if vk == VK_BACK as u32 {
            if self.link_active {
                self.link_active = false;
                return true;
            }
            if self.pending_kar.is_some() {
                self.pending_kar = None;
                return true;
            }
            if self.last_typed == LastTyped::KarWithZwj {
                // Remove the automatic ZWJ (\u{200D}) along with the u-kar
                send_backspaces(1);
            }
            self.last_typed = LastTyped::None;
            return false; // Normal backspace passes through
        }

        // Smart Coding Mode inside Coding IDEs & Terminals:
        let is_coding_ctx = self.smart_coding_mode && self.window_context != WindowContext::Normal;
        if is_coding_ctx {
            // 1. Escape / PageUp (0x21) / PageDown (0x22): editor navigation -> auto-switch to English!
            if vk == VK_ESCAPE as u32 || vk == 0x21 || vk == 0x22 {
                self.flush_pending_states();
                self.set_mode(KeyboardMode::English);
                return false;
            }

            // 2. Numbers (0-9 and Numpad 0-9): always output English digits in Coding IDEs
            //    (e.g. version: 1.0.9+13, ports, array indices never become ০-৯!)
            let is_top_row_digit = !is_shift && (0x30..=0x39).contains(&vk);
            let is_numpad_digit = (VK_NUMPAD0 as u32..=VK_NUMPAD9 as u32).contains(&vk);
            if is_top_row_digit || is_numpad_digit {
                self.flush_pending_states();
                self.last_typed = LastTyped::Other;
                // If user hasn't typed Bangla prose yet (directly editing a number in code), auto-switch to English!
                if self.bangla_chars_typed == 0 {
                    self.set_mode(KeyboardMode::English);
                }
                return false;
            }

            // 3. Code syntax & operators ({ } [ ] ; = + < > \ | ` ~ /):
            //    Output English symbol directly and auto-switch to English for coding!
            if is_code_syntax_key(vk, is_shift) {
                self.flush_pending_states();
                self.set_mode(KeyboardMode::English);
                return false;
            }
        }

        // Non-character keys (Enter, Tab, Escape, Arrow keys, etc.)
        if vk == VK_RETURN as u32
            || vk == VK_TAB as u32
            || vk == VK_ESCAPE as u32
            || (vk >= 0x25 && vk <= 0x28) // Arrows
        {
            self.flush_pending_states();
            self.last_typed = LastTyped::None;
            return false;
        }

        // Determine shift state
        let shift_state = if is_alt_gr {
            if is_shift {
                KeyShiftState::ShiftAltGr
            } else {
                KeyShiftState::AltGr
            }
        } else if is_shift {
            KeyShiftState::Shift
        } else {
            KeyShiftState::Normal
        };

        // ====================================================================
        // 1. REPH AFTER CONSONANT FIX: 'কমর্' -> 'কর্ম'
        // In traditional typing habit, users frequently press consonant (e.g. 'ম' = m) THEN Reph ('Shift+A').
        // If Shift+A is pressed immediately after a consonant, replace 'C' with 'র্' + 'C'.
        // ====================================================================
        if vk == 0x41 && is_shift {
            // Shift + A = Reph (র্)
            if let LastTyped::Consonant(cons) = self.last_typed {
                // Erase the preceding consonant from screen
                send_backspaces(1);
                // Send Reph (\u{09B0}\u{09CD}) followed by the consonant
                send_unicode_string("\u{09B0}\u{09CD}");
                send_unicode_string(cons);
                self.last_typed = LastTyped::ConsonantCluster;
                return true;
            }
        }

        // ====================================================================
        // 2. FOLA ATTACHMENT: 'z' (ro-fola ্র), 'Shift+Z' (jo-fola ্য)
        // ====================================================================
        if vk == 0x5A {
            let fola = if is_shift { "\u{09CD}\u{09AF}" } else { "\u{09CD}\u{09B0}" };

            // Classic visual typing order fix:
            // User typed: 'c' (ে) + 'Shift+R' (ফ) -> screen has 'ফে'
            // Now user presses 'z' (ro-fola ্র) -> should become 'ফ্রে'!
            if let LastTyped::ConsonantWithPendingKar { cons: _, kar } = self.last_typed {
                send_backspaces(1); // Erase the pending kar ('ে')
                send_unicode_string(fola); // Send fola (্র / ্য)
                send_unicode_string(kar); // Re-send the kar ('ে')
                self.last_typed = LastTyped::Kar;
                return true;
            }

            // Modern phonetic typing order:
            // User typed consonant base (e.g. 'ফ') -> now presses 'z' (্র) -> outputs '্র'
            // and marks last_typed as ConsonantCluster so subsequent Kar attaches to it!
            if self.last_typed.is_consonant_base() {
                send_unicode_string(fola);
                self.last_typed = LastTyped::ConsonantCluster;
                return true;
            }
        }

        // ====================================================================
        // 3. LINK KEY 'g' ACTIVE: 'g + Kar' -> Vowel, or 'g + Consonant' -> Conjunct
        // ====================================================================
        let mut was_conjunct_link = false;
        if self.link_active {
            // Check if this key forms a Swaroborno (Vowel) with 'g' (e.g. g+f=আ, g+d=ই, g+c=এ)
            if let Some(vowel) = map_link_vowel(vk, is_shift) {
                self.link_active = false;
                send_unicode_string(vowel);
                self.last_typed = LastTyped::Other;
                return true;
            }

            // If user presses 'g' again, commit Hasanta '্'
            if vk == 0x47 && !is_shift {
                send_unicode_string("\u{09CD}");
                self.last_typed = LastTyped::Other;
                return true;
            }

            // Otherwise, this is a conjunct (e.g. ক + g + ক = ক্ক, স + g + ট = স্ট)
            self.link_active = false;
            was_conjunct_link = true;
        }

        // ====================================================================
        // 4. CLASSIC VISUAL TYPING SUPPORT: 'ি + ক' -> 'কি'
        // If typing_style == Classic and this key is a pre-base kar (ি, ে, ৈ, ো, ৌ),
        // and last_typed was NOT a consonant base (i.e. visual order at start of syllable):
        // ====================================================================
        let is_after_consonant = self.last_typed.is_consonant_base();
        if !was_conjunct_link && self.typing_style == TypingStyle::Classic && !is_after_consonant {
            if let Some(kar) = get_pre_base_kar(vk, is_shift) {
                self.pending_kar = Some(kar);
                return true; // buffer it, waiting for the consonant
            }
        }

        // ====================================================================
        // 5. STANDARD BANGLA KEY MAPPING
        // ====================================================================
        match map_bangla_key(vk, shift_state) {
            KeyOutput::Text(text) => {
                self.bangla_chars_typed = self.bangla_chars_typed.saturating_add(1);
                let is_cons = is_bengali_consonant(text);

                // Case A: Forming a conjunct via Link Key 'g' (C1 + g + C2)
                if was_conjunct_link {
                    if is_cons {
                        // If C1 had a Classic pre-base kar (e.g. user typed 'ে' + 'স' -> 'সে', then 'g' + 'ট'),
                        // reorder it so the kar moves after the entire conjunct ('স্টে')!
                        if let LastTyped::ConsonantWithPendingKar { cons: _, kar } = self.last_typed {
                            send_backspaces(1); // Erase 'ে' after C1
                            send_unicode_string("\u{09CD}"); // Send Hasanta '্'
                            send_unicode_string(text); // Send C2 (e.g. 'ট')
                            send_unicode_string(kar); // Re-send 'ে' after C2 -> 'স্টে'
                            self.last_typed = LastTyped::ConsonantWithPendingKar { cons: text, kar };
                            return true;
                        }

                        // Standard conjunct: send Hasanta '্' followed by C2
                        send_unicode_string("\u{09CD}");
                        send_unicode_string(text);

                        // Check if this conjunct (e.g. স্ট, ন্ট, ল্ট, প্ট, ন্ড, ল্ড) needs ZWJ before ু/ূ in Windows Nirmala UI
                        let needs_zwj = match self.last_typed {
                            LastTyped::Consonant(c1) => needs_zwj_before_ukar(c1, text),
                            _ => false,
                        };
                        self.last_typed = if needs_zwj {
                            LastTyped::ConjunctNeedingZwjBeforeUKar
                        } else {
                            LastTyped::ConsonantCluster
                        };
                        return true;
                    } else {
                        // Not a consonant after 'g': flush the Hasanta '্' first
                        send_unicode_string("\u{09CD}");
                    }
                }

                // Case B: If a pre-base kar was waiting from Classic visual typing:
                if let Some(kar) = self.pending_kar.take() {
                    if is_cons {
                        // User typed: ি + ক -> Output: ক + ি (কি)
                        send_unicode_string(text);
                        send_unicode_string(kar);
                        self.last_typed = LastTyped::ConsonantWithPendingKar { cons: text, kar };
                        return true;
                    } else {
                        // Not a consonant, flush the kar first
                        send_unicode_string(kar);
                    }
                }

                // Case C: Windows Nirmala UI conjunct + ু/ূ ligature preservation fix!
                // Without ZWJ (\u{200D}), Nirmala UI breaks 'স্ট + ু' into 'স্‌টু' (as in 'স্টুডেন্ট').
                // Sending ZWJ before ু/ূ keeps 'স্ট' joined and places ু cleanly below it ('স্টু').
                if self.last_typed == LastTyped::ConjunctNeedingZwjBeforeUKar
                    && (text == "\u{09C1}" || text == "\u{09C2}")
                {
                    send_unicode_string("\u{200D}");
                    send_unicode_string(text);
                    self.last_typed = LastTyped::KarWithZwj;
                    return true;
                }

                send_unicode_string(text);

                if is_cons {
                    self.last_typed = LastTyped::Consonant(text);
                } else if is_bengali_kar(text) {
                    self.last_typed = LastTyped::Kar;
                } else {
                    self.last_typed = LastTyped::Other;
                }
                true
            }
            KeyOutput::LinkKey => {
                self.bangla_chars_typed = self.bangla_chars_typed.saturating_add(1);
                // If a pre-base kar was waiting, flush it
                if let Some(kar) = self.pending_kar.take() {
                    send_unicode_string(kar);
                }
                // 'g' pressed: set link_active = true
                self.link_active = true;
                true
            }
            KeyOutput::PassThrough => {
                self.flush_pending_states();
                self.last_typed = LastTyped::Other;
                false
            }
        }
    }

    /// Flush any pending link key ('্') or buffered Kar
    fn flush_pending_states(&mut self) {
        if let Some(kar) = self.pending_kar.take() {
            send_unicode_string(kar);
        }
        if self.link_active {
            send_unicode_string("\u{09CD}");
            self.link_active = false;
        }
    }
}

fn is_modifier_key(vk: u32) -> bool {
    matches!(
        vk,
        0x10 | 0xA0 | 0xA1 // Shift (Generic, Left, Right)
        | 0x11 | 0xA2 | 0xA3 // Ctrl (Generic, Left, Right)
        | 0x12 | 0xA4 | 0xA5 // Alt (Generic, Left, Right)
        | 0x14 // Caps Lock
        | 0x5B | 0x5C // Win keys
    )
}

/// Checks if a Ctrl+<key> shortcut is an IDE file/code navigation action
/// (excludes clipboard/text-editing shortcuts like Ctrl+C, Ctrl+V, Ctrl+X, Ctrl+Z, Ctrl+Y, Ctrl+A)
fn is_ide_navigation_shortcut(vk: u32) -> bool {
    matches!(
        vk,
        0x53 // Ctrl+S (Save file)
        | 0x50 // Ctrl+P (Quick Open file)
        | 0x46 // Ctrl+F (Find in code)
        | 0x47 // Ctrl+G (Go to line)
        | 0x42 // Ctrl+B (Toggle sidebar)
        | 0x4A // Ctrl+J (Toggle panel/terminal)
        | 0x09 // Ctrl+Tab (Switch editor tab)
    ) || vk == VK_OEM_2 as u32 // Ctrl+/ (Toggle code comment)
      || vk == VK_OEM_3 as u32 // Ctrl+` (Toggle integrated terminal)
}

/// Checks if a conjunct (C1 + ্ + C2) requires ZWJ (\u{200D}) before ু/ূ in Windows' Nirmala UI font
/// so that Nirmala UI doesn't break the conjunct into a visible hasanta (e.g. স্‌টু -> স্টু).
fn needs_zwj_before_ukar(c1: &str, c2: &str) -> bool {
    (matches!(c1, "স" | "ন" | "ল" | "প") && c2 == "ট")
        || (matches!(c1, "ন" | "ল") && c2 == "ড")
}

/// Checks if a key is a code syntax delimiter or operator that indicates code editing inside an IDE
fn is_code_syntax_key(vk: u32, is_shift: bool) -> bool {
    vk == VK_OEM_4 as u32 // [ or {
        || vk == VK_OEM_6 as u32 // ] or }
        || vk == VK_OEM_PLUS as u32 // = or +
        || vk == VK_ADD as u32 // Numpad +
        || vk == VK_OEM_3 as u32 // ` or ~
        || vk == VK_OEM_5 as u32 // \ or |
        || (vk == VK_OEM_1 as u32 && !is_shift) // ;
        || (vk == VK_OEM_2 as u32 && !is_shift) // /
        || (vk == VK_OEM_COMMA as u32 && is_shift) // <
        || (vk == VK_OEM_PERIOD as u32 && is_shift) // >
}

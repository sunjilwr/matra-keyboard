// Typing Engine for Borno Bangla
// Handles stateful typing: Link key ('g'), conjuncts, vowel forming, Classic/Modern Kar typing,
// Reph after consonant, and key interception.

use crate::config::TypingStyle;
use crate::layout::{
    get_pre_base_kar, is_bengali_consonant, is_bengali_kar, map_link_vowel, map_bangla_key,
    KeyOutput, KeyShiftState,
};
use crate::win32::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardMode {
    English,
    Bangla,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastTyped {
    None,
    /// Single consonant (e.g. "ম", "ক", "ফ")
    Consonant(&'static str),
    /// Consonant cluster formed by fola (ro-fola, jo-fola) or conjunct (e.g. "ফ্র", "ন্ড", "ক্র")
    ConsonantCluster,
    /// A consonant where a pre-base Kar was attached via Classic visual typing (e.g. "ফে" from "c" + "Shift+R")
    ConsonantWithPendingKar {
        cons: &'static str,
        kar: &'static str,
    },
    /// Vowel sign / Kar (e.g. ি, ী, ু, ূ, ৃ, ে, ৈ, ো, ৌ, া)
    Kar,
    Other,
}

impl LastTyped {
    pub fn is_consonant_base(&self) -> bool {
        matches!(
            self,
            LastTyped::Consonant(_)
                | LastTyped::ConsonantCluster
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
        }
    }

    /// Toggle between English and Bangla modes
    pub fn toggle_mode(&mut self) -> KeyboardMode {
        self.reset_state();

        self.mode = match self.mode {
            KeyboardMode::English => {
                turn_caps_lock_off();
                KeyboardMode::Bangla
            }
            KeyboardMode::Bangla => KeyboardMode::English,
        };

        // Play crisp, subtle confirmation beep
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
    pub fn handle_key_down(&mut self, vk: u32, is_shift: bool, is_alt: bool, is_ctrl: bool) -> bool {
        // If in English mode, do nothing - 100% passthrough
        if self.mode == KeyboardMode::English {
            return false;
        }

        // Ignore modifier keys alone (Shift, Ctrl, Alt, CapsLock, Win).
        // Crucial: They must NEVER reset last_typed, because user presses Shift BEFORE pressing 'A'!
        if is_modifier_key(vk) {
            return false;
        }

        // Allow system shortcuts (Ctrl+C, Ctrl+V, Win+Key, Alt+Tab, etc.) to pass through untouched
        let is_alt_gr = is_ctrl && is_alt;
        if (is_ctrl && !is_alt_gr) || (is_alt && !is_alt_gr) {
            self.flush_pending_states();
            self.last_typed = LastTyped::Other;
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
            self.last_typed = LastTyped::None;
            return false; // Normal backspace passes through
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

            // Otherwise, this is a conjunct (e.g. ক + g + ক = ক্ক, ন + g + ড = ন্ড)
            send_unicode_string("\u{09CD}");
            self.link_active = false;
            was_conjunct_link = true;
        }

        // ====================================================================
        // 4. CLASSIC VISUAL TYPING SUPPORT: 'ি + ক' -> 'কি'
        // If typing_style == Classic and this key is a pre-base kar (ি, ে, ৈ, ো, ৌ),
        // and last_typed was NOT a consonant base (i.e. visual order at start of syllable):
        // ====================================================================
        let is_after_consonant = self.last_typed.is_consonant_base();
        if self.typing_style == TypingStyle::Classic && !is_after_consonant {
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
                let is_cons = is_bengali_consonant(text);

                // If a pre-base kar was waiting from Classic visual typing:
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

                send_unicode_string(text);

                if was_conjunct_link && is_cons {
                    self.last_typed = LastTyped::ConsonantCluster;
                } else if is_cons {
                    self.last_typed = LastTyped::Consonant(text);
                } else if is_bengali_kar(text) {
                    self.last_typed = LastTyped::Kar;
                } else {
                    self.last_typed = LastTyped::Other;
                }
                true
            }
            KeyOutput::LinkKey => {
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

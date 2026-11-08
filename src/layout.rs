// UniBijoy Layout Definitions
// Exact 1-to-1 Unicode Bijoy mappings for English keyboard layout.

use crate::win32::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyShiftState {
    Normal,
    Shift,
    AltGr,
    ShiftAltGr,
}

/// Represents the mapped output of a key in UniBijoy layout
#[derive(Debug, Clone)]
pub enum KeyOutput {
    /// Direct text/character output
    Text(&'static str),
    /// Link key (Virama / Hasanta '্') which can combine with vowels or consonants
    LinkKey,
    /// Untranslated key (let it pass through to the OS)
    PassThrough,
}

/// Get the mapped output for a virtual key code in UniBijoy layout
pub fn map_unibijoy_key(vk: u32, state: KeyShiftState) -> KeyOutput {
    match state {
        KeyShiftState::AltGr => match vk {
            0x41 => KeyOutput::Text("ঋ"), // AltGr + A
            0x53 => KeyOutput::Text("উ"), // AltGr + S
            0x44 => KeyOutput::Text("ই"), // AltGr + D
            0x46 => KeyOutput::Text("আ"), // AltGr + F
            0x58 => KeyOutput::Text("ও"), // AltGr + X
            0x43 => KeyOutput::Text("এ"), // AltGr + C
            _ => map_unibijoy_normal_or_shift(vk, KeyShiftState::Normal),
        },
        KeyShiftState::ShiftAltGr => match vk {
            0x53 => KeyOutput::Text("ঊ"), // Shift + AltGr + S
            0x44 => KeyOutput::Text("ঈ"), // Shift + AltGr + D
            0x46 => KeyOutput::Text("অ"), // Shift + AltGr + F
            0x58 => KeyOutput::Text("ঔ"), // Shift + AltGr + X
            0x43 => KeyOutput::Text("ঐ"), // Shift + AltGr + C
            _ => map_unibijoy_normal_or_shift(vk, KeyShiftState::Shift),
        },
        KeyShiftState::Normal | KeyShiftState::Shift => {
            map_unibijoy_normal_or_shift(vk, state)
        }
    }
}

fn map_unibijoy_normal_or_shift(vk: u32, state: KeyShiftState) -> KeyOutput {
    let is_shift = state == KeyShiftState::Shift;

    // Number keys row (0x30 to 0x39: '0' - '9')
    match vk {
        0x30 => KeyOutput::Text(if is_shift { ")" } else { "০" }),
        0x31 => KeyOutput::Text(if is_shift { "!" } else { "১" }),
        0x32 => KeyOutput::Text(if is_shift { "\u{0981}" } else { "২" }), // ঁ (Chandrabindu)
        0x33 => KeyOutput::Text(if is_shift { "#" } else { "৩" }),
        0x34 => KeyOutput::Text(if is_shift { "৳" } else { "৪" }), // Taka sign
        0x35 => KeyOutput::Text(if is_shift { "%" } else { "৫" }),
        0x36 => KeyOutput::Text(if is_shift { "\u{0983}" } else { "৬" }), // ঃ (Visarga)
        0x37 => KeyOutput::Text(if is_shift { "ৎ" } else { "৭" }), // ৎ (Khanda Ta)
        0x38 => KeyOutput::Text(if is_shift { "*" } else { "৮" }),
        0x39 => KeyOutput::Text(if is_shift { "(" } else { "৯" }),

        // Grave / Tilde key (` ~)
        vk if vk == VK_OEM_3 as u32 => {
            KeyOutput::Text(if is_shift { "\u{200D}" } else { "\u{200C}" }) // Shift: ZWJ, Normal: ZWNJ
        }
        vk if vk == VK_OEM_MINUS as u32 => KeyOutput::Text(if is_shift { "_" } else { "-" }),
        vk if vk == VK_OEM_PLUS as u32 => KeyOutput::Text(if is_shift { "+" } else { "=" }),

        // Row 1 (Q to [ ])
        0x51 => KeyOutput::Text(if is_shift { "ঙ" } else { "\u{0982}" }), // Q: ঙ, q: ং
        0x57 => KeyOutput::Text(if is_shift { "য়" } else { "য" }),         // W: য়, w: য
        0x45 => KeyOutput::Text(if is_shift { "ঢ" } else { "ড" }),         // E: ঢ, e: ড
        0x52 => KeyOutput::Text(if is_shift { "ফ" } else { "প" }),         // R: ফ, r: প
        0x54 => KeyOutput::Text(if is_shift { "ঠ" } else { "ট" }),         // T: ঠ, t: ট
        0x59 => KeyOutput::Text(if is_shift { "ছ" } else { "চ" }),         // Y: ছ, y: চ
        0x55 => KeyOutput::Text(if is_shift { "ঝ" } else { "জ" }),         // U: ঝ, u: জ
        0x49 => KeyOutput::Text(if is_shift { "ঞ" } else { "হ" }),         // I: ঞ, i: হ
        0x4F => KeyOutput::Text(if is_shift { "ঘ" } else { "গ" }),         // O: ঘ, o: গ
        0x50 => KeyOutput::Text(if is_shift { "ঢ়" } else { "ড়" }),         // P: ঢ়, p: ড়
        vk if vk == VK_OEM_4 as u32 => KeyOutput::Text(if is_shift { "{" } else { "[" }),
        vk if vk == VK_OEM_6 as u32 => KeyOutput::Text(if is_shift { "}" } else { "]" }),
        vk if vk == VK_OEM_5 as u32 => KeyOutput::Text(if is_shift { "ৰ" } else { "ৱ" }), // \: ৱ, |: ৰ

        // Row 2 (A to ')
        0x41 => KeyOutput::Text(if is_shift { "\u{09B0}\u{09CD}" } else { "ৃ" }), // A: র্ (Reph), a: ৃ
        0x53 => KeyOutput::Text(if is_shift { "ূ" } else { "ু" }),                 // S: ূ, s: ু
        0x44 => KeyOutput::Text(if is_shift { "ী" } else { "ি" }),                 // D: ী, d: ি
        0x46 => KeyOutput::Text(if is_shift { "অ" } else { "া" }),                 // F: অ, f: া
        0x47 => {
            // G: link key 'g' or Shift+G '।' (Dari)
            if is_shift {
                KeyOutput::Text("।") // Shift + G = । (Dari)
            } else {
                KeyOutput::LinkKey   // g = ্ (Hasanta/Link)
            }
        }
        0x48 => KeyOutput::Text(if is_shift { "ভ" } else { "ব" }), // H: ভ, h: ব
        0x4A => KeyOutput::Text(if is_shift { "খ" } else { "ক" }), // J: খ, j: ক
        0x4B => KeyOutput::Text(if is_shift { "থ" } else { "ত" }), // K: থ, k: ত
        0x4C => KeyOutput::Text(if is_shift { "ধ" } else { "দ" }), // L: ধ, l: দ
        vk if vk == VK_OEM_1 as u32 => KeyOutput::Text(if is_shift { ":" } else { ";" }),
        vk if vk == VK_OEM_7 as u32 => KeyOutput::Text(if is_shift { "\"" } else { "'" }),

        // Row 3 (Z to /)
        0x5A => KeyOutput::Text(if is_shift { "\u{09CD}\u{09AF}" } else { "\u{09CD}\u{09B0}" }), // Z: ্য, z: ্র
        0x58 => KeyOutput::Text(if is_shift { "ৌ" } else { "ো" }),                                 // X: ৌ, x: ো
        0x43 => KeyOutput::Text(if is_shift { "ৈ" } else { "ে" }),                                 // C: ৈ, c: ে
        0x56 => KeyOutput::Text(if is_shift { "ল" } else { "র" }),                                 // V: ল, v: র
        0x42 => KeyOutput::Text(if is_shift { "ণ" } else { "ন" }),                                 // B: ণ, b: ন
        0x4E => KeyOutput::Text(if is_shift { "ষ" } else { "স" }),                                 // N: ষ, n: স
        0x4D => KeyOutput::Text(if is_shift { "শ" } else { "ম" }),                                 // M: শ, m: ম
        vk if vk == VK_OEM_COMMA as u32 => KeyOutput::Text(if is_shift { "<" } else { "," }),
        vk if vk == VK_OEM_PERIOD as u32 => KeyOutput::Text(if is_shift { ">" } else { "." }),
        vk if vk == VK_OEM_2 as u32 => KeyOutput::Text(if is_shift { "?" } else { "/" }),

        // Numpad mappings
        vk if vk == VK_NUMPAD0 as u32 => KeyOutput::Text("০"),
        vk if vk == VK_NUMPAD1 as u32 => KeyOutput::Text("১"),
        vk if vk == VK_NUMPAD2 as u32 => KeyOutput::Text("২"),
        vk if vk == VK_NUMPAD3 as u32 => KeyOutput::Text("৩"),
        vk if vk == VK_NUMPAD4 as u32 => KeyOutput::Text("৪"),
        vk if vk == VK_NUMPAD5 as u32 => KeyOutput::Text("৫"),
        vk if vk == VK_NUMPAD6 as u32 => KeyOutput::Text("৬"),
        vk if vk == VK_NUMPAD7 as u32 => KeyOutput::Text("৭"),
        vk if vk == VK_NUMPAD8 as u32 => KeyOutput::Text("৮"),
        vk if vk == VK_NUMPAD9 as u32 => KeyOutput::Text("৯"),
        vk if vk == VK_ADD as u32 => KeyOutput::Text("+"),
        vk if vk == VK_SUBTRACT as u32 => KeyOutput::Text("-"),
        vk if vk == VK_MULTIPLY as u32 => KeyOutput::Text("*"),
        vk if vk == VK_DIVIDE as u32 => KeyOutput::Text("/"),
        vk if vk == VK_DECIMAL as u32 => KeyOutput::Text("."),

        _ => KeyOutput::PassThrough,
    }
}

/// Swaroborno (Independent Vowel) formed by Link key 'g' + Vowel key
pub fn map_link_vowel(vk: u32, is_shift: bool) -> Option<&'static str> {
    match (vk, is_shift) {
        (0x46, false) => Some("আ"), // g + f = আ
        (0x44, false) => Some("ই"), // g + d = ই
        (0x44, true)  => Some("ঈ"), // g + D = ঈ
        (0x53, false) => Some("উ"), // g + s = উ
        (0x53, true)  => Some("ঊ"), // g + S = ঊ
        (0x41, false) => Some("ঋ"), // g + a = ঋ
        (0x43, false) => Some("এ"), // g + c = এ
        (0x43, true)  => Some("ঐ"), // g + C = ঐ
        (0x58, false) => Some("ও"), // g + x = ও
        (0x58, true)  => Some("ঔ"), // g + X = ঔ
        _ => None,
    }
}

/// Check if a key is a pre-base Kar (ি, ে, ৈ, ো, ৌ) for Classic Bijoy visual typing
pub fn get_pre_base_kar(vk: u32, is_shift: bool) -> Option<&'static str> {
    match (vk, is_shift) {
        (0x44, false) => Some("ি"), // d = ি
        (0x43, false) => Some("ে"), // c = ে
        (0x43, true)  => Some("ৈ"), // Shift+c = ৈ
        (0x58, false) => Some("ো"), // x = ো
        (0x58, true)  => Some("ৌ"), // Shift+x = ৌ
        _ => None,
    }
}

/// Check if a character string is a Bengali consonant
pub fn is_bengali_consonant(s: &str) -> bool {
    matches!(
        s,
        "ক" | "খ" | "গ" | "ঘ" | "ঙ"
        | "চ" | "ছ" | "জ" | "ঝ" | "ঞ"
        | "ট" | "ঠ" | "ড" | "ঢ" | "ণ"
        | "ত" | "থ" | "দ" | "ধ" | "ন"
        | "প" | "ফ" | "ব" | "ভ" | "ম"
        | "য" | "র" | "ল" | "শ" | "ষ"
        | "স" | "হ" | "ড়" | "ঢ়" | "য়"
        | "ৎ" | "ৱ" | "ৰ"
    )
}


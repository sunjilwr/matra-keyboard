// Unit tests for UniBijoy layout mappings and engine rules

#[cfg(test)]
mod tests {
    use borno::layout::*;
    use borno::engine::{Engine, KeyboardMode};

    #[test]
    fn test_vowels_via_link_key() {
        assert_eq!(map_link_vowel(0x46, false), Some("আ"));
        assert_eq!(map_link_vowel(0x44, false), Some("ই"));
        assert_eq!(map_link_vowel(0x44, true),  Some("ঈ"));
        assert_eq!(map_link_vowel(0x53, false), Some("উ"));
        assert_eq!(map_link_vowel(0x53, true),  Some("ঊ"));
        assert_eq!(map_link_vowel(0x41, false), Some("ঋ"));
        assert_eq!(map_link_vowel(0x43, false), Some("এ"));
        assert_eq!(map_link_vowel(0x43, true),  Some("ঐ"));
        assert_eq!(map_link_vowel(0x58, false), Some("ও"));
        assert_eq!(map_link_vowel(0x58, true),  Some("ঔ"));
    }

    #[test]
    fn test_direct_vowel_and_signs() {
        // Shift + F = অ
        match map_unibijoy_key(0x46, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "অ"),
            _ => panic!("Expected অ"),
        }

        // f = া
        match map_unibijoy_key(0x46, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "া"),
            _ => panic!("Expected া"),
        }

        // d = ি, Shift + D = ী
        match map_unibijoy_key(0x44, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "ি"),
            _ => panic!("Expected ি"),
        }
        match map_unibijoy_key(0x44, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ী"),
            _ => panic!("Expected ী"),
        }
    }

    #[test]
    fn test_consonants() {
        // j = ক, Shift + J = খ
        match map_unibijoy_key(0x4A, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "ক"),
            _ => panic!("Expected ক"),
        }
        match map_unibijoy_key(0x4A, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "খ"),
            _ => panic!("Expected খ"),
        }

        // h = ব, Shift + H = ভ
        match map_unibijoy_key(0x48, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "ব"),
            _ => panic!("Expected ব"),
        }
        match map_unibijoy_key(0x48, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ভ"),
            _ => panic!("Expected ভ"),
        }

        // v = র, Shift + V = ল
        match map_unibijoy_key(0x56, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "র"),
            _ => panic!("Expected র"),
        }
        match map_unibijoy_key(0x56, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ল"),
            _ => panic!("Expected ল"),
        }
    }

    #[test]
    fn test_fola_and_reph() {
        // Shift + A = র্ (Reph)
        match map_unibijoy_key(0x41, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "\u{09B0}\u{09CD}"),
            _ => panic!("Expected Reph"),
        }

        // z = ্র (ro-fola)
        match map_unibijoy_key(0x5A, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "\u{09CD}\u{09B0}"),
            _ => panic!("Expected ro-fola"),
        }

        // Shift + Z = ্য (jo-fola)
        match map_unibijoy_key(0x5A, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "\u{09CD}\u{09AF}"),
            _ => panic!("Expected jo-fola"),
        }
    }

    #[test]
    fn test_special_characters() {
        // Shift + G = । (Dari)
        match map_unibijoy_key(0x47, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "।"),
            _ => panic!("Expected ।"),
        }

        // g = LinkKey
        match map_unibijoy_key(0x47, KeyShiftState::Normal) {
            KeyOutput::LinkKey => {}
            _ => panic!("Expected LinkKey for 'g'"),
        }

        // Shift + 4 = ৳ (Taka)
        match map_unibijoy_key(0x34, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "৳"),
            _ => panic!("Expected ৳"),
        }

        // Shift + 7 = ৎ (Khanda Ta)
        match map_unibijoy_key(0x37, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ৎ"),
            _ => panic!("Expected ৎ"),
        }
    }

    #[test]
    fn test_numbers() {
        match map_unibijoy_key(0x31, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "১"),
            _ => panic!("Expected ১"),
        }
        match map_unibijoy_key(0x30, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "০"),
            _ => panic!("Expected ০"),
        }
    }

    #[test]
    fn test_engine_state_transitions() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::UniBijoy;
        engine.sound_enabled = false;

        // Press 'n' (স)
        assert!(engine.handle_key_down(0x4E, false, false, false));
        // Press 'c' (ে) -> creates 'সে'
        assert!(engine.handle_key_down(0x43, false, false, false));
        // Press 'g' (link key)
        assert!(engine.handle_key_down(0x47, false, false, false));
        assert!(engine.link_active);
        // Press 'd' (ই) -> forms 'ই'!
        assert!(engine.handle_key_down(0x44, false, false, false));
        assert!(!engine.link_active);
    }

    #[test]
    fn test_reph_after_consonant_state() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::UniBijoy;
        engine.sound_enabled = false;

        // 1. Type 'j' (ক)
        assert!(engine.handle_key_down(0x4A, false, false, false));
        // 2. Type 'm' (ম)
        assert!(engine.handle_key_down(0x4D, false, false, false));
        // 3. User presses physical Shift key (VK_SHIFT = 0x10)
        assert!(!engine.handle_key_down(0x10, true, false, false));
        // Verify last_typed is still Consonant("ম") even after Shift key was pressed!
        assert_eq!(engine.last_typed, borno::engine::LastTyped::Consonant("ম"));
        // 4. Type 'A' (0x41) with Shift held down -> triggers Reph replacement!
        assert!(engine.handle_key_down(0x41, true, false, false));
        // Verify state after Reph (now ConsonantCluster so ী or ে can attach!)
        assert_eq!(engine.last_typed, borno::engine::LastTyped::ConsonantCluster);
    }

    #[test]
    fn test_friendly_typing_sequence() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::UniBijoy;
        engine.sound_enabled = false;

        // Modern phonetic sequence for "ফ্রেন্ডলি":
        // 1. Shift + R (ফ)
        assert!(engine.handle_key_down(0x52, true, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::Consonant("ফ"));

        // 2. z (্র = ro-fola)
        assert!(engine.handle_key_down(0x5A, false, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::ConsonantCluster);

        // 3. c (ে = e-kar)
        assert!(engine.handle_key_down(0x43, false, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::Kar);
        assert!(engine.pending_kar.is_none()); // Crucial: NOT trapped in pending_kar!

        // 4. b (ন)
        assert!(engine.handle_key_down(0x42, false, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::Consonant("ন"));

        // 5. g (link key)
        assert!(engine.handle_key_down(0x47, false, false, false));
        assert!(engine.link_active);

        // 6. e (ড -> forms conjunct 'ন্ড')
        assert!(engine.handle_key_down(0x45, false, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::ConsonantCluster);

        // 7. Shift + V (ল)
        assert!(engine.handle_key_down(0x56, true, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::Consonant("ল"));

        // 8. d (ি = i-kar -> forms 'লি')
        assert!(engine.handle_key_down(0x44, false, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::Kar);
        assert!(engine.pending_kar.is_none());
    }

    #[test]
    fn test_classic_visual_fola_transformation() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::UniBijoy;
        engine.sound_enabled = false;

        // Classic sequence for "ফ্রে":
        // 1. User types 'c' (ে) first
        assert!(engine.handle_key_down(0x43, false, false, false));
        assert_eq!(engine.pending_kar, Some("ে"));

        // 2. User types Shift + R (ফ) -> outputs 'ফে', sets ConsonantWithPendingKar
        assert!(engine.handle_key_down(0x52, true, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::ConsonantWithPendingKar { cons: "ফ", kar: "ে" });

        // 3. User types 'z' (্র) -> auto-reorders 'ফে' + 'z' into 'ফ্রে'!
        assert!(engine.handle_key_down(0x5A, false, false, false));
        assert_eq!(engine.last_typed, borno::engine::LastTyped::Kar);
    }

    #[test]
    fn test_juktakkhor_formation() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::UniBijoy;
        engine.sound_enabled = false;

        // Type 'j' (ক)
        assert!(engine.handle_key_down(0x4A, false, false, false));
        // Type 'g' (link)
        assert!(engine.handle_key_down(0x47, false, false, false));
        assert!(engine.link_active);
        // Type 'j' (ক) -> commits hasanta and types ক -> ক্ক
        assert!(engine.handle_key_down(0x4A, false, false, false));
        assert!(!engine.link_active);
    }
}

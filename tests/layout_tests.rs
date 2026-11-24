// Unit tests for Matra Bangla layout mappings and engine rules

#[cfg(test)]
mod tests {
    use matra::layout::*;
    use matra::engine::{Engine, KeyboardMode};

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
        match map_bangla_key(0x46, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "অ"),
            _ => panic!("Expected অ"),
        }

        // f = া
        match map_bangla_key(0x46, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "া"),
            _ => panic!("Expected া"),
        }

        // d = ি, Shift + D = ী
        match map_bangla_key(0x44, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "ি"),
            _ => panic!("Expected ি"),
        }
        match map_bangla_key(0x44, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ী"),
            _ => panic!("Expected ী"),
        }
    }

    #[test]
    fn test_consonants() {
        // j = ক, Shift + J = খ
        match map_bangla_key(0x4A, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "ক"),
            _ => panic!("Expected ক"),
        }
        match map_bangla_key(0x4A, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "খ"),
            _ => panic!("Expected খ"),
        }

        // h = ব, Shift + H = ভ
        match map_bangla_key(0x48, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "ব"),
            _ => panic!("Expected ব"),
        }
        match map_bangla_key(0x48, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ভ"),
            _ => panic!("Expected ভ"),
        }

        // v = র, Shift + V = ল
        match map_bangla_key(0x56, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "র"),
            _ => panic!("Expected র"),
        }
        match map_bangla_key(0x56, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ল"),
            _ => panic!("Expected ল"),
        }
    }

    #[test]
    fn test_fola_and_reph() {
        // Shift + A = র্ (Reph)
        match map_bangla_key(0x41, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "\u{09B0}\u{09CD}"),
            _ => panic!("Expected Reph"),
        }

        // z = ্র (ro-fola)
        match map_bangla_key(0x5A, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "\u{09CD}\u{09B0}"),
            _ => panic!("Expected ro-fola"),
        }

        // Shift + Z = ্য (jo-fola)
        match map_bangla_key(0x5A, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "\u{09CD}\u{09AF}"),
            _ => panic!("Expected jo-fola"),
        }
    }

    #[test]
    fn test_special_characters() {
        // Shift + G = । (Dari)
        match map_bangla_key(0x47, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "।"),
            _ => panic!("Expected ।"),
        }

        // g = LinkKey
        match map_bangla_key(0x47, KeyShiftState::Normal) {
            KeyOutput::LinkKey => {}
            _ => panic!("Expected LinkKey for 'g'"),
        }

        // Shift + 4 = ৳ (Taka)
        match map_bangla_key(0x34, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "৳"),
            _ => panic!("Expected ৳"),
        }

        // Shift + 7 = ৎ (Khanda Ta)
        match map_bangla_key(0x37, KeyShiftState::Shift) {
            KeyOutput::Text(t) => assert_eq!(t, "ৎ"),
            _ => panic!("Expected ৎ"),
        }
    }

    #[test]
    fn test_numbers() {
        match map_bangla_key(0x31, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "১"),
            _ => panic!("Expected ১"),
        }
        match map_bangla_key(0x30, KeyShiftState::Normal) {
            KeyOutput::Text(t) => assert_eq!(t, "০"),
            _ => panic!("Expected ০"),
        }
    }

    #[test]
    fn test_engine_state_transitions() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
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
        engine.mode = KeyboardMode::Bangla;
        engine.sound_enabled = false;

        // 1. Type 'j' (ক)
        assert!(engine.handle_key_down(0x4A, false, false, false));
        // 2. Type 'm' (ম)
        assert!(engine.handle_key_down(0x4D, false, false, false));
        // 3. User presses physical Shift key (VK_SHIFT = 0x10)
        assert!(!engine.handle_key_down(0x10, true, false, false));
        // Verify last_typed is still Consonant("ম") even after Shift key was pressed!
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Consonant("ম"));
        // 4. Type 'A' (0x41) with Shift held down -> triggers Reph replacement!
        assert!(engine.handle_key_down(0x41, true, false, false));
        // Verify state after Reph (now ConsonantCluster so ী or ে can attach!)
        assert_eq!(engine.last_typed, matra::engine::LastTyped::ConsonantCluster);
    }

    #[test]
    fn test_friendly_typing_sequence() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
        engine.sound_enabled = false;

        // Modern phonetic sequence for "ফ্রেন্ডলি":
        // 1. Shift + R (ফ)
        assert!(engine.handle_key_down(0x52, true, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Consonant("ফ"));

        // 2. z (্র = ro-fola)
        assert!(engine.handle_key_down(0x5A, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::ConsonantCluster);

        // 3. c (ে = e-kar)
        assert!(engine.handle_key_down(0x43, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Kar);
        assert!(engine.pending_kar.is_none()); // Crucial: NOT trapped in pending_kar!

        // 4. b (ন)
        assert!(engine.handle_key_down(0x42, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Consonant("ন"));

        // 5. g (link key)
        assert!(engine.handle_key_down(0x47, false, false, false));
        assert!(engine.link_active);

        // 6. e (ড -> forms conjunct 'ন্ড')
        assert!(engine.handle_key_down(0x45, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::ConjunctNeedingZwjBeforeUKar);

        // 7. Shift + V (ল)
        assert!(engine.handle_key_down(0x56, true, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Consonant("ল"));

        // 8. d (ি = i-kar -> forms 'লি')
        assert!(engine.handle_key_down(0x44, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Kar);
        assert!(engine.pending_kar.is_none());
    }

    #[test]
    fn test_classic_visual_fola_transformation() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
        engine.sound_enabled = false;

        // Classic sequence for "ফ্রে":
        // 1. User types 'c' (ে) first
        assert!(engine.handle_key_down(0x43, false, false, false));
        assert_eq!(engine.pending_kar, Some("ে"));

        // 2. User types Shift + R (ফ) -> outputs 'ফে', sets ConsonantWithPendingKar
        assert!(engine.handle_key_down(0x52, true, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::ConsonantWithPendingKar { cons: "ফ", kar: "ে" });

        // 3. User types 'z' (্র) -> auto-reorders 'ফে' + 'z' into 'ফ্রে'!
        assert!(engine.handle_key_down(0x5A, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Kar);
    }

    #[test]
    fn test_juktakkhor_formation() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
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

    #[test]
    #[cfg(target_os = "windows")]
    fn test_per_window_state_manager() {
        use matra::hook::WindowStateManager;

        let mut wsm = WindowStateManager::new(true);

        // 1. User focuses Window 101 (e.g. Word) -> defaults to English
        assert_eq!(wsm.sync_active_window(101), Some(KeyboardMode::English));

        // 2. User toggles to Bangla in Window 101
        wsm.set_mode_for_window(101, KeyboardMode::Bangla);

        // 3. User switches to Window 202 (e.g. VS Code / Terminal) -> automatically English!
        assert_eq!(wsm.sync_active_window(202), Some(KeyboardMode::English));

        // 4. User switches back to Window 101 (Word) -> automatically restores Bangla!
        assert_eq!(wsm.sync_active_window(101), Some(KeyboardMode::Bangla));

        // 5. Still in Window 101 -> returns None (no change needed)
        assert_eq!(wsm.sync_active_window(101), None);

        // 6. Test with disabled mode (global) -> always returns None
        let mut disabled_wsm = WindowStateManager::new(false);
        assert_eq!(disabled_wsm.sync_active_window(101), None);
    }

    #[test]
    fn test_win_v_clipboard_shortcut_passthrough() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
        engine.sound_enabled = false;

        // Without Win key: 'v' (0x56) is intercepted by Bangla engine (types 'র')
        assert!(engine.handle_key_down_with_modifiers(0x56, false, false, false, false));

        // With Win key held (Win + V for Windows Clipboard History):
        // 1. LWin keydown (0x5B) passes through
        assert!(!engine.handle_key_down_with_modifiers(0x5B, false, false, false, true));
        // 2. 'V' keydown (0x56) with is_win=true MUST pass through untouched (returns false)
        assert!(!engine.handle_key_down_with_modifiers(0x56, false, false, false, true));
    }

    #[test]
    fn test_smart_coding_mode_detection_and_auto_english() {
        use matra::engine::{classify_window_context, WindowContext};
        #[cfg(target_os = "windows")]
        use matra::hook::WindowStateManager;

        // 1. Detect Antigravity IDE and pubspec.yaml from screenshot
        assert_eq!(
            classify_window_context(
                "plant_identify - Antigravity IDE - pubspec.yaml●",
                "Chrome_WidgetWin_1"
            ),
            WindowContext::CodingIde
        );
        assert_eq!(
            classify_window_context("main.rs - Visual Studio Code", "Chrome_WidgetWin_1"),
            WindowContext::CodingIde
        );
        assert_eq!(
            classify_window_context("Windows PowerShell", "CASCADIA_HOSTING_WINDOW_CLASS"),
            WindowContext::Terminal
        );
        assert_eq!(
            classify_window_context("Google Chrome", "Chrome_WidgetWin_1"),
            WindowContext::Normal
        );

        // 2. Test Smart Coding Mode inside Antigravity IDE
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
        engine.smart_coding_mode = true;
        engine.window_context = WindowContext::CodingIde;
        engine.sound_enabled = false;

        // Scenario A: User forgot Bangla was ON and edits `version: 1.0.9+1` -> presses '3' (0x33)
        // Must pass through as English '3' (returns false) AND auto-switch engine to English!
        assert!(!engine.handle_key_down(0x33, false, false, false));
        assert_eq!(engine.mode, KeyboardMode::English);

        // Scenario B: User turns on Bangla in AI Chat, types Bangla, presses Enter/Shift+Enter/Ctrl+V
        engine.set_mode(KeyboardMode::Bangla);
        assert!(engine.handle_key_down(0x4A, false, false, false)); // 'j' -> ক
        assert_eq!(engine.bangla_chars_typed, 1);
        // Enter and Shift+Enter inside AI chat keep Bangla mode ON while user is still in chat!
        assert!(!engine.handle_key_down(0x0D, true, false, false));
        assert_eq!(engine.mode, KeyboardMode::Bangla);
        assert!(!engine.handle_key_down(0x0D, false, false, false));
        assert_eq!(engine.mode, KeyboardMode::Bangla);
        // Ctrl + V (paste into AI chat) keeps Bangla mode ON!
        assert!(!engine.handle_key_down(0x56, false, false, true));
        assert_eq!(engine.mode, KeyboardMode::Bangla);
        // Escape or code syntax (e.g. ';' = 0xBA) auto-switches to English!
        assert!(!engine.handle_key_down(0xBA, false, false, false));
        assert_eq!(engine.mode, KeyboardMode::English);

        // Two-way Chat Zone geometry test:
        // Clicking inside AI chat box stays within zone; clicking Code Editor is outside zone
        #[cfg(target_os = "windows")]
        {
            assert!(matra::hook::is_point_in_chat_zone(1600, 920, 1550, 900));
            assert!(!matra::hook::is_point_in_chat_zone(650, 420, 1550, 900));
        }

        // Scenario C: Switching file tabs in IDE auto-switches to English, while dirty marker '●' does not!
        #[cfg(target_os = "windows")]
        {
            let mut wsm = WindowStateManager::new(true);
            assert_eq!(
                wsm.sync_active_window_with_context(
                    500,
                    "plant_identify - Antigravity IDE - pubspec.yaml",
                    WindowContext::CodingIde,
                    true
                ),
                Some(KeyboardMode::English)
            );
            wsm.set_mode_for_window(500, KeyboardMode::Bangla);
            // Dirty marker '●' added while typing -> should NOT reset to English
            assert_eq!(
                wsm.sync_active_window_with_context(
                    500,
                    "plant_identify - Antigravity IDE - pubspec.yaml●",
                    WindowContext::CodingIde,
                    true
                ),
                None
            );
            // User switches tab to ad_helper.dart -> automatically switches to English!
            assert_eq!(
                wsm.sync_active_window_with_context(
                    500,
                    "plant_identify - Antigravity IDE - ad_helper.dart",
                    WindowContext::CodingIde,
                    true
                ),
                Some(KeyboardMode::English)
            );
        }
    }

    #[test]
    fn test_student_conjunct_ukar_ligature_fix() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
        engine.sound_enabled = false;

        // Sequence for "স্টুডেন্ট":
        // 1. n (স)
        assert!(engine.handle_key_down(0x4E, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::Consonant("স"));
        // 2. g (link key ্)
        assert!(engine.handle_key_down(0x47, false, false, false));
        assert!(engine.link_active);
        // 3. t (ট) -> forms 'স্ট' (marked as ConjunctNeedingZwjBeforeUKar)
        assert!(engine.handle_key_down(0x54, false, false, false));
        assert_eq!(
            engine.last_typed,
            matra::engine::LastTyped::ConjunctNeedingZwjBeforeUKar
        );
        // 4. s (ু) -> outputs ZWJ (\u{200D}) + 'ু' so Windows Nirmala UI renders joined 'স্টু' instead of 'স্‌টু'!
        assert!(engine.handle_key_down(0x53, false, false, false));
        assert_eq!(engine.last_typed, matra::engine::LastTyped::KarWithZwj);

        // 5. Classic pre-base kar + conjunct reordering: 'c' (ে) + 'n' (স) + 'g' (্) + 't' (ট) -> 'স্টে'
        assert!(engine.handle_key_down(0x43, false, false, false)); // c (ে)
        assert_eq!(engine.pending_kar, Some("ে"));
        assert!(engine.handle_key_down(0x4E, false, false, false)); // n (স) -> 'সে'
        assert!(engine.handle_key_down(0x47, false, false, false)); // g (্)
        assert!(engine.handle_key_down(0x54, false, false, false)); // t (ট) -> reorders 'সে' + '্ট' into 'স্টে'!
        assert_eq!(
            engine.last_typed,
            matra::engine::LastTyped::ConsonantWithPendingKar {
                cons: "ট",
                kar: "ে"
            }
        );
    }

    #[test]
    fn test_copy_paste_shortcuts_clear_pending_kar_and_passthrough() {
        let mut engine = Engine::new();
        engine.mode = KeyboardMode::Bangla;
        engine.sound_enabled = false;

        // Simulate Classic pre-base kar buffered (e.g. user pressed 'c' -> 'ে')
        assert!(engine.handle_key_down(0x43, false, false, false));
        assert_eq!(engine.pending_kar, Some("ে"));

        // Pressing Ctrl+C (0x43, is_ctrl=true) must pass through (false) and clear pending_kar without SendInput
        assert!(!engine.handle_key_down(0x43, false, false, true));
        assert_eq!(engine.pending_kar, None);

        // Buffer again and test Ctrl+V (0x56, is_ctrl=true)
        assert!(engine.handle_key_down(0x43, false, false, false));
        assert_eq!(engine.pending_kar, Some("ে"));
        assert!(!engine.handle_key_down(0x56, false, false, true));
        assert_eq!(engine.pending_kar, None);
    }
}

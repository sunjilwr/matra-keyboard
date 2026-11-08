gf# 🇧🇩 bnkeyboard - UniBijoy & English Native Keyboard (Pure Rust)

উইন্ডোজের জন্য সম্পূর্ণ **Pure Rust** দিয়ে তৈরি একটি হালকা, সুপার-ফাস্ট সিস্টেম-ওয়াইড ইউনিবিজয় (UniBijoy) ও ইংলিশ (English) কিবোর্ড।

অন্য কোনো বাড়তি বা অপ্রয়োজনীয় লেআউট ছাড়াই শুধুমাত্র **English** এবং **UniBijoy** লেআউটের জন্য অপ্টিমাইজড। 

---

## ✨ প্রধান বৈশিষ্ট্যসমূহ (Key Features)

1. **System-wide Support**:
   - যেকোনো অ্যাপ্লিকেশন (MS Word, Notepad, VS Code, Chrome, Firefox, Discord, Terminal ইত্যাদি)-এ মসৃণভাবে কাজ করে।
   - Windows Low-Level Keyboard Hook (`WH_KEYBOARD_LL`) ও Native `SendInput` (Unicode) ব্যবহার করে ইভেন্ট হ্যান্ডল করা হয়।

2. **সংঘাতমুক্ত ল্যাঙ্গুয়েজ সুইচ শর্টকাট (Conflict-Free Shortcuts)**:
   - **`Ctrl + \`** (ডিফল্ট ও রিকমেন্ডেড): কোনো অ্যাপের শর্টকাটের সাথে সংঘর্ষ হয় না।
   - **`Ctrl + F12`**: ব্রাউজার DevTools বা Inspector চালু না করেই নিরাপদে সুইচ করার সুবিধা।

3. **অডিও ফিডব্যাক (Audio Confirmation)**:
   - কিবোর্ড মোড পরিবর্তন করলে মিষ্টি বীপ টোন শোনা যায়:
     - 🎵 **High Beep (880 Hz)** = UniBijoy (বাংলা) চালু
     - 🎵 **Low Beep (440 Hz)** = English চালু

4. **100% Pure Rust - Zero External Dependencies**:
   - কোনো এক্সটার্নাল ক্রেটের ওপর নির্ভরশীল নয় (Direct Win32 API FFI)।
   - মাত্র **১৪০ KB** ওজনের আল্ট্রা-লাইট সিঙ্গেল এক্সিকিউটেবল ফাইল।
   - মাত্র ৫ সেকেন্ডে কম্পাইল হয়।

5. **নির্ভুল ইউনিবিজয় টাইপিং ইঞ্জিন (Avro UniBijoy Standard)**:
   - স্বরবর্ণ (লিঙ্ক কি `g` + কার বা ডিরেক্ট Shift+F)
   - কার চিহ্ন (া, ি, ী, ু, ূ, ৃ, ে, ৈ, ো, ৌ)
   - যুক্তাক্ষর (Consonant + `g` + Consonant -> ক্ক, ত্ত, ব্ব ইত্যাদি)
   - রেফ (Shift+A = র্) এবং ফলা (z = ্র, Shift+Z = ্য)
   - দাঁড়ি (Shift+G = ।), টাকা চিহ্ন (Shift+4 = ৳), খন্ড-ত (Shift+7 = ৎ)
   - ZWNJ (` ` `) এবং ZWJ (`~`)

---

## 🚀 কীভাবে চালাবেন (How to Run)

### ১. সোর্স কোড থেকে রান করতে:
```bash
cargo run --release
```

### ২. সরাসরি কম্পাইল করা `.exe` ফাইল চালাতে:
`target\release\bnkeyboard.exe` ডাবল-ক্লিক করলেই চলবে!

---

## ⌨️ টাইপিং গাইড (UniBijoy Typing Cheat-Sheet)

### ১. স্বরবর্ণ (Vowels)
- **অ** = `Shift + F`
- **আ** = `g` + `f` (অথবা `AltGr + F`)
- **ই** = `g` + `d` (অথবা `AltGr + D`)
- **ঈ** = `g` + `D` (Shift+D)
- **উ** = `g` + `s` (অথবা `AltGr + S`)
- **ঊ** = `g` + `S` (Shift+S)
- **ঋ** = `g` + `a` (অথবা `AltGr + A`)
- **এ** = `g` + `c` (অথবা `AltGr + C`)
- **ঐ** = `g` + `C` (Shift+C)
- **ও** = `g` + `x` (অথবা `AltGr + X`)
- **ঔ** = `g` + `X` (Shift+X)

### ২. কার চিহ্ন (Vowel Signs)
- `f` = া
- `d` = ি, `D` (Shift+d) = ী
- `s` = ু, `S` (Shift+s) = ূ
- `a` = ৃ
- `c` = ে, `C` (Shift+c) = ৈ
- `x` = ো, `X` (Shift+x) = ৌ

### ৩. যুক্তাক্ষর (Conjuncts)
Bijoy এর নিয়ম অনুযায়ী consonant + `g` + consonant চাপুন:
- **ক্ক** = `j` + `g` + `j`
- **ত্ত** = `k` + `g` + `k`
- **ব্ব** = `h` + `g` + `h`
- **স্কুল** = `n` (স) + `g` + `j` (ক) + `s` (ু) + `Shift+v` (ল)

### ৪. ফলা এবং রেফ
- **রেফ (র্)** = `Shift + A` (যেমন: `Shift+A` + `m` = র্ম)
- **র-ফলা (্র)** = `z` (যেমন: `j` + `z` = ক্র)
- **য-ফলা (্য)** = `Shift + Z` (যেমন: `j` + `Shift+Z` = ক্য)

### ৫. অন্যান্য চিহ্ন
- **দাঁড়ি (।)** = `Shift + G`
- **টাকা (৳)** = `Shift + 4`
- **খন্ড-ত (ৎ)** = `Shift + 7`
- **অনুস্বার (ং)** = `q`
- **বিসর্গ (ঃ)** = `Shift + 6`
- **চন্দ্রবিন্দু (ঁ)** = `Shift + 2`
- **ZWNJ / ZWJ** = `` ` `` / `~`

---

## 🛠 প্রজেক্ট আর্কিটেকচার (Developer Architecture)

```
bnkeyboard/
├── Cargo.toml          # প্যাকেজ কনফিগারেশন
├── src/
│   ├── main.rs         # এন্ট্রি পয়েন্ট, কনসোল UI, উইন্ডোজ মেসেজ লুপ ও সিগন্যাল হ্যান্ডলার
│   ├── lib.rs          # লাইব্রেরি এক্সপোর্ট
│   ├── win32.rs        # Pure Win32 API FFI (Hook, SendInput, Beep, Virtual Terminal)
│   ├── layout.rs       # UniBijoy Unicode কি-ম্যাপিং এবং লিঙ্ক ভাউয়েল রুলস
│   ├── engine.rs       # স্টেটফুল টাইপিং ইঞ্জিন (Link mode, Backspace, Audio feedback)
│   └── hook.rs         # লো-লেভেল গ্লোবাল কীবোর্ড হুক ও হটকি ফিল্টারিং
└── tests/
    └── layout_tests.rs # সম্পূর্ণ ইউনিবিজয় লেআউট টেস্ট স্যুট
```

### টেস্ট রান করতে:
```bash
cargo test
```


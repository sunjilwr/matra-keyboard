# ⌨️ Matra (মাত্রা) - Ultra-Lightweight Bangla & English Keyboard

<p align="center">
  <img src="assets/logo.png" alt="Matra Logo" width="128" height="128" />
  <br>
  <b>Matra (মাত্রা)</b> হলো উইন্ডোজের জন্য সম্পূর্ণ <b>Pure Rust</b> দিয়ে তৈরি একটি আধুনিক, দ্রুতগতির এবং ওপেন সোর্স সিস্টেম-ওয়াইড বাংলা কীবোর্ড।
  <br>
  <i>Zero external crates • 0ms latency • No bloatware • Pure Win32 API</i>
</p>

<p align="center">
  <a href="https://github.com/wrsunjil/matra-keyboard/releases"><img src="https://img.shields.io/badge/Download-Installer%20v1.0.0-emerald?style=for-the-badge&logo=windows" alt="Download Matra" /></a>
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="MIT License" />
</p>

---

## ✨ প্রধান বৈশিষ্ট্যসমূহ (Key Features)

1. **⚡ আল্ট্রা-লাইট ও সুপার-ফাস্ট (Pure Rust & Win32):**
   - কোনো ভারী ফ্রেমওয়ার্ক বা অতিরিক্ত ডিপেনডেন্সি নেই। সাইজ মাত্র কয়েকশো কিলোবাইট!
   - সিস্টেমের মেমোরি ও সিপিইউ ব্যবহার প্রায় ০%।

2. **🖥️ কার্সরের পাশে আধুনিক ফ্লোটিং ব্যাজ (Sleek Floating OSD):**
   - ভাষা পরিবর্তনের সময় কোনো শব্দ না করে কার্সরের ঠিক পাশে অত্যন্ত দৃষ্টিনন্দন **`BN`** (মিন্ট গ্রিন) অথবা **`EN`** (স্কাই ব্লু) পিল ব্যাজ চোখের পলকে ভেসে উঠে মিলিয়ে যাবে।

3. **🔄 অ্যাপ অনুযায়ী স্বাধীন ভাষা (Per-Window Independent State):**
   - এক সফটওয়্যার থেকে অন্য সফটওয়্যারে গেলে আগের ভাষা স্বয়ংক্রিয়ভাবে মনে রাখে।
   - যেমন: MS Word-এ বাংলা লিখে ব্রাউজার বা VS Code-এ গেলে স্বয়ংক্রিয়ভাবে ইংরেজি থাকবে। আবার Word-এ ফিরলে নিজে নিজেই বাংলা ফিরে আসবে!

4. **🛡️ সংঘাতমুক্ত শর্টকাট (Conflict-Free Shortcut):**
   - **`Ctrl + \`** (ডিফল্ট ও রিকমেন্ডেড): উইন্ডোজের কোনো সফটওয়্যার বা ব্রাউজারের সাথে কনফ্লিক্ট করে না।
   - এছাড়াও `Ctrl+F12`, `F12`, `Alt+\`, বা `Ctrl+Space` নির্বাচন করার সুবিধা।

5. **🎯 পারফেক্ট ইউনিকোড বাংলা লেআউট (Accurate Bangla Engine):**
   - **"ফ্রেন্ডলি"** = `Shift+R` (ফ) + `z` (্র) + `c` (ে) + `b` (ন) + `g` (্) + `e` (ড) + `Shift+V` (ল) + `d` (ি)
   - **"কর্ম"** = `j` (ক) + `m` (ম) + `Shift+A` (র্) — স্বাভাবিক টাইপিং অভ্যাসের সাথে শতভাগ সামঞ্জস্যপূর্ণ।
   - **"সেই"** = `n` (স) + `c` (ে) + `g` + `d` (ই)।
   - **ক্যাপস লক সুরক্ষা:** অসাবধানতাবশত ক্যাপস লক চাপ লাগলেও বাংলা টাইপিং নষ্ট হবে না।

6. **📌 সিস্টেম ট্রে ও স্টার্টআপ সাপোর্ট (System Tray & Auto-Start):**
   - কোনো কালো কনসোল উইন্ডো ছাড়া নিঃশব্দে ব্যাকগ্রাউন্ডে ও সিস্টেম ট্রে-তে চলবে।
   - ট্রে আইকনে এক ক্লিকেই ভাষা বদল, সেটিংস ও প্রস্থান করার সুবিধা।

---

## 💾 ইনস্টলেশন গাইড (Download & Installation)

### সহজ ইনস্টলেশন (Recommended):
1. [Releases](https://github.com/wrsunjil/matra-keyboard/releases) সেকশন থেকে **`MatraSetup-v1.0.0.exe`** ডাউনলোড করুন।
2. ইনস্টলারে ডাবল ক্লিক করে ইনস্টল করুন। 
3. ব্যস! স্টার্টআপেই মাত্রা স্বয়ংক্রিয়ভাবে সিস্টেম ট্রে-তে প্রস্তুত থাকবে।

### সোর্স থেকে বিল্ড করতে চাইলে (For Developers):
```bash
git clone https://github.com/wrsunjil/matra-keyboard.git
cd matra-keyboard
cargo build --release
```
তৈরি হওয়া `target/release/matra.exe` সরাসরি যেকোনো জায়গায় নিয়ে চালানো যাবে।

---

## ⌨️ টাইপিং গাইড (Typing Cheat-Sheet)

| বর্ণ / চিহ্ন | কী-বোর্ড স্ট্রোক |
| :--- | :--- |
| **অ** | `Shift + F` |
| **আ** | `g + f` (বা `AltGr + F`) |
| **ই, ঈ** | `g + d`, `g + Shift+D` |
| **উ, ঊ** | `g + s`, `g + Shift+S` |
| **ঋ** | `g + a` |
| **এ, ঐ** | `g + c`, `g + Shift+C` |
| **ও, ঔ** | `g + x`, `g + Shift+X` |
| **কার চিহ্ন (া, ি, ী...)** | `f`=া, `d`=ি, `D`=ী, `s`=ু, `S`=ূ, `a`=ৃ, `c`=ে, `C`=ৈ, `x`=ো, `X`=ৌ |
| **রেফ (র্)** | `Shift + A` (যেমন: `j + m + Shift+A` = **কর্ম**) |
| **র-ফলা / য-ফলা** | `z` = ্র (যেমন: `Shift+R + z` = **ফ্র**), `Shift + Z` = ্য |
| **যুক্তাক্ষর** | বর্ণ + `g` + বর্ণ (যেমন: `j + g + j` = **ক্ক**, `b + g + e` = **ন্ড**) |
| **দাঁড়ি (।)** | `Shift + G` |
| **টাকা (৳)** | `Shift + 4` |
| **খন্ড-ত (ৎ)** | `Shift + 7` |
| **অনুস্বার (ং) / বিসর্গ (ঃ)** | `q` = ং, `Shift + 6` = ঃ |
| **চন্দ্রবিন্দু (ঁ)** | `Shift + 2` |

---

## ⚙️ কনফিগারেশন (`config.ini`)

অ্যাপের সাথে থাকা `config.ini` ফাইলে সেটিংস পরিবর্তন করা যায়:
```ini
shortcut = Ctrl+\        # টগল শর্টকাট (Ctrl+\, Ctrl+F12, F12, Alt+\, Ctrl+Space)
sound = false            # শব্দ সম্পূর্ণ নীরব রাখতে false (ডিফল্ট: false)
osd_badge = true         # কার্সরের পাশে ভাসমান BN/EN ব্যাজ (ডিফল্ট: true)
per_window_mode = true   # প্রতি অ্যাপে আলাদা ভাষা মনে রাখা (ডিফল্ট: true)
typing_style = Classic   # Classic (ি+ক = কি) বা Modern (ক+ি = কি)
start_hidden = true      # কনসোল উইন্ডো ছাড়া সরাসরি ব্যাকগ্রাউন্ডে চালু
```

---

## 🤝 অবদান (Contributing)

এটি একটি ওপেন সোর্স প্রজেক্ট। যেকোনো বাগ রিপোর্ট, ফিচার রিকোয়েস্ট বা Pull Request সাদরে গ্রহণযোগ্য!

## 📜 লাইসেন্স (License)

এই প্রজেক্টটি [MIT License](LICENSE) এর আওতাধীন।

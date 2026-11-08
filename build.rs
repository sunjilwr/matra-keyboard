fn main() {
    println!("cargo:rerun-if-changed=borno.rc");
    println!("cargo:rerun-if-changed=assets/icon.ico");

    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("windows") {
        let rc_candidates = [
            r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\rc.exe",
            r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\rc.exe",
        ];

        for rc_path in rc_candidates {
            if std::path::Path::new(rc_path).exists() {
                let out_dir = std::env::var("OUT_DIR").unwrap();
                let res_file = format!("{}\\borno.res", out_dir);
                let status = std::process::Command::new(rc_path)
                    .args(&["/fo", &res_file, "borno.rc"])
                    .status();

                if let Ok(s) = status {
                    if s.success() {
                        println!("cargo:rustc-link-arg={}", res_file);
                        break;
                    }
                }
            }
        }
    }
}

#!/usr/bin/env python3
import os
import zipfile

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DIST_DIR = os.path.join(PROJECT_ROOT, "dist")
os.makedirs(DIST_DIR, exist_ok=True)

zip_path = os.path.join(DIST_DIR, "Matra-macOS-v1.0.0.zip")

with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
    def add_data(arcname, data, mode=0o644):
        zi = zipfile.ZipInfo(arcname)
        zi.external_attr = (mode & 0xFFFF) << 16
        zf.writestr(zi, data)

    # Info.plist
    info_plist_path = os.path.join(PROJECT_ROOT, "packaging", "macos", "Info.plist")
    with open(info_plist_path, "rb") as f:
        add_data("Matra-macOS/Matra.app/Contents/Info.plist", f.read().replace(b"\r\n", b"\n"), 0o644)

    # Logo
    logo_path = os.path.join(PROJECT_ROOT, "assets", "logo.png")
    with open(logo_path, "rb") as f:
        add_data("Matra-macOS/Matra.app/Contents/Resources/matra.png", f.read(), 0o644)

    # Install script
    install_path = os.path.join(PROJECT_ROOT, "packaging", "macos", "install.sh")
    with open(install_path, "rb") as f:
        add_data("Matra-macOS/install.sh", f.read().replace(b"\r\n", b"\n"), 0o755)

    # README
    readme_path = os.path.join(PROJECT_ROOT, "README.md")
    with open(readme_path, "rb") as f:
        add_data("Matra-macOS/README.md", f.read().replace(b"\r\n", b"\n"), 0o644)

print(f"Generated: {zip_path} ({os.path.getsize(zip_path)} bytes)")


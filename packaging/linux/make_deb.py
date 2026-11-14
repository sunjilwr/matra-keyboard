#!/usr/bin/env python3
"""
Python script to generate a standard Debian .deb package and Linux tarball
without needing dpkg-deb or Linux tools.
"""

import os
import tarfile
import io
import time

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
DIST_DIR = os.path.join(PROJECT_ROOT, "dist")
os.makedirs(DIST_DIR, exist_ok=True)

BINARY_PATH = os.path.join(PROJECT_ROOT, "target", "x86_64-unknown-linux-musl", "release", "matra")
if not os.path.isfile(BINARY_PATH):
    raise FileNotFoundError(f"Linux binary not found at: {BINARY_PATH}")

with open(BINARY_PATH, "rb") as f:
    bin_data = f.read()

LOGO_PATH = os.path.join(PROJECT_ROOT, "assets", "logo.png")
with open(LOGO_PATH, "rb") as f:
    logo_data = f.read()

DESKTOP_PATH = os.path.join(PROJECT_ROOT, "packaging", "linux", "matra.desktop")
with open(DESKTOP_PATH, "rb") as f:
    desktop_data = f.read()

INSTALL_SH_PATH = os.path.join(PROJECT_ROOT, "packaging", "linux", "install.sh")
with open(INSTALL_SH_PATH, "rb") as f:
    install_sh_data = f.read().replace(b"\r\n", b"\n")

CONTROL_PATH = os.path.join(PROJECT_ROOT, "packaging", "linux", "control")
with open(CONTROL_PATH, "rb") as f:
    control_data = f.read().replace(b"\r\n", b"\n")

POSTINST_PATH = os.path.join(PROJECT_ROOT, "packaging", "linux", "postinst")
with open(POSTINST_PATH, "rb") as f:
    postinst_data = f.read().replace(b"\r\n", b"\n")

README_PATH = os.path.join(PROJECT_ROOT, "README.md")
with open(README_PATH, "rb") as f:
    readme_data = f.read().replace(b"\r\n", b"\n")

current_time = int(time.time())

# 1. Create matra-linux-x64.tar.gz
tar_gz_path = os.path.join(DIST_DIR, "matra-linux-x64.tar.gz")
with tarfile.open(tar_gz_path, "w:gz") as tar:
    def add_file(arcname, data, mode=0o755):
        ti = tarfile.TarInfo(name=arcname)
        ti.size = len(data)
        ti.mtime = current_time
        ti.mode = mode
        ti.uname = "root"
        ti.gname = "root"
        tar.addfile(ti, io.BytesIO(data))

    add_file("matra-linux-x64/matra", bin_data, 0o755)
    add_file("matra-linux-x64/matra.desktop", desktop_data, 0o644)
    add_file("matra-linux-x64/matra.png", logo_data, 0o644)
    add_file("matra-linux-x64/install.sh", install_sh_data, 0o755)
    add_file("matra-linux-x64/README.md", readme_data, 0o644)

print(f"Generated: {tar_gz_path} ({os.path.getsize(tar_gz_path)} bytes)")

# 2. Build Debian package (ar archive: debian-binary, control.tar.gz, data.tar.gz)
# A) debian-binary
debian_binary_bytes = b"2.0\n"

# B) control.tar.gz
control_buf = io.BytesIO()
with tarfile.open(fileobj=control_buf, mode="w:gz") as tar:
    ti = tarfile.TarInfo(name="./control")
    ti.size = len(control_data)
    ti.mtime = current_time
    ti.mode = 0o644
    ti.uname = "root"
    ti.gname = "root"
    tar.addfile(ti, io.BytesIO(control_data))

    ti = tarfile.TarInfo(name="./postinst")
    ti.size = len(postinst_data)
    ti.mtime = current_time
    ti.mode = 0o755
    ti.uname = "root"
    ti.gname = "root"
    tar.addfile(ti, io.BytesIO(postinst_data))
control_tar_gz = control_buf.getvalue()

# C) data.tar.gz
data_buf = io.BytesIO()
with tarfile.open(fileobj=data_buf, mode="w:gz") as tar:
    def add_dir(name, mode=0o755):
        ti = tarfile.TarInfo(name=name)
        ti.type = tarfile.DIRTYPE
        ti.mtime = current_time
        ti.mode = mode
        ti.uname = "root"
        ti.gname = "root"
        tar.addfile(ti)

    def add_data_file(name, data, mode=0o644):
        ti = tarfile.TarInfo(name=name)
        ti.size = len(data)
        ti.mtime = current_time
        ti.mode = mode
        ti.uname = "root"
        ti.gname = "root"
        tar.addfile(ti, io.BytesIO(data))

    add_dir("./usr")
    add_dir("./usr/bin")
    add_data_file("./usr/bin/matra", bin_data, 0o755)

    add_dir("./usr/share")
    add_dir("./usr/share/applications")
    add_data_file("./usr/share/applications/matra.desktop", desktop_data, 0o644)

    add_dir("./usr/share/icons")
    add_dir("./usr/share/icons/hicolor")
    add_dir("./usr/share/icons/hicolor/512x512")
    add_dir("./usr/share/icons/hicolor/512x512/apps")
    add_data_file("./usr/share/icons/hicolor/512x512/apps/matra.png", logo_data, 0o644)

    add_dir("./usr/share/doc")
    add_dir("./usr/share/doc/matra")
    add_data_file("./usr/share/doc/matra/README.md", readme_data, 0o644)

data_tar_gz = data_buf.getvalue()

# D) Assemble .deb as AR archive
# AR format spec:
# Global header: `!<arch>\n`
# File header:
# `name` (16 bytes, space padded, slash terminated)
# `mtime` (12 bytes)
# `owner` (6 bytes)
# `group` (6 bytes)
# `mode` (8 bytes, octal)
# `size` (10 bytes)
# `trailer` (2 bytes, `\x60\n`)
# followed by file content, padded with '\n' to even bytes
def make_ar_member(name, content):
    filename = (name + "/").ljust(16)
    mtime = str(current_time).ljust(12)
    owner = "0".ljust(6)
    group = "0".ljust(6)
    mode = "100644".ljust(8)
    size = str(len(content)).ljust(10)
    header = (filename + mtime + owner + group + mode + size + "`\n").encode("ascii")
    payload = content
    if len(payload) % 2 != 0:
        payload += b"\n"
    return header + payload

deb_path = os.path.join(DIST_DIR, "matra_1.0.0_amd64.deb")
with open(deb_path, "wb") as f:
    f.write(b"!<arch>\n")
    f.write(make_ar_member("debian-binary", debian_binary_bytes))
    f.write(make_ar_member("control.tar.gz", control_tar_gz))
    f.write(make_ar_member("data.tar.gz", data_tar_gz))

print(f"Generated: {deb_path} ({os.path.getsize(deb_path)} bytes)")


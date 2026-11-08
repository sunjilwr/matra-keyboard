@echo off
title UniBijoy Keyboard (Rust)
cd /d "%~dp0"
if exist "target\release\bnkeyboard.exe" (
    start "" "target\release\bnkeyboard.exe" %*
) else (
    echo Building bnkeyboard release binary...
    cargo build --release
    start "" "target\release\bnkeyboard.exe" %*
)

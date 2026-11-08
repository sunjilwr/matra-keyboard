@echo off
title Borno (বর্ণ) Keyboard
cd /d "%~dp0"
if exist "target\release\borno.exe" (
    start "" "target\release\borno.exe" %*
) else (
    echo Building Borno release binary...
    cargo build --release
    start "" "target\release\borno.exe" %*
)

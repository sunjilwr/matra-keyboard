@echo off
title Matra (মাত্রা) Keyboard
cd /d "%~dp0"
if exist "target\release\matra.exe" (
    start "" "target\release\matra.exe" %*
) else (
    echo Building Matra release binary...
    cargo build --release
    start "" "target\release\matra.exe" %*
)

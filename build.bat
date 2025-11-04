@echo off
echo Building X4 Trading System...
powershell -Command "& 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\Launch-VsDevShell.ps1' -Arch amd64; cargo build --release"
echo Build complete!
pause

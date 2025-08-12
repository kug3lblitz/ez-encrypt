@echo off
REM Ez-Encrypt Multi-Platform Build Script for Windows
REM This script builds the application for Windows, macOS, and Linux

setlocal enabledelayedexpansion

echo 🔨 Ez-Encrypt Multi-Platform Build Script (Windows)
echo =================================================

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust/Cargo not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo [INFO] Rust version:
rustc --version

REM Create builds directory
set BUILDS_DIR=builds
if not exist "%BUILDS_DIR%" (
    mkdir "%BUILDS_DIR%"
    echo [INFO] Created builds directory
)

REM Clean previous builds
echo [INFO] Cleaning previous builds...
cargo clean
if exist "%BUILDS_DIR%\*" del /q "%BUILDS_DIR%\*" >nul 2>nul
for /d %%i in ("%BUILDS_DIR%\*") do rmdir /s /q "%%i" >nul 2>nul

echo [INFO] Starting builds for 3 platforms...
echo.

set successful_builds=0
set total_builds=3

REM Build for Windows
echo [INFO] Building for Windows (x86_64-pc-windows-msvc)...
rustup target list --installed | find "x86_64-pc-windows-msvc" >nul
if %errorlevel% neq 0 (
    echo [INFO] Installing target x86_64-pc-windows-msvc...
    rustup target add x86_64-pc-windows-msvc
)

cargo build --release --target x86_64-pc-windows-msvc
if %errorlevel% equ 0 (
    mkdir "%BUILDS_DIR%\Windows" >nul 2>nul
    copy "target\x86_64-pc-windows-msvc\release\ez-encrypt.exe" "%BUILDS_DIR%\Windows\" >nul
    echo [SUCCESS] Built Windows successfully -^> %BUILDS_DIR%\Windows\ez-encrypt.exe
    set /a successful_builds+=1
) else (
    echo [ERROR] Build failed for Windows
)
echo.

REM Build for Linux
echo [INFO] Building for Linux (x86_64-unknown-linux-gnu)...
rustup target list --installed | find "x86_64-unknown-linux-gnu" >nul
if %errorlevel% neq 0 (
    echo [INFO] Installing target x86_64-unknown-linux-gnu...
    rustup target add x86_64-unknown-linux-gnu
)

cargo build --release --target x86_64-unknown-linux-gnu
if %errorlevel% equ 0 (
    mkdir "%BUILDS_DIR%\Linux" >nul 2>nul
    copy "target\x86_64-unknown-linux-gnu\release\ez-encrypt" "%BUILDS_DIR%\Linux\" >nul
    echo [SUCCESS] Built Linux successfully -^> %BUILDS_DIR%\Linux\ez-encrypt
    set /a successful_builds+=1
) else (
    echo [ERROR] Build failed for Linux
)
echo.

REM Build for macOS (Note: This is challenging on Windows)
echo [INFO] Building for macOS (x86_64-apple-darwin)...
echo [WARNING] macOS cross-compilation from Windows can be complex and may fail
rustup target list --installed | find "x86_64-apple-darwin" >nul
if %errorlevel% neq 0 (
    echo [INFO] Installing target x86_64-apple-darwin...
    rustup target add x86_64-apple-darwin
)

cargo build --release --target x86_64-apple-darwin
if %errorlevel% equ 0 (
    mkdir "%BUILDS_DIR%\macOS" >nul 2>nul
    copy "target\x86_64-apple-darwin\release\ez-encrypt" "%BUILDS_DIR%\macOS\" >nul
    echo [SUCCESS] Built macOS successfully -^> %BUILDS_DIR%\macOS\ez-encrypt
    set /a successful_builds+=1
) else (
    echo [WARNING] Build failed for macOS (expected on Windows without additional setup)
)
echo.

REM Summary
echo =================================================
echo [INFO] Build Summary:
echo [INFO] Successful builds: !successful_builds!/!total_builds!

if !successful_builds! equ !total_builds! (
    echo [SUCCESS] All builds completed successfully! 🎉
) else (
    echo [WARNING] Some builds failed. Check the output above for details.
)

REM List created files
echo.
echo [INFO] Created files:
if exist "%BUILDS_DIR%" (
    for /r "%BUILDS_DIR%" %%f in (ez-encrypt*) do (
        echo   %%f
    )
)

echo.
echo [INFO] Build script completed!

echo.
echo 📋 Platform-specific notes:
echo   Windows: Use builds\Windows\ez-encrypt.exe
echo   macOS:   Use builds\macOS\ez-encrypt (may need: chmod +x ez-encrypt)
echo   Linux:   Use builds\Linux\ez-encrypt (may need: chmod +x ez-encrypt)
echo.
echo 💡 For distribution:
echo   1. Test each executable on its target platform
echo   2. Consider code signing for Windows/macOS
echo   3. Package with README and license files
echo.
echo 🔧 Note for Windows users:
echo   - macOS builds from Windows require additional setup (OSX SDK)
echo   - Consider using GitHub Actions for reliable cross-platform builds
echo   - Linux builds should work but may have runtime dependencies

pause

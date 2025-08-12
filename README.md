# Ez-Encrypt 🔐

Ez-Encrypt is a simple, cross-platform file encryption/decryption application designed to be completely user-friendly. No technical knowledge required!

## Features

- **Simple Interface**: Just 3 buttons - Encrypt, Decrypt, Exit
- **Secure Encryption**: AES-256 encryption with SHA-256 key derivation
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Idiot-Friendly**: Place executable in your file directory and go!
- **Password Protection**: Secure your files with a password of your choice

## How to Use

### For End Users

1. **Download the executable** for your operating system
   - Windows: `ez-encrypt.exe`
   - macOS/Linux: `ez-encrypt`

2. **Place the executable** in the same directory as the files you want to encrypt/decrypt

3. **Run the application**
   - Windows: Double-click `ez-encrypt.exe`
   - macOS/Linux: Run `./ez-encrypt` in terminal or double-click if executable permissions are set

4. **Use the interface**:
   - Enter your password in the text field
   - Click **🔒 Encrypt** to encrypt a file
   - Click **🔓 Decrypt** to decrypt a `.enc` file
   - Click **❌ Exit** to close the application

### Encryption Process
- Select any file from your directory
- The encrypted file will be saved with a `.enc` extension
- Example: `document.txt` becomes `document.txt.enc`

### Decryption Process
- Select any `.enc` file from your directory
- Enter the same password used for encryption
- The decrypted file will restore the original filename
- Example: `document.txt.enc` becomes `document.txt`

## Security Details

- **AES-256-CTR**: Industry-standard encryption algorithm
- **Random Salt**: Each encryption uses a unique salt for key derivation
- **Random IV**: Each encryption uses a unique initialization vector
- **SHA-256 Key Derivation**: Your password is securely converted to an encryption key
- **Self-Contained**: Salt and IV are stored with the encrypted file (no separate key files needed)

## Developer Information

### Building from Source

#### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

#### Build Steps
```bash
# Clone the repository
git clone <repository-url>
cd ez-encrypt

# Build release version
cargo build --release

# The executable will be in target/release/
# - Windows: target/release/ez-encrypt.exe
# - macOS/Linux: target/release/ez-encrypt
```

#### Running in Development
```bash
# Run directly with Cargo
cargo run

# Or build and run debug version
cargo build
./target/debug/ez-encrypt
```

### Easy Multi-Platform Builds

For convenience, build scripts are provided to easily create executables for all platforms:

#### Using Build Scripts (Recommended)

**On Windows:**
```cmd
# Run the Windows batch script
build-all.bat
```

**On macOS/Linux:**
```bash
# Make script executable and run
chmod +x build-all.sh
./build-all.sh
```

Both scripts will:
- Install required Rust targets automatically
- Build for Windows, macOS, and Linux
- Create organized `builds/` directory with platform-specific folders
- Show build status and file sizes

**What are .bat files?**
- `.bat` files are Windows batch scripts (like shell scripts for Windows)
- They run commands in Windows Command Prompt automatically
- Double-click to run, or execute from Command Prompt/PowerShell
- Similar to `.sh` files on Linux/macOS, but for Windows

#### Using GitHub Actions (Best for Releases)

Push your code to GitHub and the automated workflow will:
- Build for all platforms simultaneously
- Run tests and quality checks
- Create release archives
- Automatically create releases when you tag versions

To create a release:
```bash
git tag v1.0.0
git push origin v1.0.0
```

The workflow will automatically:
- Build on native Windows, macOS, and Linux systems
- Run comprehensive tests
- Create platform-specific archives (.zip for Windows, .tar.gz for Unix)
- Upload release assets to GitHub
- Generate release notes

### Testing the Application

1. **Create test files** in the project directory
2. **Run the application** using `cargo run`
3. **Test encryption**:
   - Enter a password
   - Click Encrypt
   - Select your test file
   - Verify the `.enc` file is created
4. **Test decryption**:
   - Enter the same password
   - Click Decrypt
   - Select the `.enc` file
   - Verify the original file is restored

### Project Structure
```
ez-encrypt/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Rust dependencies
├── Cargo.lock           # Dependency lock file
├── README.md            # This file
└── target/              # Build output directory
```

### Dependencies
- `aes`: AES encryption implementation
- `ctr`: Counter mode for AES
- `sha2`: SHA-256 hashing for key derivation
- `rand`: Cryptographically secure random number generation
- `eframe`: Cross-platform GUI framework
- `rfd`: Native file dialogs
- `base64`: Base64 encoding (if needed for future features)

### Building on Windows

#### Prerequisites for Windows
- Install Rust from [rustup.rs](https://rustup.rs/)
- On Windows, you may need Visual Studio Build Tools or Visual Studio with C++ development tools
- Alternative: Install via `winget install Rustlang.Rustup` (Windows 10/11)
- may also need to do this:
```
  rustup target add x86_64-pc-windows-gnu
  cargo build --release --target x86_64-pc-windows-gnu
```

#### Prerequisites for Linux
**Arch Linux:**
```bash
# Install Rust and development tools
sudo pacman -S rust cargo base-devel gtk3 webkit2gtk
```

**Gentoo:**
```bash
# Install Rust (if not using rustup)
sudo emerge dev-lang/rust

# Required dependencies
sudo emerge x11-libs/gtk+ net-libs/webkit-gtk dev-util/pkgconfig
```

**Ubuntu/Debian:**
```bash
# Install dependencies
sudo apt-get install build-essential libgtk-3-dev libwebkit2gtk-4.0-dev
```

**General Linux (using rustup - recommended):**
```bash
# Install rustup (cross-distribution)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Windows Build Commands
```cmd
# Command Prompt or PowerShell
cargo build --release

# The executable will be: target\release\ez-encrypt.exe
```

#### Windows-Specific Notes
- The executable will be `ez-encrypt.exe`
- You may need to allow the app through Windows Defender when first running
- Place `ez-encrypt.exe` in the same folder as your files to encrypt/decrypt
- Double-click to run, or use Command Prompt: `ez-encrypt.exe`

### Cross-Platform Compilation

#### For Windows (from macOS/Linux):
```bash
# For Windows 64-bit (GNU toolchain)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Alternative for better Windows compatibility (MSVC toolchain):
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

**Cross-compilation notes:**
- **GNU target**: Works on most Linux distributions, requires mingw-w64
- **MSVC target**: Better Windows compatibility, requires Visual Studio Build Tools
- **Arch Linux**: `sudo pacman -S mingw-w64-gcc` for GNU target
- **Gentoo**: `sudo emerge cross-x86_64-w64-mingw32/gcc` for GNU target

#### For macOS (from Linux/Windows):
```bash
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

#### For Linux (from macOS/Windows):
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Security Considerations

### What This Tool Does Well
- Uses industry-standard AES-256 encryption
- Proper key derivation with SHA-256 and salts
- Random IV generation for each encryption
- No hardcoded keys or weak encryption

### What This Tool Doesn't Do
- **Key stretching**: Uses single SHA-256 pass (consider PBKDF2 for high-security needs)
- **Password strength validation**: Users can choose weak passwords
- **Secure memory**: Passwords may remain in memory after use
- **File shredding**: Original files are not securely deleted after encryption

### Recommendations for High-Security Use
- Use strong, unique passwords
- Manually delete original files after encryption if needed
- Consider additional security measures for highly sensitive data
- Keep backups of important encrypted files

## Troubleshooting

### Common Issues

**Application won't start**
- Ensure you have necessary permissions to run the executable
- On macOS, you may need to allow the app in Security & Privacy settings
- On Windows, you may need to allow through Windows Defender/SmartScreen

**Windows-specific issues**
- **"Windows protected your PC" message**: Click "More info" then "Run anyway"
- **Missing Visual C++ Redistributable**: Install Microsoft Visual C++ Redistributable
- **Antivirus blocking**: Add `ez-encrypt.exe` to your antivirus whitelist
- **Permission denied**: Right-click executable and "Run as administrator"

**Can't decrypt file**
- Verify you're using the exact same password used for encryption
- Ensure the `.enc` file hasn't been corrupted or modified
- Check that the file was encrypted with this application

**File dialog doesn't show files**
- Make sure files are in the same directory as the executable
- For decryption, ensure you're looking for `.enc` files
- On Windows, check file extensions are visible in Explorer (View > File name extensions)

**Linux-specific issues**
- **Missing GUI libraries**: Install GTK3 development packages
  - Arch: `sudo pacman -S gtk3`
  - Gentoo: `sudo emerge x11-libs/gtk+:3`
  - Ubuntu/Debian: `sudo apt install libgtk-3-dev`
- **Wayland compatibility**: Should work, but try X11 session if issues occur
- **Permissions**: Ensure executable bit is set (`chmod +x ez-encrypt`)

### Build Troubleshooting

**Rust installation issues:**
- **Arch Linux**:
  ```bash
  # If system Rust conflicts with rustup
  sudo pacman -R rust
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Gentoo**:
  ```bash
  # Use rustup instead of emerge for latest version
  sudo emerge --unmerge dev-lang/rust
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

**Cross-compilation failures:**

*Windows target from Linux:*
```bash
# Arch Linux - install mingw-w64
sudo pacman -S mingw-w64-gcc mingw-w64-binutils mingw-w64-crt mingw-w64-headers

# Gentoo - install cross-compiler
echo 'CROSSDEV_OVERLAY="/var/db/repos/crossdev"' >> /etc/portage/make.conf
sudo emerge --ask sys-devel/crossdev
sudo crossdev --target x86_64-w64-mingw32

# If linker errors occur:
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
```

*macOS target issues:*
```bash
# macOS cross-compilation often fails due to SDK requirements
# Recommended: Use GitHub Actions or build on actual macOS
echo "macOS cross-compilation requires OSX SDK and additional setup"
echo "Consider using GitHub Actions for reliable macOS builds"
```

**GUI dependency issues:**

*Arch Linux:*
```bash
# Full development environment
sudo pacman -S base-devel gtk3 webkit2gtk-4.1 libxcb pkgconf

# If X11 issues occur
sudo pacman -S xorg-server-devel libxrandr libxinerama libxcursor
```

*Gentoo:*
```bash
# Essential GUI packages
sudo emerge x11-libs/gtk+:3 net-libs/webkit-gtk:4.1 x11-libs/libXcursor

# If compilation fails, update USE flags
echo 'net-libs/webkit-gtk gstreamer' >> /etc/portage/package.use
sudo emerge --ask net-libs/webkit-gtk
```

**Memory/disk space issues:**
```bash
# Large compilation - ensure adequate space and RAM
df -h .  # Check disk space (need ~2GB)
free -h  # Check RAM (recommend 4GB+)

# For low-memory systems, reduce parallel jobs
export CARGO_BUILD_JOBS=1
cargo build --release
```

**Permission and PATH issues:**
```bash
# Ensure Rust tools are in PATH
echo 'source ~/.cargo/env' >> ~/.bashrc
source ~/.bashrc

# Fix cargo/rustup permissions
sudo chown -R $USER:$USER ~/.cargo ~/.rustup

# For system-wide installations
sudo chmod +x /usr/local/bin/ez-encrypt
```

**Specific error solutions:**

*"failed to run custom build command for 'openssl-sys'"*
```bash
# Install OpenSSL development packages
# Arch: sudo pacman -S openssl pkgconf
# Gentoo: sudo emerge dev-libs/openssl pkgconf
# Or use rustls instead: cargo build --no-default-features --features rustls
```

*"linking with `cc` failed"*
```bash
# Install GCC and development tools
# Arch: sudo pacman -S gcc binutils
# Gentoo: sudo emerge sys-devel/gcc sys-devel/binutils
```

*"failed to load source for dependency"*
```bash
# Clean and retry
cargo clean
rm -rf ~/.cargo/registry/index/*
cargo build --release
```

### Getting Help
- Check the console output for error messages when running with `cargo run`
- Ensure all dependencies are properly installed
- Verify file permissions in your target directory

### Build Files Explained
- **build-all.sh**: Unix shell script for macOS/Linux cross-compilation
- **build-all.bat**: Windows batch script for cross-compilation
- **.github/workflows/build.yml**: GitHub Actions for automated CI/CD builds
- Both local scripts create a `builds/` directory with platform-specific executables

### Distribution-Specific Notes
- **Arch Linux**: Rolling release, latest Rust usually available via pacman
- **Gentoo**: Compile-time optimizations available via USE flags
- **Ubuntu/Debian**: May have older Rust versions, rustup recommended for latest
- **Windows**: MSVC toolchain recommended for better compatibility
- **macOS**: Xcode Command Line Tools required for native compilation

## License

This project is open source. Please check the repository for license details.

## Contributing

Contributions are welcome! Please ensure any changes maintain the "idiot-friendly" philosophy of the application.

---

**Made with ❤️ for simple, secure file encryption**

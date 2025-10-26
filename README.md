# Hello GTK Pi

A minimal native GUI application for Raspberry Pi built with Rust, GTK4, and libadwaita.

## Features

- Beautiful native UI with libadwaita styling
- Tiny memory footprint: ~30MB RAM (vs 500MB+ for Electron/browser-based apps)
- Small binary size: ~473KB
- Fast startup time
- Cross-platform: runs on Mac, Linux, and Raspberry Pi

## Tech Stack

- **Rust**: Systems programming language with safety and performance
- **GTK4**: Modern, mature UI toolkit
- **libadwaita**: Beautiful adaptive widgets following GNOME design guidelines

## Project Structure

```
.
├── Cargo.toml          # Rust dependencies and project metadata
├── Cargo.lock          # Locked dependency versions
├── src/
│   └── main.rs         # Main application code
├── target/             # Build output (generated)
├── README.md           # This file
├── DEPLOY.md           # Deployment instructions for Raspberry Pi
└── build-for-pi.sh     # Helper script for Pi deployment
```

## Setup & Installation

### Prerequisites

#### On Mac:

1. **Install Homebrew** (if not already installed):
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

2. **Install Rust**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

3. **Install GTK4 and dependencies**:
```bash
brew install gtk4 libadwaita pkgconf
```

#### On Raspberry Pi (Download Pre-built Binary):

**No compilation needed!** Download pre-built binaries from GitHub Actions:

```bash
# Option 1: Use the download script (easiest)
curl -O https://raw.githubusercontent.com/YOUR_USERNAME/omma-linux/master/download-latest.sh
chmod +x download-latest.sh
./download-latest.sh

# Option 2: Manual download from GitHub releases
# Visit: https://github.com/YOUR_USERNAME/omma-linux/releases/latest
# Download the appropriate binary for your Pi
```

**Manual Raspberry Pi build:** See [DEPLOY.md](DEPLOY.md) for detailed compilation instructions.

## Building the App

### Development Build (faster compilation, larger binary):

```bash
cargo build
```

The binary will be at: `./target/debug/hello-gtk-pi`

### Release Build (optimized, smaller binary):

```bash
cargo build --release
```

The binary will be at: `./target/release/hello-gtk-pi`

**Release builds are recommended for deployment** - they're optimized and much smaller (~473KB vs ~2MB for debug).

## CI/CD - Automated Builds

This project uses **GitHub Actions** to automatically build ARM binaries for Raspberry Pi on every push. This means you **never need to compile on your Pi** - just download the pre-built binary!

### How It Works

1. **Push to `master`** → GitHub Actions automatically builds ARM64 and ARM32 binaries
2. **Create a tag** (e.g., `v1.0.0`) → Automatically creates a GitHub release with binaries
3. **Download on Pi** → Use the download script or grab binaries from GitHub releases

### Available Binaries

After each push to master, two binaries are built:

- **`omma-aarch64-unknown-linux-gnu`** - For Raspberry Pi 3/4/5 (64-bit OS)
- **`omma-armv7-unknown-linux-gnueabihf`** - For Raspberry Pi 2/3 (32-bit OS)

Binaries are available for 30 days as GitHub Actions artifacts.

### Creating a Release

To create a permanent release:

```bash
# Tag your commit
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

This will automatically:
- Build both ARM variants
- Create a GitHub release
- Attach binaries to the release
- Add installation instructions

### On Your Raspberry Pi

```bash
# Install GTK4 dependencies (one-time)
sudo apt-get update
sudo apt-get install -y libgtk-4-1 libadwaita-1-0

# Download and install latest build
./download-latest.sh

# Or manually download from releases
wget https://github.com/YOUR_USERNAME/omma-linux/releases/download/v1.0.0/omma-aarch64-unknown-linux-gnu
chmod +x omma-aarch64-unknown-linux-gnu
sudo mv omma-aarch64-unknown-linux-gnu /usr/local/bin/omma

# Run the app
omma
```

### Why This Matters for Low-RAM Devices

Compiling Rust on a 1GB RAM Raspberry Pi is painful or impossible. GitHub Actions builds use:
- **QEMU emulation** to run ARM containers on x86_64 runners
- **Native ARM compilation** inside the container for compatibility
- **Docker with Debian Bookworm** to match Pi OS environment

This means your Pi just downloads and runs - no compilation needed!

## Running the App

### On Mac:

After building, run:

```bash
# Development build
./target/debug/hello-gtk-pi

# Or release build
./target/release/hello-gtk-pi
```

Or build and run in one command:

```bash
# Development
cargo run

# Release (optimized)
cargo run --release
```

### On Raspberry Pi:

After transferring and building (see [DEPLOY.md](DEPLOY.md)):

```bash
./target/release/hello-gtk-pi
```

## Quick Start (Mac)

If you just cloned this repo and want to see it running:

```bash
# 1. Install dependencies (one-time setup)
brew install gtk4 libadwaita pkgconf

# 2. Build and run
cargo run --release
```

The app window should appear with a "Hello World" message!

## Development Workflow

1. **Edit the code**: Modify `src/main.rs`
2. **Test your changes**: Run `cargo run` to see changes immediately
3. **Build for deployment**: Run `cargo build --release` when ready to deploy

### Useful Commands:

```bash
cargo check           # Quick compile check (no binary)
cargo build           # Build debug version
cargo build --release # Build optimized version
cargo run             # Build and run (debug)
cargo run --release   # Build and run (release)
cargo clean           # Remove build artifacts
```

## Extending This App

The current app is a minimal "Hello World". Here's how to add more functionality:

### Adding a Button

```rust
let button = gtk4::Button::with_label("Click Me");
button.connect_clicked(|_| {
    println!("Button clicked!");
});
content_box.append(&button);
```

### Adding More Pages

```rust
let navigation_view = adw::NavigationView::new();
let page = adw::NavigationPage::builder()
    .title("My Page")
    .child(&content)
    .build();
navigation_view.push(&page);
```

### Adding a List

```rust
let list_box = gtk4::ListBox::new();
list_box.append(&gtk4::Label::new(Some("Item 1")));
list_box.append(&gtk4::Label::new(Some("Item 2")));
content_box.append(&list_box);
```

### Reading Sensor Data

```rust
use std::time::Duration;
use glib::timeout_add_local;

// Update label every second
let label = gtk4::Label::new(Some("Loading..."));
let label_clone = label.clone();
timeout_add_local(Duration::from_secs(1), move || {
    // Read your sensor here
    label_clone.set_text("Sensor value: 42");
    glib::ControlFlow::Continue
});
```

## Troubleshooting

### "command not found: cargo"

Rust isn't installed or not in PATH. Install with:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

### "pkg-config not found" or GTK errors

Install system dependencies:
```bash
# Mac
brew install gtk4 libadwaita pkgconf

# Raspberry Pi / Debian / Ubuntu
sudo apt install libgtk-4-dev libadwaita-1-dev build-essential
```

### App doesn't appear when running

Make sure you're on a system with a graphical display (not SSH without X11 forwarding).

## Binary Size Comparison

- **Debug build**: ~2MB
- **Release build**: ~473KB
- **With strip**: Can reduce further with `strip target/release/hello-gtk-pi`

## Resources

- [GTK4-rs Documentation](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)
- [libadwaita-rs Documentation](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/)
- [GNOME Human Interface Guidelines](https://developer.gnome.org/hig/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## Why This Stack?

- **Native Performance**: Direct system calls, no browser overhead
- **Low Resource Usage**: Perfect for Raspberry Pi's limited resources
- **Beautiful UI**: libadwaita provides modern, polished widgets
- **Type Safety**: Rust prevents many common bugs at compile time
- **Active Ecosystem**: GTK4 and Rust both have great communities
- **Single Binary**: Easy deployment - just copy one file

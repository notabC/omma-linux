# Deploying Hello GTK Pi to Raspberry Pi

## Option 1: Build Directly on Raspberry Pi (Recommended)

This is the simplest approach since cross-compiling GTK apps from Mac to ARM Linux is complex.

### On your Raspberry Pi:

1. **Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

2. **Install GTK4 and libadwaita:**
```bash
sudo apt update
sudo apt install -y libgtk-4-dev libadwaita-1-dev build-essential
```

3. **Transfer the project:**
```bash
# On your Mac, from the omma-linux directory:
scp -r hello-gtk-pi pi@raspberrypi.local:~/

# Or clone if using git:
# git clone <your-repo> && cd hello-gtk-pi
```

4. **Build on Pi:**
```bash
cd hello-gtk-pi
cargo build --release
```

5. **Run it:**
```bash
./target/release/hello-gtk-pi
```

## Option 2: Cross-Compile from Mac (Advanced)

Requires Docker for ARM cross-compilation:

```bash
# Build ARM binary using Docker
docker run --rm -v $(pwd):/project -w /project \
  rust:latest bash -c "
    apt-get update && \
    apt-get install -y libgtk-4-dev libadwaita-1-dev && \
    rustup target add aarch64-unknown-linux-gnu && \
    cargo build --release --target=aarch64-unknown-linux-gnu
  "
```

## Performance Notes

- **Binary size:** ~473KB (release build)
- **RAM usage:** ~30MB (vs 500MB+ for Electron)
- **Startup time:** Nearly instant

## Installing GTK4 Runtime on Raspberry Pi OS

If you get missing library errors:

```bash
sudo apt install -y gtk4 libadwaita-1-0
```

## Auto-start on Boot (Optional)

Create `/home/pi/.config/autostart/hello-gtk-pi.desktop`:

```ini
[Desktop Entry]
Type=Application
Name=Hello GTK Pi
Exec=/home/pi/hello-gtk-pi/target/release/hello-gtk-pi
Terminal=false
```

# Building Omma for Raspberry Pi

Quick reference for building ARM64 binaries for Raspberry Pi.

## Prerequisites

- **Docker Desktop** - Must be running
- **GitHub CLI** (`gh`) - For creating releases
- **Git** - For version control

Install with:
```bash
brew install --cask docker
brew install gh
```

## Quick Build & Release

```bash
./build-and-release-arm64.sh
```

This will:
1. Prompt for a version tag (e.g., `v1.0.2`)
2. Build ARM64 binary using Fedora Docker image (~27 seconds)
3. Create git tag and push to GitHub
4. Create GitHub release with binary attached

## Manual Build (Testing Only)

If you just want to test the build without creating a release:

```bash
docker run --rm --platform linux/arm64 \
  -v "$(pwd)":/workspace \
  -w /workspace \
  fedora:latest \
  bash -c '
    dnf install -y gtk4-devel gcc libadwaita-devel curl && \
    curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    source "$HOME/.cargo/env" && \
    cargo build --release && \
    strip target/release/omma
  '
```

Binary will be at: `target/release/omma`

## Build Performance

**On Apple M4 Pro (48GB RAM, Docker allocated 24GB):**
- Build time: ~27 seconds
- Binary size: ~450KB
- Memory usage: ~2-3GB during build

**Why Fedora?**
- GTK4 packages work perfectly out of the box
- No pkg-config configuration needed
- Matches what we need for Raspberry Pi

**Why Not Debian?**
- GTK4 dependencies have pkg-config issues in Docker
- Would need manual path configuration
- Less reliable for CI/CD

## Troubleshooting

### Docker not running
```bash
open -a Docker
# Wait 30 seconds for Docker to start
```

### Docker low on memory
1. Open Docker Desktop
2. Settings → Resources
3. Increase Memory to 16-24GB
4. Apply & Restart

### GitHub CLI not authenticated
```bash
gh auth login
```

### Build fails with "no space left"
```bash
docker system prune -a
```

## Development Builds (Local Mac)

For testing changes locally on Mac:

```bash
# Quick debug build
cargo run

# Optimized build
cargo run --release
```

Note: Mac builds are for development only. Raspberry Pi needs the ARM64 binary from Docker.

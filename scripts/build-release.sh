#!/bin/bash
set -e

# Cross-platform release build script for Railguard
# Builds binaries for macOS (Intel + ARM) and Linux (x64 + ARM64)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Create bin directory
mkdir -p bin

get_binary_name() {
  local target="$1"
  case "$target" in
    x86_64-apple-darwin) echo "rg-darwin-x64" ;;
    aarch64-apple-darwin) echo "rg-darwin-arm64" ;;
    x86_64-unknown-linux-gnu) echo "rg-linux-x64" ;;
    aarch64-unknown-linux-gnu) echo "rg-linux-arm64" ;;
    *) echo "" ;;
  esac
}

build_target() {
  local target="$1"
  local binary_name
  binary_name=$(get_binary_name "$target")

  if [[ -z "$binary_name" ]]; then
    echo "Unknown target: $target"
    return 1
  fi

  echo "Building for $target..."

  # Check if target is installed
  if ! rustup target list --installed | grep -q "$target"; then
    echo "  Installing target $target..."
    rustup target add "$target" || {
      echo "  Warning: Could not install target $target, skipping"
      return 1
    }
  fi

  # Build
  if cargo build --release --target "$target"; then
    cp "target/$target/release/railguard" "bin/$binary_name"
    echo "  Built: bin/$binary_name"
    return 0
  else
    echo "  Warning: Build failed for $target"
    return 1
  fi
}

# Check if we should build all targets or just the current platform
if [[ "$1" == "--all" ]]; then
  echo "Building for all platforms..."
  echo ""

  for target in x86_64-apple-darwin aarch64-apple-darwin x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu; do
    build_target "$target" || true
  done
else
  # Build for current platform only
  case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)
      TARGET="aarch64-apple-darwin"
      ;;
    Darwin-x86_64)
      TARGET="x86_64-apple-darwin"
      ;;
    Linux-x86_64)
      TARGET="x86_64-unknown-linux-gnu"
      ;;
    Linux-aarch64)
      TARGET="aarch64-unknown-linux-gnu"
      ;;
    *)
      echo "Unsupported platform: $(uname -s)-$(uname -m)"
      exit 1
      ;;
  esac

  build_target "$TARGET"
fi

echo ""
echo "Binaries created in bin/:"
ls -la bin/

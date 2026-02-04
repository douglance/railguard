#!/bin/bash
set -e

# Get the plugin root directory (parent of scripts/)
PLUGIN_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Detect platform and architecture
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)   BINARY="rg-darwin-arm64" ;;
  Darwin-x86_64)  BINARY="rg-darwin-x64" ;;
  Linux-x86_64)   BINARY="rg-linux-x64" ;;
  Linux-aarch64)  BINARY="rg-linux-arm64" ;;
  *)
    echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"Railguard: Unsupported platform","additionalContext":"Build Railguard for your platform or install pre-built binaries."}}'
    exit 0
    ;;
esac

# Check if binary exists
if [[ ! -x "$PLUGIN_ROOT/bin/$BINARY" ]]; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"Railguard binary not found","additionalContext":"Run scripts/build-release.sh to build the binary for your platform."}}'
  exit 0
fi

# Config resolution order: project > user > plugin default
if [[ -f "./railguard.toml" ]]; then
  CONFIG="./railguard.toml"
elif [[ -f "$HOME/.config/railguard/railguard.toml" ]]; then
  CONFIG="$HOME/.config/railguard/railguard.toml"
else
  CONFIG="$PLUGIN_ROOT/railguard.toml"
fi

# Run the binary with stdin passthrough
exec "$PLUGIN_ROOT/bin/$BINARY" hook --config "$CONFIG"

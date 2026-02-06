#!/bin/bash
set -euo pipefail

# Railgun installer
# Usage: curl -fsSL https://raw.githubusercontent.com/douglance/railgun/main/install.sh | bash

REPO="douglance/railgun"
INSTALL_DIR="${HOME}/.local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
    exit 1
}

# Detect platform
detect_platform() {
    local os arch

    case "$(uname -s)" in
        Darwin) os="darwin" ;;
        Linux)  os="linux" ;;
        *)      error "Unsupported operating system: $(uname -s)" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)   arch="x64" ;;
        arm64|aarch64)  arch="arm64" ;;
        *)              error "Unsupported architecture: $(uname -m)" ;;
    esac

    echo "${os}-${arch}"
}

# Get latest release version
get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" |
        grep '"tag_name":' |
        sed -E 's/.*"([^"]+)".*/\1/'
}

main() {
    info "Detecting platform..."
    local platform
    platform=$(detect_platform)
    info "Platform: ${platform}"

    info "Fetching latest version..."
    local version
    version=$(get_latest_version)
    if [[ -z "${version}" ]]; then
        error "Failed to determine latest version"
    fi
    info "Version: ${version}"

    local url="https://github.com/${REPO}/releases/download/${version}/railgun-${platform}.tar.gz"
    local checksum_url="https://github.com/${REPO}/releases/download/${version}/checksums.txt"

    # Create install directory
    mkdir -p "${INSTALL_DIR}"

    # Download and extract
    info "Downloading railgun..."
    local tmpdir
    tmpdir=$(mktemp -d)
    trap 'rm -rf "${tmpdir}"' EXIT

    curl -fsSL "${url}" -o "${tmpdir}/railgun.tar.gz"

    # Verify checksum
    info "Verifying checksum..."
    curl -fsSL "${checksum_url}" -o "${tmpdir}/checksums.txt"
    local expected_checksum
    expected_checksum=$(grep "railgun-${platform}.tar.gz" "${tmpdir}/checksums.txt" | awk '{print $1}')
    local actual_checksum
    if command -v sha256sum &> /dev/null; then
        actual_checksum=$(sha256sum "${tmpdir}/railgun.tar.gz" | awk '{print $1}')
    else
        actual_checksum=$(shasum -a 256 "${tmpdir}/railgun.tar.gz" | awk '{print $1}')
    fi

    if [[ "${expected_checksum}" != "${actual_checksum}" ]]; then
        error "Checksum verification failed!"
    fi
    info "Checksum verified"

    # Extract
    info "Installing to ${INSTALL_DIR}..."
    tar -xzf "${tmpdir}/railgun.tar.gz" -C "${INSTALL_DIR}"
    chmod +x "${INSTALL_DIR}/railgun"

    # Verify installation
    if ! "${INSTALL_DIR}/railgun" --version &> /dev/null; then
        error "Installation verification failed"
    fi

    info "Successfully installed railgun to ${INSTALL_DIR}/railgun"

    # Check if install dir is in PATH
    if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
        warn "${INSTALL_DIR} is not in your PATH"
        echo ""
        echo "Add it to your shell profile:"
        echo "  export PATH=\"\${HOME}/.local/bin:\${PATH}\""
        echo ""
    fi

    # Configure Claude Code
    info "Configuring Claude Code..."
    if "${INSTALL_DIR}/railgun" install 2>/dev/null; then
        info "Claude Code configured successfully"
    else
        warn "Could not configure Claude Code automatically"
        echo "Run manually: railgun install"
    fi

    echo ""
    info "Installation complete!"
    echo ""
    echo "Next steps:"
    echo "  1. Create a policy file: railgun.toml"
    echo "  2. Test your policy: railgun test Bash '{\"command\":\"ls\"}'"
    echo "  3. Validate config: railgun lint"
    echo ""
    echo "Documentation: https://github.com/${REPO}#readme"
}

main "$@"

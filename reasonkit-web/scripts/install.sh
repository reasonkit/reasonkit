#!/usr/bin/env bash
# ==============================================================================
# ReasonKit Web - Production Installation Script for Debian 13+
# ==============================================================================
#
# This script installs reasonkit-web as a production service on Debian 13+
#
# Usage:
#   sudo ./install.sh [OPTIONS]
#
# Options:
#   --binary PATH    Path to pre-built binary (default: looks in target/release)
#   --skip-user      Skip user creation (user already exists)
#   --skip-chromium  Skip Chromium installation
#   --prefix PATH    Installation prefix (default: /opt/reasonkit)
#   --help           Show this help message
#
# Requirements:
#   - Debian 13 (Trixie) or later
#   - Root or sudo privileges
#   - Pre-built binary OR Rust toolchain for building
#
# ==============================================================================

set -euo pipefail

# ------------------------------------------------------------------------------
# Configuration
# ------------------------------------------------------------------------------

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_DIR="$(dirname "${SCRIPT_DIR}")"

# Default values
PREFIX="/opt/reasonkit"
BINARY_PATH=""
SKIP_USER=false
SKIP_CHROMIUM=false
SERVICE_USER="reasonkit"
SERVICE_GROUP="reasonkit"

# Paths derived from prefix
BIN_DIR=""
CONFIG_DIR="/etc/reasonkit"
DATA_DIR="/var/lib/reasonkit"
LOG_DIR="/var/log/reasonkit"
RUN_DIR="/run/reasonkit"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# ------------------------------------------------------------------------------
# Utility Functions
# ------------------------------------------------------------------------------

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[OK]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

die() {
    log_error "$1"
    exit 1
}

# ------------------------------------------------------------------------------
# Pre-flight Checks
# ------------------------------------------------------------------------------

check_root() {
    if [[ $EUID -ne 0 ]]; then
        die "This script must be run as root or with sudo"
    fi
    log_success "Running as root"
}

check_debian_version() {
    if [[ ! -f /etc/os-release ]]; then
        die "Cannot detect OS version (/etc/os-release not found)"
    fi

    source /etc/os-release

    if [[ "${ID:-}" != "debian" ]]; then
        log_warn "This script is designed for Debian. Detected: ${ID:-unknown}"
        read -p "Continue anyway? [y/N] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi

    # Debian 13 is "trixie" with VERSION_ID=13
    # Also accept testing/unstable which may not have VERSION_ID
    local version="${VERSION_ID:-0}"

    if [[ "$version" =~ ^[0-9]+$ ]] && [[ "$version" -lt 13 ]]; then
        die "Debian 13+ required. Detected: Debian ${version}"
    fi

    log_success "Debian version check passed: ${PRETTY_NAME:-Debian}"
}

check_systemd() {
    if ! command -v systemctl &> /dev/null; then
        die "systemd is required but not found"
    fi

    if ! systemctl is-system-running &> /dev/null; then
        log_warn "systemd may not be fully operational"
    fi

    log_success "systemd is available"
}

check_dependencies() {
    local missing=()

    # Required for headless Chromium
    local deps=(
        "curl"
        "ca-certificates"
    )

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done

    if [[ ${#missing[@]} -gt 0 ]]; then
        log_info "Installing missing dependencies: ${missing[*]}"
        apt-get update -qq
        apt-get install -y -qq "${missing[@]}"
    fi

    log_success "Dependencies verified"
}

check_binary() {
    # If binary path provided, use it
    if [[ -n "$BINARY_PATH" ]]; then
        if [[ ! -f "$BINARY_PATH" ]]; then
            die "Binary not found at: $BINARY_PATH"
        fi
        if [[ ! -x "$BINARY_PATH" ]]; then
            chmod +x "$BINARY_PATH"
        fi
        log_success "Using provided binary: $BINARY_PATH"
        return
    fi

    # Look in standard locations
    local candidates=(
        "${PROJECT_DIR}/target/release/reasonkit-web"
        "${PROJECT_DIR}/target/debug/reasonkit-web"
        "./reasonkit-web"
    )

    for candidate in "${candidates[@]}"; do
        if [[ -f "$candidate" ]]; then
            BINARY_PATH="$candidate"
            log_success "Found binary: $BINARY_PATH"
            return
        fi
    done

    # No binary found, offer to build
    log_warn "No pre-built binary found"

    if command -v cargo &> /dev/null; then
        read -p "Build from source? [Y/n] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            log_info "Building release binary..."
            cd "${PROJECT_DIR}"
            cargo build --release
            BINARY_PATH="${PROJECT_DIR}/target/release/reasonkit-web"
            log_success "Build complete: $BINARY_PATH"
            return
        fi
    fi

    die "No binary available. Build with 'cargo build --release' first."
}

# ------------------------------------------------------------------------------
# Installation Functions
# ------------------------------------------------------------------------------

create_user() {
    if [[ "$SKIP_USER" == true ]]; then
        log_info "Skipping user creation (--skip-user)"
        return
    fi

    if id "$SERVICE_USER" &>/dev/null; then
        log_info "User '$SERVICE_USER' already exists"
    else
        log_info "Creating system user: $SERVICE_USER"
        useradd \
            --system \
            --no-create-home \
            --shell /usr/sbin/nologin \
            --comment "ReasonKit Web Service" \
            "$SERVICE_USER"
        log_success "User '$SERVICE_USER' created"
    fi

    # Ensure group exists
    if ! getent group "$SERVICE_GROUP" &>/dev/null; then
        groupadd --system "$SERVICE_GROUP"
        log_success "Group '$SERVICE_GROUP' created"
    fi
}

create_directories() {
    log_info "Creating directory structure..."

    BIN_DIR="${PREFIX}/bin"

    # Create directories with appropriate permissions
    mkdir -p "$BIN_DIR"
    mkdir -p "$CONFIG_DIR"
    mkdir -p "$DATA_DIR"
    mkdir -p "$LOG_DIR"
    mkdir -p "$RUN_DIR"

    # Set ownership
    chown root:root "$PREFIX"
    chown root:root "$BIN_DIR"
    chown root:"$SERVICE_GROUP" "$CONFIG_DIR"
    chown "$SERVICE_USER":"$SERVICE_GROUP" "$DATA_DIR"
    chown "$SERVICE_USER":"$SERVICE_GROUP" "$LOG_DIR"
    chown "$SERVICE_USER":"$SERVICE_GROUP" "$RUN_DIR"

    # Set permissions
    chmod 755 "$PREFIX"
    chmod 755 "$BIN_DIR"
    chmod 750 "$CONFIG_DIR"
    chmod 750 "$DATA_DIR"
    chmod 750 "$LOG_DIR"
    chmod 755 "$RUN_DIR"

    log_success "Directories created"
}

install_binary() {
    log_info "Installing binary..."

    local target="${BIN_DIR}/reasonkit-web"

    cp "$BINARY_PATH" "$target"
    chown root:root "$target"
    chmod 755 "$target"

    # Verify installation
    if "$target" --version &>/dev/null; then
        local version
        version=$("$target" --version 2>&1 | head -1)
        log_success "Installed: $version"
    else
        log_warn "Binary installed but version check failed"
    fi

    # Create symlink in /usr/local/bin
    ln -sf "$target" /usr/local/bin/reasonkit-web
    log_success "Symlink created: /usr/local/bin/reasonkit-web"
}

install_chromium() {
    if [[ "$SKIP_CHROMIUM" == true ]]; then
        log_info "Skipping Chromium installation (--skip-chromium)"
        return
    fi

    if command -v chromium &>/dev/null || command -v chromium-browser &>/dev/null; then
        log_success "Chromium already installed"
        return
    fi

    log_info "Installing Chromium and dependencies..."

    # Install chromium and required libraries for headless operation
    apt-get update -qq
    apt-get install -y -qq \
        chromium \
        chromium-sandbox \
        fonts-liberation \
        libasound2 \
        libatk-bridge2.0-0 \
        libatk1.0-0 \
        libatspi2.0-0 \
        libcups2 \
        libdbus-1-3 \
        libdrm2 \
        libgbm1 \
        libgtk-3-0 \
        libnspr4 \
        libnss3 \
        libxcomposite1 \
        libxdamage1 \
        libxfixes3 \
        libxkbcommon0 \
        libxrandr2 \
        xdg-utils \
        || log_warn "Some Chromium dependencies may not have installed"

    log_success "Chromium installed"
}

install_systemd_service() {
    log_info "Installing systemd service..."

    cat > /etc/systemd/system/reasonkit-web.service << 'EOF'
[Unit]
Description=ReasonKit Web - Web Sensing & Browser Automation Layer
Documentation=https://reasonkit.sh/docs
After=network-online.target
Wants=network-online.target
StartLimitIntervalSec=60
StartLimitBurst=3

[Service]
Type=simple
User=reasonkit
Group=reasonkit

# Binary location
ExecStart=/opt/reasonkit/bin/reasonkit-web serve

# Environment
EnvironmentFile=-/etc/reasonkit/reasonkit-web.env

# Working directory
WorkingDirectory=/var/lib/reasonkit

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096
MemoryMax=2G
CPUQuota=200%

# Security hardening
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
PrivateTmp=yes
PrivateDevices=yes
ProtectKernelTunables=yes
ProtectKernelModules=yes
ProtectControlGroups=yes
RestrictAddressFamilies=AF_INET AF_INET6 AF_UNIX
RestrictNamespaces=yes
RestrictRealtime=yes
RestrictSUIDSGID=yes
LockPersonality=yes

# Allow /tmp access for Chromium
PrivateTmp=no

# Read-write paths needed
ReadWritePaths=/var/lib/reasonkit /var/log/reasonkit /run/reasonkit

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=reasonkit-web

# Restart policy
Restart=on-failure
RestartSec=5s

# Timeout settings
TimeoutStartSec=30
TimeoutStopSec=30

[Install]
WantedBy=multi-user.target
EOF

    # Set proper permissions on service file
    chmod 644 /etc/systemd/system/reasonkit-web.service

    # Reload systemd
    systemctl daemon-reload

    log_success "Systemd service installed"
}

install_default_config() {
    log_info "Installing default configuration..."

    # Create default environment file
    if [[ ! -f "${CONFIG_DIR}/reasonkit-web.env" ]]; then
        cat > "${CONFIG_DIR}/reasonkit-web.env" << 'EOF'
# ReasonKit Web Configuration
# ==========================
#
# This file is sourced by the systemd service.
# Edit this file to configure reasonkit-web.
# After changes, run: systemctl restart reasonkit-web

# Logging level: error, warn, info, debug, trace
RUST_LOG=info

# Chrome/Chromium path (auto-detected if not set)
# CHROME_PATH=/usr/bin/chromium

# Browser options
# REASONKIT_HEADLESS=true
# REASONKIT_DISABLE_GPU=true

# MCP server settings
# MCP_TIMEOUT_SECS=30

# Performance tuning
# TOKIO_WORKER_THREADS=4
EOF
        chmod 640 "${CONFIG_DIR}/reasonkit-web.env"
        chown root:"$SERVICE_GROUP" "${CONFIG_DIR}/reasonkit-web.env"
        log_success "Default configuration created"
    else
        log_info "Configuration file already exists, preserving"
    fi
}

create_tmpfiles_config() {
    log_info "Creating tmpfiles.d configuration..."

    cat > /etc/tmpfiles.d/reasonkit-web.conf << EOF
# ReasonKit Web runtime directory
d /run/reasonkit 0755 $SERVICE_USER $SERVICE_GROUP -
EOF

    chmod 644 /etc/tmpfiles.d/reasonkit-web.conf

    # Create the directory now
    systemd-tmpfiles --create /etc/tmpfiles.d/reasonkit-web.conf 2>/dev/null || true

    log_success "tmpfiles.d configuration created"
}

create_logrotate_config() {
    log_info "Creating logrotate configuration..."

    cat > /etc/logrotate.d/reasonkit-web << EOF
/var/log/reasonkit/*.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    create 0640 $SERVICE_USER $SERVICE_GROUP
    sharedscripts
    postrotate
        systemctl reload reasonkit-web > /dev/null 2>&1 || true
    endscript
}
EOF

    chmod 644 /etc/logrotate.d/reasonkit-web

    log_success "Logrotate configuration created"
}

# ------------------------------------------------------------------------------
# Post-installation
# ------------------------------------------------------------------------------

print_summary() {
    echo
    echo "=============================================================================="
    echo -e "${GREEN}ReasonKit Web Installation Complete${NC}"
    echo "=============================================================================="
    echo
    echo "Installation summary:"
    echo "  Binary:     ${BIN_DIR}/reasonkit-web"
    echo "  Symlink:    /usr/local/bin/reasonkit-web"
    echo "  Config:     ${CONFIG_DIR}/reasonkit-web.env"
    echo "  Data:       ${DATA_DIR}"
    echo "  Logs:       ${LOG_DIR}"
    echo "  Service:    reasonkit-web.service"
    echo "  User:       ${SERVICE_USER}"
    echo
    echo "Next steps:"
    echo "  1. Configure tokens:  sudo ${SCRIPT_DIR}/configure.sh"
    echo "  2. Enable service:    sudo systemctl enable reasonkit-web"
    echo "  3. Start service:     sudo systemctl start reasonkit-web"
    echo "  4. Check status:      sudo systemctl status reasonkit-web"
    echo
    echo "Quick start:"
    echo "  sudo ${SCRIPT_DIR}/configure.sh && sudo systemctl enable --now reasonkit-web"
    echo
    echo "Documentation: https://reasonkit.sh/docs"
    echo "=============================================================================="
}

# ------------------------------------------------------------------------------
# Argument Parsing
# ------------------------------------------------------------------------------

show_help() {
    cat << EOF
ReasonKit Web - Production Installation Script

Usage:
  sudo $0 [OPTIONS]

Options:
  --binary PATH    Path to pre-built binary (default: auto-detect)
  --prefix PATH    Installation prefix (default: /opt/reasonkit)
  --skip-user      Skip user creation (user already exists)
  --skip-chromium  Skip Chromium installation
  --help           Show this help message

Examples:
  sudo $0
  sudo $0 --binary ./reasonkit-web
  sudo $0 --prefix /usr/local --skip-chromium

EOF
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --binary)
                BINARY_PATH="$2"
                shift 2
                ;;
            --prefix)
                PREFIX="$2"
                shift 2
                ;;
            --skip-user)
                SKIP_USER=true
                shift
                ;;
            --skip-chromium)
                SKIP_CHROMIUM=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                die "Unknown option: $1"
                ;;
        esac
    done
}

# ------------------------------------------------------------------------------
# Main
# ------------------------------------------------------------------------------

main() {
    parse_args "$@"

    echo
    echo "=============================================================================="
    echo "ReasonKit Web - Production Installation"
    echo "=============================================================================="
    echo

    log_info "Starting installation..."
    echo

    # Pre-flight checks
    log_info "Running pre-flight checks..."
    check_root
    check_debian_version
    check_systemd
    check_dependencies
    check_binary
    echo

    # Installation
    log_info "Installing ReasonKit Web..."
    create_user
    create_directories
    install_binary
    install_chromium
    install_systemd_service
    install_default_config
    create_tmpfiles_config
    create_logrotate_config
    echo

    # Summary
    print_summary
}

main "$@"

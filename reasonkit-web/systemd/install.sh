#!/usr/bin/env bash
# ReasonKit Web MCP Sidecar - Installation Script for Debian 13 (Trixie)
#
# This script installs reasonkit-web as a systemd service with
# proper security hardening and configuration.
#
# Usage:
#   sudo ./install.sh [OPTIONS]
#
# Options:
#   --binary PATH     Path to reasonkit-web binary (default: ./reasonkit-web)
#   --uninstall       Remove reasonkit-web service and files
#   --upgrade         Upgrade existing installation
#   --dry-run         Show what would be done without making changes
#   --help            Show this help message

set -euo pipefail

# ============================================================
# Configuration
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_NAME="reasonkit-web"
SERVICE_USER="reasonkit"
SERVICE_GROUP="reasonkit"

INSTALL_DIR="/opt/reasonkit"
BIN_DIR="${INSTALL_DIR}/bin"
CONFIG_DIR="/etc/reasonkit"
DATA_DIR="/var/lib/reasonkit"
LOG_DIR="/var/log/reasonkit"
CACHE_DIR="${DATA_DIR}/cache"

SYSTEMD_DIR="/etc/systemd/system"
SERVICE_FILE="${SYSTEMD_DIR}/${SERVICE_NAME}.service"

# Default binary path
BINARY_PATH="${SCRIPT_DIR}/../target/release/reasonkit-web"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================
# Helper Functions
# ============================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[OK]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

die() {
    log_error "$*"
    exit 1
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        die "This script must be run as root (use sudo)"
    fi
}

check_debian() {
    if [[ ! -f /etc/debian_version ]]; then
        log_warn "This script is designed for Debian. Proceeding anyway..."
    else
        local version
        version=$(cat /etc/debian_version)
        log_info "Detected Debian version: ${version}"
    fi
}

check_systemd() {
    if ! command -v systemctl &> /dev/null; then
        die "systemd is required but not found"
    fi
}

# ============================================================
# Installation Functions
# ============================================================

create_user() {
    if id "${SERVICE_USER}" &>/dev/null; then
        log_info "User '${SERVICE_USER}' already exists"
    else
        log_info "Creating service user '${SERVICE_USER}'..."
        useradd \
            --system \
            --shell /usr/sbin/nologin \
            --home-dir "${DATA_DIR}" \
            --no-create-home \
            --comment "ReasonKit Web Service" \
            "${SERVICE_USER}"
        log_success "User '${SERVICE_USER}' created"
    fi
}

create_directories() {
    log_info "Creating directories..."

    # Installation directory
    install -d -m 0755 -o root -g root "${INSTALL_DIR}"
    install -d -m 0755 -o root -g root "${BIN_DIR}"

    # Configuration directory
    install -d -m 0750 -o root -g "${SERVICE_GROUP}" "${CONFIG_DIR}"

    # Data directory
    install -d -m 0750 -o "${SERVICE_USER}" -g "${SERVICE_GROUP}" "${DATA_DIR}"
    install -d -m 0750 -o "${SERVICE_USER}" -g "${SERVICE_GROUP}" "${CACHE_DIR}"

    # Log directory
    install -d -m 0750 -o "${SERVICE_USER}" -g "${SERVICE_GROUP}" "${LOG_DIR}"

    log_success "Directories created"
}

install_binary() {
    local binary="$1"

    if [[ ! -f "${binary}" ]]; then
        die "Binary not found: ${binary}"
    fi

    if [[ ! -x "${binary}" ]]; then
        die "Binary is not executable: ${binary}"
    fi

    log_info "Installing binary to ${BIN_DIR}..."
    install -m 0755 -o root -g root "${binary}" "${BIN_DIR}/${SERVICE_NAME}"
    log_success "Binary installed"
}

install_service() {
    log_info "Installing systemd service..."

    local service_src="${SCRIPT_DIR}/${SERVICE_NAME}.service"

    if [[ ! -f "${service_src}" ]]; then
        die "Service file not found: ${service_src}"
    fi

    install -m 0644 -o root -g root "${service_src}" "${SERVICE_FILE}"
    log_success "Service file installed"
}

install_config() {
    local env_file="${CONFIG_DIR}/${SERVICE_NAME}.env"
    local env_example="${SCRIPT_DIR}/${SERVICE_NAME}.env.example"

    if [[ -f "${env_file}" ]]; then
        log_warn "Configuration file exists, not overwriting: ${env_file}"
        log_info "Review ${env_example} for new options"
    else
        log_info "Installing default configuration..."
        if [[ -f "${env_example}" ]]; then
            install -m 0640 -o root -g "${SERVICE_GROUP}" "${env_example}" "${env_file}"
            log_success "Configuration installed to ${env_file}"
        else
            log_warn "Example config not found, creating minimal config..."
            cat > "${env_file}" << 'EOF'
# ReasonKit Web MCP Sidecar Configuration
# See /etc/reasonkit/reasonkit-web.env.example for all options

RUST_LOG=info
REASONKIT_WEB_HOST=127.0.0.1
REASONKIT_WEB_PORT=3847
REASONKIT_DATA_DIR=/var/lib/reasonkit
REASONKIT_WATCHDOG_ENABLED=true
EOF
            chown root:"${SERVICE_GROUP}" "${env_file}"
            chmod 0640 "${env_file}"
            log_success "Minimal configuration created"
        fi
    fi

    # Always install the example file
    if [[ -f "${env_example}" ]]; then
        install -m 0644 -o root -g root "${env_example}" "${CONFIG_DIR}/${SERVICE_NAME}.env.example"
    fi
}

enable_service() {
    log_info "Reloading systemd daemon..."
    systemctl daemon-reload

    log_info "Enabling service..."
    systemctl enable "${SERVICE_NAME}.service"

    log_success "Service enabled"
}

start_service() {
    log_info "Starting service..."

    if systemctl start "${SERVICE_NAME}.service"; then
        log_success "Service started successfully"
        systemctl status "${SERVICE_NAME}.service" --no-pager
    else
        log_error "Failed to start service"
        log_info "Check logs with: journalctl -u ${SERVICE_NAME}.service -f"
        return 1
    fi
}

# ============================================================
# Upgrade Functions
# ============================================================

upgrade_service() {
    local binary="$1"

    log_info "Upgrading ${SERVICE_NAME}..."

    if systemctl is-active --quiet "${SERVICE_NAME}.service"; then
        log_info "Stopping service..."
        systemctl stop "${SERVICE_NAME}.service"
    fi

    install_binary "${binary}"
    install_service

    log_info "Reloading systemd daemon..."
    systemctl daemon-reload

    log_info "Starting service..."
    systemctl start "${SERVICE_NAME}.service"

    log_success "Upgrade complete"
}

# ============================================================
# Uninstall Functions
# ============================================================

uninstall_service() {
    log_info "Uninstalling ${SERVICE_NAME}..."

    if systemctl is-active --quiet "${SERVICE_NAME}.service"; then
        log_info "Stopping service..."
        systemctl stop "${SERVICE_NAME}.service"
    fi

    if systemctl is-enabled --quiet "${SERVICE_NAME}.service" 2>/dev/null; then
        log_info "Disabling service..."
        systemctl disable "${SERVICE_NAME}.service"
    fi

    if [[ -f "${SERVICE_FILE}" ]]; then
        log_info "Removing service file..."
        rm -f "${SERVICE_FILE}"
    fi

    log_info "Reloading systemd daemon..."
    systemctl daemon-reload

    log_info "Removing binary..."
    rm -f "${BIN_DIR}/${SERVICE_NAME}"

    log_warn "Preserved directories (remove manually if needed):"
    log_warn "  - ${CONFIG_DIR} (configuration)"
    log_warn "  - ${DATA_DIR} (data)"
    log_warn "  - ${LOG_DIR} (logs)"

    log_warn "Preserved user '${SERVICE_USER}' (remove manually if needed):"
    log_warn "  userdel ${SERVICE_USER}"

    log_success "Uninstall complete"
}

# ============================================================
# Main
# ============================================================

usage() {
    cat << EOF
ReasonKit Web MCP Sidecar - Installation Script

Usage: sudo $0 [OPTIONS]

Options:
    --binary PATH     Path to reasonkit-web binary
                      Default: ${BINARY_PATH}
    --uninstall       Remove reasonkit-web service and files
    --upgrade         Upgrade existing installation
    --dry-run         Show what would be done without making changes
    --help            Show this help message

Examples:
    # Fresh installation
    sudo $0 --binary ./target/release/reasonkit-web

    # Upgrade existing installation
    sudo $0 --upgrade --binary ./target/release/reasonkit-web

    # Uninstall
    sudo $0 --uninstall

EOF
}

main() {
    local action="install"
    local dry_run=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --binary)
                BINARY_PATH="$2"
                shift 2
                ;;
            --uninstall)
                action="uninstall"
                shift
                ;;
            --upgrade)
                action="upgrade"
                shift
                ;;
            --dry-run)
                dry_run=true
                shift
                ;;
            --help|-h)
                usage
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done

    echo ""
    echo "=================================================="
    echo " ReasonKit Web MCP Sidecar - Installation"
    echo "=================================================="
    echo ""

    check_root
    check_debian
    check_systemd

    if [[ "${dry_run}" == true ]]; then
        log_warn "DRY RUN MODE - No changes will be made"
        echo ""
        echo "Would perform: ${action}"
        echo "Binary: ${BINARY_PATH}"
        echo "Install dir: ${INSTALL_DIR}"
        echo "Config dir: ${CONFIG_DIR}"
        echo "Data dir: ${DATA_DIR}"
        echo "Log dir: ${LOG_DIR}"
        exit 0
    fi

    case "${action}" in
        install)
            log_info "Starting fresh installation..."
            create_user
            create_directories
            install_binary "${BINARY_PATH}"
            install_service
            install_config
            enable_service
            start_service
            echo ""
            log_success "Installation complete!"
            echo ""
            echo "Next steps:"
            echo "  1. Review configuration: ${CONFIG_DIR}/${SERVICE_NAME}.env"
            echo "  2. Check service status: systemctl status ${SERVICE_NAME}"
            echo "  3. View logs: journalctl -u ${SERVICE_NAME} -f"
            ;;
        upgrade)
            upgrade_service "${BINARY_PATH}"
            ;;
        uninstall)
            uninstall_service
            ;;
    esac
}

main "$@"

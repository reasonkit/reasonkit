#!/usr/bin/env bash
# ==============================================================================
# ReasonKit Web - Uninstallation Script
# ==============================================================================
#
# This script removes reasonkit-web from the system.
#
# Usage:
#   sudo ./uninstall.sh [OPTIONS]
#
# Options:
#   --purge         Remove all data including logs and configuration
#   --remove-user   Remove the reasonkit system user
#   --yes           Skip confirmation prompts
#   --help          Show this help message
#
# ==============================================================================

set -euo pipefail

# ------------------------------------------------------------------------------
# Configuration
# ------------------------------------------------------------------------------

readonly PREFIX="/opt/reasonkit"
readonly CONFIG_DIR="/etc/reasonkit"
readonly DATA_DIR="/var/lib/reasonkit"
readonly LOG_DIR="/var/log/reasonkit"
readonly RUN_DIR="/run/reasonkit"
readonly SERVICE_USER="reasonkit"
readonly SERVICE_GROUP="reasonkit"

# Options
PURGE=false
REMOVE_USER=false
SKIP_CONFIRM=false

# Colors
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m'

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

confirm() {
    if [[ "$SKIP_CONFIRM" == true ]]; then
        return 0
    fi

    read -p "$1 [y/N] " -n 1 -r
    echo
    [[ $REPLY =~ ^[Yy]$ ]]
}

# ------------------------------------------------------------------------------
# Pre-flight Checks
# ------------------------------------------------------------------------------

check_root() {
    if [[ $EUID -ne 0 ]]; then
        die "This script must be run as root or with sudo"
    fi
}

# ------------------------------------------------------------------------------
# Uninstallation Functions
# ------------------------------------------------------------------------------

stop_service() {
    log_info "Stopping reasonkit-web service..."

    if systemctl is-active --quiet reasonkit-web 2>/dev/null; then
        systemctl stop reasonkit-web
        log_success "Service stopped"
    else
        log_info "Service was not running"
    fi

    if systemctl is-enabled --quiet reasonkit-web 2>/dev/null; then
        systemctl disable reasonkit-web
        log_success "Service disabled"
    fi
}

remove_service() {
    log_info "Removing systemd service..."

    local files=(
        "/etc/systemd/system/reasonkit-web.service"
        "/etc/tmpfiles.d/reasonkit-web.conf"
        "/etc/logrotate.d/reasonkit-web"
    )

    for file in "${files[@]}"; do
        if [[ -f "$file" ]]; then
            rm -f "$file"
            log_success "Removed: $file"
        fi
    done

    # Reload systemd
    systemctl daemon-reload 2>/dev/null || true

    log_success "Service files removed"
}

remove_binary() {
    log_info "Removing binary and symlinks..."

    local files=(
        "${PREFIX}/bin/reasonkit-web"
        "/usr/local/bin/reasonkit-web"
    )

    for file in "${files[@]}"; do
        if [[ -e "$file" ]]; then
            rm -f "$file"
            log_success "Removed: $file"
        fi
    done

    # Remove bin directory if empty
    if [[ -d "${PREFIX}/bin" ]]; then
        rmdir "${PREFIX}/bin" 2>/dev/null && log_success "Removed: ${PREFIX}/bin" || true
    fi

    # Remove prefix directory if empty
    if [[ -d "${PREFIX}" ]]; then
        rmdir "${PREFIX}" 2>/dev/null && log_success "Removed: ${PREFIX}" || true
    fi
}

remove_config() {
    if [[ ! -d "$CONFIG_DIR" ]]; then
        log_info "Configuration directory does not exist"
        return
    fi

    if [[ "$PURGE" == true ]]; then
        log_info "Removing configuration directory..."
        rm -rf "$CONFIG_DIR"
        log_success "Removed: $CONFIG_DIR"
    else
        log_warn "Configuration preserved: $CONFIG_DIR"
        log_info "Use --purge to remove configuration"
    fi
}

remove_data() {
    if [[ "$PURGE" == true ]]; then
        log_info "Removing data directories..."

        for dir in "$DATA_DIR" "$LOG_DIR" "$RUN_DIR"; do
            if [[ -d "$dir" ]]; then
                rm -rf "$dir"
                log_success "Removed: $dir"
            fi
        done
    else
        if [[ -d "$DATA_DIR" ]] || [[ -d "$LOG_DIR" ]]; then
            log_warn "Data directories preserved:"
            [[ -d "$DATA_DIR" ]] && echo "  - $DATA_DIR"
            [[ -d "$LOG_DIR" ]] && echo "  - $LOG_DIR"
            log_info "Use --purge to remove all data"
        fi
    fi
}

remove_user() {
    if [[ "$REMOVE_USER" != true ]]; then
        if id "$SERVICE_USER" &>/dev/null; then
            log_info "User '$SERVICE_USER' preserved"
            log_info "Use --remove-user to delete the service user"
        fi
        return
    fi

    if ! id "$SERVICE_USER" &>/dev/null; then
        log_info "User '$SERVICE_USER' does not exist"
        return
    fi

    log_info "Removing system user..."

    # Kill any remaining processes
    pkill -u "$SERVICE_USER" 2>/dev/null || true

    # Remove user
    userdel "$SERVICE_USER" 2>/dev/null && log_success "User '$SERVICE_USER' removed" || \
        log_warn "Could not remove user '$SERVICE_USER'"

    # Remove group if empty
    if getent group "$SERVICE_GROUP" &>/dev/null; then
        groupdel "$SERVICE_GROUP" 2>/dev/null && log_success "Group '$SERVICE_GROUP' removed" || true
    fi
}

# ------------------------------------------------------------------------------
# Summary
# ------------------------------------------------------------------------------

print_summary() {
    echo
    echo "=============================================================================="
    echo -e "${GREEN}ReasonKit Web Uninstallation Complete${NC}"
    echo "=============================================================================="
    echo

    if [[ "$PURGE" != true ]]; then
        echo "The following may have been preserved:"
        [[ -d "$CONFIG_DIR" ]] && echo "  - Configuration: $CONFIG_DIR"
        [[ -d "$DATA_DIR" ]] && echo "  - Data: $DATA_DIR"
        [[ -d "$LOG_DIR" ]] && echo "  - Logs: $LOG_DIR"
        id "$SERVICE_USER" &>/dev/null && echo "  - User: $SERVICE_USER"
        echo
        echo "To completely remove all files:"
        echo "  sudo $0 --purge --remove-user --yes"
    else
        echo "All files have been removed."
    fi
    echo
    echo "=============================================================================="
}

# ------------------------------------------------------------------------------
# Argument Parsing
# ------------------------------------------------------------------------------

show_help() {
    cat << EOF
ReasonKit Web - Uninstallation Script

Usage:
  sudo $0 [OPTIONS]

Options:
  --purge         Remove all data including logs and configuration
  --remove-user   Remove the reasonkit system user
  --yes           Skip confirmation prompts
  --help          Show this help message

Examples:
  sudo $0                     # Basic uninstall (preserves config/data)
  sudo $0 --purge             # Remove all files except user
  sudo $0 --purge --remove-user --yes   # Complete removal

EOF
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --purge)
                PURGE=true
                shift
                ;;
            --remove-user)
                REMOVE_USER=true
                shift
                ;;
            --yes|-y)
                SKIP_CONFIRM=true
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
    echo "ReasonKit Web - Uninstallation"
    echo "=============================================================================="
    echo

    check_root

    # Show what will be removed
    echo "This will remove:"
    echo "  - Binary: ${PREFIX}/bin/reasonkit-web"
    echo "  - Symlink: /usr/local/bin/reasonkit-web"
    echo "  - Service: reasonkit-web.service"

    if [[ "$PURGE" == true ]]; then
        echo "  - Configuration: $CONFIG_DIR"
        echo "  - Data: $DATA_DIR"
        echo "  - Logs: $LOG_DIR"
    fi

    if [[ "$REMOVE_USER" == true ]]; then
        echo "  - User: $SERVICE_USER"
    fi

    echo

    if ! confirm "Proceed with uninstallation?"; then
        log_info "Uninstallation cancelled"
        exit 0
    fi

    echo

    # Perform uninstallation
    stop_service
    remove_service
    remove_binary
    remove_config
    remove_data
    remove_user

    # Summary
    print_summary
}

main "$@"

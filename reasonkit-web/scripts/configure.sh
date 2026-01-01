#!/usr/bin/env bash
# ==============================================================================
# ReasonKit Web - Configuration Script
# ==============================================================================
#
# Interactive configuration for reasonkit-web service.
#
# Usage:
#   sudo ./configure.sh [OPTIONS]
#
# Options:
#   --non-interactive    Use environment variables for configuration
#   --start              Start/restart service after configuration
#   --enable             Enable service to start on boot
#   --help               Show this help message
#
# Environment variables (for --non-interactive):
#   RUST_LOG             Log level (error, warn, info, debug, trace)
#   CHROME_PATH          Path to Chrome/Chromium binary
#   MCP_TIMEOUT_SECS     MCP request timeout in seconds
#
# ==============================================================================

set -euo pipefail

# ------------------------------------------------------------------------------
# Configuration
# ------------------------------------------------------------------------------

readonly CONFIG_DIR="/etc/reasonkit"
readonly CONFIG_FILE="${CONFIG_DIR}/reasonkit-web.env"
readonly SERVICE_NAME="reasonkit-web"
readonly SERVICE_GROUP="reasonkit"

# Options
NON_INTERACTIVE=false
START_SERVICE=false
ENABLE_SERVICE=false

# Colors
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly CYAN='\033[0;36m'
readonly NC='\033[0m'

# Configuration values
declare -A CONFIG=(
    [RUST_LOG]="info"
    [CHROME_PATH]=""
    [REASONKIT_HEADLESS]="true"
    [REASONKIT_DISABLE_GPU]="true"
    [MCP_TIMEOUT_SECS]="30"
    [TOKIO_WORKER_THREADS]="4"
)

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

prompt_value() {
    local prompt="$1"
    local default="$2"
    local var_name="$3"
    local value

    if [[ -n "$default" ]]; then
        echo -en "${CYAN}${prompt}${NC} [${default}]: "
    else
        echo -en "${CYAN}${prompt}${NC}: "
    fi

    read -r value

    if [[ -z "$value" ]]; then
        value="$default"
    fi

    CONFIG[$var_name]="$value"
}

prompt_choice() {
    local prompt="$1"
    local options="$2"
    local default="$3"
    local var_name="$4"
    local value

    echo -e "${CYAN}${prompt}${NC}"
    echo "  Options: $options"
    echo -n "  Choice [$default]: "
    read -r value

    if [[ -z "$value" ]]; then
        value="$default"
    fi

    CONFIG[$var_name]="$value"
}

prompt_bool() {
    local prompt="$1"
    local default="$2"
    local var_name="$3"
    local value

    local default_display="y/N"
    [[ "$default" == "true" ]] && default_display="Y/n"

    echo -en "${CYAN}${prompt}${NC} [${default_display}]: "
    read -r value

    case "${value,,}" in
        y|yes)
            CONFIG[$var_name]="true"
            ;;
        n|no)
            CONFIG[$var_name]="false"
            ;;
        *)
            CONFIG[$var_name]="$default"
            ;;
    esac
}

# ------------------------------------------------------------------------------
# Pre-flight Checks
# ------------------------------------------------------------------------------

check_root() {
    if [[ $EUID -ne 0 ]]; then
        die "This script must be run as root or with sudo"
    fi
}

check_installation() {
    if [[ ! -f "${CONFIG_DIR}/reasonkit-web.env" ]] && [[ ! -d "$CONFIG_DIR" ]]; then
        die "ReasonKit Web is not installed. Run install.sh first."
    fi

    # Ensure config directory exists
    mkdir -p "$CONFIG_DIR"
}

detect_chrome() {
    local paths=(
        "/usr/bin/chromium"
        "/usr/bin/chromium-browser"
        "/usr/bin/google-chrome"
        "/usr/bin/google-chrome-stable"
        "/snap/bin/chromium"
        "/opt/google/chrome/chrome"
    )

    for path in "${paths[@]}"; do
        if [[ -x "$path" ]]; then
            CONFIG[CHROME_PATH]="$path"
            return 0
        fi
    done

    return 1
}

# ------------------------------------------------------------------------------
# Configuration Functions
# ------------------------------------------------------------------------------

load_existing_config() {
    if [[ -f "$CONFIG_FILE" ]]; then
        log_info "Loading existing configuration..."

        # Read existing values
        while IFS='=' read -r key value; do
            # Skip comments and empty lines
            [[ "$key" =~ ^[[:space:]]*# ]] && continue
            [[ -z "$key" ]] && continue

            # Remove leading/trailing whitespace
            key=$(echo "$key" | xargs)
            value=$(echo "$value" | xargs)

            # Store if it's a known key
            if [[ -v "CONFIG[$key]" ]]; then
                CONFIG[$key]="$value"
            fi
        done < "$CONFIG_FILE"

        log_success "Existing configuration loaded"
    fi
}

interactive_configure() {
    echo
    echo "=============================================================================="
    echo "ReasonKit Web - Configuration"
    echo "=============================================================================="
    echo
    echo "This wizard will help you configure reasonkit-web."
    echo "Press Enter to accept the default value shown in brackets."
    echo

    # Section: Logging
    echo -e "${GREEN}=== Logging ===${NC}"
    echo
    prompt_choice "Log level" "error, warn, info, debug, trace" "${CONFIG[RUST_LOG]}" "RUST_LOG"
    echo

    # Section: Browser
    echo -e "${GREEN}=== Browser Settings ===${NC}"
    echo

    # Detect Chrome if not set
    if [[ -z "${CONFIG[CHROME_PATH]}" ]]; then
        if detect_chrome; then
            log_info "Detected Chrome: ${CONFIG[CHROME_PATH]}"
        else
            log_warn "Chrome/Chromium not found. You may need to specify the path."
        fi
    fi

    prompt_value "Chrome/Chromium path (leave empty for auto-detect)" "${CONFIG[CHROME_PATH]}" "CHROME_PATH"
    prompt_bool "Run in headless mode?" "${CONFIG[REASONKIT_HEADLESS]}" "REASONKIT_HEADLESS"
    prompt_bool "Disable GPU acceleration?" "${CONFIG[REASONKIT_DISABLE_GPU]}" "REASONKIT_DISABLE_GPU"
    echo

    # Section: Performance
    echo -e "${GREEN}=== Performance ===${NC}"
    echo
    prompt_value "MCP request timeout (seconds)" "${CONFIG[MCP_TIMEOUT_SECS]}" "MCP_TIMEOUT_SECS"
    prompt_value "Tokio worker threads" "${CONFIG[TOKIO_WORKER_THREADS]}" "TOKIO_WORKER_THREADS"
    echo

    # Confirmation
    echo -e "${GREEN}=== Configuration Summary ===${NC}"
    echo
    echo "  RUST_LOG=${CONFIG[RUST_LOG]}"
    [[ -n "${CONFIG[CHROME_PATH]}" ]] && echo "  CHROME_PATH=${CONFIG[CHROME_PATH]}"
    echo "  REASONKIT_HEADLESS=${CONFIG[REASONKIT_HEADLESS]}"
    echo "  REASONKIT_DISABLE_GPU=${CONFIG[REASONKIT_DISABLE_GPU]}"
    echo "  MCP_TIMEOUT_SECS=${CONFIG[MCP_TIMEOUT_SECS]}"
    echo "  TOKIO_WORKER_THREADS=${CONFIG[TOKIO_WORKER_THREADS]}"
    echo

    echo -n "Save this configuration? [Y/n]: "
    read -r confirm

    if [[ "${confirm,,}" == "n" ]]; then
        log_info "Configuration cancelled"
        exit 0
    fi
}

non_interactive_configure() {
    log_info "Using non-interactive configuration..."

    # Override from environment
    [[ -n "${RUST_LOG:-}" ]] && CONFIG[RUST_LOG]="$RUST_LOG"
    [[ -n "${CHROME_PATH:-}" ]] && CONFIG[CHROME_PATH]="$CHROME_PATH"
    [[ -n "${REASONKIT_HEADLESS:-}" ]] && CONFIG[REASONKIT_HEADLESS]="$REASONKIT_HEADLESS"
    [[ -n "${REASONKIT_DISABLE_GPU:-}" ]] && CONFIG[REASONKIT_DISABLE_GPU]="$REASONKIT_DISABLE_GPU"
    [[ -n "${MCP_TIMEOUT_SECS:-}" ]] && CONFIG[MCP_TIMEOUT_SECS]="$MCP_TIMEOUT_SECS"
    [[ -n "${TOKIO_WORKER_THREADS:-}" ]] && CONFIG[TOKIO_WORKER_THREADS]="$TOKIO_WORKER_THREADS"

    # Auto-detect Chrome if not set
    if [[ -z "${CONFIG[CHROME_PATH]}" ]]; then
        detect_chrome || true
    fi
}

write_config() {
    log_info "Writing configuration to $CONFIG_FILE..."

    # Backup existing config
    if [[ -f "$CONFIG_FILE" ]]; then
        cp "$CONFIG_FILE" "${CONFIG_FILE}.bak"
        log_info "Backup created: ${CONFIG_FILE}.bak"
    fi

    cat > "$CONFIG_FILE" << EOF
# ReasonKit Web Configuration
# Generated: $(date -Iseconds)
# ==========================
#
# This file is sourced by the systemd service.
# After changes, run: systemctl restart reasonkit-web

# Logging level: error, warn, info, debug, trace
RUST_LOG=${CONFIG[RUST_LOG]}

EOF

    # Add Chrome path only if set
    if [[ -n "${CONFIG[CHROME_PATH]}" ]]; then
        cat >> "$CONFIG_FILE" << EOF
# Chrome/Chromium path
CHROME_PATH=${CONFIG[CHROME_PATH]}

EOF
    fi

    cat >> "$CONFIG_FILE" << EOF
# Browser options
REASONKIT_HEADLESS=${CONFIG[REASONKIT_HEADLESS]}
REASONKIT_DISABLE_GPU=${CONFIG[REASONKIT_DISABLE_GPU]}

# MCP server settings
MCP_TIMEOUT_SECS=${CONFIG[MCP_TIMEOUT_SECS]}

# Performance tuning
TOKIO_WORKER_THREADS=${CONFIG[TOKIO_WORKER_THREADS]}
EOF

    # Set permissions
    chmod 640 "$CONFIG_FILE"
    chown root:"$SERVICE_GROUP" "$CONFIG_FILE"

    log_success "Configuration saved"
}

# ------------------------------------------------------------------------------
# Service Management
# ------------------------------------------------------------------------------

manage_service() {
    if [[ "$ENABLE_SERVICE" == true ]]; then
        log_info "Enabling service..."
        systemctl enable "$SERVICE_NAME"
        log_success "Service enabled"
    fi

    if [[ "$START_SERVICE" == true ]]; then
        if systemctl is-active --quiet "$SERVICE_NAME"; then
            log_info "Restarting service..."
            systemctl restart "$SERVICE_NAME"
        else
            log_info "Starting service..."
            systemctl start "$SERVICE_NAME"
        fi
        log_success "Service started"
    fi
}

# ------------------------------------------------------------------------------
# Verification
# ------------------------------------------------------------------------------

verify_installation() {
    echo
    echo -e "${GREEN}=== Verification ===${NC}"
    echo

    # Check service status
    if systemctl is-active --quiet "$SERVICE_NAME"; then
        log_success "Service is running"

        # Get service info
        local pid
        pid=$(systemctl show --property=MainPID --value "$SERVICE_NAME")
        echo "  PID: $pid"

        # Memory usage
        if [[ -d "/proc/$pid" ]]; then
            local mem
            mem=$(awk '/VmRSS/{print $2}' /proc/$pid/status 2>/dev/null || echo "N/A")
            echo "  Memory: ${mem} kB"
        fi
    else
        log_warn "Service is not running"
        echo "  Start with: sudo systemctl start $SERVICE_NAME"
    fi

    # Check if enabled
    if systemctl is-enabled --quiet "$SERVICE_NAME" 2>/dev/null; then
        log_success "Service is enabled (starts on boot)"
    else
        log_info "Service is not enabled"
        echo "  Enable with: sudo systemctl enable $SERVICE_NAME"
    fi

    # Test health endpoint (if available)
    # Note: This depends on the MCP server having a health check
    echo

    echo "Configuration file: $CONFIG_FILE"
    echo "View logs: journalctl -u $SERVICE_NAME -f"
    echo
}

# ------------------------------------------------------------------------------
# Argument Parsing
# ------------------------------------------------------------------------------

show_help() {
    cat << EOF
ReasonKit Web - Configuration Script

Usage:
  sudo $0 [OPTIONS]

Options:
  --non-interactive    Use environment variables for configuration
  --start              Start/restart service after configuration
  --enable             Enable service to start on boot
  --help               Show this help message

Environment variables (for --non-interactive):
  RUST_LOG             Log level (error, warn, info, debug, trace)
  CHROME_PATH          Path to Chrome/Chromium binary
  REASONKIT_HEADLESS   Run browser in headless mode (true/false)
  MCP_TIMEOUT_SECS     MCP request timeout in seconds
  TOKIO_WORKER_THREADS Number of Tokio worker threads

Examples:
  sudo $0                           # Interactive configuration
  sudo $0 --start --enable          # Configure and start service

  # Non-interactive with environment variables
  sudo RUST_LOG=debug MCP_TIMEOUT_SECS=60 $0 --non-interactive --start

EOF
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --non-interactive)
                NON_INTERACTIVE=true
                shift
                ;;
            --start)
                START_SERVICE=true
                shift
                ;;
            --enable)
                ENABLE_SERVICE=true
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

    check_root
    check_installation
    load_existing_config

    if [[ "$NON_INTERACTIVE" == true ]]; then
        non_interactive_configure
    else
        interactive_configure
    fi

    write_config
    manage_service
    verify_installation

    echo "=============================================================================="
    echo -e "${GREEN}Configuration Complete${NC}"
    echo "=============================================================================="
}

main "$@"

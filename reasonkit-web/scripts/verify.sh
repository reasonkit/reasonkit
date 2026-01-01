#!/usr/bin/env bash
# ==============================================================================
# ReasonKit Web - Post-Installation Verification Script
# ==============================================================================
#
# Verifies that reasonkit-web is properly installed and running.
#
# Usage:
#   ./verify.sh [OPTIONS]
#
# Options:
#   --verbose    Show detailed output
#   --json       Output results as JSON
#   --help       Show this help message
#
# Exit codes:
#   0 - All checks passed
#   1 - One or more checks failed
#   2 - Script error
#
# ==============================================================================

set -euo pipefail

# ------------------------------------------------------------------------------
# Configuration
# ------------------------------------------------------------------------------

readonly PREFIX="/opt/reasonkit"
readonly BIN_PATH="${PREFIX}/bin/reasonkit-web"
readonly CONFIG_DIR="/etc/reasonkit"
readonly CONFIG_FILE="${CONFIG_DIR}/reasonkit-web.env"
readonly DATA_DIR="/var/lib/reasonkit"
readonly LOG_DIR="/var/log/reasonkit"
readonly SERVICE_NAME="reasonkit-web"
readonly SERVICE_USER="reasonkit"

# Options
VERBOSE=false
JSON_OUTPUT=false

# Colors
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m'

# Results tracking
declare -a RESULTS=()
PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0

# ------------------------------------------------------------------------------
# Utility Functions
# ------------------------------------------------------------------------------

log_check() {
    local status="$1"
    local name="$2"
    local detail="${3:-}"

    case "$status" in
        pass)
            RESULTS+=("{\"name\":\"$name\",\"status\":\"pass\",\"detail\":\"$detail\"}")
            ((PASS_COUNT++))
            if [[ "$JSON_OUTPUT" != true ]]; then
                echo -e "${GREEN}[PASS]${NC} $name"
                [[ "$VERBOSE" == true ]] && [[ -n "$detail" ]] && echo "       $detail"
            fi
            ;;
        fail)
            RESULTS+=("{\"name\":\"$name\",\"status\":\"fail\",\"detail\":\"$detail\"}")
            ((FAIL_COUNT++))
            if [[ "$JSON_OUTPUT" != true ]]; then
                echo -e "${RED}[FAIL]${NC} $name"
                [[ -n "$detail" ]] && echo "       $detail"
            fi
            ;;
        warn)
            RESULTS+=("{\"name\":\"$name\",\"status\":\"warn\",\"detail\":\"$detail\"}")
            ((WARN_COUNT++))
            if [[ "$JSON_OUTPUT" != true ]]; then
                echo -e "${YELLOW}[WARN]${NC} $name"
                [[ -n "$detail" ]] && echo "       $detail"
            fi
            ;;
    esac
}

# ------------------------------------------------------------------------------
# Verification Checks
# ------------------------------------------------------------------------------

check_binary() {
    if [[ -f "$BIN_PATH" ]]; then
        if [[ -x "$BIN_PATH" ]]; then
            local version
            version=$("$BIN_PATH" --version 2>&1 | head -1 || echo "unknown")
            log_check "pass" "Binary installed" "$version at $BIN_PATH"
        else
            log_check "fail" "Binary installed" "Not executable: $BIN_PATH"
        fi
    else
        log_check "fail" "Binary installed" "Not found: $BIN_PATH"
    fi
}

check_symlink() {
    local symlink="/usr/local/bin/reasonkit-web"

    if [[ -L "$symlink" ]]; then
        local target
        target=$(readlink -f "$symlink")
        if [[ "$target" == "$BIN_PATH" ]]; then
            log_check "pass" "Symlink correct" "$symlink -> $BIN_PATH"
        else
            log_check "warn" "Symlink exists but points elsewhere" "$symlink -> $target"
        fi
    else
        log_check "warn" "Symlink missing" "$symlink"
    fi
}

check_user() {
    if id "$SERVICE_USER" &>/dev/null; then
        local uid gid shell
        uid=$(id -u "$SERVICE_USER")
        gid=$(id -g "$SERVICE_USER")
        shell=$(getent passwd "$SERVICE_USER" | cut -d: -f7)
        log_check "pass" "Service user exists" "uid=$uid gid=$gid shell=$shell"
    else
        log_check "fail" "Service user exists" "User '$SERVICE_USER' not found"
    fi
}

check_directories() {
    local all_ok=true

    for dir in "$CONFIG_DIR" "$DATA_DIR" "$LOG_DIR"; do
        if [[ -d "$dir" ]]; then
            local owner perms
            owner=$(stat -c '%U:%G' "$dir")
            perms=$(stat -c '%a' "$dir")

            if [[ "$VERBOSE" == true ]] && [[ "$JSON_OUTPUT" != true ]]; then
                echo "       $dir: $owner ($perms)"
            fi
        else
            all_ok=false
        fi
    done

    if [[ "$all_ok" == true ]]; then
        log_check "pass" "Directories exist" "$CONFIG_DIR, $DATA_DIR, $LOG_DIR"
    else
        log_check "fail" "Directories exist" "One or more directories missing"
    fi
}

check_config_file() {
    if [[ -f "$CONFIG_FILE" ]]; then
        local perms owner
        perms=$(stat -c '%a' "$CONFIG_FILE")
        owner=$(stat -c '%U:%G' "$CONFIG_FILE")

        if [[ "$perms" == "640" ]]; then
            log_check "pass" "Configuration file" "$CONFIG_FILE ($perms, $owner)"
        else
            log_check "warn" "Configuration file" "Permissions should be 640, got $perms"
        fi
    else
        log_check "warn" "Configuration file" "Not found: $CONFIG_FILE"
    fi
}

check_systemd_service() {
    if [[ -f "/etc/systemd/system/${SERVICE_NAME}.service" ]]; then
        log_check "pass" "Systemd service file" "/etc/systemd/system/${SERVICE_NAME}.service"
    else
        log_check "fail" "Systemd service file" "Not found"
    fi
}

check_service_enabled() {
    if systemctl is-enabled --quiet "$SERVICE_NAME" 2>/dev/null; then
        log_check "pass" "Service enabled" "Will start on boot"
    else
        log_check "warn" "Service enabled" "Service will not start on boot"
    fi
}

check_service_running() {
    if systemctl is-active --quiet "$SERVICE_NAME" 2>/dev/null; then
        local pid uptime
        pid=$(systemctl show --property=MainPID --value "$SERVICE_NAME")

        # Get uptime
        local start_time
        start_time=$(systemctl show --property=ActiveEnterTimestamp --value "$SERVICE_NAME")
        uptime="since $start_time"

        log_check "pass" "Service running" "PID $pid, $uptime"
    else
        local status
        status=$(systemctl is-active "$SERVICE_NAME" 2>/dev/null || echo "unknown")
        log_check "fail" "Service running" "Status: $status"
    fi
}

check_chromium() {
    local chrome_path=""

    # Check from config
    if [[ -f "$CONFIG_FILE" ]]; then
        chrome_path=$(grep -E "^CHROME_PATH=" "$CONFIG_FILE" 2>/dev/null | cut -d= -f2 || true)
    fi

    # Auto-detect if not configured
    if [[ -z "$chrome_path" ]]; then
        for path in /usr/bin/chromium /usr/bin/chromium-browser /usr/bin/google-chrome; do
            if [[ -x "$path" ]]; then
                chrome_path="$path"
                break
            fi
        done
    fi

    if [[ -n "$chrome_path" ]] && [[ -x "$chrome_path" ]]; then
        local version
        version=$("$chrome_path" --version 2>/dev/null || echo "unknown")
        log_check "pass" "Chromium available" "$version at $chrome_path"
    else
        log_check "fail" "Chromium available" "Chrome/Chromium not found"
    fi
}

check_memory_usage() {
    if ! systemctl is-active --quiet "$SERVICE_NAME" 2>/dev/null; then
        log_check "warn" "Memory usage" "Service not running"
        return
    fi

    local pid
    pid=$(systemctl show --property=MainPID --value "$SERVICE_NAME")

    if [[ -f "/proc/$pid/status" ]]; then
        local rss_kb rss_mb
        rss_kb=$(awk '/VmRSS/{print $2}' "/proc/$pid/status" 2>/dev/null || echo "0")
        rss_mb=$((rss_kb / 1024))

        if [[ $rss_mb -lt 500 ]]; then
            log_check "pass" "Memory usage" "${rss_mb} MB RSS"
        elif [[ $rss_mb -lt 1500 ]]; then
            log_check "warn" "Memory usage" "${rss_mb} MB RSS (consider monitoring)"
        else
            log_check "warn" "Memory usage" "${rss_mb} MB RSS (high usage)"
        fi
    else
        log_check "warn" "Memory usage" "Could not read process info"
    fi
}

check_port_listening() {
    # MCP uses stdio, not ports, so this is informational
    # But we can check if there are any unexpected port bindings

    if ! systemctl is-active --quiet "$SERVICE_NAME" 2>/dev/null; then
        log_check "warn" "Port bindings" "Service not running"
        return
    fi

    local pid
    pid=$(systemctl show --property=MainPID --value "$SERVICE_NAME")

    if command -v ss &>/dev/null; then
        local ports
        ports=$(ss -tlnp 2>/dev/null | grep "pid=$pid" | awk '{print $4}' || true)

        if [[ -n "$ports" ]]; then
            log_check "pass" "Port bindings" "$ports"
        else
            log_check "pass" "Port bindings" "No ports (MCP uses stdio)"
        fi
    else
        log_check "pass" "Port bindings" "ss not available, skipping"
    fi
}

check_recent_errors() {
    if ! command -v journalctl &>/dev/null; then
        log_check "warn" "Recent errors" "journalctl not available"
        return
    fi

    local error_count
    error_count=$(journalctl -u "$SERVICE_NAME" --since "1 hour ago" -p err --no-pager -q 2>/dev/null | wc -l || echo "0")

    if [[ "$error_count" -eq 0 ]]; then
        log_check "pass" "Recent errors" "No errors in last hour"
    else
        log_check "warn" "Recent errors" "$error_count errors in last hour"

        if [[ "$VERBOSE" == true ]] && [[ "$JSON_OUTPUT" != true ]]; then
            echo "       Last 3 errors:"
            journalctl -u "$SERVICE_NAME" --since "1 hour ago" -p err --no-pager -q 2>/dev/null | tail -3 | while read -r line; do
                echo "         $line"
            done
        fi
    fi
}

# ------------------------------------------------------------------------------
# Output
# ------------------------------------------------------------------------------

print_summary() {
    if [[ "$JSON_OUTPUT" == true ]]; then
        echo "{"
        echo "  \"timestamp\": \"$(date -Iseconds)\","
        echo "  \"hostname\": \"$(hostname)\","
        echo "  \"summary\": {"
        echo "    \"passed\": $PASS_COUNT,"
        echo "    \"failed\": $FAIL_COUNT,"
        echo "    \"warnings\": $WARN_COUNT"
        echo "  },"
        echo "  \"checks\": ["
        local first=true
        for result in "${RESULTS[@]}"; do
            if [[ "$first" == true ]]; then
                first=false
            else
                echo ","
            fi
            echo -n "    $result"
        done
        echo
        echo "  ]"
        echo "}"
    else
        echo
        echo "=============================================================================="
        echo "Verification Summary"
        echo "=============================================================================="
        echo
        echo -e "  ${GREEN}Passed:${NC}   $PASS_COUNT"
        echo -e "  ${RED}Failed:${NC}   $FAIL_COUNT"
        echo -e "  ${YELLOW}Warnings:${NC} $WARN_COUNT"
        echo
        echo "------------------------------------------------------------------------------"

        if [[ $FAIL_COUNT -eq 0 ]]; then
            echo -e "${GREEN}All critical checks passed!${NC}"
        else
            echo -e "${RED}Some checks failed. Review the output above.${NC}"
        fi

        echo "=============================================================================="
    fi
}

# ------------------------------------------------------------------------------
# Argument Parsing
# ------------------------------------------------------------------------------

show_help() {
    cat << EOF
ReasonKit Web - Post-Installation Verification Script

Usage:
  $0 [OPTIONS]

Options:
  --verbose    Show detailed output
  --json       Output results as JSON
  --help       Show this help message

Exit codes:
  0 - All checks passed
  1 - One or more checks failed
  2 - Script error

Examples:
  $0                    # Basic verification
  $0 --verbose          # Detailed output
  $0 --json             # JSON output for monitoring

EOF
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --verbose|-v)
                VERBOSE=true
                shift
                ;;
            --json)
                JSON_OUTPUT=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                echo "Unknown option: $1" >&2
                exit 2
                ;;
        esac
    done
}

# ------------------------------------------------------------------------------
# Main
# ------------------------------------------------------------------------------

main() {
    parse_args "$@"

    if [[ "$JSON_OUTPUT" != true ]]; then
        echo
        echo "=============================================================================="
        echo "ReasonKit Web - Verification"
        echo "=============================================================================="
        echo
    fi

    # Run all checks
    check_binary
    check_symlink
    check_user
    check_directories
    check_config_file
    check_systemd_service
    check_service_enabled
    check_service_running
    check_chromium
    check_memory_usage
    check_port_listening
    check_recent_errors

    # Print summary
    print_summary

    # Exit code based on failures
    if [[ $FAIL_COUNT -gt 0 ]]; then
        exit 1
    fi

    exit 0
}

main "$@"

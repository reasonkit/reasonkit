#!/bin/bash
# Run All Tutorial Steps
# Usage: ./run_all.sh [--mock]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Pass through mock flag
MOCK_FLAG=""
if [[ "$1" == "--mock" ]]; then
    MOCK_FLAG="--mock"
    echo "Running in demo mode (no API calls)"
    echo ""
fi

echo "================================================"
echo "  ReasonKit Interactive Tutorial"
echo "  Total time: ~10 minutes"
echo "================================================"
echo ""

for step in step1_basic.sh step2_profiles.sh step3_individual_tools.sh step4_json_output.sh step5_audit_trail.sh; do
    if [ -f "$SCRIPT_DIR/$step" ]; then
        echo "Running $step..."
        echo ""
        bash "$SCRIPT_DIR/$step" $MOCK_FLAG
        
        if [ "$step" != "step5_audit_trail.sh" ]; then
            echo ""
            echo "Press Enter to continue to next step..."
            read -r
        fi
    fi
done

echo ""
echo "Tutorial complete! See docs/ for more resources."

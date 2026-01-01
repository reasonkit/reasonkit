#!/bin/bash
# Step 5: Execution Traces (Audit Trail)
# Time: ~90 seconds
#
# What you'll learn:
# - How ReasonKit logs every reasoning step
# - Viewing execution traces
# - Debugging and auditing decisions

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Step 5: Execution Traces (Audit Trail)${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}Why Audit Trails?${NC}"
echo ""
echo "  Every ReasonKit execution is logged:"
echo "    - What ThinkTools ran"
echo "    - What each step produced"
echo "    - Confidence scores at each stage"
echo "    - Total tokens and cost"
echo ""
echo "  This enables:"
echo "    - Debugging unexpected results"
echo "    - Compliance and auditing"
echo "    - Reproducing analysis"
echo "    - Learning from past decisions"
echo ""

# Check for mock mode
MOCK_FLAG=""
if [[ "$1" == "--mock" ]]; then
    MOCK_FLAG="--mock"
fi

echo -e "${YELLOW}Save an execution trace:${NC}"
echo ""
echo -e "  ${GREEN}rk-core think \"Evaluate cloud provider options\" --profile balanced --save-trace${NC}"
echo ""
echo "Press Enter to run..."
read -r

rk-core think "Evaluate cloud provider options" --profile balanced --save-trace $MOCK_FLAG 2>&1 | head -30

echo ""
echo -e "${YELLOW}View saved traces:${NC}"
echo ""
echo -e "  ${GREEN}rk-core trace list${NC}"
echo ""

# Show trace commands (may or may not be implemented)
echo "Trace commands:"
echo "  rk-core trace list              # Show all saved traces"
echo "  rk-core trace view <id>         # View a specific trace"
echo "  rk-core trace export <id>       # Export as JSON"
echo "  rk-core trace replay <id>       # Re-run with same parameters"
echo ""

echo -e "${YELLOW}Trace location:${NC}"
echo ""
echo "  Traces are stored in SQLite at:"
echo "  ~/.local/share/reasonkit/.rk_telemetry.db"
echo ""

echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Key Takeaway${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "  Every reasoning chain is auditable."
echo "  You can always see HOW a decision was made."
echo "  This is what 'structured reasoning' really means."
echo ""

echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Tutorial Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "  You've learned:"
echo "    [x] Basic ThinkTool usage"
echo "    [x] Choosing profiles for different stakes"
echo "    [x] Individual ThinkTools for specific tasks"
echo "    [x] JSON output for automation"
echo "    [x] Execution traces for auditing"
echo ""
echo "  Next steps:"
echo "    - Real-world examples: docs/USE_CASES.md"
echo "    - Full CLI reference: docs/CLI_REFERENCE.md"
echo "    - ThinkTool deep dive: docs/THINKTOOLS_GUIDE.md"
echo ""
echo "  Website: https://reasonkit.sh"
echo ""
echo -e "${CYAN}  Turn prompts into protocols.${NC}"
echo ""

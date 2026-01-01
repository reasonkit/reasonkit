#!/bin/bash
# Step 4: JSON Output for Automation
# Time: ~60 seconds
#
# What you'll learn:
# - Getting machine-readable output
# - Parsing with jq
# - Integrating with scripts and CI/CD

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Step 4: JSON Output for Automation${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}Why JSON?${NC}"
echo ""
echo "  Human-readable output is great for exploring."
echo "  JSON output is great for:"
echo "    - CI/CD pipelines"
echo "    - Script automation"
echo "    - Storing results"
echo "    - Programmatic decisions"
echo ""

# Check for mock mode
MOCK_FLAG=""
if [[ "$1" == "--mock" ]]; then
    MOCK_FLAG="--mock"
fi

echo -e "${YELLOW}Get JSON output with --format json:${NC}"
echo ""
echo -e "  ${GREEN}rk-core think \"Is this query safe?\" --profile quick --format json${NC}"
echo ""
echo "Press Enter to run..."
read -r

echo ""
rk-core think "Is this SQL query safe: SELECT * FROM users WHERE id = 1" --profile quick --format json $MOCK_FLAG 2>&1

echo ""
echo -e "${YELLOW}Parse specific fields with jq:${NC}"
echo ""

if command -v jq &> /dev/null; then
    echo "Extract just the verdict:"
    echo -e "  ${GREEN}rk-core think \"...\" --format json | jq -r '.verdict'${NC}"
    echo ""
    
    echo "Extract confidence score:"
    echo -e "  ${GREEN}rk-core think \"...\" --format json | jq '.confidence'${NC}"
    echo ""
    
    echo "Example CI/CD usage:"
    echo ""
    cat << 'EOF'
    # In your CI pipeline:
    RESULT=$(rk-core think "Is this PR safe to merge?" --format json)
    CONFIDENCE=$(echo "$RESULT" | jq '.confidence')
    
    if (( $(echo "$CONFIDENCE > 0.80" | bc -l) )); then
      echo "Auto-approving: confidence $CONFIDENCE"
      gh pr review --approve
    else
      echo "Manual review needed: confidence only $CONFIDENCE"
      gh pr review --request-changes
    fi
EOF
else
    echo "(jq not installed - install with: brew install jq or apt install jq)"
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Key Takeaway${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "  --format json gives you structured data"
echo "  Perfect for automation, CI/CD, and scripting"
echo "  Combine with jq for powerful pipelines"
echo ""
echo -e "${YELLOW}Next: Run ./step5_audit_trail.sh to learn about execution traces${NC}"
echo ""

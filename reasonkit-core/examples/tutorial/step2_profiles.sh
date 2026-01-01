#!/bin/bash
# Step 2: Choosing the Right Profile
# Time: ~60 seconds
#
# What you'll learn:
# - The 4 built-in profiles
# - When to use each one
# - How profiles affect speed vs thoroughness

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Step 2: Choosing the Right Profile${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}The 4 Built-in Profiles:${NC}"
echo ""
echo "  --profile quick      2 tools, ~30s, 70% confidence"
echo "                       Good for: Daily decisions, sanity checks"
echo ""
echo "  --profile balanced   5 tools, ~2min, 80% confidence"
echo "                       Good for: Important choices, PRs, architecture"
echo ""
echo "  --profile deep       5 tools + meta, ~5min, 85% confidence"
echo "                       Good for: Major decisions, design docs"
echo ""
echo "  --profile paranoid   5 tools + validation, ~10min, 95% confidence"
echo "                       Good for: Production releases, security"
echo ""

echo -e "${YELLOW}Let's compare quick vs balanced on the same question:${NC}"
echo ""

# Check for mock mode
MOCK_FLAG=""
if [[ "$1" == "--mock" ]]; then
    MOCK_FLAG="--mock"
fi

echo -e "${GREEN}Quick profile:${NC}"
echo ""
time rk-core think "Should I use TypeScript for a new project?" --profile quick $MOCK_FLAG 2>&1 | head -20

echo ""
echo -e "${YELLOW}Notice:${NC}"
echo "  - Fewer steps (GigaThink + LaserLogic only)"
echo "  - Faster execution"
echo "  - Lower confidence score"
echo ""

echo -e "${YELLOW}Press Enter to try balanced profile...${NC}"
read -r

echo ""
echo -e "${GREEN}Balanced profile:${NC}"
echo ""
time rk-core think "Should I use TypeScript for a new project?" --profile balanced $MOCK_FLAG 2>&1 | head -30

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Key Takeaway${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "  Match the profile to the stakes:"
echo "  - Slack message? --quick"
echo "  - PR review? --balanced"
echo "  - Architecture decision? --deep"
echo "  - Production deploy? --paranoid"
echo ""
echo -e "${YELLOW}Next: Run ./step3_individual_tools.sh to learn specific ThinkTools${NC}"
echo ""

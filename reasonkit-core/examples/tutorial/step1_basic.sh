#!/bin/bash
# Step 1: Basic ThinkTool Usage
# Time: ~60 seconds
#
# What you'll learn:
# - Running your first ReasonKit command
# - Understanding the output format
# - What "structured reasoning" means

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Step 1: Basic ThinkTool Usage${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}What we're doing:${NC}"
echo "  Running a simple analysis to see how ReasonKit structures thinking."
echo ""

echo -e "${YELLOW}The command:${NC}"
echo -e "  ${GREEN}rk-core think \"What makes a good API design?\" --profile quick${NC}"
echo ""

echo -e "${YELLOW}Press Enter to run...${NC}"
read -r

echo ""
echo -e "${CYAN}Running...${NC}"
echo ""

# Check for mock mode
MOCK_FLAG=""
if [[ "$1" == "--mock" ]]; then
    MOCK_FLAG="--mock"
    echo "(Running in demo mode - no API calls)"
    echo ""
fi

# Run the command
rk-core think "What makes a good API design?" --profile quick $MOCK_FLAG

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  What just happened?${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "  1. GigaThink generated multiple perspectives on API design"
echo "  2. LaserLogic validated the logic and found hidden assumptions"
echo "  3. ReasonKit synthesized a verdict with confidence score"
echo ""
echo "  This is 'structured reasoning' - instead of a blob of text,"
echo "  you get organized, auditable thinking."
echo ""
echo -e "${YELLOW}Next: Run ./step2_profiles.sh to learn about profiles${NC}"
echo ""

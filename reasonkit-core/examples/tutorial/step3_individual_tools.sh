#!/bin/bash
# Step 3: Individual ThinkTools
# Time: ~90 seconds
#
# What you'll learn:
# - What each ThinkTool does
# - When to use individual tools vs profiles
# - How to combine tools

set -e

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Step 3: Individual ThinkTools${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}The 5 ThinkTools:${NC}"
echo ""
echo -e "  ${MAGENTA}GigaThink (gt)${NC}     - Generate 10+ perspectives"
echo "                      Use when: You need diverse viewpoints"
echo ""
echo -e "  ${MAGENTA}LaserLogic (ll)${NC}    - Detect logical fallacies"
echo "                      Use when: Validating an argument"
echo ""
echo -e "  ${MAGENTA}BedRock (br)${NC}       - First principles decomposition"
echo "                      Use when: Cutting through complexity"
echo ""
echo -e "  ${MAGENTA}ProofGuard (pg)${NC}    - Multi-source verification"
echo "                      Use when: Fact-checking claims"
echo ""
echo -e "  ${MAGENTA}BrutalHonesty (bh)${NC} - Adversarial self-critique"
echo "                      Use when: Stress-testing your idea"
echo ""

# Check for mock mode
MOCK_FLAG=""
if [[ "$1" == "--mock" ]]; then
    MOCK_FLAG="--mock"
fi

echo -e "${YELLOW}Example 1: Generate perspectives with GigaThink${NC}"
echo ""
echo -e "  ${GREEN}rk-core think \"AI safety concerns\" --protocol gigathink${NC}"
echo ""
echo "Press Enter to run..."
read -r

rk-core think "AI safety concerns" --protocol gigathink $MOCK_FLAG 2>&1 | head -25

echo ""
echo -e "${YELLOW}Example 2: Check logic with LaserLogic${NC}"
echo ""
echo -e "  ${GREEN}rk-core think \"Rust is faster than Go, therefore Rust is better\" --protocol laserlogic${NC}"
echo ""
echo "Press Enter to run..."
read -r

rk-core think "Rust is faster than Go, therefore Rust is better" --protocol laserlogic $MOCK_FLAG 2>&1 | head -25

echo ""
echo -e "${YELLOW}Example 3: First principles with BedRock${NC}"
echo ""
echo -e "  ${GREEN}rk-core think \"What really matters for startup success?\" --protocol bedrock${NC}"
echo ""
echo "Press Enter to run..."
read -r

rk-core think "What really matters for startup success?" --protocol bedrock $MOCK_FLAG 2>&1 | head -25

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Key Takeaway${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "  Profiles run ALL tools in sequence."
echo "  Individual tools focus on ONE cognitive operation."
echo ""
echo "  Use individual tools when you need specific analysis,"
echo "  profiles when you want comprehensive evaluation."
echo ""
echo -e "${YELLOW}Next: Run ./step4_json_output.sh to learn about automation${NC}"
echo ""

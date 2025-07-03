#!/bin/bash

# TinyOS Test Suite Verification Script
# Verifies the organized test suite structure and provides usage examples

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}TinyOS Test Suite Organization Verification${NC}"
echo "==========================================="
echo

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "src" ]; then
    echo -e "${RED}Error: Must be run from TinyOS project root${NC}"
    exit 1
fi

echo -e "${BLUE}1. Verifying consolidated test suite files...${NC}"

# Check for new consolidated test suites
declare -a REQUIRED_SUITES=(
    "test_memory_suite.sh"
    "test_interrupt_suite.sh" 
    "test_hardware_suite.sh"
    "run_test_suites.sh"
    "run_tests.sh"
)

for suite in "${REQUIRED_SUITES[@]}"; do
    if [[ -f "$suite" ]]; then
        echo -e "  ${GREEN}✓${NC} $suite"
        chmod +x "$suite" 2>/dev/null || true
    else
        echo -e "  ${RED}✗${NC} $suite (missing)"
    fi
done

echo
echo -e "${BLUE}2. Verifying archived redundant scripts...${NC}"

# Check that old scripts are archived
declare -a ARCHIVED_SCRIPTS=(
    "test_memory.sh"
    "test_memory_comprehensive.sh"
    "test_memory_automated.sh"
    "test_interrupts.sh"
    "test_interrupts_automated.sh"
    "test_interrupts_simple.sh"
    "test_memory_simple.sh"
    "test_enhanced_memory.sh"
)

ARCHIVED_COUNT=0
for script in "${ARCHIVED_SCRIPTS[@]}"; do
    if [[ -f "archived_tests/$script" ]]; then
        echo -e "  ${GREEN}✓${NC} $script (archived)"
        ((ARCHIVED_COUNT++))
    elif [[ -f "$script" ]]; then
        echo -e "  ${YELLOW}⚠${NC} $script (still in root - should be archived)"
    else
        echo -e "  ${GREEN}✓${NC} $script (removed/archived)"
        ((ARCHIVED_COUNT++))
    fi
done

echo
echo -e "${BLUE}3. Test suite organization summary...${NC}"
echo -e "  ${GREEN}Memory Management Suite:${NC} test_memory_suite.sh"
echo -e "    - Allocation/deallocation tests"
echo -e "    - Memory protection and boundaries"
echo -e "    - Fragmentation analysis"
echo -e "    - Performance benchmarks"
echo
echo -e "  ${GREEN}Interrupt Management Suite:${NC} test_interrupt_suite.sh"
echo -e "    - Interrupt handler testing"
echo -e "    - Priority management"
echo -e "    - Nested interrupt scenarios"
echo -e "    - Performance analysis"
echo
echo -e "  ${GREEN}Hardware/Driver Suite:${NC} test_hardware_suite.sh"
echo -e "    - GPIO functionality"
echo -e "    - UART communication"
echo -e "    - Timer operations"
echo -e "    - Hardware diagnostics"

echo
echo -e "${BLUE}4. Usage examples...${NC}"
echo
echo -e "${YELLOW}Run all tests (recommended):${NC}"
echo "  ./run_tests.sh"
echo
echo -e "${YELLOW}Run only integration test suites:${NC}"
echo "  ./run_tests.sh integration"
echo
echo -e "${YELLOW}Run specific test suite:${NC}"
echo "  ./run_test_suites.sh memory"
echo "  ./run_test_suites.sh interrupt"
echo "  ./run_test_suites.sh hardware"
echo
echo -e "${YELLOW}Run tests in automated mode (for CI/CD):${NC}"
echo "  ./run_tests.sh all --mode automated"
echo
echo -e "${YELLOW}Quick validation tests:${NC}"
echo "  ./run_tests.sh integration --mode quick"

echo
echo -e "${BLUE}5. Documentation files...${NC}"
declare -a DOCS=(
    "TEST_SUITE_ORGANIZATION.md"
    "README.md"
)

for doc in "${DOCS[@]}"; do
    if [[ -f "$doc" ]]; then
        echo -e "  ${GREEN}✓${NC} $doc"
    else
        echo -e "  ${YELLOW}?${NC} $doc (check if exists)"
    fi
done

echo
echo -e "${GREEN}Test suite organization verification complete!${NC}"
echo
echo -e "${CYAN}Summary:${NC}"
echo -e "  • ${GREEN}${#REQUIRED_SUITES[@]}${NC} consolidated test suite files"
echo -e "  • ${GREEN}$ARCHIVED_COUNT${NC} redundant scripts archived/removed"
echo -e "  • ${GREEN}3${NC} functional test categories (Memory, Interrupt, Hardware)"
echo -e "  • ${GREEN}3${NC} test modes (interactive, automated, quick)"
echo
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Review TEST_SUITE_ORGANIZATION.md for detailed documentation"
echo "  2. Run './run_tests.sh --help' to see all options"
echo "  3. Try './run_test_suites.sh --list' to see available suites"
echo "  4. Execute './run_tests.sh' to verify everything works"

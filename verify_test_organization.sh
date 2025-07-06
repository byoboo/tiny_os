#!/bin/bash

# TinyOS Test Suite Verification Script
# Verifies the organized test suite structure and provides usage examples

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
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
    "test_tinyos.sh"
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
    "run_tests.sh"
    "run_test_suites.sh"
    "test_interactive.sh"
)

ARCHIVED_COUNT=0
for script in "${ARCHIVED_SCRIPTS[@]}"; do
    if [[ -f "archived_tests/$script" ]]; then
        echo -e "  ${GREEN}✓${NC} $script (archived)"
        ARCHIVED_COUNT=$((ARCHIVED_COUNT + 1))
    elif [[ -f "$script" ]]; then
        echo -e "  ${YELLOW}⚠${NC} $script (still in root - should be archived)"
    else
        echo -e "  ${GREEN}✓${NC} $script (removed/archived)"
        ARCHIVED_COUNT=$((ARCHIVED_COUNT + 1))
    fi
done

echo
echo -e "${BLUE}3. Test suite organization summary...${NC}"
echo -e "  ${GREEN}Unified Test Runner:${NC} test_tinyos.sh"
echo -e "    - Feature-organized testing (boot, memory, interrupts, hardware, unit)"
echo -e "    - Multiple modes: interactive, automated, quick"
echo -e "    - Comprehensive test reporting"
echo
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
echo "  ./test_tinyos.sh"
echo
echo -e "${YELLOW}Test specific OS features:${NC}"
echo "  ./test_tinyos.sh memory"
echo "  ./test_tinyos.sh interrupts hardware"
echo
echo -e "${YELLOW}Different test modes:${NC}"
echo "  ./test_tinyos.sh --mode automated all"
echo "  ./test_tinyos.sh --mode quick boot"
echo "  ./test_tinyos.sh --validate-only"
echo
echo -e "${YELLOW}Individual suite testing (legacy):${NC}"
echo "  ./test_memory_suite.sh --mode automated"
echo "  ./test_interrupt_suite.sh --mode quick"
echo "  ./test_hardware_suite.sh --mode interactive"

echo
echo -e "${BLUE}5. Documentation files...${NC}"
declare -a DOCS=(
    "README.md"
    "DOCS.md"
)

for doc in "${DOCS[@]}"; do
    if [[ -f "$doc" ]]; then
        echo -e "  ${GREEN}✓${NC} $doc"
    else
        echo -e "  ${YELLOW}?${NC} $doc (check if exists)"
    fi
done

echo
echo -e "${BLUE}6. Archived documentation...${NC}"
if [[ -d "archived_docs" ]]; then
    archived_count=$(find archived_docs -name "*.md" | wc -l)
    echo -e "  ${GREEN}✓${NC} archived_docs directory ($archived_count files archived)"
else
    echo -e "  ${YELLOW}?${NC} archived_docs directory (missing)"
fi

echo
echo -e "${GREEN}Test suite organization verification complete!${NC}"
echo
echo -e "${CYAN}Summary:${NC}"
echo -e "  • ${GREEN}1${NC} unified test runner (test_tinyos.sh)"
echo -e "  • ${GREEN}3${NC} feature-specific test suites"
echo -e "  • ${GREEN}$ARCHIVED_COUNT${NC} redundant scripts archived/removed"
echo -e "  • ${GREEN}5${NC} OS feature categories (boot, memory, interrupts, hardware, unit)"
echo -e "  • ${GREEN}3${NC} test modes (interactive, automated, quick)"
echo
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Review README.md for quick start and setup"
echo "  2. Review DOCS.md for detailed technical documentation"
echo "  3. Run './test_tinyos.sh --help' to see all test options"
echo "  4. Try './test_tinyos.sh --list' to see available features"
echo "  5. Execute './test_tinyos.sh' to verify everything works"

exit 0

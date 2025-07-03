#!/bin/bash

# TinyOS Comprehensive Test Runner
# This script runs the complete TinyOS test suite

set -e

echo "🧪 TinyOS Comprehensive Test Runner"
echo "═══════════════════════════════════════════════════════════════════════════════"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the TinyOS project root."
    exit 1
fi

# Check if this is the TinyOS project
if ! grep -q "name = \"tiny_os\"" Cargo.toml; then
    print_error "This doesn't appear to be the TinyOS project directory."
    exit 1
fi

print_status "Starting TinyOS comprehensive test suite..."

# Create test results directory
mkdir -p test_results
cd test_results

print_status "Cleaning previous test results..."
rm -f *.json *.xml *.log

cd ..

# Function to run tests with timeout
run_test_with_timeout() {
    local test_name="$1"
    local timeout_duration="$2"
    local test_command="$3"
    
    print_status "Running $test_name..."
    
    if timeout $timeout_duration $test_command; then
        print_success "$test_name completed successfully"
        return 0
    else
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            print_error "$test_name timed out after $timeout_duration"
        else
            print_error "$test_name failed with exit code $exit_code"
        fi
        return $exit_code
    fi
}

# Test execution flags
RUN_UNIT_TESTS=true
RUN_INTEGRATION_TESTS=true
RUN_PERFORMANCE_TESTS=true
RUN_COMPREHENSIVE=true
GENERATE_REPORTS=true

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --unit-only)
            RUN_UNIT_TESTS=true
            RUN_INTEGRATION_TESTS=false
            RUN_PERFORMANCE_TESTS=false
            RUN_COMPREHENSIVE=false
            shift
            ;;
        --integration-only)
            RUN_UNIT_TESTS=false
            RUN_INTEGRATION_TESTS=true
            RUN_PERFORMANCE_TESTS=false
            RUN_COMPREHENSIVE=false
            shift
            ;;
        --performance-only)
            RUN_UNIT_TESTS=false
            RUN_INTEGRATION_TESTS=false
            RUN_PERFORMANCE_TESTS=true
            RUN_COMPREHENSIVE=false
            shift
            ;;
        --no-reports)
            GENERATE_REPORTS=false
            shift
            ;;
        --help|-h)
            echo "TinyOS Test Runner"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --unit-only       Run only unit tests"
            echo "  --integration-only Run only integration tests"
            echo "  --performance-only Run only performance tests"
            echo "  --no-reports      Skip generating test reports"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "By default, all tests are run with full reporting."
            exit 0
            ;;
        *)
            print_warning "Unknown option: $1"
            shift
            ;;
    esac
done

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

# Function to update test counters
update_test_results() {
    if [ $1 -eq 0 ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
}

print_status "Checking Rust environment..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust and Cargo."
    exit 1
fi

# Check if required dependencies are available
print_status "Checking dependencies..."
if ! cargo check --lib > /dev/null 2>&1; then
    print_warning "Some dependencies may be missing. Attempting to install..."
    cargo update
fi

print_status "Starting test execution..."

# Run unit tests
if [ "$RUN_UNIT_TESTS" = true ]; then
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    print_status "Running Unit Tests"
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    
    if run_test_with_timeout "Unit Tests" "30s" "cargo test --lib unit_tests_only"; then
        update_test_results 0
    else
        update_test_results 1
    fi
fi

# Run integration tests
if [ "$RUN_INTEGRATION_TESTS" = true ]; then
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    print_status "Running Integration Tests"
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    
    if run_test_with_timeout "Integration Tests" "60s" "cargo test --lib integration_tests_only"; then
        update_test_results 0
    else
        update_test_results 1
    fi
fi

# Run performance tests
if [ "$RUN_PERFORMANCE_TESTS" = true ]; then
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    print_status "Running Performance Tests"
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    
    if run_test_with_timeout "Performance Tests" "30s" "cargo test --lib performance_benchmark"; then
        update_test_results 0
    else
        update_test_results 1
    fi
fi

# Run comprehensive test suite
if [ "$RUN_COMPREHENSIVE" = true ]; then
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    print_status "Running Comprehensive Test Suite"
    print_status "═══════════════════════════════════════════════════════════════════════════════"
    
    if run_test_with_timeout "Comprehensive Tests" "120s" "cargo test --lib run_comprehensive_tests"; then
        update_test_results 0
    else
        update_test_results 1
    fi
fi

# Move test results to results directory
if [ "$GENERATE_REPORTS" = true ]; then
    print_status "Generating test reports..."
    
    # Move JSON reports if they exist
    if [ -f "unit_test_results.json" ]; then
        mv unit_test_results.json test_results/
        print_success "Unit test results saved to test_results/unit_test_results.json"
    fi
    
    if [ -f "integration_test_results.json" ]; then
        mv integration_test_results.json test_results/
        print_success "Integration test results saved to test_results/integration_test_results.json"
    fi
    
    # Generate summary report
    cat > test_results/test_summary.txt << EOF
TinyOS Test Suite Summary
========================
Generated: $(date)

Test Results:
- Total Tests: $TOTAL_TESTS
- Passed: $TESTS_PASSED
- Failed: $TESTS_FAILED
- Pass Rate: $(( TESTS_PASSED * 100 / TOTAL_TESTS ))%

Test Categories:
$([ "$RUN_UNIT_TESTS" = true ] && echo "- Unit Tests: Executed")
$([ "$RUN_INTEGRATION_TESTS" = true ] && echo "- Integration Tests: Executed")
$([ "$RUN_PERFORMANCE_TESTS" = true ] && echo "- Performance Tests: Executed")
$([ "$RUN_COMPREHENSIVE" = true ] && echo "- Comprehensive Suite: Executed")

System Information:
- OS: $(uname -s)
- Architecture: $(uname -m)
- Rust Version: $(rustc --version)
- Cargo Version: $(cargo --version)

EOF
    
    print_success "Test summary saved to test_results/test_summary.txt"
fi

# Final results
print_status "═══════════════════════════════════════════════════════════════════════════════"
print_status "Test Execution Complete"
print_status "═══════════════════════════════════════════════════════════════════════════════"

echo ""
echo "📊 Final Results:"
echo "   Total Tests: $TOTAL_TESTS"
echo "   Passed: $TESTS_PASSED"
echo "   Failed: $TESTS_FAILED"

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "🎉 ALL TESTS PASSED! TinyOS is ready for deployment! 🎉"
    exit 0
elif [ $TESTS_PASSED -gt $TESTS_FAILED ]; then
    print_warning "⚠️  Most tests passed, but $TESTS_FAILED test(s) failed. Please investigate."
    exit 1
else
    print_error "❌ Significant test failures detected. System needs attention."
    exit 1
fi

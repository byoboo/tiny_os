#!/bin/bash

# TinyOS Version Management Script
# Helps manage semantic versioning for releases

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
}

# Update version in Cargo.toml
update_version() {
    local new_version=$1
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    echo -e "${GREEN}✓${NC} Updated version to $new_version"
}

# Show help
show_help() {
    echo -e "${BLUE}TinyOS Version Management${NC}"
    echo "========================="
    echo ""
    echo "Usage: $0 [command] [version]"
    echo ""
    echo "Commands:"
    echo "  current                 Show current version"
    echo "  set <version>          Set specific version (e.g., 0.3.0)"
    echo "  bump major             Bump major version (0.2.0 → 1.0.0)"
    echo "  bump minor             Bump minor version (0.2.0 → 0.3.0)"
    echo "  bump patch             Bump patch version (0.2.0 → 0.2.1)"
    echo "  dev                    Set development version (0.2.0-dev.1)"
    echo "  stable                 Remove pre-release suffix (0.2.0-dev.1 → 0.2.0)"
    echo "  check                  Validate version format"
    echo ""
    echo "Examples:"
    echo "  $0 current"
    echo "  $0 set 0.3.0"
    echo "  $0 bump minor"
    echo "  $0 dev"
    echo "  $0 stable"
}

# Validate version format
validate_version() {
    local version=$1
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
        echo -e "${RED}✗${NC} Invalid version format: $version"
        echo -e "${YELLOW}Expected format: X.Y.Z or X.Y.Z-prerelease${NC}"
        exit 1
    fi
}

# Parse version components
parse_version() {
    local version=$1
    
    # Remove pre-release suffix if present
    local base_version=$(echo "$version" | sed 's/-.*//')
    
    # Extract major, minor, patch
    MAJOR=$(echo "$base_version" | cut -d. -f1)
    MINOR=$(echo "$base_version" | cut -d. -f2)
    PATCH=$(echo "$base_version" | cut -d. -f3)
    
    # Extract pre-release suffix if present
    if [[ $version == *-* ]]; then
        PRERELEASE=$(echo "$version" | sed 's/[^-]*-//')
    else
        PRERELEASE=""
    fi
}

# Bump version
bump_version() {
    local bump_type=$1
    local current_version=$(get_current_version)
    
    parse_version "$current_version"
    
    case $bump_type in
        major)
            MAJOR=$((MAJOR + 1))
            MINOR=0
            PATCH=0
            ;;
        minor)
            MINOR=$((MINOR + 1))
            PATCH=0
            ;;
        patch)
            PATCH=$((PATCH + 1))
            ;;
        *)
            echo -e "${RED}✗${NC} Invalid bump type: $bump_type"
            echo -e "${YELLOW}Valid types: major, minor, patch${NC}"
            exit 1
            ;;
    esac
    
    local new_version="$MAJOR.$MINOR.$PATCH"
    update_version "$new_version"
    echo -e "${GREEN}✓${NC} Bumped $bump_type version: $current_version → $new_version"
}

# Set development version
set_dev_version() {
    local current_version=$(get_current_version)
    parse_version "$current_version"
    
    local base_version="$MAJOR.$MINOR.$PATCH"
    local dev_version="$base_version-dev.1"
    
    update_version "$dev_version"
    echo -e "${GREEN}✓${NC} Set development version: $current_version → $dev_version"
}

# Set stable version (remove pre-release)
set_stable_version() {
    local current_version=$(get_current_version)
    parse_version "$current_version"
    
    local stable_version="$MAJOR.$MINOR.$PATCH"
    
    if [[ -z "$PRERELEASE" ]]; then
        echo -e "${YELLOW}⚠${NC} Version is already stable: $current_version"
        return
    fi
    
    update_version "$stable_version"
    echo -e "${GREEN}✓${NC} Set stable version: $current_version → $stable_version"
}

# Check version and build info
check_version() {
    local current_version=$(get_current_version)
    echo -e "${BLUE}Version Information${NC}"
    echo "==================="
    echo "Current version: $current_version"
    
    parse_version "$current_version"
    echo "Major: $MAJOR"
    echo "Minor: $MINOR"
    echo "Patch: $PATCH"
    
    if [[ -n "$PRERELEASE" ]]; then
        echo "Pre-release: $PRERELEASE"
        echo -e "${YELLOW}⚠${NC} This is a pre-release version"
    else
        echo -e "${GREEN}✓${NC} This is a stable version"
    fi
    
    # Check if we're on a git branch
    if command -v git &> /dev/null && git rev-parse --git-dir > /dev/null 2>&1; then
        local branch=$(git rev-parse --abbrev-ref HEAD)
        local commit=$(git rev-parse --short HEAD)
        echo "Git branch: $branch"
        echo "Git commit: $commit"
        
        # Suggest appropriate versioning
        case $branch in
            master)
                if [[ -n "$PRERELEASE" ]]; then
                    echo -e "${YELLOW}💡${NC} Consider setting stable version for master branch"
                fi
                ;;
            dev)
                if [[ -z "$PRERELEASE" ]]; then
                    echo -e "${YELLOW}💡${NC} Consider setting dev version for dev branch"
                fi
                ;;
            feature/*)
                echo -e "${YELLOW}💡${NC} Feature branch detected - version will not be published"
                ;;
        esac
    fi
}

# Main script logic
case "${1:-help}" in
    current)
        echo $(get_current_version)
        ;;
    set)
        if [[ -z "$2" ]]; then
            echo -e "${RED}✗${NC} Version required"
            echo "Usage: $0 set <version>"
            exit 1
        fi
        validate_version "$2"
        update_version "$2"
        ;;
    bump)
        if [[ -z "$2" ]]; then
            echo -e "${RED}✗${NC} Bump type required"
            echo "Usage: $0 bump [major|minor|patch]"
            exit 1
        fi
        bump_version "$2"
        ;;
    dev)
        set_dev_version
        ;;
    stable)
        set_stable_version
        ;;
    check)
        check_version
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo -e "${RED}✗${NC} Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac

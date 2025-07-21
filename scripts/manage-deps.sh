#!/bin/bash
# MACAWI AI External Dependency Management Script

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Functions
print_help() {
    cat << EOF
MACAWI AI Dependency Management

Usage: $0 [command] [options]

Commands:
  init              Initialize all git submodules
  update [name]     Update specific submodule or all if no name given
  status            Show status of all dependencies
  audit             Run security audit on dependencies
  add-submodule     Interactive guide to add new submodule
  vendor [path]     Vendor a critical dependency
  clean             Remove untracked files from submodules

Examples:
  $0 init                    # First time setup
  $0 update metaspt          # Update specific submodule
  $0 status                  # Check all dependencies
  $0 audit                   # Security check

EOF
}

init_submodules() {
    echo -e "${GREEN}Initializing submodules...${NC}"
    cd "$PROJECT_ROOT"
    git submodule update --init --recursive
    echo -e "${GREEN}✓ Submodules initialized${NC}"
}

update_submodules() {
    local target="$1"
    cd "$PROJECT_ROOT"
    
    if [[ -z "$target" ]]; then
        echo -e "${YELLOW}Updating all submodules...${NC}"
        git submodule update --remote
    else
        echo -e "${YELLOW}Updating $target...${NC}"
        git submodule update --remote "external/$target"
    fi
    
    echo -e "${GREEN}✓ Update complete${NC}"
}

show_status() {
    echo -e "${GREEN}=== Dependency Status ===${NC}"
    cd "$PROJECT_ROOT"
    
    echo -e "\n${YELLOW}Git Submodules:${NC}"
    if [[ -f .gitmodules ]]; then
        git submodule status
    else
        echo "No submodules configured"
    fi
    
    echo -e "\n${YELLOW}Vendored Dependencies:${NC}"
    if [[ -d vendor/critical ]] && [[ "$(ls -A vendor/critical)" ]]; then
        find vendor/critical -maxdepth 1 -type d -name "*" ! -path vendor/critical | while read -r dep; do
            if [[ -f "$dep/VENDOR_INFO.txt" ]]; then
                echo "- $(basename "$dep"): $(head -n1 "$dep/VENDOR_INFO.txt")"
            fi
        done
    else
        echo "No vendored dependencies"
    fi
}

security_audit() {
    echo -e "${YELLOW}Running security audit...${NC}"
    cd "$PROJECT_ROOT"
    
    # Check for security tools
    local has_trivy=$(command -v trivy &> /dev/null && echo "yes" || echo "no")
    local has_safety=$(command -v safety &> /dev/null && echo "yes" || echo "no")
    
    if [[ "$has_trivy" == "no" ]] && [[ "$has_safety" == "no" ]]; then
        echo -e "${RED}No security scanning tools found${NC}"
        echo "Consider installing:"
        echo "  - Trivy: https://github.com/aquasecurity/trivy"
        echo "  - Safety: pip install safety"
        exit 1
    fi
    
    # Scan submodules
    if [[ -d external ]] && [[ "$(ls -A external)" ]]; then
        for submodule in external/*; do
            if [[ -d "$submodule" ]]; then
                echo -e "\n${YELLOW}Scanning $(basename "$submodule")...${NC}"
                
                if [[ "$has_trivy" == "yes" ]]; then
                    trivy fs "$submodule" || true
                fi
            fi
        done
    fi
    
    echo -e "\n${GREEN}✓ Audit complete${NC}"
}

add_submodule_interactive() {
    echo -e "${GREEN}=== Add New Submodule ===${NC}"
    
    read -rp "Repository URL: " repo_url
    read -rp "Submodule name: " submodule_name
    read -rp "Specific tag/commit (leave empty for latest): " version
    
    cd "$PROJECT_ROOT"
    
    # Add submodule
    git submodule add "$repo_url" "external/$submodule_name"
    
    # Pin to specific version if provided
    if [[ -n "$version" ]]; then
        cd "external/$submodule_name"
        git checkout "$version"
        cd "$PROJECT_ROOT"
        git add "external/$submodule_name"
        echo -e "${GREEN}✓ Pinned to version: $version${NC}"
    fi
    
    # Update documentation
    echo -e "\n${YELLOW}Remember to update EXTERNAL_DEPS.md with:${NC}"
    echo "- Purpose of this dependency"
    echo "- Security considerations"
    echo "- Update policy"
}

vendor_dependency() {
    local source_path="$1"
    
    if [[ ! -d "$source_path" ]]; then
        echo -e "${RED}Error: $source_path does not exist${NC}"
        exit 1
    fi
    
    local dep_name=$(basename "$source_path")
    local vendor_path="$PROJECT_ROOT/vendor/critical/$dep_name"
    
    echo -e "${YELLOW}Vendoring $dep_name...${NC}"
    
    # Create vendor directory
    mkdir -p "$vendor_path"
    
    # Copy source
    cp -r "$source_path"/* "$vendor_path/"
    
    # Create vendor info
    cat > "$vendor_path/VENDOR_INFO.txt" << EOF
Vendored from: $source_path
Date: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Commit: $(cd "$source_path" && git rev-parse HEAD 2>/dev/null || echo "unknown")

Reason: [MANUAL EDIT REQUIRED - Add vendoring justification]

Security Audit:
- Date: $(date -u +"%Y-%m-%d")
- Reviewer: [MANUAL EDIT REQUIRED]
- Result: [MANUAL EDIT REQUIRED]
EOF
    
    echo -e "${GREEN}✓ Vendored to: $vendor_path${NC}"
    echo -e "${YELLOW}⚠ Edit VENDOR_INFO.txt to complete documentation${NC}"
}

clean_submodules() {
    echo -e "${YELLOW}Cleaning submodules...${NC}"
    cd "$PROJECT_ROOT"
    
    git submodule foreach 'git clean -fdx'
    
    echo -e "${GREEN}✓ Submodules cleaned${NC}"
}

# Main script logic
case "${1:-help}" in
    init)
        init_submodules
        ;;
    update)
        update_submodules "${2:-}"
        ;;
    status)
        show_status
        ;;
    audit)
        security_audit
        ;;
    add-submodule)
        add_submodule_interactive
        ;;
    vendor)
        if [[ -z "${2:-}" ]]; then
            echo -e "${RED}Error: Please provide path to vendor${NC}"
            exit 1
        fi
        vendor_dependency "$2"
        ;;
    clean)
        clean_submodules
        ;;
    help|--help|-h)
        print_help
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        print_help
        exit 1
        ;;
esac
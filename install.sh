#!/usr/bin/env bash

set -e

REPO_URL="https://github.com/GabrielEVP/gabriel-configurations.git"
CLONE_DIR="/tmp/gabriel-configurations-install"
CONFIG_DIR="$HOME/.config"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

info()    { echo -e "${GREEN}[INFO]${NC} $1"; }
warn()    { echo -e "${YELLOW}[WARN]${NC} $1"; }
error()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

check_dependency() {
  if ! command -v "$1" &>/dev/null; then
    error "Required dependency '$1' is not installed. Please install it and try again."
  fi
}

backup_existing() {
  local target="$1"
  if [ -d "$target" ] || [ -f "$target" ]; then
    local backup="${target}.backup.$(date +%Y%m%d%H%M%S)"
    warn "Backing up existing $target to $backup"
    mv "$target" "$backup"
  fi
}

info "Checking dependencies..."
check_dependency git
check_dependency rsync

info "Cloning gabriel-configurations..."
rm -rf "$CLONE_DIR"
git clone "$REPO_URL" "$CLONE_DIR"

info "Installing ghostty configuration..."
backup_existing "$CONFIG_DIR/ghostty"
mkdir -p "$CONFIG_DIR/ghostty"
rsync -a "$CLONE_DIR/ghostty/" "$CONFIG_DIR/ghostty/"

info "Installing nvim configuration..."
backup_existing "$CONFIG_DIR/nvim"
mkdir -p "$CONFIG_DIR/nvim"
rsync -a "$CLONE_DIR/nvim/" "$CONFIG_DIR/nvim/"

info "Installing opencode configuration..."
backup_existing "$CONFIG_DIR/opencode"
mkdir -p "$CONFIG_DIR/opencode"
rsync -a "$CLONE_DIR/opencode/" "$CONFIG_DIR/opencode/"

info "Cleaning up..."
rm -rf "$CLONE_DIR"

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo ""
echo -e "${YELLOW}IMPORTANT — You must add your API tokens manually:${NC}"
echo ""
echo "  Edit: $CONFIG_DIR/opencode/config.json"
echo ""
echo "  Replace the following placeholders with your real tokens:"
echo "    YOUR_GITHUB_TOKEN   → GitHub Personal Access Token"
echo "    YOUR_GITLAB_TOKEN   → GitLab Personal Access Token"
echo "    YOUR_FIGMA_API_KEY  → Figma API Key"
echo "    YOUR_NOTION_TOKEN   → Notion Integration Token"
echo ""
echo "  Also update the github-mcp-server path if needed:"
echo "    Default path: /home/linuxbrew/.linuxbrew/bin/github-mcp-server"
echo ""
echo -e "${YELLOW}Font requirement:${NC}"
echo "  ghostty uses 'IosevkaTerm NF' — install it from:"
echo "  https://www.nerdfonts.com/font-downloads"
echo ""

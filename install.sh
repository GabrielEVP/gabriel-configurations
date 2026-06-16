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

detect_os() {
  case "$(uname -s)" in
    Darwin*)  echo "macos" ;;
    Linux*)   echo "linux" ;;
    *)        error "Unsupported OS: $(uname -s)" ;;
  esac
}

detect_brew_prefix() {
  if command -v brew &>/dev/null; then
    brew --prefix 2>/dev/null || echo "/opt/homebrew"
  elif [ -f "/home/linuxbrew/.linuxbrew/bin/brew" ]; then
    echo "/home/linuxbrew/.linuxbrew"
  else
    echo ""
  fi
}

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

OS=$(detect_os)
BREW_PREFIX=$(detect_brew_prefix)
info "Detected OS: $OS"
[ -n "$BREW_PREFIX" ] && info "Homebrew prefix: $BREW_PREFIX" || warn "Homebrew not found — some features may not work"

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

# Fix GitHub MCP server path for non-macOS
if [ "$OS" != "macos" ] && [ -n "$BREW_PREFIX" ]; then
  if [ -f "$CONFIG_DIR/opencode/opencode.json" ]; then
    sed -i "s|/opt/homebrew|$BREW_PREFIX|g" "$CONFIG_DIR/opencode/opencode.json"
    info "Updated brew paths in opencode.json for $OS"
  fi
fi

info "Installing zsh configuration..."
backup_existing "$HOME/.zshrc"
backup_existing "$HOME/.zprofile"
backup_existing "$HOME/.zshenv"
ln -sf "$CLONE_DIR/zsh/zshrc" "$HOME/.zshrc"
ln -sf "$CLONE_DIR/zsh/zprofile" "$HOME/.zprofile"
ln -sf "$CLONE_DIR/zsh/zshenv" "$HOME/.zshenv"

info "Cleaning up..."
rm -rf "$CLONE_DIR"

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo ""

if [ "$OS" = "macos" ]; then
  echo -e "${YELLOW}Post-install:${NC}"
  echo "  1. Ensure your API tokens are in ~/.config/opencode/opencode.json"
  echo "  2. Update the GitHub MCP server path if your brew prefix differs"
  echo ""
  echo -e "${YELLOW}Font:${NC}"
  echo "  ghostty uses 'IosevkaTerm NF' — install via: brew install --cask font-iosevka-term-nerd-font"
  echo ""
  echo -e "${YELLOW}Dependencies (brew):${NC}"
  echo "  brew install neovim git rsync zsh-autosuggestions zsh-syntax-highlighting zsh-autocomplete"
  echo "  brew install --cask ghostty"
else
  echo -e "${YELLOW}Post-install:${NC}"
  echo "  1. Edit ~/.config/opencode/opencode.json and replace tokens:"
  echo "     - YOUR_GITHUB_TOKEN"
  echo "     - YOUR_GITLAB_TOKEN"
  echo "     - YOUR_FIGMA_API_KEY"
  echo "     - YOUR_NOTION_TOKEN"
  echo "  2. Update the GitHub MCP server path if needed"
  echo ""
  echo -e "${YELLOW}Font:${NC}"
  echo "  ghostty uses 'IosevkaTerm NF' — download from: https://www.nerdfonts.com/font-downloads"
  echo ""
  echo -e "${YELLOW}Dependencies:${NC}"
  echo "  Install neovim, git, rsync, zsh plugins via your package manager"
fi

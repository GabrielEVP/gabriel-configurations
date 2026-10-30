---
name: os-macos
description: "Trigger: macos, mac, osx, homebrew, brew, defaults, plist, mac terminal, macOS setup, dotfiles mac. macOS system administration — Homebrew, system preferences, defaults, app management, dotfiles, troubleshooting."
---

# macOS Administration

## Homebrew (Package Manager)

### Core Commands
```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install a formula
brew install <formula>

# Install a cask (GUI app)
brew install --cask <cask>

# Update all
brew update && brew upgrade

# Cleanup old versions
brew cleanup

# List installed
brew list
brew list --cask

# Show info
brew info <formula>

# Search
brew search <keyword>

# Outdated packages
brew outdated

# Pin a version (prevent upgrade)
brew pin <formula>
brew unpin <formula>
```

### Brew Services
```bash
# Start a service (e.g. postgresql, mysql, redis)
brew services start <formula>

# Stop a service
brew services stop <formula>

# Restart a service
brew services restart <formula>

# List all services
brew services list
```

### Common Formulas & Casks
```bash
# CLI essentials
brew install git curl wget jq yq tree ripgrep fd fzf bat exa htop btop neovim tmux

# Languages / runtimes
brew install node python go java openjdk rust

# Docker
brew install --cask docker

# Browsers
brew install --cask google-chrome firefox

# Dev tools
brew install --cask visual-studio-code iterm2 warp postman ngrok

# Media
brew install --cask spotify vlc
```

## System Preferences (defaults write)

### Read current setting
```bash
defaults read <domain> <key>
```

### Examples
```bash
# Show hidden files in Finder
defaults write com.apple.finder AppleShowAllFiles -bool true
killall Finder

# Show path bar in Finder
defaults write com.apple.finder ShowPathbar -bool true

# Show status bar in Finder
defaults write com.apple.finder ShowStatusBar -bool true

# Keep folders on top when sorting by name
defaults write com.apple.finder _FXSortFoldersFirst -bool true

# Disable .DS_Store on network volumes
defaults write com.apple.desktopservices DSDontWriteNetworkStores -bool true

# Enable tap to click on trackpad
defaults write com.apple.driver.AppleBluetoothMultitouch.trackpad Clicking -bool true
defaults -currentHost write NSGlobalDomain com.apple.mouse.tapBehavior -int 1

# Faster key repeat
defaults write NSGlobalDomain KeyRepeat -int 1
defaults write NSGlobalDomain InitialKeyRepeat -int 15

# Disable auto-correct
defaults write NSGlobalDomain NSAutomaticSpellingCorrectionEnabled -bool false

# Disable smart quotes
defaults write NSGlobalDomain NSAutomaticQuoteSubstitutionEnabled -bool false

# Show battery percentage in menu bar
defaults write com.apple.controlcenter BatteryShowPercentage -bool true

# Set screenshot location
defaults write com.apple.screencapture location ~/Screenshots
killall SystemUIServer

# Save screenshots in PNG (default)
defaults write com.apple.screencapture type -string "png"
```

## Dock & Menu Bar

```bash
# Auto-hide dock
defaults write com.apple.dock autohide -bool true
defaults write com.apple.dock autohide-delay -float 0
defaults write com.apple.dock autohide-time-modifier -float 0.5

# Magnification off
defaults write com.apple.dock magnification -bool false

# Minimize to application icon
defaults write com.apple.dock minimize-to-application -bool true

# Remove all icons from Dock (start fresh)
defaults write com.apple.dock persistent-apps -array
killall Dock

# Reset Dock to defaults
defaults delete com.apple.dock
killall Dock
```

## Application Management

### macOS built-in
```bash
# Open app
open -a <ApplicationName>

# Open file with app
open -a <ApplicationName> <file>

# Open current directory in Finder
open .

# Uninstall (drag to Trash from /Applications)
# Check for leftovers:
ls ~/Library/Preferences/ | grep <app-name>
ls ~/Library/Application\ Support/ | grep <app-name>
ls ~/Library/Caches/ | grep <app-name>
```

### Using Homebrew
```bash
# Install a cask
brew install --cask <app-name>

# Uninstall a cask
brew uninstall --cask <app-name>

# Zap (remove all config and data too)
brew uninstall --zap --cask <app-name>
```

### Using MAS (Mac App Store)
```bash
# Install mas
brew install mas

# Search the App Store
mas search <app-name>

# Install an app by ID
mas install <app-id>

# List installed from App Store
mas list

# Upgrade all App Store apps
mas upgrade
```

## macOS Shell & Terminal

```bash
# Default shell config files
~/.zshrc                # Zsh config
~/.zprofile             # Login shell config
~/.zshenv               # Env vars for all zsh sessions

# Switch to zsh (if on bash)
chsh -s $(which zsh)

# Install Oh My Zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Set hostname
sudo scutil --set HostName <newname>
sudo scutil --set LocalHostName <newname>
sudo scutil --set ComputerName <newname>
```

## macOS Specific Tools

### Disk Utility
```bash
# List disks
diskutil list

# Show disk space
df -h

# Eject a volume
diskutil eject /Volumes/<name>

# Check filesystem
diskutil verifyVolume /

# Repair permissions (old macOS)
# Not needed in APFS — APFS validates automatically
```

### Network
```bash
# Show Wi-Fi info
/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I

# List available Wi-Fi networks
/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -s

# Flush DNS cache
sudo dscacheutil -flushcache
sudo killall -HUP mDNSResponder

# Show network interfaces
ifconfig

# Show open ports
lsof -i -P | grep LISTEN
```

### Process Management
```bash
# Show all processes
ps aux

# Kill a process by PID
kill -9 <PID>

# Kill by name
pkill -f <process-name>

# Resource monitor (like top but better)
htop                    # Requires: brew install htop
```

### System Info
```bash
# Hardware info
system_profiler SPHardwareDataType

# macOS version
sw_vers

# Kernel version
uname -a

# Memory info
vm_stat

# System uptime
uptime

# Serial number
system_profiler SPHardwareDataType | grep "Serial Number"

# Battery health
system_profiler SPPowerDataType | grep -E "Condition|Maximum Capacity"
```

## Common Fixes

### "App is damaged and can't be opened"
```bash
# Remove quarantine attribute
sudo xattr -d com.apple.quarantine /Applications/<AppName>.app
```

### "The application can't be opened" (unsigned)
```bash
# Allow from anywhere (security)
sudo spctl --master-disable
# Or just for one app:
sudo spctl --add /Applications/<AppName>.app
```

### Reset PRAM / NVRAM (Intel only)
```bash
# Restart and hold: Command + Option + P + R for 20s
```

### Reset SMC (Intel only)
```bash
# T2 chip: shut down, hold Control + Option + Shift for 7s, then add power button for 7s
# Apple Silicon: just restart
```

### Flush DNS
```bash
sudo dscacheutil -flushcache
sudo killall -HUP mDNSResponder
```

## Important Folders

| Path | Purpose |
|------|---------|
| `/Applications` | System-wide apps |
| `~/Applications` | User-specific apps |
| `~/Library/Preferences/` | App preferences (plist files) |
| `~/Library/Application Support/` | App data |
| `~/Library/Caches/` | Temporary caches |
| `~/Library/Logs/` | App logs |
| `/usr/local/` | Homebrew installed files |
| `/opt/homebrew/` | Homebrew on Apple Silicon |
| `~/.ssh/` | SSH keys and config |
| `~/.config/` | Modern app configs |
| `/etc/paths` | System PATH |
| `/etc/paths.d/` | Additional PATH entries |

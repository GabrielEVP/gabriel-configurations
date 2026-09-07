---
name: os-windows
description: "Trigger: windows, winget, powershell, cmd, choco, chocolatey, wsl, windows terminal, windows config, dotfiles windows. Windows system administration — PowerShell, Winget, Chocolatey, WSL, environment variables, services, networking, troubleshooting."
license: Apache-2.0
metadata:
  author: gabriel
  version: "1.0"
---

# Windows Administration

## Package Managers

### Winget (Built-in)
```powershell
# Search for a package
winget search <keyword>

# Install a package
winget install <package-id>

# Install with exact ID
winget install --id <package-id>

# List installed
winget list

# Upgrade a package
winget upgrade <package-id>

# Upgrade all
winget upgrade --all

# Uninstall
winget uninstall <package-id>

# Show package info
winget show <package-id>

# Export installed list
winget export -o packages.json

# Import from file
winget import -i packages.json
```

### Chocolatey
```powershell
# Install Chocolatey (as Admin)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Search
choco search <keyword>

# Install
choco install <package> -y

# List installed
choco list --local-only

# Upgrade all
choco upgrade all -y

# Uninstall
choco uninstall <package> -y
```

### Scoop
```powershell
# Install Scoop
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# Add buckets
scoop bucket add extras
scoop bucket add main

# Install
scoop install <package>

# List installed
scoop list

# Update all
scoop update *
scoop update --all

# Remove
scoop uninstall <package>
```

## PowerShell

### Profile & Config
```powershell
# Profile paths
$PROFILE                    # Current user, current host
$PROFILE.AllUsersAllHosts   # All users, all hosts

# Edit profile
notepad $PROFILE

# Reload profile
. $PROFILE

# Common profile additions
# Add to $PROFILE:
# Set-PSReadLineOption -EditMode Emacs
# Set-Alias ll ls
# Set-Alias g git
# function grep { param($pattern, $path); Select-String -Pattern $pattern -Path $path }
```

### Essential Cmdlets
```powershell
# Get help
Get-Help <command>

# Find commands
Get-Command *<keyword>*

# List files (like ls)
Get-ChildItem
Get-ChildItem -Recurse
Get-ChildItem -Filter "*.txt"

# Read file
Get-Content <file>

# Pipe to file
Get-Process | Out-File processes.txt

# Filter (like grep)
Get-ChildItem | Where-Object { $_.Name -match "pattern" }

# Map (like sed)
Get-ChildItem | ForEach-Object { $_.Name.ToLower() }

# Sort
Get-ChildItem | Sort-Object Length -Descending

# Select first N
Get-ChildItem | Select-Object -First 10
```

### Common Tasks
```powershell
# Find a file by name
Get-ChildItem -Recurse -Filter "*keyword*" -ErrorAction SilentlyContinue

# Find a string in files (like grep -r)
Get-ChildItem -Recurse -Filter "*.cs" | Select-String "TODO"

# Kill a process by name
Stop-Process -Name "notepad" -Force

# Check port usage
Get-NetTCPConnection -LocalPort <port>
netstat -ano | findstr :<port>

# Environment variables
$env:PATH
[Environment]::GetEnvironmentVariable("PATH", "User")
[Environment]::SetEnvironmentVariable("PATH", "$env:PATH;C:\tools", "User")
```

## Windows Terminal & Console

```powershell
# Install Windows Terminal
winget install Microsoft.WindowsTerminal

# Settings file location (JSON)
# %LOCALAPPDATA%\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json

# Set default terminal to Windows Terminal (Windows 11)
# Settings → Privacy & Security → For Developers → Terminal → "Windows Terminal"
```

## WSL (Windows Subsystem for Linux)

```powershell
# Install WSL
wsl --install

# Install specific distro
wsl --install -d Ubuntu

# List distros
wsl --list --verbose

# Set default distro
wsl --set-default Ubuntu

# Run a command in WSL
wsl ls -la

# Enter WSL shell
wsl

# Shutdown WSL
wsl --shutdown

# Export a distro
wsl --export Ubuntu ubuntu-backup.tar

# Import a distro
wsl --import Ubuntu <install-path> ubuntu-backup.tar

# Set WSL version (1 or 2)
wsl --set-version Ubuntu 2

# Access Windows files from WSL
cd /mnt/c/Users/<username>

# Access WSL files from Windows
# \\wsl.localhost\Ubuntu\home\<username>
```

## Services

```powershell
# List services
Get-Service

# List running services
Get-Service | Where-Object { $_.Status -eq "Running" }

# Start a service
Start-Service <service-name>

# Stop a service
Stop-Service <service-name>

# Restart a service
Restart-Service <service-name>

# Set startup type
Set-Service -Name <service-name> -StartupType Automatic
Set-Service -Name <service-name> -StartupType Disabled

# Using sc.exe (legacy)
sc query <service-name>
sc start <service-name>
sc stop <service-name>
sc config <service-name> start=auto
sc config <service-name> start=disabled
```

## Networking

```powershell
# Show IP config
ipconfig /all

# Show routing table
route print

# Test connectivity
Test-Connection <host>
ping <host>
Test-NetConnection <host> -Port <port>

# Path tracing
tracert <host>

# DNS lookup
Resolve-DnsName <domain>
nslookup <domain>

# Show open TCP connections
netstat -an | findstr ESTABLISHED
Get-NetTCPConnection

# Firewall rules
Get-NetFirewallRule | Where-Object { $_.Enabled -eq "True" }
New-NetFirewallRule -DisplayName "Allow Port 8080" -Direction Inbound -Protocol TCP -LocalPort 8080 -Action Allow
```

## Disk & Storage

```powershell
# Disk usage
Get-PSDrive
Get-Volume

# Free space
Get-CimInstance Win32_LogicalDisk | Select-Object DeviceID, @{n="FreeGB";e={$_.FreeSpace/1GB -as [int]}}

# Directory size
Get-ChildItem -Recurse | Measure-Object -Property Length -Sum

# Disk cleanup
cleanmgr

# Check disk
chkdsk C: /f

# Disk partition manager
diskmgmt.msc
```

## Registry

```powershell
# Read a registry value
Get-ItemProperty -Path "HKLM:\Software\Microsoft\Windows\CurrentVersion"

# Set a registry value
Set-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "MenuShowDelay" -Value 0

# Create a registry key
New-Item -Path "HKCU:\Software\MyApp"

# Delete a registry key
Remove-Item -Path "HKCU:\Software\MyApp" -Recurse

# Search registry
Get-ChildItem -Path HKLM:\Software -Recurse -ErrorAction SilentlyContinue | Where-Object { $_.Name -match "keyword" }
```

## Environment Variables

```powershell
# View all
Get-ChildItem Env:

# Get specific
$env:PATH
[Environment]::GetEnvironmentVariable("PATH", "Machine")

# Set permanent (User scope)
[Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Java\jdk-21", "User")

# Set permanent (Machine scope — Admin)
[Environment]::SetEnvironmentVariable("JAVA_HOME", "C:\Program Files\Java\jdk-21", "Machine")

# Path management
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$newPath = "$currentPath;C:\tools"
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
```

## Common Fixes

### Windows Update stuck
```powershell
# Stop update services
Stop-Service wuauserv
Stop-Service cryptSvc
Stop-Service bits
Stop-Service msiserver

# Rename SoftwareDistribution
Rename-Item C:\Windows\SoftwareDistribution SoftwareDistribution.old
Rename-Item C:\Windows\System32\catroot2 catroot2.old

# Restart services
Start-Service wuauserv
Start-Service cryptSvc
Start-Service bits
Start-Service msiserver
```

### SFC / DISM
```powershell
# System file check
sfc /scannow

# DISM repair
DISM /Online /Cleanup-Image /CheckHealth
DISM /Online /Cleanup-Image /ScanHealth
DISM /Online /Cleanup-Image /RestoreHealth
```

### Network reset
```powershell
# Reset Winsock
netsh winsock reset

# Reset IP stack
netsh int ip reset

# Flush DNS
ipconfig /flushdns

# Release / renew IP
ipconfig /release
ipconfig /renew
```

### Clear temporary files
```powershell
# Manual cleanup locations
del /q/f/s %TEMP%\*
del /q/f/s C:\Windows\Temp\*

# Modern way
Get-ChildItem $env:TEMP | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
CleanMgr /sagerun:1
```

## Useful Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Win + X` | Power Menu (admin terminal, etc.) |
| `Win + I` | Settings |
| `Win + R` | Run dialog |
| `Win + E` | File Explorer |
| `Win + V` | Clipboard history |
| `Win + .` | Emoji picker |
| `Win + Shift + S` | Screenshot / Snipping Tool |
| `Ctrl + Shift + Esc` | Task Manager |
| `Win + Pause` | System properties |
| `Win + ~` | Focus terminal in Windows Terminal |

## Important Locations

| Path | Purpose |
|------|---------|
| `%APPDATA%` | AppData\Roaming — app settings |
| `%LOCALAPPDATA%` | AppData\Local — local app data |
| `%TEMP%` | Temporary files |
| `%USERPROFILE%` | C:\Users\<username> |
| `%PROGRAMFILES%` | Program Files (64-bit apps) |
| `%PROGRAMFILES(X86)%` | Program Files (32-bit apps) |
| `%SYSTEMROOT%` | C:\Windows |
| `%USERPROFILE%\.ssh\` | SSH keys |
| `C:\ProgramData\` | System-wide app data |

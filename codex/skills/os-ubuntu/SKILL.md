---
name: os-ubuntu
description: "Trigger: ubuntu, apt, debian, linux terminal, dpkg, systemctl, linux config. Ubuntu/Debian system administration — packages, services, permissions, networking, Docker, dotfiles."
---

# Ubuntu / Debian Administration

## Package Management

### APT (Primary)
```bash
# Update package index
sudo apt update

# Upgrade all packages
sudo apt upgrade -y

# Install a package
sudo apt install -y <package>

# Remove a package (keep config)
sudo apt remove <package>

# Remove package + config
sudo apt purge <package>

# Remove unused dependencies
sudo apt autoremove -y

# Search for a package
apt search <keyword>

# Show package info
apt show <package>

# List manually installed packages
apt list --manual-installed

# List installed packages matching pattern
apt list --installed | grep <pattern>
```

### Snap
```bash
# Install a snap
sudo snap install <package>

# List installed snaps
snap list

# Remove a snap
sudo snap remove <package>
```

### Adding Repositories
```bash
# Add PPA
sudo add-apt-repository ppa:<name>/<ppa>
sudo apt update

# Add official Docker repo
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
```

## System Services (systemd)

```bash
# Start a service
sudo systemctl start <service>

# Stop a service
sudo systemctl stop <service>

# Enable service on boot
sudo systemctl enable <service>

# Disable service on boot
sudo systemctl disable <service>

# Check service status
systemctl status <service>

# View service logs
journalctl -u <service> -f

# List all running services
systemctl list-units --type=service --state=running

# Reload systemd after adding a unit file
sudo systemctl daemon-reload
```

## User & Permission Management

```bash
# Add a user
sudo adduser <username>

# Add user to a group
sudo usermod -aG <group> <username>

# Create a group
sudo groupadd <groupname>

# Change file owner
sudo chown <user>:<group> <file>

# Change permissions (octal)
chmod 755 <file>    # rwxr-xr-x
chmod 644 <file>    # rw-r--r--
chmod 600 <file>    # rw-------

# Change permissions (symbolic)
chmod u+x <file>    # add execute for user
chmod go-w <file>   # remove write for group and others

# Give sudo access to a user
sudo usermod -aG sudo <username>
```

## Networking

```bash
# Show IP addresses
ip addr show

# Show routing table
ip route show

# Check open ports
ss -tulpn

# Test connectivity
ping -c 4 <host>

# DNS lookup (deprecated — prefer `dig` or `resolvectl`)
dig <domain>
nslookup <domain>

# Show listening services
sudo lsof -i -P -n | grep LISTEN

# Firewall (UFW)
sudo ufw enable
sudo ufw allow <port>/tcp
sudo ufw allow <port>/udp
sudo ufw status verbose
```

## Disk & Storage

```bash
# Disk usage by partition
df -h

# Disk usage by directory
du -sh <directory>

# Show mounted filesystems
mount | column -t

# Check disk health (SMART)
sudo smartctl -a /dev/sda

# Filesystem usage by file type
ncdu <directory>  # interactive, install with apt
```

## System Info

```bash
# OS version
cat /etc/os-release
lsb_release -a

# Kernel version
uname -a

# CPU info
lscpu
nproc                   # core count

# Memory
free -h
cat /proc/meminfo

# Hardware
lshw                    # full hardware tree
lspci                   # PCI devices
lsusb                   # USB devices

# System uptime
uptime

# Boot log
journalctl -b
```

## Docker (on Ubuntu)

```bash
# Install Docker Engine
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# Add user to docker group (avoids sudo)
sudo usermod -aG docker $USER
# Log out and back in for this to take effect

# Basic Docker commands
docker ps               # running containers
docker ps -a            # all containers
docker images           # list images
docker system df        # disk usage
docker system prune     # clean up unused

# Docker Compose
docker compose up -d
docker compose down
docker compose logs -f
docker compose exec <service> <command>
```

## Shell & Config

```bash
# Default shell files
~/.bashrc               # Bash config (non-login)
~/.bash_profile         # Bash config (login)
~/.profile              # Universal login config
~/.bash_logout          # On logout
~/.bash_aliases         # Common pattern for aliases

# Reload config after editing
source ~/.bashrc

# Set default shell
chsh -s $(which zsh)
```

## Common Fixes

### dpkg lock issue
```bash
sudo rm /var/lib/dpkg/lock-frontend /var/lib/dpkg/lock
sudo rm /var/cache/apt/archives/lock
sudo dpkg --configure -a
```

### apt lock issue
```bash
sudo killall apt apt-get dpkg
sudo rm /var/lib/apt/lists/lock
sudo rm /var/cache/apt/archives/lock
```

### Missing key / GPG error
```bash
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys <KEY_ID>
# Modern way:
sudo gpg --dearmor -o /etc/apt/keyrings/<name>.gpg <downloaded-key-file>
```

### Broken packages
```bash
sudo apt --fix-broken install
sudo dpkg --configure -a
```

## Important Files & Directories

| Path | Purpose |
|------|---------|
| `/etc/apt/sources.list` | APT repositories |
| `/etc/apt/sources.list.d/` | Additional repo configs |
| `/etc/systemd/system/` | Systemd unit files |
| `/etc/environment` | System-wide environment vars |
| `/etc/hostname` | Hostname |
| `/etc/hosts` | Static hostname resolution |
| `/etc/fstab` | Filesystem mount table |
| `/etc/sudoers` | Sudo permissions |
| `/etc/ssh/sshd_config` | SSH server config |
| `~/.ssh/authorized_keys` | Public keys for SSH access |
| `/var/log/syslog` | System log |
| `/var/log/auth.log` | Authentication log |

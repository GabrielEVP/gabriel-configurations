---
name: linux-advanced
description: "Trigger: linux advanced, systemd unit, strace, perf, namespace, cgroup, kernel, sysctl, hardening, linux performance. Advanced Linux — systemd custom units, strace, perf, namespaces, cgroups, kernel tuning, security hardening."
---

# Advanced Linux

## Systemd Custom Units

```ini
# /etc/systemd/system/myapp.service
[Unit]
Description=My Application
After=network.target mysql.service
Requires=mysql.service

[Service]
Type=simple
User=app
Group=app
WorkingDirectory=/opt/myapp
EnvironmentFile=/etc/myapp/env.conf
ExecStart=/usr/bin/java -jar /opt/myapp/app.jar
ExecStop=/bin/kill -SIGTERM $MAINPID
ExecReload=/bin/kill -SIGHUP $MAINPID
Restart=on-failure
RestartSec=10
TimeoutStopSec=30

# Security hardening
ProtectSystem=full
ProtectHome=true
PrivateTmp=true
NoNewPrivileges=true
CapabilityBoundingSet=CAP_NET_BIND_SERVICE
ReadWritePaths=/opt/myapp/logs

[Install]
WantedBy=multi-user.target
```

```bash
# Reload systemd after adding/changing a unit
sudo systemctl daemon-reload

# Enable on boot
sudo systemctl enable myapp

# Start
sudo systemctl start myapp

# View logs
journalctl -u myapp -f

# View logs since last boot
journalctl -u myapp -b

# View logs for a time range
journalctl -u myapp --since "1 hour ago"
journalctl -u myapp --since "2024-01-01" --until "2024-01-02"

# Timer unit (like cron)
# /etc/systemd/system/backup.timer
[Unit]
Description=Daily backup

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
```

## strace (System Call Tracing)

```bash
# Trace all syscalls of a running process
strace -p <PID>

# Trace only specific syscalls
strace -e trace=open,read,write -p <PID>

# Trace with timestamps
strace -t -p <PID>

# Trace and write to file
strace -o /tmp/trace.log -p <PID>

# Count syscalls
strace -c -p <PID>

# Trace child processes too
strace -f -p <PID>

# Trace file operations only
strace -e trace=file <command>

# Trace network operations
strace -e trace=network <command>

# Follow file descriptor
strace -e trace=desc <command>

# Common debugging:
# "No such file" → strace -e trace=open <command>
# "Permission denied" → strace -e trace=access,open <command>
# "Connection refused" → strace -e trace=connect,listen <command>
```

## perf (Performance Profiling)

```bash
# Install
sudo apt install linux-tools-common linux-tools-generic

# List events
perf list

# Record CPU cycles for a command
perf record <command>
perf report

# Sampling: profile a running process for 10 seconds
perf record -p <PID> --sleep 10
perf report

# Show CPU cache misses
perf stat -e cache-misses,cache-references <command>

# Show branch mispredictions
perf stat -e branch-misses,branch-instructions <command>

# Show all CPU counters
perf stat <command>

# Flame graph
perf record -F 99 -p <PID> -g --sleep 30
perf script > out.perf
git clone https://github.com/brendangregg/FlameGraph
./FlameGraph/stackcollapse-perf.pl out.perf > out.folded
./FlameGraph/flamegraph.pl out.folded > flame.svg

# Top CPU consumers (live)
perf top
```

## Namespaces

```bash
# List namespaces
lsns
ls -la /proc/<PID>/ns/

# Show which namespace type a process uses
ls -la /proc/$$/ns/

# Common namespace types:
# pid     → Process IDs
# net     → Network stack
# mnt     → Mount points
# uts     → Hostname
# ipc     → IPC
# user    → User IDs
# cgroup  → Cgroups

# Create a process in a new namespace
unshare --fork --pid bash
# or
unshare -n bash          # new network namespace
unshare -m bash          # new mount namespace
unshare -u bash          # new UTS namespace (hostname)

# Join a namespace by PID
nsenter -t <PID> -n bash       # join network namespace
nsenter -t <PID> -p bash       # join pid namespace
nsenter -t <PID> -a bash       # join all namespaces
```

## Cgroups (Control Groups)

```bash
# Show cgroup hierarchy
cat /proc/cgroups

# Show cgroup for a process
cat /proc/<PID>/cgroup

# List cgroups (cgroup v2)
ls /sys/fs/cgroup/

# Set CPU limit (cgroup v2)
echo "50000 100000" > /sys/fs/cgroup/<group>/cpu.max    # 0.5 CPU

# Set memory limit
echo "512M" > /sys/fs/cgroup/<group>/memory.max
echo "1G" > /sys/fs/cgroup/<group>/memory.high

# Add process to cgroup
echo <PID> > /sys/fs/cgroup/<group>/cgroup.procs

# View CPU/memory stats
cat /sys/fs/cgroup/<group>/cpu.stat
cat /sys/fs/cgroup/<group>/memory.current
```

## Kernel Tuning (sysctl)

```bash
# View all parameters
sysctl -a

# View specific parameter
sysctl net.ipv4.tcp_tw_reuse

# Set parameter temporarily
sudo sysctl -w net.core.somaxconn=65535

# Common production tunings
# /etc/sysctl.conf or /etc/sysctl.d/99-tuning.conf
```

```ini
# Network tuning
net.core.somaxconn = 65535
net.core.netdev_max_backlog = 50000
net.ipv4.tcp_tw_reuse = 1
net.ipv4.tcp_fin_timeout = 15
net.ipv4.tcp_keepalive_time = 300
net.ipv4.tcp_max_syn_backlog = 65535
net.ipv4.tcp_slow_start_after_idle = 0

# File limits
fs.file-max = 2097152

# Virtual memory
vm.swappiness = 10
vm.dirty_ratio = 30
vm.dirty_background_ratio = 5
vm.max_map_count = 262144
```

```bash
# Apply changes
sudo sysctl -p /etc/sysctl.d/99-tuning.conf
```

## Security Hardening

```bash
# File permission audit
find / -perm -4000 -type f 2>/dev/null      # suid binaries
find / -nouser -o -nogroup 2>/dev/null       # orphaned files
find / -perm -2 -type f 2>/dev/null | grep -v proc  # world-writable files

# SSH hardening
# /etc/ssh/sshd_config:
# PermitRootLogin no
# PasswordAuthentication no
# PubkeyAuthentication yes
# Port 2222
# MaxAuthTries 3
# ClientAliveInterval 300
# ClientAliveCountMax 0

# Disable root login on console
# /etc/securetty — remove all terminals

# Fail2ban (intrusion prevention)
sudo apt install fail2ban
sudo systemctl enable fail2ban
# /etc/fail2ban/jail.local:
# [sshd]
# enabled = true
# maxretry = 5
# bantime = 3600

# AppArmor
sudo aa-status                                   # status
sudo aa-enforce /path/to/binary                  # enforce mode
sudo aa-complain /path/to/binary                 # log-only mode

# Auditd
sudo apt install auditd
sudo auditctl -w /etc/passwd -p wa -k passwd_changes
sudo ausearch -k passwd_changes                  # view audit logs
```

## Process Management

```bash
# Tree view of processes
pstree -p

# Process with resource usage
htop

# Show I/O stats per process
iotop

# Show network per process
nethogs

# Show open files per process
lsof -p <PID>
lsof -i :8080                          # who's listening on port 8080
lsof +D /path/to/dir                   # who's using files in a dir

# Change process priority (nice)
nice -n -10 <command>                  # higher priority
renice -n -10 -p <PID>                 # change priority of running process

# CPU affinity (pin to specific cores)
taskset -c 0,1 <command>               # run on cores 0 and 1
taskset -cp 0 <PID>                    # pin process to core 0

# Limit process resources (ulimit)
ulimit -n 65535                        # max open files
ulimit -u 1024                         # max user processes
ulimit -a                              # view all limits
```

## Disk & Filesystem

```bash
# I/O stats per device
iostat -x 2                            # extended stats, every 2s

# I/O stats per process
iotop -o                               # only processes doing I/O

# Filesystem benchmarks
sudo hdparm -Tt /dev/sda               # cache + buffered reads
fio --randwrite --size=1G              # random write benchmark

# Filesystem creation
sudo mkfs.ext4 /dev/sdb1
sudo mkfs.xfs /dev/sdc1

# Mount options for performance
# /etc/fstab:
# /dev/sdb1 /data ext4 defaults,noatime,nodiratime,data=ordered 0 2

# Check filesystem
sudo fsck /dev/sdb1
sudo tune2fs -l /dev/sdb1               # filesystem info

# RAID status
cat /proc/mdstat                        # software RAID status
sudo mdadm --detail /dev/md0

# LVM
sudo pvs                                # physical volumes
sudo lvs                                # logical volumes
sudo vgs                                # volume groups
```

## Memory Debugging

```bash
# Memory map of a process
cat /proc/<PID>/maps
cat /proc/<PID>/smaps                   # detailed per-mapping

# Huge pages
cat /proc/meminfo | grep Huge
# Enable transparent huge pages
echo always > /sys/kernel/mm/transparent_hugepage/enabled

# OOM Killer
cat /proc/<PID>/oom_score               # higher = more likely to be killed
echo -1000 > /proc/<PID>/oom_score_adj  # protect from OOM

# Memory pressure
cat /proc/pressure/memory
cat /proc/pressure/cpu
cat /proc/pressure/io
```

#!/usr/bin/env bash
# verify-env.sh — Airlock kernel environment checker

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PASS=0
FAIL=0

pass() {
    echo -e "${GREEN}[PASS]${NC} $1"
    PASS=$((PASS + 1))
}

fail() {
    echo -e "${RED}[FAIL]${NC} $1"
    echo -e "       FIX: $2"
    FAIL=$((FAIL + 1))
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

echo "=============================="
echo " Airlock Environment Checker"
echo "=============================="
echo ""

# --- Kernel version ---
KERNEL=$(uname -r)
KERNEL_MAJOR=$(echo "$KERNEL" | cut -d. -f1)
KERNEL_MINOR=$(echo "$KERNEL" | cut -d. -f2)

echo "[1] Kernel Version"

if [ "$KERNEL_MAJOR" -gt 5 ] || { [ "$KERNEL_MAJOR" -eq 5 ] && [ "$KERNEL_MINOR" -ge 8 ]; }; then
    pass "Kernel $KERNEL (>= 5.8 required)"
else
    fail "Kernel $KERNEL is too old" "Upgrade to >= 5.8"
fi

echo ""

# --- CONFIG_BPF_LSM ---
echo "[2] CONFIG_BPF_LSM"

if grep CONFIG_BPF_LSM /boot/config-$(uname -r) 2>/dev/null | grep -q "=y"; then
    pass "CONFIG_BPF_LSM=y"
else
    fail "CONFIG_BPF_LSM missing" "Kernel must be compiled with CONFIG_BPF_LSM=y"
fi

echo ""

# --- Runtime LSM chain ---
echo "[3] Runtime LSM Chain"

if [ -f /sys/kernel/security/lsm ]; then
    LSM_LIST=$(cat /sys/kernel/security/lsm)

    echo "    Active: $LSM_LIST"

    if echo "$LSM_LIST" | grep -q "bpf"; then
        pass "bpf present in runtime LSM chain"
    else
        fail "bpf missing from runtime LSM chain" \
            "Add lsm=...,bpf to GRUB_CMDLINE_LINUX_DEFAULT"
    fi
else
    fail "/sys/kernel/security/lsm missing" \
        "Kernel securityfs unavailable"
fi

echo ""

# --- Root check ---
echo "[4] Privileges"

if [ "$EUID" -eq 0 ]; then
    pass "Running as root"
else
    fail "Not root" "Run via sudo"
fi

echo ""

# --- bpftool ---
echo "[5] bpftool"

if command -v bpftool &>/dev/null; then
    pass "bpftool found"
else
    fail "bpftool missing" \
        "sudo apt install linux-tools-common linux-tools-$(uname -r)"
fi

echo ""

# --- cargo ---
echo "[6] Rust Toolchain"

if command -v cargo &>/dev/null; then
    pass "cargo found"
else
    fail "cargo missing" \
        "Install Rust via rustup"
fi

echo ""

# --- bpf-linker ---
echo "[7] bpf-linker"

if command -v bpf-linker &>/dev/null; then
    pass "bpf-linker found"
else
    fail "bpf-linker missing" \
        "cargo install bpf-linker"
fi

echo ""

# --- ls resolution ---
echo "[8] /bin/ls Resolution"

LS_REAL=$(readlink -f "$(which ls)")

echo "    which ls → $(which ls)"
echo "    realpath → $LS_REAL"

if [ "$LS_REAL" = "/bin/ls" ]; then
    pass "deny path matches /bin/ls"
else
    warn "/bin/ls resolves to $LS_REAL — add to deny list"
fi

echo ""

# --- debugfs ---
echo "[9] debugfs"

if mountpoint -q /sys/kernel/debug 2>/dev/null; then
    pass "debugfs mounted"
else
    warn "debugfs not mounted — trace_pipe unavailable"
fi

echo ""

echo "=============================="
echo " Result: $PASS passed, $FAIL failed"
echo "=============================="

if [ "$FAIL" -gt 0 ]; then
    exit 1
else
    exit 0
fi

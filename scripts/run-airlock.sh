#!/usr/bin/env bash
# run-airlock.sh — Airlock deterministic build + load + observe
# Produces: attach proof, enforcement proof, allow proof
# Records: all output to tests/run-$(date).log

set -euo pipefail

# Auto-escalate to root while preserving Rust toolchain PATH
if [ "$EUID" -ne 0 ]; then
    exec sudo env "PATH=$PATH" "$0" "$@"
fi

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()   { echo -e "${GREEN}[airlock]${NC} $1"; }
step()  { echo -e "${CYAN}[step]${NC} $1"; }
warn()  { echo -e "${YELLOW}[warn]${NC} $1"; }
die()   { echo -e "${RED}[fatal]${NC} $1"; exit 1; }

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="$REPO_ROOT/tests"
LOG_FILE="$LOG_DIR/run-$(date +%Y%m%d-%H%M%S).log"

LOADER_PID=""

mkdir -p "$LOG_DIR"

cleanup() {
    if [[ -n "${LOADER_PID:-}" ]] && kill -0 "$LOADER_PID" 2>/dev/null; then
        log "Detaching loader (PID $LOADER_PID)..."
        kill "$LOADER_PID" 2>/dev/null || true
        wait "$LOADER_PID" 2>/dev/null || true
        log "Loader detached. LSM hook removed."
    fi
}

trap cleanup EXIT INT TERM

# Tee all output to log file
exec > >(tee -a "$LOG_FILE") 2>&1

echo "=============================="
echo " Airlock Run — $(date)"
echo " Kernel: $(uname -r)"
echo " Log: $LOG_FILE"
echo "=============================="
echo ""

# --- Step 1: Environment gate ---
step "1/6 — Environment verification"
bash "$REPO_ROOT/scripts/verify-env.sh" || die "Environment check failed."
echo ""

# --- Step 2: Build ---
step "2/6 — Build eBPF + userspace"
cd "$REPO_ROOT"

cargo +nightly run -p xtask -- build-ebpf

log "Build complete."
echo ""

# --- Step 3: Load + attach (background) ---
step "3/6 — Load and attach LSM hook"

cargo +nightly run -p xtask -- run &
LOADER_PID=$!

sleep 3

# Liveness gate
if ! kill -0 "$LOADER_PID" 2>/dev/null; then
    die "Loader exited before attach completed. Check loader output above."
fi

# Verify attach via bpftool
log "Checking bpftool for attached LSM program..."

BPFTOOL_OUT=$(bpftool prog list 2>/dev/null | grep bprm || true)

if [ -z "$BPFTOOL_OUT" ]; then
    die "bpftool shows no bprm_check_security program. Attach failed."
fi

echo "    $BPFTOOL_OUT"

log "Attach confirmed via bpftool."
echo ""

# --- Step 4: Deny proof — absolute path ---
step "4/6 — Enforcement test: absolute deny (/bin/ls)"

LS_PATH=$(readlink -f "$(which ls)")

DENY_OUT=$(strace "$LS_PATH" 2>&1 | head -n 5 || true)

echo "$DENY_OUT"

if echo "$DENY_OUT" | grep -q "EPERM\|Operation not permitted"; then
    log "DENY confirmed: $LS_PATH → EPERM ✅"
else
    warn "EPERM not observed on $LS_PATH. Check docs/runtime-proof.md"
fi

echo ""

# --- Step 5: Deny proof — relative path ---
step "5/6 — Enforcement test: relative deny (./ls)"

cd "$(dirname "$LS_PATH")"

REL_OUT=$(strace ./ls 2>&1 | head -n 5 || true)

echo "$REL_OUT"

if echo "$REL_OUT" | grep -q "EPERM\|Operation not permitted"; then
    log "DENY confirmed: ./ls → EPERM ✅"
else
    warn "EPERM not observed on ./ls."
fi

cd "$REPO_ROOT"

echo ""

# --- Step 6: Allow proof ---
step "6/6 — Pass-through test: allow (/bin/cat)"

ALLOW_OUT=$(strace /bin/cat /etc/hostname 2>&1 | head -n 5 || true)

echo "$ALLOW_OUT"

if echo "$ALLOW_OUT" | grep -q "execve.*= 0"; then
    log "ALLOW confirmed: /bin/cat → 0 ✅"
else
    warn "Allow not confirmed. Check if hook is globally denying."
fi

echo ""

echo "=============================="
echo " Run complete."
echo " Full log: $LOG_FILE"
echo "=============================="

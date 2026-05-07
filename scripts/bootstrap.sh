#!/usr/bin/env bash
# bootstrap.sh — Airlock dependency installer

set -euo pipefail

if [ "$EUID" -ne 0 ]; then
    exec sudo env "PATH=$PATH" "$0" "$@"
fi

echo "[bootstrap] Installing system dependencies..."

apt-get update

apt-get install -y \
    build-essential \
    pkg-config \
    clang \
    llvm \
    libelf-dev \
    linux-tools-common \
    linux-tools-$(uname -r) \
    linux-headers-$(uname -r) \
    bpftool \
    bpftrace \
    strace \
    curl

echo "[bootstrap] Installing Rust nightly..."

if ! command -v rustup &>/dev/null; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source "$HOME/.cargo/env"
fi

rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

echo "[bootstrap] Installing bpf-linker..."

cargo install bpf-linker || true

echo ""
echo "[bootstrap] Complete."
echo ""
echo "Next:"
echo "  ./scripts/verify-env.sh"
echo "  ./scripts/run-airlock.sh"

# Airlock

Kernel-level execution control using Rust, eBPF, and Linux Security Modules (LSM).

Intercept and control process execution before userspace execution occurs.

**WARNING:** Airlock requires root privileges, BPF LSM enabled in the kernel, and can deny execution system-wide. Test inside a VM or isolated environment first.

---

# What Airlock Does

Airlock intercepts execution requests at the Linux kernel boundary using BPF LSM hooks.

The current prototype focuses on:

- BPF LSM execution interception
- Verifier-safe eBPF map interaction
- Runtime userspace-controlled blocklist updates
- Kernel-level EPERM execution denial
- Runtime BTF / CO-RE adaptation

This repository explores kernel-level execution control through BPF LSM enforcement.

The current implementation validates that userspace-controlled execution policies can be enforced through a BPF LSM hook before normal userspace execution begins.

---

# Current Status

Verified on:

- Ubuntu ARM64 VM
- Linux kernel `6.7+`
- Apple Silicon virtualization environments (UTM / VMware Fusion)
- Aya eBPF runtime

Verified capabilities:

- BPF LSM hook attachment
- Runtime BTF / CO-RE adaptation
- Verifier-safe eBPF program loading
- Userspace-controlled runtime blocklist updates
- Kernel-level execution denial via `EPERM`
- Stable runtime userspace/kernel interaction

The current prototype uses a runtime-controlled blocklist map for validation and testing purposes.

More advanced inode-backed identity enforcement was explored experimentally but is not the currently committed runtime path.

---

# Verified Runtime Flow

```text
userspace rules
    -> BLOCKLIST map
    -> bprm_check_security hook
    -> kernel enforcement verdict
```

The current prototype validates runtime userspace-controlled execution governance through a BPF LSM hook.

---

# Example Enforcement

Example execution denial:

```bash
$ /usr/lib/cargo/bin/coreutils/ls
bash: /usr/lib/cargo/bin/coreutils/ls: Operation not permitted
```

The denial occurs inside the Linux kernel through a BPF LSM hook.

---

# Why This Exists

Airlock explores enforcing execution policy at the kernel boundary rather than entirely in userspace.

Airlock explores a different direction:

- deterministic execution governance
- explicit execution boundaries
- kernel-level interception
- transparent runtime behavior
- observable enforcement semantics

The project is intentionally small in scope and focused on validating the kernel enforcement substrate first.

---

# Important Scope Note

Airlock is currently an experimental research/runtime prototype.

This repository does NOT yet provide:

- production endpoint security
- enterprise EDR functionality
- inode-backed production enforcement
- namespace/container isolation
- signed policy management
- hardened recovery tooling
- distributed telemetry infrastructure

The current goal is validating correctness and stability of the kernel enforcement path.

---

# Repository Layout

Current active runtime components:

- `ebpf/` — kernel eBPF LSM program
- `userspace/` — runtime loader and control utilities
- `xtask/` — development/build orchestration helpers

Additional directories contain earlier experimental phases and retained research iterations.

---

# Quickstart

## Prerequisites

### Rust Toolchains

```bash
rustup toolchain install stable
rustup toolchain install nightly --component rust-src
```

### Required Tools

```bash
cargo install bpf-linker
```

### Kernel Requirements

Airlock requires:

- Linux kernel with BPF LSM enabled
- BTF support enabled
- `debugfs` mounted
- root privileges

Verify BPF LSM availability:

```bash
grep CONFIG_BPF_LSM /boot/config-$(uname -r)

cat /sys/kernel/security/lsm
```

The active LSM list should contain:

```text
bpf
```

---

# Build

## Userspace

```bash
cargo build -p airlock-user
```

## eBPF Program

```bash
cargo build -Z build-std=core \
    -p airlock-ebpf \
    --target bpfel-unknown-none \
    --release
```

---

# Runtime Verification

Verified runtime attach example:

```bash
sudo env "PATH=$PATH" \
RUSTUP_TOOLCHAIN=nightly \
target/debug/airlock-user
```

Expected output:

```text
LSM attached
```

Runtime verification:

```bash
sudo bpftool prog list | grep -A5 bprm
```

Example verified output:

```text
lsm  name bprm_check_secu
```

---

# CO-RE Binding Generation

Airlock generates runtime kernel bindings directly from the live kernel BTF.

Example:

```bash
bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h
```

Bindings are generated using `bindgen` with scoped allowlists.

This enables runtime adaptation across kernel layouts without relying on hardcoded offsets.

---

# Cross-Compiling on macOS

Cross-compilation works on both Intel and Apple Silicon Macs.

```bash
CC=${ARCH}-linux-musl-gcc cargo build \
  --package airlock-clean \
  --release \
  --target=${ARCH}-unknown-linux-musl \
  --config=target.${ARCH}-unknown-linux-musl.linker="${ARCH}-linux-musl-gcc"
```

The resulting binary can be copied into a Linux VM or server for execution.

---

# Freeze Tags

Important repository milestones:

- `v1-lsm-baseline`
- `phase2-core-path-extraction`
- `phase2.1-enforcement-verified`
- `phase3-canonical-inode-enforcement`

The tag progression reflects the architectural evolution from:

```text
hook attach
    -> pathname extraction
    -> verified enforcement
    -> inode-backed exploration
```

---

# Planned Exploration Areas

Planned evolution areas include:

- dynamic kernel policy maps
- inode-backed enforcement return
- signed policy loading
- namespace-aware enforcement
- structured audit telemetry
- runtime governance tooling

---

# Repository Philosophy

Airlock intentionally prioritizes:

- deterministic behavior
- transparent execution flow
- explicit governance boundaries
- verifier-safe kernel interaction
- observable enforcement semantics

The repository is structured to show the progression from:

```text
initial LSM attach
    -> runtime blocklist enforcement
    -> future identity-backed governance exploration
```

# Airlock

Kernel-level execution enforcement using Rust, eBPF, and Linux Security Modules (LSM).

Intercept and govern process execution before userspace execution occurs.

> **WARNING:** Airlock requires root privileges, BPF LSM enabled in the kernel, and can deny execution system-wide. Test inside a VM or isolated environment first.

# What Airlock Does

Airlock intercepts execution requests at the Linux kernel boundary using BPF LSM hooks.

The current prototype focuses on:

- BPF LSM execution interception
- CO-RE runtime kernel adaptation
- Canonical kernel object identity extraction
- Verifier-safe kernel object traversal
- Kernel-level EPERM execution denial

This repository explores deterministic execution enforcement through kernel-level enforcement rather than userspace trust boundaries.

This prevents compromised or injected userspace components from bypassing execution policies, since enforcement occurs before userspace execution begins.


# Repository Layout

Active runtime components:

- `airlock-clean/` — userspace runtime loader and governance logic
- `airlock-clean-ebpf/` — eBPF LSM enforcement program
- `airlock-clean-common/` — shared ABI-safe policy and telemetry types

Earlier experimental/runtime exploration directories remain preserved in:

- `ebpf/`
- `userspace/`

These older directories are retained for architectural progression and historical reference, but are not the current canonical runtime path.

# Current Status

Verified on:

- Ubuntu ARM64 VM
- Linux kernel `6.7+`
- Apple Silicon virtualization environments (UTM / VMware Fusion)
- Aya eBPF runtime

Verified capabilities:

- BPF LSM hook attachment
- Runtime BTF / CO-RE adaptation
- `linux_binprm -> file -> f_path -> dentry -> d_inode` traversal
- Canonical `(i_ino + s_dev)` extraction
- Kernel-level execution denial via `EPERM`
- Stable verifier-safe kernel object traversal

The current prototype validates userspace-governed execution enforcement
through a verifier-safe `POLICY_MAP` integrated directly into the
BPF LSM execution path.

Verified capabilities include:

- Runtime policy insertion
- Canonical `FileIdentity(dev, ino)` matching
- POLICY_MAP identity lookup
- Observable ALLOW / DENY telemetry transitions
- ACTION_DENY -> EPERM kernel enforcement
- Canonical userspace `st_dev` normalization into kernel-compatible `dev_t`
- Zero bypass paths remaining in the runtime

Verified Governance Flow

userspace policy
    -> canonical FileIdentity(dev, ino)
    -> POLICY_MAP insertion
    -> bprm_check_security hook
    -> kernel identity extraction
    -> POLICY_MAP lookup
    -> ALLOW / DENY verdict
    -> EPERM enforcement

# Verified Execution Path

```text
linux_binprm
    -> file
    -> f_path
    -> dentry
    -> d_inode
    -> (i_ino + s_dev)
    -> enforcement verdict
```

This moved Airlock away from brittle pathname-only matching toward canonical kernel object identity.

Airlock currently enforces against operational kernel object identity
(`i_ino + s_dev`), not cryptographic file identity.

Future phases may augment this with fs-verity or IMA-backed
measurement signals, but the current runtime intentionally focuses
on deterministic governance using canonical kernel identity.


# Example Enforcement

Example execution denial:

```bash
$ /usr/lib/cargo/bin/coreutils/ls
bash: /usr/lib/cargo/bin/coreutils/ls: Operation not permitted
```

The denial occurs inside the Linux kernel through a BPF LSM hook.

# Why This Exists

Most AI and automation runtimes rely entirely on userspace trust boundaries.

Airlock explores a different direction:

- deterministic execution enforcement
- explicit execution boundaries
- kernel-level interception
- transparent runtime behavior
- canonical execution identity

The project is intentionally small in scope and focused on validating the kernel enforcement substrate first.

# Important Scope Note

Airlock is currently an experimental research/runtime prototype.

This repository does NOT yet provide:

- production endpoint security
- enterprise EDR functionality
- namespace/container isolation
- signed policy management
- hardened recovery tooling
- distributed telemetry infrastructure

The current goal is validating correctness and stability of the kernel enforcement path.

# Quickstart

## Prerequisites

### Rust Toolchains

```bash
rustup toolchain install stable
rustup toolchain install nightly --component rust-src
```

### Rust Targets (optional)

```bash
rustup target add ${ARCH}-unknown-linux-musl
```

### Required Tools

```bash
cargo install bpf-linker
```

### macOS Cross-Compile Dependencies

```bash
brew install llvm
brew install filosottile/musl-cross/musl-cross
```

# Kernel Requirements

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

Runtime environment verification checks are included in the repository scripts.

# Build & Run

## Build

```bash
cargo build
```

## Run

```bash
sudo cargo run -p xtask -- run
```

# CO-RE Binding Generation

Airlock generates runtime kernel bindings directly from the live kernel BTF.

Example:

```bash
bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h
```

Bindings are generated using `bindgen` with scoped allowlists.

This enables runtime adaptation across kernel layouts without relying on hardcoded offsets.

# Cross-Compiling on macOS

Cross-compilation works on both Intel and Apple Silicon Macs.

```bash
CC=${ARCH}-linux-musl-gcc cargo build --package airlock-clean --release \
  --target=${ARCH}-unknown-linux-musl \
  --config=target.${ARCH}-unknown-linux-musl.linker="${ARCH}-linux-musl-gcc"
```

The resulting binary can be copied into a Linux VM or server for execution.

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
    -> canonical inode-backed identity
```

# Planned Exploration Areas

Planned evolution areas include:

- dynamic kernel policy maps
- signed policy loading
- namespace-aware enforcement
- structured audit telemetry
- runtime enforcement tooling

# Repository Philosophy

Airlock intentionally prioritizes:

- deterministic behavior
- transparent execution flow
- explicit enforcement boundaries
- verifier-safe kernel interaction
- observable enforcement semantics

The repository is structured to show the progression from:

`initial LSM attach -> runtime extraction -> canonical inode traversal -> kernel enforcement`

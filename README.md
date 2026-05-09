# Airlock

Kernel-level execution interception research using Rust, eBPF, and Linux Security Modules (LSM).

Airlock explores execution control at the Linux kernel boundary using BPF LSM hooks and Aya.

**WARNING:** Airlock requires:
- root privileges
- Linux with BPF LSM enabled
- BTF support
- isolated VM/testing environment

Do not run on production systems.

---

# Current Status

Current repository state:

- Aya-based eBPF LSM loader
- Runtime eBPF ELF loading
- BPF LSM hook attachment
- Verifier-accepted eBPF program loading
- Runtime userspace rule loading
- Experimental execution policy research

Verified during testing:

- successful `bprm_check_security` hook attachment
- successful eBPF verifier acceptance
- successful runtime load visibility through `bpftool`
- Linux ARM64 VM execution
- nightly Rust eBPF build pipeline

Example verification:

```bash
sudo bpftool prog list | grep bprm
```

Example observed output:

```text
156: lsm  name bprm_check_secu
```

---

# What Is Implemented Today

Current implementation validates:

- Linux BPF LSM integration
- Aya runtime loading flow
- eBPF ELF build pipeline
- userspace-to-eBPF loader interaction
- early-stage execution policy plumbing

The repository currently demonstrates:
- real kernel hook attachment
- real eBPF runtime loading
- real verifier interaction
- real Linux runtime execution

---

# What Is NOT Finished Yet

The following are NOT fully implemented yet:

- canonical inode-backed identity enforcement
- production-safe execution policy enforcement
- stable kernel object traversal bindings
- namespace-aware policy isolation
- structured telemetry
- runtime recovery tooling
- signed policy management

Current enforcement logic is still experimental.

---

# Current Technical Limitation

Current Aya bindings do not yet expose all required kernel structure fields needed for stable executable identity traversal.

Specifically:
- `linux_binprm` field access requires deeper CO-RE/BTF binding generation work
- executable identity extraction is still under active investigation

This repository currently focuses on:
- stable hook loading
- verifier-safe runtime execution
- repeatable Linux eBPF development flow

before deeper enforcement semantics.

---

# Build Requirements

Rust nightly is currently required for eBPF target builds.

Install:

```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo install bpf-linker
```

---

# Build eBPF Target

```bash
cargo build -Z build-std=core \
  -p airlock-ebpf \
  --target bpfel-unknown-none \
  --release
```

---

# Build Userspace Loader

```bash
cargo build -p airlock-user
```

---

# Run Loader

```bash
sudo env "PATH=$PATH" \
RUSTUP_TOOLCHAIN=nightly \
target/debug/airlock-user
```

Expected output:

```text
LSM attached
```

---

# Runtime Verification

Verify hook attachment:

```bash
sudo bpftool prog list | grep -A5 bprm
```

Expected result:

```text
lsm  name bprm_check_secu
```

---

# Repository Scope

Airlock is currently an experimental systems research repository.

The project currently prioritizes:
- verifier-safe runtime behavior
- reproducible Linux eBPF workflow
- transparent runtime debugging
- explicit kernel/runtime investigation
- minimal experimental scope

The current goal is validating the kernel/runtime substrate before expanding enforcement complexity.

---

# Tested Environment

Verified during development on:

- Ubuntu ARM64 VM
- Linux kernel 7.x
- Apple Silicon virtualization environments
- Aya eBPF runtime
- Rust nightly toolchain

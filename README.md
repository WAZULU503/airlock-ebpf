# Airlock — Minimal LSM Nucleus

## Kernel Enforcement Root

Airlock is a minimal kernel enforcement nucleus built with Rust and eBPF (CO-RE). It establishes a kernel-governance root utilizing Linux Security Modules (LSM) to provide:

* stable enforcement semantics via `bprm_check_security` hooks
* canonical executable identity resolution using `(dev, ino)` anchoring
* fail-closed governance for unauthorized execution attempts
* audit-ordered telemetry emitted prior to enforcement verdicts

---

# Verified Runtime Substrate

The enforcement substrate has been verified for:

* stable BPF LSM attachment
* verifier-safe kernel object traversal
* canonical executable identity extraction
* deterministic `-EPERM` denial behavior
* userspace ↔ kernel identity parity
* signed policy activation
* structured execution telemetry emission

Verified execution path:

```text id="srhzx9"
linux_binprm
    -> file
    -> f_path
    -> dentry
    -> d_inode
    -> (i_ino + s_dev)
    -> enforcement verdict
```

Verified on:

* Ubuntu ARM64 VM
* Linux kernel 6.7+
* Apple Silicon virtualization environments
* Aya eBPF runtime

---

# Operational Proof

Verified Phase 18 behavior:

```bash id="w8mzcq"
# Authorized Execution
/usr/bin/dash -c 'echo integrity_verified'

# Unauthorized Execution
/usr/bin/ping -c 1 127.0.0.1
```

Expected output:

```text id="nnjlwm"
integrity_verified
bash: /usr/bin/ping: Operation not permitted
```

Enforcement occurs inside the Linux kernel through the `bprm_check_security` LSM hook before userspace execution begins.

---

# Technical Specifications

| Component           | Specification                                    |
| ------------------- | ------------------------------------------------ |
| Kernel Hook         | `bprm_check_security`                            |
| Runtime             | eBPF CO-RE                                       |
| Enforcement         | Deterministic policy evaluation semantics        |
| Identity Model      | Canonical `(dev, ino)` resolution                |
| Runtime Transport   | RingBuf telemetry                                |
| Policy Verification | Detached Ed25519 signatures                      |
| Architectures       | `aarch64`, `x86_64`                              |
| Dependency Surface  | Linux Kernel 5.8+, BPF subsystem, Rust toolchain |

---

# Core Invariants

## Reduced Attack Surface

The enforcement nucleus is intentionally constrained to minimize runtime complexity and reduce enforcement surface complexity.

## Verified Identity

Execution decisions are derived from canonical executable identity resolution `(dev, ino)` rather than pathname-only matching.

## Deterministic Enforcement Semantics

Governance decisions produce reproducible allow/deny behavior across verified runtime environments.

## Signed Verification Chain

Policy activation paths are cryptographically verifiable and auditable.

Governance artifacts are:

* canonically encoded
* detached-signature verified
* fail-closed on mismatch
* hash-identifiable

---

# Structured Execution Telemetry

Execution telemetry is emitted from the kernel enforcement path through an eBPF RingBuf transport.

Current execution states:

* `ALLOW`
* `DENY`
* `MISS`

Telemetry schema:

```json id="97mn0f"
{"v":1,"ts":1778485154,"dev":265289730,"ino":2621960,"action":"ALLOW"}
```

Execution flow:

```text id="0yhnx2"
LSM hook
    -> canonical identity extraction
    -> POLICY_MAP lookup
    -> ExecutionEvent emission
    -> RingBuf transport
    -> kernel verdict
```

Telemetry emission occurs before verdict return, preserving telemetry-before-verdict semantics even during execution denial.

---

# Signed Governance

Airlock verifies detached Ed25519-signed policy payloads before kernel governance activation.

Verified activation flow:

```text id="tzk5v9"
policy.bin
    -> Ed25519 verification
    -> fail-closed gate
    -> POLICY_MAP population
    -> LSM enforcement
    -> RingBuf telemetry
```

Verified tamper behavior:

```text id="bpr3n4"
tampered policy
    -> POLICY_REJECTED
    -> immediate exit(1)
    -> no kernel governance activation
```

Runtime privilege alone is insufficient to authorize governance changes without a valid policy signature.

---

# Replay Research Layer (Experimental)

Separate from the verified enforcement substrate, the repository includes an experimental replay interpretability research layer.

Research areas include:

* attribution ambiguity
* replay ordering reconstruction
* concurrent replay interpretability
* delayed observation resilience
* reconstruction confidence thresholds

Current replay progression:

| Phase    | Focus                           |
| -------- | ------------------------------- |
| Phase 10 | semantic parity verification    |
| Phase 11 | controlled nondeterminism       |
| Phase 12 | controlled concurrency          |
| Phase 13 | buffered telemetry observation  |
| Phase 14 | replay window drift             |
| Phase 15 | replay attribution stress       |
| Phase 16 | replay saturation threshold     |
| Phase 17 | partial attribution ambiguity        |
| Phase 18 | partial ordering reconstruction      |
| Phase 19 | replay pressure modeling             |
| Phase 20 | temporal reconstruction degradation  |
| Phase 21 | composite replay pressure            |
| Phase 22 | replay confidence scoring            |
| Phase 23 | replay collapse surface mapping      |

Verified replay properties include:

* replay evidence preservation
* concurrent replay interpretability
* replay attribution stability
* bounded attribution ambiguity reconstruction
* bounded replay ordering reconstruction
* replay degradation pressure modeling
* replay confidence scoring
* replay collapse surface classification
* stable `-EPERM` enforcement semantics

The current repository boundary intentionally excludes:

* distributed replay systems
* asynchronous telemetry queues
* probabilistic replay reconstruction
* replay flooding
* unbounded concurrency

---

# Verification Ledger

Repository tags act as a chronological verification chain.

| Phase Range | Focus                                    |
| ----------- | ---------------------------------------- |
| Phase 1–9   | identity and governance substrate        |
| Phase 10–16 | LSM interception and replay verification |
---

# Build & Run

## Prerequisites

Install Rust toolchains:

```bash id="q3yl0s"
rustup toolchain install stable
rustup toolchain install nightly --component rust-src
```

Install required tooling:

```bash id="o6nl8x"
cargo install bpf-linker
```

Verify kernel support:

```bash id="j5e1t2"
grep CONFIG_BPF_LSM /boot/config-$(uname -r)
cat /sys/kernel/security/lsm
```

The active LSM list should contain:

```text id="g6k0yl"
bpf
```

## Build

```bash id="nyms2s"
cargo build
```

## Run

```bash id="v10e1v"
sudo cargo run -p xtask -- run
```

---

# Scope Boundary

Airlock is currently an experimental kernel/runtime research prototype.

This repository does NOT currently provide:

* enterprise EDR functionality
* production endpoint protection
* namespace/container isolation
* distributed telemetry infrastructure
* advanced recovery tooling
* multi-node governance orchestration

The present focus is correctness, enforcement stability, replay interpretability, and verifier-safe kernel interaction.

---

# Legal & License

Copyright (c) 2026 James / WAZULU.
All Rights Reserved.

Source available for audit and research review.

* unauthorized commercial reuse prohibited
* access is provided strictly for technical audit and peer verification
* the Airlock enforcement composition, governance workflow, and replay verification architecture are proprietary

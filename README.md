# Airlock

Kernel-level execution governance using Rust, eBPF, and Linux Security Modules (LSM).

Intercept and govern process execution before userspace execution occurs.

> **WARNING:** Airlock requires root privileges, BPF LSM enabled in the kernel, and can deny execution system-wide. Test inside a VM or isolated environment first.

# What Airlock Does

Airlock intercepts execution requests at the Linux kernel boundary using BPF LSM hooks.

The current prototype focuses on:

- BPF LSM execution interception
- CO-RE runtime kernel adaptation
- Canonical inode-backed identity extraction
- Verifier-safe kernel object traversal
- Kernel-level EPERM execution denial

This repository explores deterministic execution governance through kernel-level enforcement rather than userspace trust boundaries.

This prevents compromised or injected userspace components from bypassing execution policies, since enforcement occurs before userspace execution begins.

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
- Live execve interception verified on active kernel runtime
- Structured kernel telemetry emission verified
- Dynamic POLICY_MAP enforcement verified
- Detached Ed25519 policy signature verification verified
- Deterministic SHA256-derived policy identity verified
- Stable verifier-safe kernel object traversal

The current prototype uses dynamic POLICY_MAP-based enforcement with structured telemetry and detached Ed25519-signed policy verification with deterministic SHA256-derived governance identity (Phase 9.1).

Runtime policy loading, signed governance verification, and POLICY_MAP population have been verified against a live kernel runtime.

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

- deterministic execution governance
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
- advanced multi-key policy lifecycle management
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
- `v0.6-live-kernel-substrate`
- `v0.9.1`

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
- multi-key trust rotation and policy revocation
- namespace-aware enforcement
- structured audit telemetry
- runtime governance tooling
- semantic parity verification between Rust model and BPF runtime
- adversarial stress harness validation

# Semantic Parity Verification

Phase 10 established minimal live semantic parity verification between:

- signed governance policy
- canonical executable identity
- POLICY_MAP runtime state
- BPF LSM enforcement
- observable userspace behavior
- structured telemetry emission

Verified live parity targets:

- ALLOW: `/usr/bin/dash`
- DENY: `/usr/bin/ping`

Parity verification confirmed that governance intent and live kernel enforcement behavior remained semantically equivalent under runtime execution.


# Replay Verification Progression

The repository now includes progressively layered replay verification phases built on top of the governed execution runtime.

## Phase 10 — Semantic Parity

Verified semantic equivalence between:

- signed governance policy
- canonical executable identity
- POLICY_MAP runtime state
- BPF LSM enforcement
- observable userspace behavior
- runtime telemetry emission

Verified targets:

- ALLOW: `/usr/bin/dash`
- DENY: `/usr/bin/ping`

Tag:

- `phase10-live-semantic-parity`

---

## Phase 11 — Controlled Nondeterminism

Validated replay integrity under bounded repeated execution pressure.

Verified properties:

- repeated replay stability
- deterministic ALLOW/DENY behavior
- preserved replay ordering
- preserved EPERM enforcement semantics
- no observed replay drift

Tag:

- `phase11-controlled-nondeterminism`

---

## Phase 12 — Controlled Concurrency

Validated concurrent replay execution against a shared runtime enforcement surface.

Verified properties:

- concurrent replay lane execution
- interleaved global scheduling
- preserved lane-local replay integrity
- preserved concurrent enforcement semantics
- interpretable concurrent replay artifacts

Tag:

- `phase12-controlled-concurrency`

---

These phases establish replay-oriented runtime verification layers before introducing asynchronous telemetry ingestion, replay burst pressure, or higher-order concurrency stress.



# Replay Verification Research

The repository now includes progressively layered replay verification and interpretability experiments built on top of the governed execution runtime.

These phases explore how replay evidence, telemetry interpretation, and causal reconstruction behave under increasing replay pressure while preserving deterministic kernel enforcement semantics.

## Replay Verification Progression

| Phase | Focus |
|---|---|
| Phase 10 | semantic parity verification |
| Phase 11 | controlled nondeterminism |
| Phase 12 | controlled concurrency |
| Phase 13 | buffered telemetry observation |
| Phase 14 | replay window drift |
| Phase 15 | replay attribution stress |
| Phase 16 | replay saturation threshold |
| Phase 17 | partial attribution ambiguity |
| Phase 18 | partial ordering reconstruction |

Verified replay properties include:

- replay evidence preservation
- EPERM enforcement stability
- concurrent replay interpretability
- delayed observation resilience
- replay attribution stability
- bounded replay saturation resilience
- bounded attribution ambiguity reconstruction
- bounded replay ordering reconstruction

Replay artifacts and verification boundaries are documented in:

- `VERIFICATION_INDEX.md`
- `REPLAY_ARTIFACTS.md`



Phase 17 introduced the first observable replay cognition friction boundary.

Replay interpretation remained viable and reconstructable, but replay ownership and causal grouping became less visually immediate under bounded semantic adjacency pressure.



Phase 18 introduced the first observable replay ordering recovery boundary.

Replay interpretation remained viable and reconstructable, but causal replay sequencing became less visually immediate under bounded replay ordering incompleteness pressure.

The current repository boundary intentionally stops before:

- distributed replay systems
- asynchronous telemetry queues
- probabilistic replay reconstruction
- replay flooding
- unbounded concurrency



# Verified vs Planned

Verified:

- live BPF LSM interception
- canonical executable identity extraction
- kernel execution denial (`-EPERM`)
- POLICY_MAP runtime enforcement
- detached signed governance verification
- structured runtime telemetry
- live kernel execution traversal

In Progress / Exploratory:

- semantic parity verification harness
- adversarial stress testing
- attach lifecycle validation
- telemetry loss tolerance analysis
- verifier complexity boundary analysis

# Repository Philosophy

Airlock intentionally prioritizes:

- deterministic behavior
- transparent execution flow
- explicit governance boundaries
- verifier-safe kernel interaction
- observable enforcement semantics

The repository is structured to show the progression from:

`initial LSM attach -> runtime extraction -> canonical inode traversal -> kernel enforcement`

## Governed Execution Flow

```text
userspace policy loader
    ↓
canonical FileIdentity extraction
    ↓
POLICY_MAP insertion
    ↓
BPF LSM lookup
    ↓
ACTION_ALLOW / ACTION_DENY
    ↓
kernel enforcement (-EPERM)
```

Execution decisions are enforced inside the Linux kernel through the `bprm_check_security` LSM hook before userspace execution begins.

## Verified Enforcement Example

```bash
/usr/bin/ping -c 1 127.0.0.1
bash: /usr/bin/ping: Operation not permitted
```

This denial was verified through live `POLICY_MAP` governance and kernel-level enforcement.


## Structured Execution Telemetry

Airlock now emits structured execution telemetry from the kernel enforcement path through an eBPF RingBuf transport.

Execution events are emitted before verdict return, preserving telemetry-before-verdict semantics even during execution denial.

Current execution states:

- ALLOW
- DENY
- MISS

Telemetry schema (`v=1`):

```json
{"v":1,"ts":1778485154,"dev":265289730,"ino":2621960,"action":"ALLOW"}
```

Execution flow:

```text
LSM hook
    -> canonical identity extraction
    -> POLICY_MAP lookup
    -> ExecutionEvent emission
    -> RingBuf transport
    -> kernel verdict
    -> userspace JSONL telemetry
```

## Signed Governance (Phase 9.1)

Airlock now verifies detached Ed25519-signed policy payloads before kernel governance activation.

Policies are:

- canonically encoded using Postcard
- signed using detached Ed25519 signatures
- verified before `POLICY_MAP` population
- rejected fail-closed on signature mismatch

Verified governance activation sequence:

```text
policy.bin
    -> Ed25519 verification
    -> fail-closed gate
    -> POLICY_MAP population
    -> LSM enforcement
    -> RingBuf telemetry
```

Verified tamper behavior:

```text
tampered policy
    -> POLICY_REJECTED
    -> immediate exit(1)
    -> no kernel governance activation
```

The private signing authority (`policy/master.key`) is excluded from version control.

Runtime privilege alone is insufficient to authorize governance changes without a valid policy signature.

Verified runtime activation event:

```json
{"event":"POLICY_VERIFIED","version":1,"policy_id":"46d6bd0c73f51274"}
```


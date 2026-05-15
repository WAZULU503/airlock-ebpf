# Airlock

Kernel-level execution enforcement and replay reconstruction research
using Rust, eBPF, Aya, and Linux Security Modules (LSM).

---

# WARNING

Airlock requires:

- root privileges
- BPF LSM enabled in the kernel
- modern Linux kernel with BTF support

The project can deny execution system-wide through kernel-level
enforcement paths.

Test inside isolated environments, VMs, or disposable systems only.

---

# What Airlock Does

Airlock intercepts execution requests at the Linux kernel boundary
using the `bprm_check_security` BPF LSM hook.

The enforcement layer is complete and frozen at v0.9.1.

---

# Enforcement Layer — COMPLETE / FROZEN (v0.9.1)

## Verified Capabilities

- `bprm_check_security` LSM hook attachment
- CO-RE / BTF runtime kernel adaptation
- `linux_binprm -> file -> f_path -> dentry -> d_inode` traversal
- canonical `(i_ino + s_dev)` identity extraction
- verifier-safe kernel object traversal
- kernel-level execution denial via EPERM
- POLICY_MAP runtime governance integration
- ExecutionEvent RingBuf JSONL telemetry emission (schema v=1, frozen)
- detached Ed25519 signed policy
- `SHA256(policy.bin)[0..8]` = `policy_id`
- postcard serialization
- fail-closed tamper rejection
- `deny_unknown_fields` enforced on all deserialization paths

## Verdict Model

```text
ACTION_ALLOW = 1
ACTION_DENY  = 2
ACTION_MISS  = 3   (identity absent from POLICY_MAP → fail-closed)
```

## Telemetry Invariant

Every enforcement decision must be reconstructable from telemetry alone.
Every signal that influenced the decision must appear in the record.
No hidden state.

## Verified Execution Path

```text
linux_binprm
    -> file
    -> f_path
    -> dentry
    -> d_inode
    -> (i_ino + s_dev)
    -> POLICY_MAP lookup
    -> verdict (ALLOW / DENY / MISS)
    -> ExecutionEvent RingBuf emission
    -> EPERM on DENY or MISS
```

## Verified On

- Ubuntu ARM64 VM
- Linux kernel 6.7+
- Apple Silicon virtualization environments
- Aya eBPF runtime

## Intentionally Not Built

- EDR
- PKI
- TPM
- Sigstore
- container orchestration
- distributed enforcement

---

# Research Layer — ACTIVE (Phase 19)

Phase 19 uses Airlock telemetry as the replay substrate for
bounded causal reconstruction research.

The enforcement layer is not modified.
The research layer operates on replay windows derived from
ExecutionEvent JSONL output.

## Research Question

At what point does a structurally valid, cryptographically intact
telemetry record become causally underdetermined — where multiple
incompatible causal interpretations are equally supported by the
same replay evidence?

## Pressure Model

```text
Axis 1 — Observation density     (positional omission, deterministic)
Axis 2 — Temporal resolution     (jitter ratio against event interval)
Axis 3 — Interleaving depth      (multiplier only, not primary pressure)
```

## Reconstruction Confidence Conditions

```text
C1 — causal predecessor uniquely assignable
C2 — event attribution unambiguous
C3 — cross-chain ordering resolves to single valid sequence
```

Confidence is non-viable when any condition reaches COLLAPSED,
or when two or more reach CONTESTED simultaneously.

## Phase 19 Scope Boundary

```text
EXCLUDED:  class-based omission
EXCLUDED:  semantic event weighting
EXCLUDED:  stochastic omission
EXCLUDED:  random corruption
EXCLUDED:  modifications to the enforcement layer
```

---

# Permanent Principle

Every enforcement decision must be reconstructable from telemetry alone.
Every signal that influenced the decision must appear in the record.
No hidden state.


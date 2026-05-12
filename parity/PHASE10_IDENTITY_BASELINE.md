# Phase 10 — Canonical Identity Baseline

Environment:

- Ubuntu ARM64 VM
- Linux 7.0.0-14-generic
- BPF LSM enabled
- Airlock parity bootstrap phase

---

# Allow Target

Requested Path:

/usr/bin/true

Resolved Executable:

/usr/bin/dash

Canonical Identity:

- inode: 2621960
- device: fd02

Executable Type:

- standalone executable

---

# Deny Target

Requested Path:

/usr/bin/false

Resolved Executable:

/usr/bin/ping

Canonical Identity:

- inode: 2622563
- device: fd02

Executable Type:

- multi-call shared binary
- Rust coreutils implementation

---

# Architectural Observation

Visible pathnames are NOT authoritative.

Canonical executable identity must be resolved from the final executable object after symlink traversal.

This validates the Airlock design decision to enforce against:

    (i_ino + s_dev)

rather than pathname strings.


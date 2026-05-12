# Phase 10 — Minimal Policy Fixture

Purpose:

Establish a deterministic allow/deny policy pair for minimal parity verification.

---

# Allow Policy

Executable:

/usr/bin/gnutrue

Canonical Identity:

- inode: 2622222
- device: fd02

Expected Verdict:

ALLOW

Reason:

Minimal deterministic executable for successful execution parity.

---

# Deny Policy

Executable:

/usr/lib/cargo/bin/coreutils/false

Canonical Identity:

- inode: 2621951
- device: fd02

Expected Verdict:

DENY

Expected Runtime Result:

- kernel returns -EPERM
- userspace execution blocked before process runtime

Reason:

Deterministic denial target using canonical executable identity.

---

# Enforcement Principle

Policies bind ONLY to canonical executable identity:

    (i_ino + s_dev)

The following are NOT authoritative:

- pathname strings
- symlink names
- argv values
- shell aliases

---

# Phase 10 Boundary

This fixture validates only:

- minimal allow parity
- minimal deny parity
- canonical identity enforcement
- telemetry observation

No replay, governance rotation, lifecycle churn, or stress conditions are included in this phase.


# Phase 10 — Live Minimal Parity Result

Status:

SUCCESS

---

# Runtime State

Verified against live Airlock runtime with:

- active BPF LSM attachment
- live POLICY_MAP enforcement
- detached signed governance verification
- canonical executable identity enforcement
- structured telemetry emission

Policy ID:

8c32ec7c3df00b13

---

# Allow Verification

Target:

/usr/bin/dash

Canonical Identity:

- dev: 265289730
- ino: 2621960

Observed Runtime Result:

- execution succeeded
- telemetry emitted ALLOW
- userspace execution completed

Observed Evidence:

ALLOW_PATH_EXECUTED

Telemetry:

{"dev":265289730,"ino":2621960,"action":"ALLOW"}

---

# Deny Verification

Target:

/usr/bin/ping

Canonical Identity:

- dev: 265289730
- ino: 2622563

Observed Runtime Result:

- execution denied before userspace runtime
- kernel returned EPERM
- telemetry emitted DENY

Observed Evidence:

bash: /usr/bin/ping: Operation not permitted

Telemetry:

{"dev":265289730,"ino":2622563,"action":"DENY"}

---

# Semantic Parity Result

Parity verified between:

- signed governance policy
- canonical executable identity
- POLICY_MAP runtime state
- BPF LSM enforcement
- userspace observable behavior
- runtime telemetry emission

---

# Architectural Observation

Kernel canonical device identity differs from shell-visible hexadecimal device representation.

Parity preserved after kernel canonicalization.


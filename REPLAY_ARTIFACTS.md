# Replay Artifact Model

Purpose:

Document replay evidence artifacts, replay interpretation semantics, and artifact boundary rules.

---

# Artifact Directory

Replay artifacts are stored under:

parity/artifacts/

Generated replay artifacts are intentionally excluded from git history.

---

# Artifact Types

## Phase 10 — Parity Capture

Artifact format:

phase10_capture_<unix_timestamp>.log

Purpose:

- capture deterministic ALLOW/DENY replay evidence
- preserve userspace-visible runtime behavior
- preserve observable EPERM denial evidence

---

## Phase 10.2 — Telemetry-Correlated Replay

Artifact format:

phase10_telemetry_<unix_timestamp>.log

Purpose:

- correlate replay execution with runtime telemetry
- preserve replay ↔ telemetry observation windows

---

## Phase 10.3 — Ordered Replay

Artifact format:

phase10_ordered_<unix_timestamp>.log

Purpose:

- preserve deterministic replay sequencing
- preserve replay completion boundaries
- establish ordered replay semantics

---

## Phase 11 — Controlled Nondeterminism

Artifact format:

phase11_loop_<unix_timestamp>.log

Purpose:

- validate replay stability under repeated execution pressure
- preserve repeated ALLOW/DENY replay evidence

---

## Phase 12 — Controlled Concurrency

Artifact format:

phase12_concurrent_<unix_timestamp>.log

Purpose:

- preserve concurrent replay evidence
- preserve replay lane interleaving
- preserve lane-local replay sequencing

---

# Replay Interpretation Rules

## Authoritative Ordering

Replay interpretation is lane-local.

Under concurrent replay conditions:

- lane-local sequence ordering is authoritative
- global replay ordering may interleave naturally

---

## Expected Deny Behavior

The following userspace-visible denial behavior is expected:

Operation not permitted

This indicates successful BPF LSM enforcement through EPERM denial.

---

## Replay Integrity Assumptions

Current replay verification assumes:

- bounded replay execution
- synchronous replay flow
- bounded worker counts
- deterministic replay targets

---

# Current Artifact Boundary

The current artifact model intentionally excludes:

- asynchronous telemetry ingestion
- replay buffering
- burst replay flooding
- partial ordering reconstruction
- distributed replay coordination

These remain future architectural research boundaries.


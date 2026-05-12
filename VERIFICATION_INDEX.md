# Airlock Verification Index

Purpose:

Provide a navigable map of verified runtime enforcement, replay verification, and concurrency validation milestones.

---

# Verification Progression

## Phase 1 — Hardened LSM Baseline

Verified:

- BPF LSM attachment
- baseline kernel enforcement viability

---

## Phase 2 — CO-RE Pathname Extraction

Verified:

- verifier-safe pathname extraction
- runtime kernel adaptation

---

## Phase 3 — Canonical Inode Enforcement

Verified:

- inode-backed executable identity
- canonical execution targeting

---

## Phase 4 — Shared Identity ABI

Verified:

- shared FileIdentity substrate
- userspace ↔ kernel identity consistency

---

## Phase 5 — Governed Execution

Verified:

- POLICY_MAP runtime governance
- userspace-controlled enforcement decisions

Tags:

- `phase5-policy-map-integration`
- `phase5-governed-execution-enforcement`

---

## Phase 6 — Canonical Identity Proven

Verified:

- live canonical identity parity
- kernel ↔ userspace identity synchronization

Tags:

- `phase6-canonical-identity-proven`
- `v0.6-live-kernel-substrate`

---

## Phase 7 — Structured Telemetry

Verified:

- structured runtime execution telemetry
- observable runtime enforcement traces

---

## Phase 8 — Signed Governance

Verified:

- detached Ed25519 policy verification
- fail-closed signature rejection

Tags:

- `v0.8.0`

---

## Phase 9.1 — Governance Identity Freeze

Verified:

- canonical signed governance identity
- policy_id correlation
- governance freeze baseline

Tags:

- `v0.9.1`

---

## Phase 10 — Semantic Parity

Verified:

- governance intent ↔ kernel enforcement parity
- replayable parity evidence
- telemetry-correlated replay
- ordered replay semantics

Tags:

- `phase10-live-semantic-parity`

Artifacts:

- parity replay logs
- telemetry replay logs
- ordered replay logs

---

## Phase 11 — Controlled Nondeterminism

Verified:

- bounded replay pressure stability
- repeated semantic replay integrity
- preserved EPERM enforcement behavior

Tags:

- `phase11-controlled-nondeterminism`

Artifacts:

- replay loop artifacts

---

## Phase 12 — Controlled Concurrency

Verified:

- concurrent replay lane execution
- shared-runtime replay interleaving
- preserved lane-local semantic integrity
- interpretable concurrent replay artifacts

Tags:

- `phase12-controlled-concurrency`

Artifacts:

- concurrent replay artifacts

---

# Current Architectural Boundary

The repository currently stops before:

- asynchronous telemetry ingestion
- replay burst flooding
- scheduler contention stress
- attach lifecycle churn
- high-order concurrency pressure

These remain future research boundaries.


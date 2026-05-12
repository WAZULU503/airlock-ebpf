# Phase 10.2 — Telemetry-Correlated Replay

Purpose:

Correlate replayable userspace execution evidence with live Airlock kernel telemetry.

---

# Correlation Scope

This phase verifies synchronized observation between:

- replay trigger execution
- userspace-visible runtime behavior
- Airlock telemetry emission
- kernel enforcement decisions

---

# Verified Targets

ALLOW:

- /usr/bin/dash

Expected telemetry:

- action = ALLOW

DENY:

- /usr/bin/ping

Expected telemetry:

- action = DENY

Expected userspace result:

- Operation not permitted

---

# Replay Artifact Model

Telemetry replay artifacts stored under:

parity/artifacts/

Artifact format:

phase10_telemetry_<unix_timestamp>.log

---

# Verified Correlation Behavior

Observed userspace behavior:

- successful ALLOW execution
- deterministic DENY enforcement
- EPERM denial preserved in replay artifact

Observed telemetry behavior:

- ALLOW telemetry emitted during replay window
- DENY telemetry emitted during replay window

---

# Architectural Role

This phase establishes deterministic correlation between:

userspace replay evidence
    ↔
kernel telemetry decision traces

before introducing stress, concurrency, or adversarial runtime conditions.


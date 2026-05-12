# Phase 10 — Minimal Parity Bootstrap

## Objective

Prove a minimal semantic equivalence cycle between:

1. Rust reference model
2. Live BPF kernel runtime
3. Shared canonical executable identity

This phase intentionally validates only:

- ONE allow path
- ONE deny path

No stress conditions are included in this phase.

---

# Scope

Included:

- canonical (dev, ino) extraction
- POLICY_MAP lookup
- ACTION_ALLOW verification
- ACTION_DENY verification
- live execve interception
- kernel telemetry observation
- parity reasoning between model and runtime

Excluded:

- stress harness
- replay fuzzing
- lifecycle reloads
- verifier boundary testing
- attach churn
- telemetry loss testing
- nonce replay semantics
- epoch/governance fuzzing

---

# Minimal Success Criteria

## Allow Case

Given:

- executable identity exists in POLICY_MAP
- action = ACTION_ALLOW

Expected:

- Rust model => ALLOW
- BPF runtime => execution succeeds
- telemetry => ALLOW event observed

---

## Deny Case

Given:

- executable identity exists in POLICY_MAP
- action = ACTION_DENY

Expected:

- Rust model => DENY
- BPF runtime => -EPERM
- telemetry => DENY event observed

---

# Freeze Condition

Phase freezes ONLY after:

1. identical executable identity observed
2. allow parity verified
3. deny parity verified
4. telemetry semantics verified
5. repo state committed and tagged


# Phase 20 — Temporal Resolution Pressure

Goal:
Apply deterministic timestamp jitter at replay-read time
without modifying underlying telemetry records.

Pressure Variable:
timestamp_jitter / mean_event_interval

Baseline Event Interval:
100ms

Initial Ratios:
0.1
0.3
0.5
0.8
1.0

Invariant:
Replay records remain structurally valid.
Only replay ordering confidence is pressured.

Read-Time Only:
Baseline JSON artifacts are never modified.
Jitter is applied to replay views only.

# Phase 21 — Composite Replay Pressure

Goal:
Combine omission pressure and temporal jitter
within the same replay reconstruction pass.

Purpose:
Measure reconstruction stability under
simultaneous degradation vectors.

Composite Variables:
- positional omission
- temporal distortion
- ordering ambiguity

Baseline Model:
P1 periodic omission
+
50ms temporal jitter

Invariant:
Underlying baseline telemetry remains immutable.

Read-Time Only:
All pressure transformations operate on replay views only.

Expected Boundary:
C3 degradation accelerates faster than isolated
Phase19 or Phase20 pressure alone.

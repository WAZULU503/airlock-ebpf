cd ~/airlock-ebpf

mkdir -p parity/phase19

cat > parity/phase19/PHASE19_MINIMAL_HARNESS_SPEC.md <<'EOF'
## Phase 19 — Minimal Harness Specification

**Five items. Nothing else.**

---

### 1. Replay Record Shape

One event. Minimum viable fields.

event = {
  event_id:        string   # unique, e.g. "evt_001"
  chain_id:        string   # which causal chain this belongs to
  position:        int      # 1-based index within chain
  timestamp:       int      # monotonic, milliseconds
  action:          string   # what occurred
  predecessor_id:  string   # event_id of direct predecessor, null if origin
}

Chain length: 12 events (locked, R1).
All runs use this shape. No extensions during Phase 19.

---

### 2. Omission Application Rules

Four variants. Applied sequentially. N=3 locked.

P1 — Periodic
     Drop every Nth event where N=3
     Positions dropped: (phase_offset), (phase_offset + 3), (phase_offset + 6)...
     phase_offset determined by seed (see section 3)

P2 — Terminal
     Drop final event of each chain
     Position dropped: position == chain_length (12)

P3 — Origin
     Drop first event of each chain
     Position dropped: position == 1

P4 — Middle-third
     Drop positions 5 through 8 inclusive
     (middle third of a 12-event chain)

Omission is applied to the replay window only.
The underlying record is not modified.

---

### 3. Deterministic Seed Handling

Each P1 run receives one seed value (integer).

seed → phase_offset = seed % N   (where N=3)

phase_offset=0 → drop positions 3, 6, 9, 12
phase_offset=1 → drop positions 1, 4, 7, 10
phase_offset=2 → drop positions 2, 5, 8, 11

Seed recorded in run manifest.
Same seed produces identical omission pattern.
Seed is chosen before the run, not after.

P2, P3, P4 require no seed — positions are fixed.

---

### 4. Scoring Output Schema

One record per run.

run = {
  run_id:    string   # e.g. "p19_p1_d0.67_s1"
  variant:   string   # P1 | P2 | P3 | P4
  density:   float    # events_present / events_total
  seed:      int      # P1 only, null otherwise
  C1:        string   # HOLDS | CONTESTED | COLLAPSED
  C2:        string   # HOLDS | CONTESTED | COLLAPSED
  C3:        string   # HOLDS | CONTESTED | COLLAPSED
  notes:     string   # reviewer observations, free text
}

---

### 5. Reviewer Procedure

Three questions. Answered per run after reviewing the replay window.

C1 — For each present event:
     Can you identify exactly one valid predecessor?

     YES → HOLDS

     Multiple valid predecessors exist for any event
     → CONTESTED

     No unique predecessor assignment remains recoverable
     for one or more events
     → COLLAPSED

C2 — For each present event:
     Can you assign it to exactly one chain without ambiguity?

     YES → HOLDS

     Assignment is uncertain for any event
     → CONTESTED

     Chain assignment becomes non-recoverable
     for one or more events
     → COLLAPSED

C3 — Across all chains:
     Is there exactly one valid ordering of cross-chain events?

     YES → HOLDS

     Multiple valid orderings exist
     → CONTESTED

     Cross-chain ordering becomes non-recoverable
     → COLLAPSED

Reviewer answers before seeing any other run's scores.
One reviewer per run set.
Scores are not revised after submission.

---

### Stop Condition Per Variant

Advance P1 → P2 only when:

all density steps across the ladder have a recorded run entry.

No step skipped.
Collapsed steps are recorded and run continues.

---

That is the complete harness specification.
Ready for implementation.


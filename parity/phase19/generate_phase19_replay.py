from dataclasses import dataclass, asdict
import json
import sys


CHAIN_LENGTH = 12
DROP_N = 3


@dataclass
class Event:
    event_id: str
    chain_id: str
    position: int
    timestamp: int
    action: str
    predecessor_id: str | None


def build_chain(chain_id: str, start_ts: int):
    events = []

    for i in range(1, CHAIN_LENGTH + 1):
        predecessor = None

        if i > 1:
            predecessor = f"{chain_id}_evt_{i-1:03d}"

        event = Event(
            event_id=f"{chain_id}_evt_{i:03d}",
            chain_id=chain_id,
            position=i,
            timestamp=start_ts + (i * 100),
            action="EXEC",
            predecessor_id=predecessor,
        )

        events.append(event)

    return events


def apply_periodic_drop(events, phase_offset):
    filtered = []

    for event in events:
        drop_position = (
            (event.position - phase_offset) % DROP_N == 0
        )

        if not drop_position:
            filtered.append(event)

    return filtered


def main():
    phase_offset = 0

    if len(sys.argv) > 1:
        phase_offset = int(sys.argv[1])

    replay = []

    replay.extend(build_chain("chain_a", 1000))
    replay.extend(build_chain("chain_b", 1050))
    replay.extend(build_chain("chain_c", 1100))

    baseline_output = [asdict(event) for event in replay]

    with open(
        "parity/phase19/results/baseline_replay.json",
        "w",
    ) as file:
        json.dump(baseline_output, file, indent=2)

    periodic = apply_periodic_drop(
        replay,
        phase_offset,
    )

    periodic_output = [
        asdict(event) for event in periodic
    ]

    output_path = (
        f"parity/phase19/results/"
        f"p1_periodic_drop_s{phase_offset}.json"
    )

    with open(output_path, "w") as file:
        json.dump(periodic_output, file, indent=2)

    print(f"Phase offset: {phase_offset}")
    print(f"Baseline events: {len(replay)}")
    print(f"P1 events: {len(periodic)}")


if __name__ == "__main__":
    main()

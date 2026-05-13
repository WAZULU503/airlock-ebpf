import json


DROP_N = 3
JITTER_MS = 50


INPUT_PATH = "parity/phase19/results/baseline_replay.json"


def apply_periodic_drop(events):
    filtered = []

    for event in events:
        if event["position"] % DROP_N != 0:
            filtered.append(event)

    return filtered


def apply_jitter(events):
    output = []

    for index, event in enumerate(events):
        mutated = dict(event)

        direction = 1

        if index % 2 == 0:
            direction = -1

        mutated["timestamp"] = (
            mutated["timestamp"] +
            (direction * JITTER_MS)
        )

        output.append(mutated)

    return output


def main():
    with open(INPUT_PATH) as f:
        replay = json.load(f)

    omitted = apply_periodic_drop(replay)

    composite = apply_jitter(omitted)

    with open(
        "parity/phase21/results/composite_p1_j50.json",
        "w",
    ) as f:
        json.dump(composite, f, indent=2)

    print(f"Baseline events: {len(replay)}")
    print(f"Composite events: {len(composite)}")


if __name__ == "__main__":
    main()

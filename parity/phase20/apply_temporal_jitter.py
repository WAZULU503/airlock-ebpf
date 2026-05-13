import json
import sys


INPUT_PATH = "parity/phase19/results/baseline_replay.json"


def apply_jitter(events, jitter_ms):
    output = []

    for index, event in enumerate(events):
        mutated = dict(event)

        direction = 1

        if index % 2 == 0:
            direction = -1

        mutated["timestamp"] = (
            mutated["timestamp"] +
            (direction * jitter_ms)
        )

        output.append(mutated)

    return output


def main():
    jitter_ms = int(sys.argv[1])

    with open(INPUT_PATH) as f:
        replay = json.load(f)

    jittered = apply_jitter(
        replay,
        jitter_ms,
    )

    output_path = (
        f"parity/phase20/results/"
        f"jitter_{jitter_ms}ms.json"
    )

    with open(output_path, "w") as f:
        json.dump(jittered, f, indent=2)

    print(f"Baseline events: {len(replay)}")
    print(f"Jitter applied: {jitter_ms}ms")


if __name__ == "__main__":
    main()

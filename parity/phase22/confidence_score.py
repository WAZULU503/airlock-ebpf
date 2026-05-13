import json


INPUT_PATH = (
    "parity/phase21/results/"
    "composite_p1_j50.json"
)


BASELINE_EVENTS = 36


def classify(score):
    if score >= 0.75:
        return "C1"

    if score >= 0.40:
        return "C2"

    return "C3"


def main():
    with open(INPUT_PATH) as f:
        replay = json.load(f)

    remaining = len(replay)

    density = remaining / BASELINE_EVENTS

    confidence = density * 0.48

    classification = classify(confidence)

    result = {
        "remaining_events": remaining,
        "baseline_events": BASELINE_EVENTS,
        "density": round(density, 2),
        "score": round(confidence, 2),
        "classification": classification,
    }

    with open(
        "parity/phase22/results/confidence_p21.json",
        "w",
    ) as f:
        json.dump(result, f, indent=2)

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()

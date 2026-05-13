import json


DROP_START = 5
DROP_END = 8


with open(
    "parity/phase19/results/baseline_replay.json"
) as f:
    replay = json.load(f)


filtered = []

for event in replay:
    position = event["position"]

    if not (DROP_START <= position <= DROP_END):
        filtered.append(event)


with open(
    "parity/phase19/results/p4_middle_third_drop.json",
    "w",
) as f:
    json.dump(filtered, f, indent=2)


print(f"Baseline events: {len(replay)}")
print(f"P4 events: {len(filtered)}")

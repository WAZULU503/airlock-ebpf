import json


with open(
    "parity/phase19/results/baseline_replay.json"
) as f:
    replay = json.load(f)


filtered = []

for event in replay:
    if event["position"] != 1:
        filtered.append(event)


with open(
    "parity/phase19/results/p3_origin_drop.json",
    "w",
) as f:
    json.dump(filtered, f, indent=2)


print(f"Baseline events: {len(replay)}")
print(f"P3 events: {len(filtered)}")

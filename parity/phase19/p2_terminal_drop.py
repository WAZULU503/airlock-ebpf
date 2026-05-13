import json


CHAIN_LENGTH = 12


with open(
    "parity/phase19/results/baseline_replay.json"
) as f:
    replay = json.load(f)


filtered = []

for event in replay:
    if event["position"] != CHAIN_LENGTH:
        filtered.append(event)


with open(
    "parity/phase19/results/p2_terminal_drop.json",
    "w",
) as f:
    json.dump(filtered, f, indent=2)


print(f"Baseline events: {len(replay)}")
print(f"P2 events: {len(filtered)}")

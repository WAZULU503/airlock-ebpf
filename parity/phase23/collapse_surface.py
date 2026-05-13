import json


SURFACE = [
    {
        "omission": 0.00,
        "jitter_ms": 0,
        "classification": "C1",
    },
    {
        "omission": 0.33,
        "jitter_ms": 50,
        "classification": "C3",
    },
    {
        "omission": 0.33,
        "jitter_ms": 100,
        "classification": "C3",
    },
]


with open(
    "parity/phase23/results/collapse_surface.json",
    "w",
) as f:
    json.dump(SURFACE, f, indent=2)


print(json.dumps(SURFACE, indent=2))

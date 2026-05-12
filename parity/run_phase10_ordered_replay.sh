#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase10_ordered_${TS}.log"

seq_id=0

record() {
    seq_id=$((seq_id + 1))

    echo "[SEQ=${seq_id}] $1" \
        | tee -a "${OUT}"
}

echo "===== PHASE10 ORDERED REPLAY =====" \
    | tee "${OUT}"

record "timestamp=${TS}"

record "ALLOW trigger begin"

/usr/bin/dash -c 'echo ORDERED_ALLOW_TRIGGER' \
    >> "${OUT}" 2>&1

record "ALLOW trigger complete"

sleep 1

record "DENY trigger begin"

if /usr/bin/ping -c 1 127.0.0.1 \
    >> "${OUT}" 2>&1
then
    record "DENY unexpected success"
else
    record "DENY expected failure"
fi

record "ordered replay complete"

echo "artifact=${OUT}"

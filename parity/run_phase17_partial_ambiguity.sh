#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase17_partial_${TS}.log"

record() {
    echo "$1" | tee -a "${OUT}"
}

echo "===== PHASE17 PARTIAL ATTRIBUTION AMBIGUITY =====" \
    | tee "${OUT}"

record "timestamp=${TS}"

for i in $(seq 1 4)
do
    /usr/bin/dash -c "echo ALLOW_A_${i}" \
        >> "${OUT}" 2>&1 &

    sleep 0.05

    /usr/bin/dash -c "echo ALLOW_B_${i}" \
        >> "${OUT}" 2>&1 &

    sleep 0.05

    record "deny trigger"

    if /usr/bin/ping -c 1 127.0.0.1 \
        >> "${OUT}" 2>&1
    then
        record "unexpected deny success"
    else
        record "expected deny failure"
    fi
done

wait

record "phase17 partial ambiguity complete"

echo "artifact=${OUT}"

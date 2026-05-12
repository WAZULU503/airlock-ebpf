#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase15_attribution_${TS}.log"

record() {
    echo "$1" | tee -a "${OUT}"
}

echo "===== PHASE15 ATTRIBUTION STRESS =====" \
    | tee "${OUT}"

record "timestamp=${TS}"

for i in 1 2 3
do
    record "cycle=${i} allow trigger"

    /usr/bin/dash -c "echo ATTRIBUTION_ALLOW_${i}" \
        >> "${OUT}" 2>&1 &

    sleep 0.2

    record "cycle=${i} deny trigger"

    if /usr/bin/ping -c 1 127.0.0.1 \
        >> "${OUT}" 2>&1
    then
        record "cycle=${i} unexpected deny success"
    else
        record "cycle=${i} expected deny failure"
    fi

    sleep 0.2
done

wait

record "phase15 attribution stress complete"

echo "artifact=${OUT}"

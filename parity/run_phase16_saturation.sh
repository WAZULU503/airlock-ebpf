#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase16_saturation_${TS}.log"

record() {
    echo "$1" | tee -a "${OUT}"
}

echo "===== PHASE16 SATURATION THRESHOLD =====" \
    | tee "${OUT}"

record "timestamp=${TS}"

for i in $(seq 1 6)
do
    record "cycle=${i} allow trigger"

    /usr/bin/dash -c "echo SATURATION_ALLOW_${i}" \
        >> "${OUT}" 2>&1 &

    sleep 0.1

    record "cycle=${i} deny trigger"

    if /usr/bin/ping -c 1 127.0.0.1 \
        >> "${OUT}" 2>&1
    then
        record "cycle=${i} unexpected deny success"
    else
        record "cycle=${i} expected deny failure"
    fi

    sleep 0.1
done

wait

record "phase16 saturation replay complete"

echo "artifact=${OUT}"

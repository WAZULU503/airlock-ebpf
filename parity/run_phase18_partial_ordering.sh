#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase18_ordering_${TS}.log"

record() {
    echo "$1" | tee -a "${OUT}"
}

echo "===== PHASE18 PARTIAL ORDERING =====" \
    | tee "${OUT}"

record "timestamp=${TS}"

for i in $(seq 1 5)
do
    /usr/bin/dash -c "echo FLOW_A_${i}" \
        >> "${OUT}" 2>&1 &

    sleep 0.03

    /usr/bin/dash -c "echo FLOW_B_${i}" \
        >> "${OUT}" 2>&1 &

    sleep 0.03

    if /usr/bin/ping -c 1 127.0.0.1 \
        >> "${OUT}" 2>&1
    then
        record "unexpected deny success"
    else
        record "deny observed"
    fi
done

wait

record "phase18 partial ordering complete"

echo "artifact=${OUT}"

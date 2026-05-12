#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase11_loop_${TS}.log"

ITERATIONS=5

seq_id=0

record() {
    seq_id=$((seq_id + 1))

    echo "[SEQ=${seq_id}] $1" \
        | tee -a "${OUT}"
}

echo "===== PHASE11 CONTROLLED NONDETERMINISM =====" \
    | tee "${OUT}"

record "timestamp=${TS}"

record "iterations=${ITERATIONS}"

for i in $(seq 1 "${ITERATIONS}")
do
    record "cycle=${i} allow begin"

    /usr/bin/dash -c "echo LOOP_ALLOW_${i}" \
        >> "${OUT}" 2>&1

    record "cycle=${i} allow complete"

    record "cycle=${i} deny begin"

    if /usr/bin/ping -c 1 127.0.0.1 \
        >> "${OUT}" 2>&1
    then
        record "cycle=${i} deny unexpected success"
    else
        record "cycle=${i} deny expected failure"
    fi

    sleep 1
done

record "phase11 replay loop complete"

echo "artifact=${OUT}"

#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase12_concurrent_${TS}.log"

ITERATIONS=3

record() {
    local lane="$1"
    local seq="$2"
    local msg="$3"

    echo "[LANE=${lane} SEQ=${seq}] ${msg}" \
        | tee -a "${OUT}"
}

run_lane() {
    local lane="$1"

    local seq=0

    for i in $(seq 1 "${ITERATIONS}")
    do
        seq=$((seq + 1))
        record "${lane}" "${seq}" "cycle=${i} allow begin"

        /usr/bin/dash -c "echo CONCURRENT_ALLOW_${lane}_${i}" \
            >> "${OUT}" 2>&1

        seq=$((seq + 1))
        record "${lane}" "${seq}" "cycle=${i} allow complete"

        seq=$((seq + 1))
        record "${lane}" "${seq}" "cycle=${i} deny begin"

        if /usr/bin/ping -c 1 127.0.0.1 \
            >> "${OUT}" 2>&1
        then
            seq=$((seq + 1))
            record "${lane}" "${seq}" \
                "cycle=${i} deny unexpected success"
        else
            seq=$((seq + 1))
            record "${lane}" "${seq}" \
                "cycle=${i} deny expected failure"
        fi

        sleep 1
    done
}

echo "===== PHASE12 CONTROLLED CONCURRENCY =====" \
    | tee "${OUT}"

run_lane "A" &
PID_A=$!

run_lane "B" &
PID_B=$!

wait "${PID_A}"
wait "${PID_B}"

echo "===== CONCURRENT REPLAY COMPLETE =====" \
    | tee -a "${OUT}"

echo "artifact=${OUT}"

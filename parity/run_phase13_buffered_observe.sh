#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase13_buffered_${TS}.log"

echo "===== PHASE13 BUFFERED TELEMETRY =====" \
    | tee "${OUT}"

echo "timestamp=${TS}" \
    | tee -a "${OUT}"

echo "-----" \
    | tee -a "${OUT}"

echo "ALLOW trigger" \
    | tee -a "${OUT}"

/usr/bin/dash -c 'echo BUFFERED_ALLOW_TRIGGER' \
    >> "${OUT}" 2>&1

sleep 2

echo "-----" \
    | tee -a "${OUT}"

echo "DENY trigger" \
    | tee -a "${OUT}"

if /usr/bin/ping -c 1 127.0.0.1 \
    >> "${OUT}" 2>&1
then
    echo "UNEXPECTED DENY SUCCESS" \
        | tee -a "${OUT}"
else
    echo "EXPECTED DENY FAILURE" \
        | tee -a "${OUT}"
fi

sleep 2

echo "-----" \
    | tee -a "${OUT}"

echo "Observe delayed telemetry correlation window." \
    | tee -a "${OUT}"

echo "artifact=${OUT}"

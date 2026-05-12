#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase10_telemetry_${TS}.log"

echo "===== PHASE10 TELEMETRY CAPTURE =====" \
    | tee "${OUT}"

echo "timestamp=${TS}" \
    | tee -a "${OUT}"

echo "-----" \
    | tee -a "${OUT}"

echo "Triggering ALLOW target" \
    | tee -a "${OUT}"

/usr/bin/dash -c 'echo TELEMETRY_ALLOW_TRIGGER'

sleep 1

echo "-----" \
    | tee -a "${OUT}"

echo "Triggering DENY target" \
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

echo "-----" \
    | tee -a "${OUT}"

echo "Observe correlated telemetry in Airlock runtime terminal." \
    | tee -a "${OUT}"

echo "artifact=${OUT}"

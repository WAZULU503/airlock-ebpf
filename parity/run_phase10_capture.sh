#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase10_capture_${TS}.log"

echo "===== PHASE10 CAPTURE =====" | tee "${OUT}"

echo "timestamp=${TS}" | tee -a "${OUT}"

echo "-----" | tee -a "${OUT}"
echo "ALLOW TEST" | tee -a "${OUT}"

/usr/bin/dash -c 'echo ALLOW_PATH_EXECUTED' \
    >> "${OUT}" 2>&1

echo "-----" | tee -a "${OUT}"
echo "DENY TEST" | tee -a "${OUT}"

if /usr/bin/ping -c 1 127.0.0.1 \
    >> "${OUT}" 2>&1
then
    echo "DENY TEST: UNEXPECTED SUCCESS" \
        | tee -a "${OUT}"
else
    echo "DENY TEST: EXPECTED FAILURE" \
        | tee -a "${OUT}"
fi

echo "-----" | tee -a "${OUT}"
echo "CAPTURE COMPLETE" | tee -a "${OUT}"

echo "artifact=${OUT}"

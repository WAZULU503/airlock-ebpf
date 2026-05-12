#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_DIR="parity/artifacts"

mkdir -p "${ARTIFACT_DIR}"

TS="$(date +%s)"

OUT="${ARTIFACT_DIR}/phase14_window_${TS}.log"

record() {
    echo "$1" | tee -a "${OUT}"
}

echo "===== PHASE14 REPLAY WINDOW DRIFT =====" \
    | tee "${OUT}"

record "timestamp=${TS}"

record "window=A allow trigger"

/usr/bin/dash -c 'echo WINDOW_A_ALLOW' \
    >> "${OUT}" 2>&1 &

sleep 1

record "window=B allow trigger"

/usr/bin/dash -c 'echo WINDOW_B_ALLOW' \
    >> "${OUT}" 2>&1 &

sleep 1

record "window=A deny trigger"

if /usr/bin/ping -c 1 127.0.0.1 \
    >> "${OUT}" 2>&1
then
    record "window=A unexpected deny success"
else
    record "window=A expected deny failure"
fi

sleep 1

record "window=B deny trigger"

if /usr/bin/ping -c 1 127.0.0.1 \
    >> "${OUT}" 2>&1
then
    record "window=B unexpected deny success"
else
    record "window=B expected deny failure"
fi

wait

record "window drift replay complete"

echo "artifact=${OUT}"

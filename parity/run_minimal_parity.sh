#!/usr/bin/env bash
set -euo pipefail

echo "===== PHASE10 MINIMAL PARITY RUN ====="

ALLOW_TARGET="/usr/bin/dash"
DENY_TARGET="/usr/bin/ping"

echo "-----"
echo "ALLOW TARGET"

stat --format="inode=%i device=%D" "${ALLOW_TARGET}"

echo "EXECUTING: ${ALLOW_TARGET}"

if "${ALLOW_TARGET}"; then
    echo "ALLOW RESULT: EXECUTION SUCCEEDED"
else
    rc=$?
    echo "ALLOW RESULT: EXECUTION FAILED rc=${rc}"
fi

echo "-----"
echo "DENY TARGET"

stat --format="inode=%i device=%D" "${DENY_TARGET}"

echo "EXECUTING: ${DENY_TARGET}"

stderr_file="$(mktemp)"

if "${DENY_TARGET}" 2>"${stderr_file}"; then
    echo "DENY RESULT: PROCESS EXECUTED"
else
    rc=$?

    echo "DENY RESULT: FAILURE rc=${rc}"

    echo "STDERR:"
    cat "${stderr_file}"

    if grep -qi "Operation not permitted" "${stderr_file}"; then
        echo "KERNEL VERDICT: EPERM CONFIRMED"
    else
        echo "KERNEL VERDICT: PROCESS EXITED NORMALLY"
    fi
fi

rm -f "${stderr_file}"

echo "-----"
echo "PHASE10 PARITY RUN COMPLETE"

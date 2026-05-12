#!/usr/bin/env bash
set -euo pipefail

echo "===== PARITY TARGET IDENTITIES ====="

for target in /usr/bin/true /usr/bin/false
do
    echo "-----"
    echo "TARGET: ${target}"

    stat --format="inode=%i device=%D" "${target}"

    echo "resolved_path=$(readlink -f "${target}")"

    ls -li "${target}"
done

echo "-----"
echo "IDENTITY PROBE COMPLETE"

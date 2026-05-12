#!/usr/bin/env bash
set -euo pipefail

echo "===== FINAL EXECUTABLE IDENTITIES ====="

for target in /usr/bin/true /usr/bin/false
do
    echo "-----"

    resolved="$(readlink -f "${target}")"

    echo "TARGET=${target}"
    echo "RESOLVED=${resolved}"

    stat --format="FINAL inode=%i device=%D" "${resolved}"

    ls -li "${resolved}"
done

echo "-----"
echo "FINAL IDENTITY PROBE COMPLETE"

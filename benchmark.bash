#!/usr/bin/env bash
set -euxo pipefail

# no spaces please
snapshot=/home/aatif/chainsafe/snapshots/567120_2023_05_17T16_13_00Z.car
snapshot_height=567120
tipsets_to_validate=30
((start_height = snapshot_height - tipsets_to_validate))

hyperfine \
--warmup=1 \
--min-runs=20 \
--export-json=benchmark.json \
--export-markdown=benchmark.md \
--parameter-list variant avx2,no_avx2 \
--prepare="target_{variant}/release/forest-cli db clean --force" \
--command-name="{variant}" \
"\
target_{variant}/release/forest \
--encrypt-keystore=false \
--chain=calibnet \
--import-snapshot=$snapshot \
--halt-after-import \
--no-gc \
"
# "\
# target_{variant}/release/forest \
# --encrypt-keystore=false \
# --chain=calibnet \
# --import-snapshot=$snapshot \
# --skip-load=true \
# --no-gc \
# --height=$start_height \
# "

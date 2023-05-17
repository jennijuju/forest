#!/usr/bin/env bash
set -euxo pipefail

# no spaces please
snapshot=/home/aatif/chainsafe/snapshots/567120_2023_05_17T16_13_00Z.car
snapshot_height=567120
tipsets_to_validate=100
((start_height = snapshot_height - tipsets_to_validate))

function do-benchmark() {
    hyperfine \
        --warmup=1 \
        --min-runs=20 \
        "--export-json=$slug-benchmark.json" \
        "--export-markdown=$slug-benchmark.md" \
        --parameter-list variant avx2,no_avx2 \
        --prepare="target_{variant}/release/forest-cli db clean --force" \
        --command-name="{variant}" \
        "$@"

}

slug=import do-benchmark \
"\
target_{variant}/release/forest \
--encrypt-keystore=false \
--chain=calibnet \
--import-snapshot=$snapshot \
--halt-after-import \
--no-gc \
"

slug=validate do-benchmark \
"\
target_{variant}/release/forest \
--encrypt-keystore=false \
--chain=calibnet \
--import-snapshot=$snapshot \
--halt-after-import \
--skip-load=true \
--no-gc \
--height=$start_height \
"

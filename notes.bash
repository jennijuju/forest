#!/usr/bin/env bash
set -euxo pipefail

avx2=target_avx2
no_avx2=target_no_avx2
snapshot=/home/aatif/chainsafe/snapshots/2867520_2023_05_17T14_00_00Z.car.zst
snapshot_height=2867520
tipsets_to_validate=30
(( start_height = snapshot_height - tipsets_to_validate ))

# Build
for dir in $avx2 $no_avx2; do
    mkdir --parents -- "$dir"
    echo '*' > "$dir/.gitignore"
done

CARGO_TARGET_DIR=$avx2 \
RUSTFLAGS="-C target-feature=+avx2,+fma" \
cargo build --release --bins

CARGO_TARGET_DIR=$no_avx2 \
RUSTFLAGS="" \
cargo build --release --bins

false

# Run
hyperfine --warmup 2

#!/usr/bin/env bash
set -euxo pipefail

# Download a mainnet snapshot
aria2c "https://snapshots.mainnet.filops.net/minimal/latest.zst"
# Decompress the snapshot
zstd ...

# Make sure the paritydb folder is empty
cargo run --bin forest-cli -- db stats # print path of db
cargo run --bin forest-cli -- db clean

# benchmark this bit
cargo run --bin forest -- --import-snapshot "$path" --halt-after-import --no-gc

# import time
./target/release/forest --encrypt-keystore false --import-snapshot ../../Snapshots/2866560_2023_05_17T06_00_00Z.car --halt-after-import --no-gc

# validation time
# validates from height..current_height
./target/release/forest --encrypt-keystore false --import-snapshot ../../Snapshots/2866560_2023_05_17T06_00_00Z.car --skip-load true --no-gc --height 2866530

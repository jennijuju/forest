#!/usr/bin/env bash

hyperfine \
'./do-link.bash' \
'./do-link.bash -C linker=clang' \
'./do-link.bash -C link-arg=-fuse-ld=lld' \
'./do-link.bash -C link-arg=-fuse-ld=mold' \
'./do-link.bash -C linker=clang -C link-arg=-fuse-ld=lld' \
'./do-link.bash -C linker=clang -C link-arg=-fuse-ld=mold' \

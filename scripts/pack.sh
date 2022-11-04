#!/bin/bash

TARGETS=(
    ./target/release/hello
)

for target in "${TARGETS[@]}"; do
    strip -v "${target}"
done

mkdir -pv ./src/workspace/scripts
tar cvz "${TARGETS[@]}" | base64 | cat ./scripts/unpack.sh.in - > ./src/workspace/scripts/unpack.sh
chmod -v +x ./src/workspace/scripts/unpack.sh

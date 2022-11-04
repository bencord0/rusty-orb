#!/bin/bash

TARGETS=(
    ./target/release/hello
)

for target in "${TARGETS[@]}"; do
    strip -v "${target}"
done

tar cvz "${TARGETS[@]}" | base64 | cat ./scripts/unpack.sh.in - > ./src/scripts/unpack.sh
chmod -v +x ./src/scripts/unpack.sh

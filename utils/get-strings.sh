#!/bin/env bash

# CD into the root of the repo
PATH_REPO=$(dirname $(dirname $(realpath $0)))
cd "${PATH_REPO}"

# Check output
if [[ ! -d "output" ]]; then
    echo "Run the extractor first"
    exit 1
fi

cd output
# Get all files
PATHS_ALL_FILES=$(ls -1 | grep "dat" | sort | uniq)
paths=()
for f in $PATHS_ALL_FILES; do
    f=$(basename $f ".dat")
    if [[ -f "$f.zlib" ]]; then
        paths+=("$f.zlib")
    else
        paths+=("$f.dat")
    fi
done
# Get strings
for f in "${paths[@]}"; do
    PATH_STRINGS=$(basename $(basename $f ".zlib") ".dat")
    strings $f > "$PATH_STRINGS.txt"
done


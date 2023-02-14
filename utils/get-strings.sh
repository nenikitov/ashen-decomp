#!/bin/env bash

# CD into the root of the repo
path_repo=$(dirname $(dirname $(realpath $0)))
cd "${path_repo}"

# Check output
if [[ -d "output" ]];
then
    cd output
    # Get all files
    paths_all_files=$(ls -1 | grep "dat" | sort | uniq)
    paths=()
    for f in $paths_all_files; do
        f=$(basename $f ".dat")
        if [[ -f "$f.zlib" ]]; then
            paths+=("$f.zlib")
        else
            paths+=("$f.dat")
        fi
    done
    # Get strings
    for f in "${paths[@]}"; do
        path_strings=$(basename $(basename $f ".zlib") ".dat")
        strings $f > "$path_strings.txt"
    done
else
    echo "Run the extractor first"
    exit 1
fi


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

# Run analysis tools
for f in "${paths[@]}"; do
    NO_EXTENSION=$(basename $(basename $f ".zlib") ".dat")
    # Strings
    strings $f > "${NO_EXTENSION}_strings.txt"
    # Binwalk
    binwalk -B $f > "${NO_EXTENSION}_binwalk.txt"
    sed -i '1,3d' "${NO_EXTENSION}_binwalk.txt"
    sed -i '/^$/d' "${NO_EXTENSION}_binwalk.txt"
done

# Remove empty files
find . -type f -empty -delete


#!/bin/env bash

# CD into the root of the repo
PATH_REPO=$(dirname $(dirname $(realpath $0)))
cd "${PATH_REPO}"

# Check output
PATH_OUTPUT="${PATH_REPO}/output"
PATH_OUTPUT_DEFLATED="${PATH_OUTPUT}/deflated"
PATH_OUTPUT_ANALYZED="${PATH_OUTPUT}/analyzed"

cd "${PATH_OUTPUT}"
if [[ ! -d "${PATH_OUTPUT_DEFLATED}" ]] || [[ -z "$(ls -A ${PATH_OUTPUT_DEFLATED})" ]]; then
    echo "Run the extractor first"
    exit 1
fi

# Run analysis tools
rm -rf "${PATH_OUTPUT_ANALYZED}"
mkdir "${PATH_OUTPUT_ANALYZED}"
for f in ${PATH_OUTPUT_DEFLATED}/*; do
    FILE_NAME=$(basename $f)
    FILE_NAME_NO_EXTENSION=$(basename $FILE_NAME ".dat")

    # Print
    echo "Analyzing ${FILE_NAME}"

    # Strings
    strings "${f}" > "${PATH_OUTPUT_ANALYZED}/${FILE_NAME_NO_EXTENSION}_strings.txt"

    # Bindwalk
    binwalk -B "${f}" > "${PATH_OUTPUT_ANALYZED}/${FILE_NAME_NO_EXTENSION}_binwalk.txt"
    sed -i '1,3d' "${PATH_OUTPUT_ANALYZED}/${FILE_NAME_NO_EXTENSION}_binwalk.txt"
    sed -i '/^$/d' "${PATH_OUTPUT_ANALYZED}/${FILE_NAME_NO_EXTENSION}_binwalk.txt"
done

# Remove empty files
find "${PATH_OUTPUT_ANALYZED}" -type f -empty -delete


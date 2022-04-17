#!/bin/sh

PROJECT_DIR="$(pwd)"

if (cd "$PROJECT_DIR" && ls | grep "Cargo.toml") > /dev/null; then
    PROJECT_DIR="$PROJECT_DIR"
else
    PROJECT_DIR="$(echo "$PROJECT_DIR" | sed "s;Raytracing_in_One_Weekend/[A-Z|a-z|0-9|_]*;Raytracing_in_One_Weekend;")"
fi

echo "$PROJECT_DIR"
EXISTING_IMAGE_FILES="$(find "${PROJECT_DIR}/images" -type f -name "*.ppm")"
EXISTING_IMAGE_FILES_COUNT="$(echo "$EXISTING_IMAGE_FILES" | wc -l)"

echo "======================= Pre-existing ppm files... ======================="
echo "$EXISTING_IMAGE_FILES" | awk -F/ '{print $NF}'
echo "========================================================================="

echo "Running rtweekend..."
echo "=== Enter filename for ppm file (file to save output from program): ==="
read filename

if echo "$EXISTING_IMAGE_FILES" | grep "${filename}\.ppm"; then
    echo "This file seems to already exist."
    echo "Are you sure you want to overwrite existing file? [Y/N]"
    read ans

    if [ "$ans" != "Y" ] && [ "$ans" != "y" ]; then
        return 1
    fi
fi

#: Contain in subshell
(cd "$PROJECT_DIR" \
    && cargo run | tee "${PROJECT_DIR}/images/${filename}.ppm" > /dev/null \
    && mogrify -format jpg ${PROJECT_DIR}/images/*.ppm)

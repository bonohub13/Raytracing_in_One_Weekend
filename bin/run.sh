#!/bin/sh -eu
PROJECT_ROOT="$(pwd | sed "s;/bin;;")"
IMAGE_DIR="${PROJECT_ROOT}/images"

find_existing_images_and_create() {
    file_count=$(find "${IMAGE_DIR}" -type f -name "*.ppm" | wc -l)
    images="$(find "$IMAGE_DIR" -type f | grep "ppm$\|png$")"

    if [ $file_count -gt 0 ]; then
        echo "==================== Existing image files ===================="
        echo "$images" | awk -F/ '{print$NF}'
        echo "=============================================================="
    fi

    echo "Enter filename to create:"
    read filename

    if echo "$images" | grep "${filename}\.ppm$\|${filename}\.png$" > /dev/null; then
        echo "File seems to already exist. Do you want to overwrite? [Y/n]"
        read ans
        if [ "$ans" != "y" ] && [ "$ans" != "Y" ]; then
            return 1
        fi
    fi

    ${PROJECT_ROOT}/bin/rtweekend "${IMAGE_DIR}/${filename}"

    return $?
}

find_existing_images_and_create

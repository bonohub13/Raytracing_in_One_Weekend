#!/bin/sh -eu
PROJECT_ROOT="$(pwd | sed "s;/bin;;")"
IMAGE_DIR="${PROJECT_ROOT}/images"

find_existing_images_and_create() {
    file_count=$(find "${IMAGE_DIR}" -type f -name "*.ppm" | wc -l)
    images="$(find "$IMAGE_DIR" -type f -name "*.ppm")"

    if [ $file_count -gt 0 ]; then
        echo "==================== Existing image files ===================="
        echo "$images" | awk -F/ '{print$NF}'
        echo "=============================================================="
    fi

    echo "Enter filename to create:"
    read filename

    if echo "$images" | grep "$filename\.ppm" > /dev/null; then
        echo "File seems to already exist. Do you want to overwrite? [Y/n]"
        read ans
        if [ "$ans" != "y" ] && [ "$ans" != "Y" ]; then
            return 1
        fi
    fi

    ${PROJECT_ROOT}/bin/rtweekend \
        | tee "${IMAGE_DIR}/${filename}.ppm" > /dev/null
    mogrify -format jpg ${IMAGE_DIR}/*.ppm

    return $?
}

find_existing_images_and_create

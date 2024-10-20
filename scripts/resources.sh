#!/usr/bin/env sh
set -eu

EARTHMAP_URL="https://raytracing.github.io/images/earthmap.jpg"
RESOURCES_DIR="$(pwd | sed "s;/scripts/.*$;;")/resources"

prepare() {
    mkdir -v "${RESOURCES_DIR}"

    return $?
}

download_resources() {
    curl -o "${RESOURCES_DIR}/earthmap.jpg" "${EARTHMAP_URL}"

    return $?
}


prepare
download_resources

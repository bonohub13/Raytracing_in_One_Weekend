#!/bin/sh -eu
SHADER_DIR="$2"
SHADER_FILES=""

if [ -d "$SHADER_DIR/src" ]
then
    SHADER_FILES="$(find "$SHADER_DIR/src" -type f)"
fi

if [ ! -d "$SHADER_DIR/spv" ]
then
    mkdir -p "$SHADER_DIR/spv"
fi

cleanup() {
    existing_spirv_files="$(find "$SHADER_DIR/spv" -type f)"

    if [ $(echo "$existing_spirv_files" | wc -l) -gt 0 ] \
        && [ "$existing_spirv_files" != "" ]
    then
        echo "$existing_spirv_files" | while read file
        do
            echo "Deleting $file"
            rm "$file"
        done
    fi

    return 0
}

compile() {
    if [ "$SHADER_FILES" = "" ]; then
        return 1
    else
        echo "$SHADER_FILES" | while read file
        do
            out_file="$(echo "$file" | awk -F/ '{print$NF}')"

            glslc "$file" -o "$SHADER_DIR/spv/${out_file}.spv"
        done
    fi

    return 0
}

case "$1" in
"clean")
    cleanup
    ;;
"build")
    compile
    ;;
*)
    echo "Invalid mode: \'$1\' does not exist"
    false
    ;;
esac

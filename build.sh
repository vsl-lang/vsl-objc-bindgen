#!/usr/bin/env bash

VERSION=11.2
ARCH="x86_64"
SYS="ios"
PLATFORM=iphonesimulator

echo -e "Building for \033[32;1m${SYS}\033[0m version \033[32;1m${VERSION}\033[0m"
echo -e "Selected platform '\033[32;1m${PLATFORM}\033[0m' with arch '\033[32;1m${ARCH}\033[0m'"

# Artifact folder
ARTIFACTS=artifacts
ARTIFACT_PREFIX="${ARTIFACTS}/vsl-${PLATFORM}-${VERSION}"
mkdir -p "${ARTIFACTS}"

# Variables
TARGET_TRIPLE="${ARCH}-apple-${SYS}${VERSION}"
SDK_NAME="${PLATFORM}${VERSION}"

echo -e "Using target triple '\033[1m${TARGET_TRIPLE}\033[0m'"

# SDK Information
SDK_PATH="$(xcrun --sdk "${SDK_NAME}" --show-sdk-path)"
PLATFORM_PATH="$(xcrun --sdk "${SDK_NAME}" --show-sdk-platform-path)"

# Compilation files
SOURCE_FILES=(vsl/*.m)
OUT_FILES=()

for source_file in "${SOURCE_FILES[@]}"; do
    out_path="${ARTIFACT_PREFIX}-$(basename "$source_file" .m).bc"
    OUT_FILES+=("${out_path}")

    clang "${source_file}" \
        -ObjC++ \
        -emit-llvm -S -o "${out_path}" \
        -isysroot "${SDK_PATH}" \
        -fmodules -fobjc-arc \
        "-F${PLATFORM_PATH}/Developer/SDKs/${PLATFORM}.sdk/System/Library/Frameworks/" \
        --target="${TARGET_TRIPLE}" \
        "-m${SYS}-version-min=${VERSION}"

    echo -e "Compiling class \033[1m$(basename "$source_file" .m)\033[0m"
done

OUTPUT_PATH="${ARTIFACT_PREFIX}.bc"

echo -e "Emitting BC to: \033[1m${OUTPUT_PATH}\033[0m"
llvm-link "${OUT_FILES[@]}" -o="${OUTPUT_PATH}"

echo -e "\033[32;1mDone\033[0m"

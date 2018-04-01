#!/usr/bin/env sh

IOS_VERSION=11.2
PLATFORM=iPhoneOS
ARTIFACTS=artifacts

mkdir -p "${ARTIFACTS}"

OUTPUT="${ARTIFACTS}/vsl-${PLATFORM}-${IOS_VERSION}.bc"


swift -frontend -c vsl/Stub.swift -primary-file vsl/AppDelegate.swift \
    -emit-ir -o "${OUTPUT}" \
    -sdk "/Applications/Xcode.app/Contents/Developer/Platforms/${PLATFORM}.platform/Developer/SDKs/${PLATFORM}.sdk" \
    -Fsystem "/Applications/Xcode.app/Contents/Developer/Platforms/${PLATFORM}.platform/Developer/SDKs/${PLATFORM}.sdk/System/Library/Frameworks/" \
    -import-objc-header vsl/vsl-ios.h \
    -module-name vslios \
    -target "x86_64-apple-ios${IOS_VERSION}" -Xcc -D__arm64__

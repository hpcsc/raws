#!/usr/bin/env bash

BUILD_NO=$1
if [[ "$BUILD_NO" = "" ]]; then
    echo "Build number is required"
    echo "Example usage: $0 123"
    exit 1
fi;

cat Cargo.toml | sed -E 's/^version = "([0-9.-]+)"/version = "\1-'${BUILD_NO}'"/' | tee Cargo.toml

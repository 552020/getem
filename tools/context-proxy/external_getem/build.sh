#!/bin/bash
set -e

cd "$(dirname $0)"

# TARGET="${CARGO_TARGET_DIR:-../../target}"
TARGET="${CARGO_TARGET_DIR:-target}"  # Changed this line to look in local target


rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown

mkdir -p res

cp $TARGET/wasm32-unknown-unknown/debug/external_getem.wasm ./res/

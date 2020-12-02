#!/bin/bash

CUR_DIR=$(pwd)
WORKSPACE_DIR=$(dirname $CUR_DIR)
echo $WORKSPACE_DIR
builder="docker run --rm -it -v $WORKSPACE_DIR:/home/rust/src ekidd/rust-musl-builder"
$builder cargo build --release
APP_NAME=$(grep "name\s*=" ../Cargo.toml | sed -e 's/"//g' | awk  '{print $3}')
ls -lh $WORKSPACE_DIR/target/x86_64-unknown-linux-musl/release/$APP_NAME

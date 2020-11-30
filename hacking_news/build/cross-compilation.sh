#!/bin/bash

CUR_DIR=$(pwd)
WORKSPACE_DIR=$(dirname $CUR_DIR)
echo $WORKSPACE_DIR
builder="docker run --rm -it -v $WORKSPACE_DIR:/home/rust/src ekidd/rust-musl-builder"
$builder cargo build --release
ls -lh target/x86_64-unknown-linux-musl/release/hacking_news_app

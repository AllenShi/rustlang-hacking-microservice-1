#!/bin/bash

CWD=$(pwd)
PARENT_DIR=$(dirname $CWD)
docker build --no-cache -t allenshi/hacking_news_app $PARENT_DIR

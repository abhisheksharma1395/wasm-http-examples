#!/bin/bash

docker buildx build --platform wasi/wasm32 -t adroitx/wasm-http-examples .
docker compose up
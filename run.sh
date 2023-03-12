#!/bin/bash

docker buildx build --platform wasi/wasm32 -t adroit/wasm-http-examples .
docker compose up
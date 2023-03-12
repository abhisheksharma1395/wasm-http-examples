FROM scratch
COPY wasm-http-examples.wasm .
ENTRYPOINT [ "wasm-http-examples.wasm" ]



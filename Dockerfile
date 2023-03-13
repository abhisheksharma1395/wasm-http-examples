FROM scratch
COPY wasm-http-examples.wasm .
COPY index.html .
ENTRYPOINT [ "wasm-http-examples.wasm" ]
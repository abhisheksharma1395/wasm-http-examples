# Simple HTTP web server built in Rust compiled to WebAssembly and hosted using Docker

## Prerequisites
- Docker desktop is installed
- ContainerD images for Docker desktop is enabled

### To enable ContainerD images for Docker desktop:
1. Open Docker desktop
2. Navigate to docker settings
3. Navigate to "Features in development"
4. Select "Use containerd for pulling and storing images"
5. Click apply and restart

## To run:
Make sure that if you open the dockerfile, you check that it has been saved using LF (Line Feed) and not CRLF (Carriage Return Line Feed). For VSCode, this can be seen and changed through the status bar

```bash
git clone https://github.com/abhisheksharma1395/wasm-http-examples.git

cd ./wasm-http-examples
```


If using bash: 
```bash
bash run.sh
```

If using other:
```bash
docker buildx build --platform wasi/wasm32 -t adroitx/wasm-http-examples.
docker compose up
```




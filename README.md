# Wasm projects

# First Cli

## references
[Blog link](https://blog.ediri.io/lets-build-a-cli-in-rust)

## wasi-bot (HTTPS)
> Using wasmedge

```bash
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -p /usr/local

wget https://github.com/WasmEdge/WasmEdge/releases/download/0.11.1/WasmEdge-plugin-wasmedge_httpsreq-0.11.1-manylinux2014_x86_64.tar.gz\n

tar -xzf WasmEdge-plugin-wasmedge_httpsreq-0.11.1-manylinux2014_x86_64.tar.gz

sudo cp libwasmedgePluginHttpsReq.so /usr/local/lib/wasmedge/

cargo build --target wasm32-wasi --release
```

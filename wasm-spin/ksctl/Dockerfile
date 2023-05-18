FROM scratch

COPY ./spin.toml /
COPY ./target/wasm32-wasi/release/ksctl.wasm /target/wasm32-wasi/release/ksctl.wasm
COPY ./create-cluster/target/wasm32-wasi/release/create_cluster.wasm /create-cluster/target/wasm32-wasi/release/create_cluster.wasm
COPY ./delete-cluster/target/wasm32-wasi/release/delete_cluster.wasm /delete-cluster/target/wasm32-wasi/release/delete_cluster.wasm

CMD ["/"]

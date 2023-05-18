# First WASM project using Spin by fermyontech

their are 3 routes available
- /
- /create
- /delete


> **Note**
> for the create cluster where for the create you need to provide the <clusterName,nodes> as data 
```sh
curl -i -d 'dipankar-cluster,34' http://127.0.0.1:3000/create
```

## For the docker

```bash
docker buildx build --platform wasi/wasm32 -t try-local:latest .
docker container run --rm -p 3000:80 --runtime io.containerd.spin.v1 --platform wasi/wasm32 try-latest:latest
```

%[https://youtu.be/sJPILenoa6E]

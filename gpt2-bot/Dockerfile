FROM rust:latest

WORKDIR /app

COPY . .

RUN rustup target add wasm32-wasi

RUN curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

RUN mv spin /usr/local/bin/

RUN spin build

ENTRYPOINT [ "spin", "up", "--listen", "0.0.0.0:80" ]

EXPOSE 80

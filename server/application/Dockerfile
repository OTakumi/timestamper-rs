# syntax=docker/dockerfile:experimental
FROM rust:1.75-buster as builder

WORKDIR /usr/src/app

COPY ./app .
COPY ./lib ../lib

RUN	cargo install --path .

RUN cargo build --release

CMD ["/bin/bash", "-c", "cargo run -r"]

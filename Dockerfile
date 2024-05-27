FROM rust:1.78 as build_base

RUN apt update &&  apt install -y libgdal-dev

WORKDIR /usr/src/stackmap-consume

COPY Cargo.toml Cargo.lock /usr/src/stackmap-consume/
COPY src /usr/src/stackmap-consume/src

RUN cargo update


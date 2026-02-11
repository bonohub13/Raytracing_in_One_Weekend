FROM rust:latest

RUN apt update
RUN apt upgrade -y

RUN rustup update
RUN rustup component add rustfmt
RUN rustup component add clippy

WORKDIR /app

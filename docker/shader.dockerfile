FROM rust:latest

RUN apt update
RUN apt upgrade -y
RUN apt install -y \
    spirv-tools

RUN rustup update
RUN cargo install naga-cli

WORKDIR /app

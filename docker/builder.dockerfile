FROM debian:13-slim

RUN apt update
RUN apt upgrade -y
RUN apt install -y \
    build-essential

WORKDIR /app

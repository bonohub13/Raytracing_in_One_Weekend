FROM rust:latest

RUN apt update
RUN apt upgrade -y
RUN apt install -y \
    clang \
    lld \
    llvm \
    spirv-tools \
    build-essential \
    wget \
    cmake \
    git \
    golang \
    jq

RUN rustup update
RUN rustup component add rustfmt
RUN rustup component add clippy

ENV PATH="/usr/local/go/bin:/root/go/bin:${PATH}"

RUN go install github.com/google/addlicense@latest

WORKDIR /vulkan_sdk

RUN git clone https://github.com/KhronosGroup/SPIRV-Reflect.git
WORKDIR /vulkan_sdk/SPIRV-Reflect
RUN cmake -B build -DSPIRV_REFLECT_BUILD_SHARED_LIB=ON
RUN cmake --build build
RUN cmake --install build

WORKDIR /slang
RUN wget https://github.com/shader-slang/slang/releases/download/v2026.1.1/slang-2026.1.1-linux-x86_64.tar.gz
RUN mkdir -pv /opt/slang
RUN tar xzvf slang-2026.1.1-linux-x86_64.tar.gz -C /opt/slang

ENV PATH="/opt/slang/bin:${PATH}"

WORKDIR /app

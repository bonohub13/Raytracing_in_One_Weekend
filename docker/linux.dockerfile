FROM rtiow:base

RUN apt install -y \
    libvulkan-dev \
    vulkan-validationlayers

RUN rustup target add x86_64-unknown-linux-gnu

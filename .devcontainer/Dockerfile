FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install required packages
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain for WASI
RUN rustup target add wasm32-wasi
RUN rustup target add wasm32-wasip1
RUN rustup target add wasm32-unknown-unknown

# Install cargo-component
RUN cargo install cargo-component

# Install Wasmtime
RUN curl -L https://github.com/bytecodealliance/wasmtime/releases/download/v16.0.0/wasmtime-v16.0.0-x86_64-linux.tar.xz | \
    tar -xJf - --strip-components=1 -C /usr/local/bin wasmtime-v16.0.0-x86_64-linux/wasmtime

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash

# Add Bun to PATH
ENV PATH="/root/.bun/bin:${PATH}"

# Set working directory
WORKDIR /workspace
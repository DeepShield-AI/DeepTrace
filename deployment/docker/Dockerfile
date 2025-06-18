FROM 47.97.67.233:5000/deepshield/ubuntu:24.04 AS builder
ARG TZ=Asia/Shanghai
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential clang llvm-18 llvm-18-dev llvm-18-tools \
    curl ca-certificates git libelf-dev && \
    rm -rf /var/lib/apt/lists/*

ENV LLVM_PATH="/lib/llvm-18"
ENV PATH="$LLVM_PATH/bin:$PATH"
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

RUN git clone --depth=1 --recurse-submodules https://github.com/libbpf/bpftool.git && \
    cd bpftool/src && make -j$(nproc) && make install && cd / && rm -rf bpftool

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable

ENV PATH="/root/.cargo/bin:$PATH"

RUN mkdir -p /root/.cargo && \
    echo "[source.crates-io]" > /root/.cargo/config.toml && \
    echo "replace-with = 'ustc'" >> /root/.cargo/config.toml && \
    echo "" >> /root/.cargo/config.toml && \
    echo "[source.ustc]" >> /root/.cargo/config.toml && \
    echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> /root/.cargo/config.toml

RUN rustup component add rust-src && \
    rustup toolchain install nightly --component rust-src && \
    cargo install --features=llvm-sys/prefer-dynamic bpf-linker && \
    cargo install bindgen-cli && \
    cargo install --git https://github.com/aya-rs/aya -- aya-tool
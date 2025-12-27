FROM 47.97.67.233:5000/deepshield/ubuntu:24.04 AS chef
ARG TZ=Asia/Shanghai
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# install basic tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential curl ca-certificates git && \
    rm -rf /var/lib/apt/lists/*

# configure rust mirrors
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable
ENV PATH="/root/.cargo/bin:$PATH"

# configure cargo mirrors
RUN mkdir -p /root/.cargo && \
    echo "[source.crates-io]" > /root/.cargo/config.toml && \
    echo "replace-with = 'ustc'" >> /root/.cargo/config.toml && \
    echo "" >> /root/.cargo/config.toml && \
    echo "[source.ustc]" >> /root/.cargo/config.toml && \
    echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> /root/.cargo/config.toml

# install cargo-chef
RUN cargo install cargo-chef

WORKDIR /workspace

# prepare dependencies cache
FROM chef AS planner
COPY . /DeepTrace
RUN cd /DeepTrace/agent && \
    cargo chef prepare --recipe-path recipe.json && \ 
    mv recipe.json /workspace/recipe.json && \
    rm -rf /DeepTrace

# build stage
FROM chef AS builder

# install system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    clang \
    libclang-18-dev \
    llvm-18 \
    llvm-18-dev \
    llvm-18-tools \
    libelf-dev \
    pkg-config \
    libssl-dev \
    openssl \
    make \
    && rm -rf /var/lib/apt/lists/*

# configure llvm
ENV LLVM_PATH="/usr/lib/llvm-18"
ENV PATH="$LLVM_PATH/bin:$PATH"

# build libbpf
WORKDIR /workspace/libbpf
RUN git clone https://github.com/libbpf/libbpf.git --branch libbpf-1.6.2 --depth 1 . && \
    cd src && \
    BUILD_STATIC_ONLY=y make && \
    make install && \
    ldconfig

# configure rust toolchain
RUN rustup component add rust-src && \
    rustup toolchain install nightly --component rust-src && \
    rustup target add aarch64-unknown-linux-gnu

# install bpf related tools
RUN cargo install bpf-linker

WORKDIR /workspace
COPY --from=planner /workspace/recipe.json recipe.json

# build dependencies (support workspace)
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    cargo chef cook --release --recipe-path recipe.json

CMD ["/bin/bash"]
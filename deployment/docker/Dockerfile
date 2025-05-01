FROM ubuntu:24.04

ARG TZ=Asia/Shanghai

RUN apt-get update && apt-get install -y --no-install-suggests --no-install-recommends \
    build-essential clang llvm-18 llvm-18-dev llvm-18-tools \
    curl ca-certificates git make libelf-dev && \
    export LLVM_PATH="/lib/llvm-18" && export PATH="$LLVM_PATH/bin:$PATH" && \
    rm -rf /var/lib/apt/lists/* && \
    ln -snf /usr/share/zoneinfo/${TZ} /etc/localtime && echo ${TZ} > /etc/timezone && \
    git clone --recurse-submodules https://github.com/libbpf/bpftool.git && \
    cd bpftool/src && make -j$(nproc) && make install && cd / && rm -rf bpftool && \
    mkdir -p /sys/kernel/tracing

WORKDIR /deeptrace
COPY . .

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable && \
    . "$HOME/.cargo/env" && \
    rustup component add rust-src && \
    rustup toolchain install nightly --component rust-src && \
    cargo install --features=llvm-sys/prefer-dynamic bpf-linker && \
    cargo install bindgen-cli && \
    cargo install --git https://github.com/aya-rs/aya -- aya-tool && \
    . "$HOME/.cargo/env" && \
    mkdir -p agent/src/ebpf/src && \
    aya-tool generate task_struct user_msghdr mmsghdr tcp_sock socket files_struct > agent/src/ebpf/src/vmlinux.rs && \
    sed -i '2i\#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unnecessary_transmutes)]' agent/src/ebpf/src/vmlinux.rs && \
    cargo build --release

ENV RUST_LOG=info
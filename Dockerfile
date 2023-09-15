FROM ubuntu:20.04

RUN apt-get update -y && apt-get install -y \
    build-essential \
    curl \
    git \
    grub-pc-bin \
    grub-common \
    xorriso \
    nasm

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup override set nightly-2023-09-01

RUN rustup component add rust-src --toolchain nightly-2023-09-01-x86_64-unknown-linux-gnu

WORKDIR /kfs

CMD bash

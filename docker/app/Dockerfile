FROM ubuntu:22.04

# prevent timezone dialogue
ENV DEBIAN_FRONTEND=noninteractive

RUN apt update --fix-missing && \
    apt upgrade -y
RUN apt install -y --fix-missing \
        gcc \
        xz-utils \
        ca-certificates \
        vim \
        curl \
        git \
        libssl-dev \
        pkg-config \

WORKDIR /root
# ==================== Nim ====================
RUN curl https://nim-lang.org/choosenim/init.sh -sSf | sh -s -- -y
ENV PATH $PATH:/root/.nimble/bin

# ==================== Rust ====================
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /application

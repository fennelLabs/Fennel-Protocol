FROM rust:1.67
RUN DEBIAN_FRONTEND=noninteractive \
    apt-get update -y && \
    ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime && \
    apt-get install -y tzdata && \
    dpkg-reconfigure --frontend noninteractive tzdata && \
    apt-get install unzip curl build-essential protobuf-compiler -y && \
    apt-get install clang libclang-dev libclang1 llvm llvm-dev clang-tools -y && \
    apt-get upgrade -y

RUN rustup update nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:1.67 as base
RUN DEBIAN_FRONTEND=noninteractive \
    apt-get update -y && \
    ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime && \
    apt-get install -y tzdata && \
    dpkg-reconfigure --frontend noninteractive tzdata && \
    apt-get install unzip curl build-essential protobuf-compiler -y && \
    apt-get install clang libclang-dev libclang1 llvm llvm-dev clang-tools -y && \
    apt-get upgrade -y

RUN rustup update nightly && \
    rustup default nightly-2022-11-15 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2022-11-15

FROM base as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM base as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release

CMD cargo run --release -- \
    --dev \
    --tmp \
    --unsafe-ws-external \
    --rpc-external \
    --prometheus-external \
    --rpc-methods unsafe \
    --rpc-cors all

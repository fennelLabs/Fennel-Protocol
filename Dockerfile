FROM rust:1.64 as base
RUN DEBIAN_FRONTEND=noninteractive \
    apt-get update -y && \
    ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime && \
    apt-get install -y tzdata && \
    dpkg-reconfigure --frontend noninteractive tzdata && \
    apt-get install unzip curl build-essential protobuf-compiler -y && \
    apt-get install clang libclang-dev libclang1 llvm llvm-dev clang-tools -y && \
    apt-get upgrade -y

RUN rustup default nightly && \
    rustup update nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

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

CMD ["cargo", "run", "--release", "--", "--chain=dev", "--ws-external", "--rpc-external", "--rpc-cors=all", "--rpc-methods=unsafe", "--ws-port=9944", "--rpc-port=9933", "--tmp"]
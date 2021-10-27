FROM ubuntu:latest

RUN DEBIAN_FRONTEND=noninteractive \
    apt-get update -y && \
    ln -fs /usr/share/zoneinfo/America/New_York /etc/localtime && \
    apt-get install -y tzdata && \
    dpkg-reconfigure --frontend noninteractive tzdata && \
    apt-get install unzip curl build-essential protobuf-compiler -y && \
    apt-get install clang libclang-dev libclang1 llvm llvm-dev clang-tools -y && \
    apt-get upgrade -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
RUN /root/.cargo/bin/rustup update nightly
RUN /root/.cargo/bin/rustup update stable
RUN /root/.cargo/bin/rustup default nightly
RUN /root/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain nightly
EXPOSE 9944

COPY . /opt/app
WORKDIR /opt/app

RUN /root/.cargo/bin/cargo build
RUN /root/.cargo/bin/cargo test
RUN /root/.cargo/bin/cargo bench
RUN /root/.cargo/bin/cargo doc

ENTRYPOINT [ "/root/.cargo/bin/cargo", "run" ]

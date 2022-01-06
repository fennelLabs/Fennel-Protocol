rustup update nightly
rustup update stable
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

cargo build
cargo test
cargo bench
cargo doc

cargo build --release
cargo doc --release
cp -rf ./target/doc ./docs

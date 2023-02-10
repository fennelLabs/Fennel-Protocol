with import <nixpkgs> {};
let
  pkgs = import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; };
  myrust = (pkgs.rust-bin.nightly."2023-01-31".default.override {
    extensions = [ "rust-src" "rust-analysis" "rust-std" "clippy-preview" "rustfmt-preview" ];
    targets = [ "wasm32-unknown-unknown" ];
  });
in
  pkgs.mkShell {
    buildInputs = [
      myrust gcc openssl pkgconfig cmake python3 llvmPackages.clang gnuplot libbfd libopcodes libunwind autoconf automake libtool rsync yarn nodejs nodePackages.typescript rocksdb zstd lz4 snappy zlib bzip2 llvmPackages.libclang rust-analyzer
    ];
    LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [ pkgs.zlib pkgs.bzip2 pkgs.lz4 pkgs.snappy pkgs.zstd llvmPackages.libclang stdenv.cc.cc ]}";
    RUST_SRC_PATH = "${myrust}/lib/rustlib/src/rust/library";
    ROCKSDB_LIB_DIR="${rocksdb}/lib";
    PROTOC = "${protobuf}/bin/protoc";
  }

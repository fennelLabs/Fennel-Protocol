#!/usr/bin/env bash
set -e

git submodule init
git submodule update

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup update nightly
   rustup update stable
fi

rustup override set nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

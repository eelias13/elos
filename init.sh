#!/bin/bash

# install qemu (the emulator) and curl neded for cargo
apt install curl qemu-system-x86 -y

# install rust
curl https://sh.rustup.rs -sSf | sh -s -- -y

# install rust compiler tools
rustup component add rust-src --toolchain nightly-2021-07-27-x86_64-unknown-linux-gnu
rustup component add llvm-tools-preview
rustup update

# install bootimage
cargo install bootimage
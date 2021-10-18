#!/bin/bash
apt install qemu-system-x86 -y
rustup component add rust-src --toolchain nightly-2021-07-27-x86_64-unknown-linux-gnu
rustup component add llvm-tools-preview
rustup update
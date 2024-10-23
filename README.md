# GY-521-i2c
Download toolchain and extract int some location: /opt

export PATH=/path/to/extract/toolchain/bin:$PATH

rustup update
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target armv7-unknown-linux-gnueabihf --release

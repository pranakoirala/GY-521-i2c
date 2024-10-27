# Embedded Rust Development

### Cross compiling Rust for specific target
Download toolchain and extract int some location: /opt

I will be using raspberry PI model 4B for my experiment examples. With some minor change in example it should work for other linux targets.

```bash 
export PATH=/path/to/extract/toolchain/bin:$PATH
rustup update
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target armv7-unknown-linux-gnueabihf --release
```

For raspberry gpio pin number are availble under: ```cat /sys/kernel/debug/gpio to see```
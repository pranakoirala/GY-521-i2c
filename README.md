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

Note: VCC 3.3V/5V can be external too but common reference ground must be connected to one of GND in raspberry PI.

### Connection to Raspberry PI-4B

| Sensor Name   | Sensor PIN    | RPI4B J8      | Pin Number |
|---------------|---------------|---------------|------------|
| HC-SR04       | VCC           | 5V            |            |
|               | GND           | GND           |            |
|               | TRIG          | GPIO17 (11)   | 529        |
|               | ECHO          | GPIO27 (13)   | 539        |
| HC-SR501      | VCC           | 5V            |            |
|               | GND           | GND           |            |
|               | OUT           | GPIO22 (15)   | 534        |
| GY-521        | VCC           | 3.3V          |            |
|               | GND           | GND           |            |
|               | SDA           | SDA (3)       |            |
|               | SCL           | SCL (5)       |            |
| MPR121        | VCC           | 3.3V          |            |
|               | GND           | GND           |            |
|               | SDA           | SDA (3)       |            |
|               | SCL           | SCL (5)       |            |
|               | IRQ           | GPIO4 (7)     | 516        |

## Arduino with Rust
#### Compile

```
cargo b -r
```

#### convert to hex file

```
avr-objcopy -j .text -j .data -O ihex ./target/avr-atmega328p/release/rust-arduino.elf ./out/rust-arduino.hex 
```
#### Guide

[complete-guide-running-rust-arduino](https://blog.logrocket.com/complete-guide-running-rust-arduino/)

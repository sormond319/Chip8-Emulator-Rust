# CHIP-8 Emulator in Rust

A simple CHIP-8 emulator built in Rust, featuring a core library and an SDL2-based desktop frontend. Built using An Introduction to Chip-8 Emulation using the Rust Programming Language by aquova.

# Reference
https://github.com/aquova/chip8-book

## Features
- Full CHIP-8 instruction set implementation.
- SDL2 graphics and input handling.

## Build and Run
1. Install Rust: [rustup.rs](https://rustup.rs).
2. Install SDL2 dev libs.
3. Clone and build:
   ```bash
   git clone https://github.com/sormond319/Chip8-Emulator-Rust.git
   cd chip8-emulator-rust
   cargo build --release
   cargo run "rom.ch8"
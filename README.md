# Rust Rewrite of the Example for the Rust RISC-V Simulator

This project is a Rust rewrite of the example from the [Rust RISC-V Simulator (rrs)](https://github.com/GregAC/rrs) repository. 

## Overview

The Rust RISC-V Simulator (rrs) is a tool for simulating RISC-V programs. This project re-implements the example provided in the original `rrs` repository.

## Getting Started

### Prerequisites install riscv32im-unknown-none-elf && rss-cli

```bash
$ rustup default nightly
$ rustup target add riscv32im-unknown-none-elf
$ cargo install --git https://github.com/GregAC/rrs.git --bin rrs-cli
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

### Running the Example

```bash
$ cargo build --release 
# Convert the ELF file to a binary file
$ rust-objcopy target/riscv32im-unknown-none-elf/release/rrs-rust-example -O binary target/riscv32im-unknown-none-elf/release/rrs-rust-example.bin
$ rrs-cli --binary target/riscv32im-unknown-none-elf/release/rrs-rust-example.bin -l target/riscv32im-unknown-none-elf/release/execution.log
Loading binary target/riscv32im-unknown-none-elf/release/rrs-rust-example.bin to address 00100000
Hello, world!
39 instructions executed in 0 ms 1.625 MHz
```
## Acknowledgements

Special thanks to [GregAC](https://github.com/GregAC) for creating the original Rust RISC-V Simulator (rrs).

## Links

- [Rust RISC-V Simulator (rrs) on GitHub](https://github.com/GregAC/rrs)
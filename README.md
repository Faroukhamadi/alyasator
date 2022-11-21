# Alyasator

![alyasator demo](assets/demo.gif)

## About

This is a simple tool to create aliases for your commands. It is written in Rust and is definitely blazingly fast.

## Requirements

Rust and Cargo are required to build this project. You can install them from [here](https://rustup.rs).

## Installation

```bash
git clone --depth 1 https://github.com/Faroukhamadi/alyasator.git ~/.alyasator
cd ~/.alyasator && cargo build --release
sudo cp ~/.alyasator/target/release/alyasator /usr/local/bin
```

## Supported Shells

- Bash
- Zsh
- Fish

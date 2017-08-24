# Getting Started

## Rust & Initial Setup
This tutorial assumes a beginner level of programming knowledge and the Rust
programming language, but if you're new to programming or Rust it's best to
first read the [Rust Book](https://doc.rust-lang.org/book/).

To start out you need a new cargo binary package.
```sh
$ cargo new <package name> --bin
$ cd <package name>
```

Calcium is split up into different crates, so you can only bring in what you
need and crates can be updated independently. For 2D rendering you need the
following crates.
- `calcium-rendering` Generic rendering systems and resources
- `calcium-rendering-simple2d` 2D rendering systems
- `calcium-rendering-static` Compiles your code with multiple backends
- `cgmath` Generic game math types, used in the calcium API
- `pistoncore-input` Types related to input and input handling
- `pistoncore-window` Types related to receiving events and input from windows

Add them to your dependencies in your Cargo.toml file like this.
```toml
[dependencies]
calcium-rendering = "0.1"
calcium-rendering-simple2d = "0.1"
calcium-rendering-static = {version = "0.1", features = ["simple2d"]}
cgmath = "0.15"
pistoncore-input = "0.18"
pistoncore-window = "0.27"
```

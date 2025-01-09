# hello-world

This is a Hello World program in Rust, demonstrating basic functionality of
rustc and cargo.

## Table of contents

- [Installation](#installation)
- [Basics](#basics)
- [Build modes](#build-modes)
- [Static linking](#static-linking)
- [Using github as upstream](#using-github-as-upstream)

## Installation

To install the Rust toolkit, we can use `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | shg
```

## Basics

To use cargo, we initialize the project directory:

```sh
cargo init
```

define a dependency for pretty formatting - `pretty-hex`:

```toml
[dependencies]
pretty-hex = "0.2.0"
```

We can also add this dependency by running `cargo add pretty-hex` in the
project directory.

Then we can use it in our main file - `src/main.rs`:

```rust
use pretty_hex::*;

fn main() {
    let data = vec![0x27, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30];
    println!("{:?}", data.hex_dump());
}
```

To build this project, we run:

```sh
cargo build
```

Then to run it, we run:

```sh
cargo run
```

This will print the following output:

```
0000:   27 2a 2b 2c  2d 2e 2f 30                             '*+,-./0
```

## Build modes

We can build the project in different modes:
- `cargo build` - build the project in debug mode
- `cargo build --release` - build the project in release mode

The release mode will optimize the binary for performance.

## Static linking

To build the project with static linking, we first have to install the `musl`
target:

```sh
rustup target add x86_64-unknown-linux-musl
```

Then we can run:

```sh
cargo build --release --target x86_64-unknown-linux-musl
```

The resulting binary will be statically linked and can be run on any Linux
system without the need for additional dependencies.

## Using github as upstream

To use a github repository as a dependency, we can add a flag to `cargo add`
command:

```sh
cargo add pretty-hex --git https://github.com/wolandr/pretty-hex
```

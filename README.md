# Turn Me Off

<div align="center">

![Format and Lints](https://github.com/SamyAB/genetic-sat/actions/workflows/check.yml/badge.svg)

</div>

An API to turn off devices it is deployed on.

## Compatibility

`turn-me-off` is compatible with any operating system that uses `systemd` as its init system,
which is most of linux distributions these days.

## Installation

### Build from source

To build `turn-me-off`, you need to have the Rust toolchain installed on your machine,
if you don't have it, install it using [rustup](https://rustup.rs/).

To build the executable you can run:

```Shell
cargo build -r
```

The `turn-me-off` binary should located in `target/release/`.

## Usage

Executing the binary starts the HTTP server, and you should be able to access a
Swagger UI documenting the API at the address http://localhost:3000/docs/#/

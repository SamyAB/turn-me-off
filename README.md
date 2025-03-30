# Turn Me Off

<div align="center">

![Format and Lints](https://github.com/SamyAB/turn-me-off/actions/workflows/check.yml/badge.svg)

</div>

An API to turn off devices it is deployed on.

## Compatibility

`turn-me-off` is compatible with any operating system that uses `systemd` as its init system,
which is most of linux distributions these days.

## Installation

### Build from source

To build `turn-me-off`, you need to have the Rust toolchain installed on your machine,
if you don't have it, install it using [rustup](https://rustup.rs/).

To build and install the executable you can run:

```Shell
cargo install --git https://github.com/SamyAB/turn-me-off
```

## Usage

### Starting the server

Executing the binary starts the HTTP server, and you should be able to access a
Swagger UI documenting the API at the address http://localhost:3000/docs/#/

### Changing the port

Note that the port 3000 is the default port, to control the port on which it listens,
you can use the environment variable `TMF_PORT`:

```Shell
TMF_PORT=3001 turn-me-off
```

### Controlling log levels

By default the server will only log the minimum information (startup, shutdown, ...),
you can enable more logs, for example all connections established with the server, by
setting the env variable [`RUST_LOG`](https://docs.rs/env_logger/0.11.1/env_logger/#enabling-logging):

```Shell
RUST_LOG=trace turn-me-off
```

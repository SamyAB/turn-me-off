# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`turn-me-off` is an HTTP API server written in Rust that allows remote control of a Linux device's power state (shutdown, reboot, suspend). It is designed for systemd-based Linux distributions and uses systemctl commands to control the system.

## Architecture

### Entry Point (src/main.rs)

- Sets up the Axum HTTP server with OpenAPI documentation (Swagger UI)
- Configures tracing/logging via `RUST_LOG` environment variable (defaults to `turn_me_off=info`)
- Server port is controlled by `TMF_PORT` environment variable (defaults to 3000)
- Implements graceful shutdown handling for Ctrl+C and SIGTERM signals
- Routes are defined in main.rs but handlers are in api.rs

### API Handlers (src/api.rs)

All power management endpoints use `/usr/bin/systemctl` with `--no-ask-password` flag:
- `/alive` (GET): Health check endpoint
- `/turn-off` (PUT): Executes `systemctl poweroff`
- `/reboot` (PUT): Executes `systemctl reboot`
- `/suspend` (PUT): Executes `systemctl suspend`
- `/hostname` (GET): Reads `/etc/hostname`

Error handling pattern: Each systemctl command checks exit codes and detects permission issues by parsing stderr for "Interactive authentication required."

## Development Commands

### Build and Run
```bash
cargo build                    # Debug build
cargo build --release          # Optimized release build
cargo run                      # Run in debug mode
TMF_PORT=3001 cargo run        # Run on custom port
RUST_LOG=trace cargo run       # Run with verbose logging
```

### Code Quality
```bash
cargo fmt                      # Format code
cargo fmt --check              # Check formatting without modifying
cargo clippy                   # Run linter
```

### Testing the API
Once running, access Swagger UI at: http://localhost:3000/docs/

## Linting Rules

This project uses **strict Clippy lints** (Cargo.toml:22-31):
- `pedantic = "deny"` - All pedantic lints are errors
- `unwrap_used = "deny"` - Must use proper error handling instead of `.unwrap()`
- Use `.expect("descriptive message")` for errors that should never occur
- Several additional deny-level lints for code quality

When adding new code, ensure it passes `cargo clippy` with these strict settings.

## Release Profile

The release build (Cargo.toml:15-20) is optimized for size:
- Optimization level 'z' (optimize for binary size)
- Link-time optimization (LTO) enabled
- Single codegen unit
- Symbols stripped

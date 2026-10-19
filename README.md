# rustrs

Rust API scaffolding CLI.

## Status

This project is under development. Project generation and code generators are
not available yet; only the minimal executable entry point exists.

## Prerequisites

- Rust stable >= 1.85.0 (MSRV).

## Setup

```bash
git clone <repository-url>
cd rustrs
cargo check
```

## Verify

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Roadmap

See `docs/roadmap/` for phase documents. Planned commands (not yet available)
include `rustrs new`, `rustrs generate`, and related tooling.

# rustrs

Rust API scaffolding CLI.

## Status

Phase 3 (Template and Filesystem Engine) is complete. The CLI currently provides:

- `--help`
- `--version`
- Global options: `--verbose` / `-v`, `--quiet` / `-q`, `--no-color`

Internally, typed manifest parsing, validation, schema versioning, compatibility resolution, capability rules, read-only manifest discovery, template resolution, rendering, generation planning, path safety, and filesystem writing are available.

Project scaffolding, templates, and code generators are not available yet.

## Prerequisites

- Rust stable >= 1.85.0 (MSRV).

## Setup

```bash
git clone https://github.com/xRiot45/rustrs.git
cd rustrs
cargo check
```

## Usage

```bash
rustrs --help
rustrs --version
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

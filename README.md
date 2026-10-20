# rustrs

Rust API scaffolding CLI.

## Status

Phase 4 (Project Scaffolding) is complete. The CLI currently provides:

- `--help`
- `--version`
- Global options: `--verbose` / `-v`, `--quiet` / `-q`, `--no-color`
- `rustrs new <project-name>` with `--template`, `--framework`, `--database`, `--orm`, `--defaults`, `--non-interactive`, `--dry-run`

Supported templates: `minimal`, `simple`, `standard`, `enterprise` (all currently build Axum + Postgres + SQLx starter projects).

Internally, typed manifest parsing, validation, schema versioning, compatibility resolution, capability rules, read-only manifest discovery, template resolution, rendering, generation planning, path safety, filesystem writing, and project scaffolding are available.

Code generators are not available yet.

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
rustrs new my-api --defaults
rustrs new my-api --template standard --defaults
rustrs new my-api --defaults --dry-run
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

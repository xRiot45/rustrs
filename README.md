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

## Inspection and diagnostics

```bash
rustrs info
rustrs modules
rustrs routes
rustrs deps
rustrs doctor
```

All inspection commands are read-only. `--path` and `--json` are supported where applicable.

## Database workflow

```bash
rustrs db create-migration <name>
rustrs db migrate
rustrs db rollback
rustrs db status
rustrs db seed
# Note: seeding is currently unsupported because no real seeder mechanism exists
```

Database workflow requires a supported project (`Axum + Postgres + SQLx`) and, where needed, `sqlx-cli` and a reachable PostgreSQL database.

## Development workflow

```bash
rustrs dev
rustrs dev --watch
rustrs dev up
rustrs dev down
rustrs dev logs
```

`rustrs dev` runs the generated application. `--watch` restarts it when relevant source files change. Managed lifecycle commands store minimal state under `.rustrs/dev/`.

## Quality commands

```bash
rustrs check
rustrs lint
rustrs test
rustrs format
rustrs format --write
rustrs audit
```

`format` is check-only by default. `audit` requires `cargo-audit`.

## Documentation helper

```bash
rustrs docs
rustrs docs --json
```

The helper validates OpenAPI configuration and does not fabricate endpoints.

## Shell completion

```bash
rustrs completion bash
rustrs completion zsh
rustrs completion fish
```

## Generated assets

```bash
rustrs generate docker
rustrs generate compose
rustrs generate ci-github
rustrs generate ci-gitlab
rustrs generate release
```

These commands generate files with the existing safe-write and conflict policy.

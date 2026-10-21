# rustrs

Rust API scaffolding CLI.

## Status

Phase 9 (Hardening & MVP Release) is in progress. The CLI currently provides project scaffolding, code generators, project inspection, diagnostics, database workflows, development workflows, quality commands, an OpenAPI configuration helper, shell completion, and delivery asset generation.

## Supported MVP scope

- Templates: `minimal`, `simple`, `standard`, `enterprise`.
- Framework/database/data access: `Axum + PostgreSQL + SQLx`.
- Verified platform: Linux.
- MSRV: Rust 1.85.0.
- Edition: Rust 2024.

See `docs/support-matrix.md` for the full support matrix.

## Prerequisites

- Rust stable >= 1.85.0.
- For database workflows: `sqlx-cli` and a reachable PostgreSQL database.
- For audit: `cargo-audit`.
- For Docker assets validation: Docker and Docker Compose.

## Setup

```bash
git clone https://github.com/xRiot45/rustrs.git
cd rustrs
cargo check
```

## Usage

### Create a project

```bash
rustrs new my-api --defaults
rustrs new my-api --template standard --defaults
rustrs new my-api --defaults --dry-run
```

### Generate code

```bash
rustrs generate resource products
rustrs generate migration add_status
rustrs g resource products --dry-run
```

### Inspect a project

```bash
rustrs info
rustrs modules
rustrs routes
rustrs deps
rustrs doctor
```

### Database workflow

```bash
rustrs db create-migration <name>
rustrs db migrate
rustrs db migrate --dry-run
rustrs db rollback
rustrs db status
rustrs db seed
```

`db seed` is currently unsupported because no real seeder mechanism exists. `db rollback` requires down migration files; generated migrations are up-only by default.

### Development workflow

```bash
rustrs dev
rustrs dev --watch
rustrs dev up
rustrs dev down
rustrs dev logs
```

### Quality commands

```bash
rustrs check
rustrs lint
rustrs test
rustrs format
rustrs format --write
rustrs audit
```

`format` is check-only by default. `audit` requires `cargo-audit`.

### Documentation helper

```bash
rustrs docs
rustrs docs --json
```

The helper validates OpenAPI configuration. It does not fabricate endpoints or runtime documentation code.

### Shell completion

```bash
rustrs completion bash
rustrs completion zsh
rustrs completion fish
```

### Generated delivery assets

```bash
rustrs generate docker
rustrs generate compose
rustrs generate ci-github
rustrs generate ci-gitlab
rustrs generate release
```

These commands use the existing safe-write and conflict policy.

## Verify

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Known limitations

- Only `Axum + PostgreSQL + SQLx` is supported.
- macOS and Windows are not verified.
- `rustrs db seed` is unsupported.
- Generated migrations are up-only by default.
- OpenAPI helper validates configuration only.
- `rustrs audit` requires `cargo-audit`.
- Docker build is not run automatically during asset generation.

## Roadmap

See `docs/roadmap/` for phase documents.
EOF

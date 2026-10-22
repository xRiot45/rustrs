# rustrs

Rust API scaffolding CLI.

## Status

Phase 9 (Hardening & MVP Release) is in progress. The CLI currently provides project scaffolding, code generators, project inspection, diagnostics, database workflows, development workflows, quality commands, an OpenAPI configuration helper, shell completion, and delivery asset generation.

## Supported MVP scope

- Templates: `minimal`, `simple`, `standard`, `enterprise`.
- Framework: `Axum`, `Actix Web`, `Rocket`.
- Database: `PostgreSQL`, `MySQL`, `SQLite`, `None`.
- Data access: `SQLx` is the full generated path; `SeaORM` and `Diesel` generate minimal repository placeholders; `None` disables database-dependent generators.
- Architecture: `clean_modular`.
- Verified platform: Linux.
- MSRV: Rust 1.85.0.
- Edition: Rust 2024.

See `docs/support-matrix.md` for the full support matrix.

## Prerequisites

Required:

- Rust stable >= 1.85.0
- Cargo
- Git

Optional, depending on workflow:

- `sqlx-cli` and a reachable PostgreSQL database for `rustrs db ...`
- `cargo-audit` for `rustrs audit`
- Docker and Docker Compose for validating generated Docker assets
- A terminal with stdin for interactive prompts

## Installation

### Install from source with Cargo (recommended MVP route)

```bash
git clone https://github.com/xRiot45/rustrs.git
cd rustrs
cargo install --locked --path .
```

Cargo installs the executable to `~/.cargo/bin/rustrs` by default. Verify the installation:

```bash
rustrs --version
rustrs --help
```

If `rustrs` is not found, add Cargo's bin directory to your `PATH`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then add that line to your shell profile so it persists.

### Build without installing

Use this for local development or when you do not want to modify your Cargo installation:

```bash
git clone https://github.com/xRiot45/rustrs.git
cd rustrs
cargo build --release
```

The executable is then available at:

```bash
./target/release/rustrs
```

You can run it directly or copy/symlink it into a directory on your `PATH`.

### Upgrade or reinstall

Run the install command again. Use `--force` if Cargo reports that the package is already installed:

```bash
cd rustrs
cargo install --locked --path . --force
```

### Uninstall

If installed with Cargo:

```bash
cargo uninstall rustrs
```

If you copied the binary manually, remove the `rustrs` executable from your `PATH`.

### Isolated installation check

To test installation without changing your global toolchain, use a temporary root:

```bash
cargo install --locked --path . --root /tmp/rustrs-install
export PATH="/tmp/rustrs-install/bin:$PATH"
rustrs --version
rustrs --help
```

## Quick start

```bash
rustrs new my-api --defaults
cd my-api
rustrs generate resource products
rustrs info
rustrs doctor
cargo check
```

For a non-mutating preview:

```bash
rustrs new my-api --defaults --dry-run
rustrs new my-api --defaults --git
```

## Usage

Global options:

```bash
rustrs -v
rustrs -q
rustrs --no-color
```

Primary output is written to stdout. Diagnostics and errors are written to stderr.

### Create a project

```bash
rustrs new my-api
rustrs new my-api --defaults
rustrs new my-api --template standard --defaults
rustrs new my-api --defaults --dry-run
rustrs new my-api --defaults --git
```

Supported project name rules:

- Must start with a lowercase ASCII letter.
- Use the other `--template`, `--framework`, `--database`, and `--orm` values only when you understand the support matrix; the supported MVP combination is `axum + postgres + sqlx`.

Interactive mode is used when stdin is a terminal and `--defaults` is not passed. Non-interactive mode requires the needed flags:

```bash
rustrs new my-api --template minimal --framework axum --database postgres --orm sqlx --non-interactive
```

`--dry-run` does not modify the filesystem.

### Generate code

```bash
rustrs generate resource products
rustrs generate migration add_status
rustrs g resource products --dry-run
```

Available generators:

`module`, `entity`, `dto`, `service`, `repository`, `controller`, `handler`, `middleware`, `test`, `migration`, `event-handler`, `resource`, `docker`, `compose`, `ci-github`, `ci-gitlab`, `release`.

Useful flags:

```bash
rustrs generate resource products --dry-run
rustrs generate resource products --force
rustrs generate resource products --skip-tests
rustrs generate resource products --skip-docs
rustrs generate resource products --no-interactive
rustrs generate guard products
rustrs generate event order
rustrs generate job cleanup
rustrs generate validator product
rustrs generate resource products --path ./my-api
rustrs generate resource products --singular product --plural products
```

`--force` overwrites conflicting generated files according to the generation policy. It does not delete or recreate the whole project.

### Inspect a project

Run these inside a generated project, or pass `--path`:

```bash
rustrs info
rustrs modules
rustrs routes
rustrs deps
rustrs doctor
rustrs config show
rustrs config get features.auth
rustrs config set features.auth true
rustrs env check
rustrs update check
rustrs info --json
```

### Project configuration

```bash
rustrs config show
rustrs config show --json
rustrs config get features.auth
rustrs config set features.auth true
rustrs config set project_name my-api
```

`config set` validates the manifest and compatibility before writing. It updates `.rustrs/config.toml` only; it does not regenerate source files.

### Environment check

```bash
rustrs env check
rustrs env check --json
```

This checks available tools and the `DATABASE_URL` format when it is set. Missing optional tools are reported without failing the command.

### Add a capability

```bash
rustrs add docker
rustrs add docker --dry-run
rustrs add docker --force
rustrs add cicd --provider github
rustrs add cicd --provider gitlab
rustrs add auth
rustrs add rbac
rustrs add openapi
rustrs add redis
rustrs add email
rustrs add storage
rustrs add audit_log
rustrs add rate_limit
rustrs add idempotency
rustrs add observability
```

In the current MVP, only `docker` and `cicd` have concrete generated effects. Other capabilities are reported as not implemented instead of being enabled silently.

### Database workflow

Requires `sqlx-cli` and a reachable PostgreSQL database where applicable.

```bash
rustrs db create-migration <name>
rustrs db migrate
rustrs db migrate --dry-run
rustrs db rollback
rustrs db status
rustrs db seed
```

Limitations:

- `db seed` runs the generated Rust seeder module.
- `db rollback` requires down migration files; generated migrations include paired down files.

### Development workflow

```bash
rustrs dev
rustrs dev --watch
rustrs dev up
rustrs dev down
rustrs dev logs
```

`dev up`, `dev down`, and `dev logs` manage a background application process.

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

This helper validates OpenAPI configuration. It does not fabricate endpoints or runtime documentation code.

### Shell completion

```bash
rustrs completion bash
rustrs completion zsh
rustrs completion fish
```

Install completion manually according to your shell's documentation. `rustrs` does not edit shell startup files automatically.

Examples:

```bash
# Bash
rustrs completion bash > ~/.local/share/bash-completion/completions/rustrs

# Zsh
rustrs completion zsh > ~/.zfunc/_rustrs

# Fish
rustrs completion fish > ~/.config/fish/completions/rustrs.fish
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

## Troubleshooting

See `docs/troubleshooting.md` for installation and runtime troubleshooting.

Common checks:

- Confirm `rustc --version` and `cargo --version` meet MSRV 1.85.0.
- Confirm `~/.cargo/bin` is on `PATH`.
- Run `rustrs doctor` inside a generated project.
- Use `--dry-run` to preview file changes without writing them.
- Run `cargo check` in the generated project before running the application.

## Verify (contributors)

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Known limitations

- Only `Axum + PostgreSQL + SQLx` is supported.
- macOS and Windows are not verified.
- `rustrs db seed` requires a reachable database and runs the generated seeder module.
- Generated migrations are up-only by default.
- OpenAPI helper validates configuration only.
- `rustrs audit` requires `cargo-audit`.
- Docker build is not run automatically during asset generation.
- Prebuilt binary distribution is not provided; source/Cargo install is the verified MVP route.

## Roadmap

See `docs/roadmap/` for phase documents.

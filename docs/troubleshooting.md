# Troubleshooting — rustrs

## Installation

### `rustrs: command not found`

Cargo likely installed the binary outside your `PATH`.

Check:

```bash
cargo --version
ls "$HOME/.cargo/bin/rustrs"
echo "$PATH"
```

Fix by adding Cargo's bin directory to your shell profile:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Rust version is too old

`rustrs` requires Rust 1.85.0 or newer.

```bash
rustc --version
rustup update stable
rustup default stable
```

### `cargo install` fails while resolving dependencies

Use the locked dependency set:

```bash
cargo install --locked --path .
```

If that fails, run from the repository root and ensure the working tree contains `Cargo.lock`.

### Installing into a temporary root

Use this for a clean or isolated check:

```bash
cargo install --locked --path . --root /tmp/rustrs-install
export PATH="/tmp/rustrs-install/bin:$PATH"
rustrs --version
```

## Project creation

### `project_name must start with a lowercase ASCII letter`

Project names must begin with a lowercase ASCII letter. Use a name such as `my-api` instead of `/tmp/my-api` or `MyApi`.

### Target directory already exists

`rustrs new` refuses to overwrite a non-empty target directory. Choose a new name or remove/move the existing directory after confirming it is not needed.

### Missing flags in non-interactive mode

When stdin is not a terminal, or `--non-interactive` is used, provide the required choices:

```bash
rustrs new my-api --template minimal --framework axum --database postgres --orm sqlx --non-interactive
```

Or use:

```bash
rustrs new my-api --defaults
```

### Unsupported framework, database, or data access

The MVP supports only `axum + postgres + sqlx`. Other values may appear in CLI choices but are not supported for generation.

## Runtime and project workflows

### `rustrs doctor` reports missing tools

Install the required external tools and ensure they are on `PATH`:

- `sqlx-cli` for database workflows
- `cargo-audit` for `rustrs audit`
- Docker and Docker Compose for generated asset validation

`rustrs` does not install these tools automatically.

### `rustrs db seed` is unsupported

No real seeder mechanism exists in the MVP. This command reports unsupported behavior.

### Rollback fails

Generated migrations are up-only by default. Rollback requires a down migration file. Create or provide the required down migration before running `rustrs db rollback`.

### Generated project does not compile

Run the generated project checks directly:

```bash
cd my-api
cargo fmt --check
cargo check
cargo test
```

Then run:

```bash
rustrs doctor
```

### `--dry-run` changed files

`--dry-run` is designed not to modify the filesystem. If files changed, check for another process or rerun with `--verbose` and inspect the reported operations.

## Shell completion

Completion output is printed to stdout. Redirect it to the completion file expected by your shell. `rustrs` does not edit shell startup files automatically.

## Configuration

### `config set` rejected a value

`rustrs config set` validates the manifest and compatibility before writing. Invalid keys, invalid values, invalid project names, invalid capability dependencies, and unsupported technology combinations are rejected without changing the manifest.

### Manifest formatting changed after `config set`

`config set` writes the manifest in canonical TOML form. Comments and custom formatting in `.rustrs/config.toml` may be removed. Generated project files are not regenerated automatically.

## Git initialization

### `--git` fails

`rustrs new --git` requires Git to be installed and available on `PATH`. The project files remain created; rerun Git initialization manually if needed.

## Capability management

### `rustrs add` reports a capability is not implemented

Only `docker` and `cicd` have concrete generated effects in the current MVP. Other capabilities are reported as not implemented instead of being enabled silently.

### `rustrs add` did not overwrite an existing file

`add` preserves existing files by default. Use `--force` to overwrite conflicting generated assets.

## Authentication

### Generated auth project requires `DATABASE_URL`

Auth-enabled projects use PostgreSQL. Set `DATABASE_URL` before running the application or migrations.

### Generated auth project requires `JWT_SECRET`

Set `JWT_SECRET` in the environment. The generated code uses a development fallback only when the variable is missing.

### `rustrs add auth` is unsupported on a template

Auth is implemented for `standard` and `enterprise` templates in the current MVP. Minimal and simple templates reject `add auth` with a clear error.

## OpenAPI

### OpenAPI route is unavailable

OpenAPI runtime code is only generated when the `openapi` capability is enabled. Run `rustrs add openapi` on a `standard` or `enterprise` project.

## Redis

### Redis connection fails

Set `REDIS_URL` before running the generated application. The generated client uses `redis://127.0.0.1/` as a fallback.

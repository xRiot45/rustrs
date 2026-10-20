use crate::cli::GenerateArgs;
use crate::error::CliError;
use crate::filesystem::apply_plan;
use crate::generation::plan::GenerationPlan;
use crate::generation::plan::{OperationOutcome, PlannedOperation, WritePolicy};
use crate::manifest::discovery::discover_manifest;
use crate::manifest::model::{Database, ProjectManifest};
const DOCKERFILE: &str = r#"FROM rust:1.85-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/{binary} /usr/local/bin/{binary}
CMD ["{binary}"]
"#;

const DOCKERIGNORE: &str = r#"target/
.git/
.env
*.log
"#;

const GITHUB_CI: &str = r#"name: CI

on:
  push:
  pull_request:

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo check
      - run: cargo test
      - run: cargo fmt --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
"#;

const GITLAB_CI: &str = r#"stages:
  - check
  - test
  - lint
  - format

variables:
  CARGO_HOME: ${CI_PROJECT_DIR}/.cargo

cache:
  paths:
    - target/
    - .cargo/

check:
  stage: check
  image: rust:1.85
  script:
    - cargo check

test:
  stage: test
  image: rust:1.85
  script:
    - cargo test

lint:
  stage: lint
  image: rust:1.85
  script:
    - cargo clippy --all-targets --all-features -- -D warnings

format:
  stage: format
  image: rust:1.85
  script:
    - cargo fmt --check
"#;

const RELEASE_WORKFLOW: &str = r#"name: Release

on:
  push:
    tags:
      - "v*.*.*"

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo check
      - run: cargo test
      - run: cargo fmt --check
      - run: cargo clippy --all-targets --all-features -- -D warnings

  build:
    needs: check
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v4
        with:
          name: rustrs-${{ matrix.os }}
          path: target/release/rustrs*
"#;

pub fn execute(
    kind: crate::generator::kind::GeneratorKind,
    args: &GenerateArgs,
    output: &crate::cli::Output,
) -> Result<(), CliError> {
    let start = args
        .path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let discovered = discover_manifest(&start)?;
    discovered.manifest.validate()?;
    crate::compatibility::check_compatibility(&discovered.manifest)?;

    let mut manifest = discovered.manifest.clone();
    let root = discovered.project_root.clone();
    let binary = manifest.project_name.clone();

    let mut operations = Vec::new();

    match kind {
        crate::generator::kind::GeneratorKind::Docker => {
            manifest.features.docker = true;
            let dockerfile = DOCKERFILE.replace("{binary}", &binary);
            add_file(&mut operations, "Dockerfile", dockerfile)?;
            add_file(&mut operations, ".dockerignore", DOCKERIGNORE.to_string())?;
        }
        crate::generator::kind::GeneratorKind::Compose => {
            manifest.features.docker = true;
            let compose = compose_content(&manifest);
            add_file(&mut operations, "docker-compose.yml", compose)?;
        }
        crate::generator::kind::GeneratorKind::CiGithub => {
            manifest.features.cicd = true;
            add_file(
                &mut operations,
                ".github/workflows/ci.yml",
                GITHUB_CI.to_string(),
            )?;
        }
        crate::generator::kind::GeneratorKind::CiGitlab => {
            manifest.features.cicd = true;
            add_file(&mut operations, ".gitlab-ci.yml", GITLAB_CI.to_string())?;
        }
        crate::generator::kind::GeneratorKind::Release => {
            manifest.features.cicd = true;
            add_file(
                &mut operations,
                ".github/workflows/release.yml",
                RELEASE_WORKFLOW.to_string(),
            )?;
        }
        _ => {
            return Err(CliError::InvalidInput(
                "unsupported asset generator".to_string(),
            ));
        }
    }

    let manifest_contents = toml::to_string_pretty(&manifest)
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let manifest_path = crate::filesystem::paths::validate_relative_path(std::path::Path::new(
        ".rustrs/config.toml",
    ))
    .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    operations.push(PlannedOperation {
        relative_path: manifest_path,
        contents: manifest_contents.into_bytes(),
        description: "update .rustrs/config.toml".to_string(),
        overwrite: true,
    });

    let plan = GenerationPlan {
        target_root: root,
        operations,
    };

    let policy = if args.force {
        WritePolicy::Overwrite
    } else {
        WritePolicy::PreserveExisting
    };

    if args.dry_run {
        let report = crate::filesystem::report::preview_plan(&plan);
        print_report(output, &report);
        return Ok(());
    }

    let report = apply_plan(&plan, policy);
    print_report(output, &report);

    let mut failed = false;
    for result in &report.results {
        if matches!(result.outcome, OperationOutcome::Failed { .. }) {
            failed = true;
        }
    }
    if failed {
        return Err(CliError::InvalidInput(
            "asset generation failed".to_string(),
        ));
    }

    if !output.quiet {
        output.info("Asset generation completed.");
    }
    Ok(())
}

fn add_file(
    operations: &mut Vec<PlannedOperation>,
    relative: &str,
    contents: String,
) -> Result<(), CliError> {
    let path = crate::filesystem::paths::validate_relative_path(std::path::Path::new(relative))
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    operations.push(PlannedOperation {
        relative_path: path,
        contents: contents.into_bytes(),
        description: format!("generate {}", relative),
        overwrite: false,
    });
    Ok(())
}

fn compose_content(manifest: &ProjectManifest) -> String {
    let mut services = String::new();
    services.push_str("services:\n");
    services.push_str("  app:\n");
    services.push_str("    build: .\n");
    services.push_str("    ports:\n");
    services.push_str("      - \"3000:3000\"\n");
    services.push_str("    environment:\n");
    services.push_str("      DATABASE_URL: ${DATABASE_URL}\n");

    if manifest.database == Database::Postgres {
        services.push_str("  db:\n");
        services.push_str("    image: postgres:16\n");
        services.push_str("    environment:\n");
        services.push_str("      POSTGRES_USER: ${POSTGRES_USER:-postgres}\n");
        services.push_str("      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}\n");
        services.push_str("      POSTGRES_DB: ${POSTGRES_DB:-app}\n");
        services.push_str("    ports:\n");
        services.push_str("      - \"5432:5432\"\n");
        services.push_str("    volumes:\n");
        services.push_str("      - postgres_data:/var/lib/postgresql/data\n");
    }

    if manifest.features.redis {
        services.push_str("  redis:\n");
        services.push_str("    image: redis:7\n");
        services.push_str("    ports:\n");
        services.push_str("      - \"6379:6379\"\n");
    }

    if manifest.database == Database::Postgres {
        services.push_str("\nvolumes:\n");
        services.push_str("  postgres_data:\n");
    }

    services
}

fn print_report(output: &crate::cli::Output, report: &crate::generation::plan::OperationReport) {
    for result in &report.results {
        let line = match &result.outcome {
            OperationOutcome::Planned => format!("  PLAN   {}", result.relative_path.display()),
            OperationOutcome::Created => format!("  CREATE {}", result.relative_path.display()),
            OperationOutcome::Skipped { reason } => {
                format!("  SKIP   {} ({})", result.relative_path.display(), reason)
            }
            OperationOutcome::Failed { error } => {
                format!("  FAIL   {} ({})", result.relative_path.display(), error)
            }
        };
        if !output.quiet {
            output.info(&line);
        }
    }
}

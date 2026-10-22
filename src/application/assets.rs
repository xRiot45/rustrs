use crate::cli::Output;
use crate::cli::{CiProvider, GenerateArgs};
use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, OperationOutcome, PlannedOperation, WritePolicy};
use crate::manifest::discovery::discover_manifest;
use crate::manifest::model::{Capability, Database, ProjectManifest};
use std::path::Path;

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
    output: &Output,
) -> Result<(), CliError> {
    let start = args
        .path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let discovered = discover_manifest(&start)?;
    discovered.manifest.validate()?;
    crate::compatibility::check_compatibility(&discovered.manifest)?;

    let root = discovered.project_root.clone();
    let mut manifest = discovered.manifest.clone();

    match kind {
        crate::generator::kind::GeneratorKind::Docker => {
            generate_docker_assets(&root, &mut manifest, args.force, args.dry_run, output)?;
        }
        crate::generator::kind::GeneratorKind::Compose => {
            manifest.features.docker = true;
            manifest.validate()?;
            crate::compatibility::check_compatibility(&manifest)?;

            let compose = compose_content(&manifest);
            let mut operations = Vec::new();
            add_file(&mut operations, "docker-compose.yml", compose)?;
            write_manifest_operation(&mut operations, &manifest)?;
            let plan = GenerationPlan {
                target_root: root,
                operations,
            };
            apply_generated_plan(&plan, args.force, args.dry_run, output)?;
        }
        crate::generator::kind::GeneratorKind::CiGithub => {
            generate_ci_assets(
                &root,
                &mut manifest,
                CiProvider::Github,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        crate::generator::kind::GeneratorKind::CiGitlab => {
            generate_ci_assets(
                &root,
                &mut manifest,
                CiProvider::Gitlab,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        crate::generator::kind::GeneratorKind::Release => {
            manifest.features.cicd = true;
            manifest.validate()?;
            crate::compatibility::check_compatibility(&manifest)?;

            let mut operations = Vec::new();
            add_file(
                &mut operations,
                ".github/workflows/release.yml",
                RELEASE_WORKFLOW.to_string(),
            )?;
            write_manifest_operation(&mut operations, &manifest)?;
            let plan = GenerationPlan {
                target_root: root,
                operations,
            };
            apply_generated_plan(&plan, args.force, args.dry_run, output)?;
        }
        _ => {
            return Err(CliError::InvalidInput(
                "unsupported asset generator".to_string(),
            ));
        }
    }

    if !output.quiet && !args.dry_run {
        output.info("Asset generation completed.");
    }
    Ok(())
}

pub fn generate_docker_assets(
    root: &Path,
    manifest: &mut ProjectManifest,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError> {
    manifest.features.docker = true;
    manifest.validate()?;
    crate::compatibility::check_compatibility(manifest)?;

    let binary = manifest.project_name.clone();
    let mut operations = Vec::new();
    let dockerfile = DOCKERFILE.replace("{binary}", &binary);
    add_file(&mut operations, "Dockerfile", dockerfile)?;
    add_file(&mut operations, ".dockerignore", DOCKERIGNORE.to_string())?;
    write_manifest_operation(&mut operations, manifest)?;

    let plan = GenerationPlan {
        target_root: root.to_path_buf(),
        operations,
    };
    apply_generated_plan(&plan, force, dry_run, output)
}

pub fn generate_ci_assets(
    root: &Path,
    manifest: &mut ProjectManifest,
    provider: CiProvider,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError> {
    manifest.features.cicd = true;
    manifest.validate()?;
    crate::compatibility::check_compatibility(manifest)?;

    let mut operations = Vec::new();
    match provider {
        CiProvider::Github => add_file(
            &mut operations,
            ".github/workflows/ci.yml",
            GITHUB_CI.to_string(),
        )?,
        CiProvider::Gitlab => add_file(&mut operations, ".gitlab-ci.yml", GITLAB_CI.to_string())?,
    }
    write_manifest_operation(&mut operations, manifest)?;

    let plan = GenerationPlan {
        target_root: root.to_path_buf(),
        operations,
    };
    apply_generated_plan(&plan, force, dry_run, output)
}

pub fn generate_auth_assets(
    root: &Path,
    manifest: &mut ProjectManifest,
    enable_rbac: bool,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError> {
    if !matches!(
        manifest.template,
        crate::manifest::model::TemplateId::Standard
            | crate::manifest::model::TemplateId::Enterprise
    ) {
        return Err(CliError::InvalidInput(
            "auth is only supported on standard and enterprise templates in the current MVP"
                .to_string(),
        ));
    }

    manifest.features.auth = true;
    if enable_rbac {
        manifest.features.rbac = true;
    }
    manifest.validate()?;
    crate::compatibility::check_compatibility(manifest)?;

    let mut operations = Vec::new();
    add_file(
        &mut operations,
        "src/modules/auth.rs",
        crate::template::metadata::AUTH_MODULE_RS.to_string(),
    )?;
    add_file(
        &mut operations,
        "migrations/0001_create_auth_tables.sql",
        crate::template::metadata::AUTH_MIGRATION_SQL.to_string(),
    )?;

    let modules_path = "src/modules/mod.rs";
    let modules_relative = validate_relative_path(std::path::Path::new(modules_path))
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let existing_modules = std::fs::read_to_string(root.join(modules_path)).ok();
    let (modules_begin, modules_end) = crate::generator::registration::marker_pair("modules");
    let updated_modules = crate::generator::registration::insert_between_markers(
        existing_modules.as_deref(),
        &modules_begin,
        &modules_end,
        "pub mod auth;",
    )?;
    operations.push(PlannedOperation {
        relative_path: modules_relative.clone(),
        contents: updated_modules.clone().into_bytes(),
        description: "register auth module".to_string(),
        overwrite: true,
    });

    match manifest.template {
        crate::manifest::model::TemplateId::Standard => {
            let (routes_begin, routes_end) = crate::generator::registration::marker_pair("routes");
            let updated_routes = crate::generator::registration::insert_between_markers(
                Some(&updated_modules),
                &routes_begin,
                &routes_end,
                ".nest(\"/auth\", auth::router())",
            )?;
            // Avoid duplicate operation for same file: replace previous modules operation contents.
            if let Some(last) = operations.last_mut() {
                if last.relative_path == std::path::Path::new("src/modules/mod.rs") {
                    last.contents = updated_routes.into_bytes();
                    last.description = "register auth module and routes".to_string();
                }
            }
        }
        crate::manifest::model::TemplateId::Enterprise => {
            let http_path = "src/interfaces/http.rs";
            let http_relative = validate_relative_path(std::path::Path::new(http_path))
                .map_err(|error| CliError::InvalidInput(error.to_string()))?;
            let existing_http = std::fs::read_to_string(root.join(http_path)).ok();
            let (http_begin, http_end) = crate::generator::registration::marker_pair("routes");
            let updated_http = crate::generator::registration::insert_between_markers(
                existing_http.as_deref(),
                &http_begin,
                &http_end,
                ".nest(\"/auth\", crate::modules::auth::router())",
            )?;
            operations.push(PlannedOperation {
                relative_path: http_relative,
                contents: updated_http.into_bytes(),
                description: "register auth routes".to_string(),
                overwrite: true,
            });

            let modules_path = "src/modules/mod.rs";
            let modules_relative = validate_relative_path(std::path::Path::new(modules_path))
                .map_err(|error| CliError::InvalidInput(error.to_string()))?;
            let existing_modules = std::fs::read_to_string(root.join(modules_path)).ok();
            let (modules_begin, modules_end) =
                crate::generator::registration::marker_pair("modules");
            let updated_modules = crate::generator::registration::insert_between_markers(
                existing_modules.as_deref(),
                &modules_begin,
                &modules_end,
                "pub mod auth;",
            )?;
            // Replace earlier modules operation to avoid duplicate path writes.
            if let Some(op) = operations
                .iter_mut()
                .find(|op| op.relative_path == std::path::Path::new("src/modules/mod.rs"))
            {
                op.contents = updated_modules.into_bytes();
            } else {
                operations.push(PlannedOperation {
                    relative_path: modules_relative,
                    contents: updated_modules.into_bytes(),
                    description: "register auth module".to_string(),
                    overwrite: true,
                });
            }
        }
        _ => {}
    }

    write_manifest_operation(&mut operations, manifest)?;
    let plan = GenerationPlan {
        target_root: root.to_path_buf(),
        operations,
    };
    apply_generated_plan(&plan, force, dry_run, output)
}

pub fn generate_openapi_assets(
    root: &Path,
    manifest: &mut ProjectManifest,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError> {
    if !matches!(
        manifest.template,
        crate::manifest::model::TemplateId::Standard
            | crate::manifest::model::TemplateId::Enterprise
    ) {
        return Err(CliError::InvalidInput(
            "openapi is only supported on standard and enterprise templates in the current MVP"
                .to_string(),
        ));
    }

    manifest.features.openapi = true;
    manifest.validate()?;
    crate::compatibility::check_compatibility(manifest)?;

    let mut operations = Vec::new();
    add_file(
        &mut operations,
        "src/modules/openapi.rs",
        crate::template::metadata::OPENAPI_MODULE_RS.to_string(),
    )?;

    let modules_path = "src/modules/mod.rs";
    let existing_modules = std::fs::read_to_string(root.join(modules_path)).ok();
    let (modules_begin, modules_end) = crate::generator::registration::marker_pair("modules");
    let updated_modules = crate::generator::registration::insert_between_markers(
        existing_modules.as_deref(),
        &modules_begin,
        &modules_end,
        "pub mod openapi;",
    )?;

    match manifest.template {
        crate::manifest::model::TemplateId::Standard => {
            let (routes_begin, routes_end) = crate::generator::registration::marker_pair("routes");
            let updated_routes = crate::generator::registration::insert_between_markers(
                Some(&updated_modules),
                &routes_begin,
                &routes_end,
                ".nest(\"/docs\", openapi::router())",
            )?;
            let modules_relative = validate_relative_path(std::path::Path::new(modules_path))
                .map_err(|error| CliError::InvalidInput(error.to_string()))?;
            operations.retain(|op| op.relative_path != std::path::Path::new("src/modules/mod.rs"));
            operations.push(PlannedOperation {
                relative_path: modules_relative,
                contents: updated_routes.into_bytes(),
                description: "register openapi module and routes".to_string(),
                overwrite: true,
            });
        }
        crate::manifest::model::TemplateId::Enterprise => {
            let modules_relative = validate_relative_path(std::path::Path::new(modules_path))
                .map_err(|error| CliError::InvalidInput(error.to_string()))?;
            operations.retain(|op| op.relative_path != std::path::Path::new("src/modules/mod.rs"));
            operations.push(PlannedOperation {
                relative_path: modules_relative,
                contents: updated_modules.into_bytes(),
                description: "register openapi module".to_string(),
                overwrite: true,
            });

            let http_path = "src/interfaces/http.rs";
            let http_relative = validate_relative_path(std::path::Path::new(http_path))
                .map_err(|error| CliError::InvalidInput(error.to_string()))?;
            let existing_http = std::fs::read_to_string(root.join(http_path)).ok();
            let (http_begin, http_end) = crate::generator::registration::marker_pair("routes");
            let updated_http = crate::generator::registration::insert_between_markers(
                existing_http.as_deref(),
                &http_begin,
                &http_end,
                ".nest(\"/docs\", crate::modules::openapi::router())",
            )?;
            operations.push(PlannedOperation {
                relative_path: http_relative,
                contents: updated_http.into_bytes(),
                description: "register openapi routes".to_string(),
                overwrite: true,
            });
        }
        _ => {}
    }

    write_manifest_operation(&mut operations, manifest)?;
    let plan = GenerationPlan {
        target_root: root.to_path_buf(),
        operations,
    };
    apply_generated_plan(&plan, force, dry_run, output)
}

pub fn generate_redis_assets(
    root: &Path,
    manifest: &mut ProjectManifest,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError> {
    if !matches!(
        manifest.template,
        crate::manifest::model::TemplateId::Standard
            | crate::manifest::model::TemplateId::Enterprise
    ) {
        return Err(CliError::InvalidInput(
            "redis is only supported on standard and enterprise templates in the current MVP"
                .to_string(),
        ));
    }

    manifest.features.redis = true;
    manifest.validate()?;
    crate::compatibility::check_compatibility(manifest)?;

    let mut operations = Vec::new();
    add_file(
        &mut operations,
        "src/modules/redis.rs",
        crate::template::metadata::REDIS_MODULE_RS.to_string(),
    )?;

    let modules_path = "src/modules/mod.rs";
    let modules_relative = validate_relative_path(std::path::Path::new(modules_path))
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let existing_modules = std::fs::read_to_string(root.join(modules_path)).ok();
    let (modules_begin, modules_end) = crate::generator::registration::marker_pair("modules");
    let updated_modules = crate::generator::registration::insert_between_markers(
        existing_modules.as_deref(),
        &modules_begin,
        &modules_end,
        "pub mod redis;",
    )?;
    operations.push(PlannedOperation {
        relative_path: modules_relative,
        contents: updated_modules.into_bytes(),
        description: "register redis module".to_string(),
        overwrite: true,
    });

    write_manifest_operation(&mut operations, manifest)?;
    let plan = GenerationPlan {
        target_root: root.to_path_buf(),
        operations,
    };
    apply_generated_plan(&plan, force, dry_run, output)
}

#[allow(clippy::too_many_arguments)]
pub fn generate_simple_capability_assets(
    root: &Path,
    manifest: &mut ProjectManifest,
    capability: Capability,
    module_name: &str,
    source: &str,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError> {
    if !matches!(
        manifest.template,
        crate::manifest::model::TemplateId::Standard
            | crate::manifest::model::TemplateId::Enterprise
    ) {
        return Err(CliError::InvalidInput(format!(
            "{} is only supported on standard and enterprise templates in the current MVP",
            capability
        )));
    }

    match capability {
        Capability::Email => manifest.features.email = true,
        Capability::Storage => manifest.features.storage = true,
        Capability::AuditLog => manifest.features.audit_log = true,
        Capability::RateLimit => manifest.features.rate_limit = true,
        Capability::Idempotency => manifest.features.idempotency = true,
        Capability::Observability => manifest.features.observability = true,
        _ => {}
    }
    manifest.validate()?;
    crate::compatibility::check_compatibility(manifest)?;

    let module_path = format!("src/modules/{module_name}.rs");
    let mut operations = Vec::new();
    add_file(&mut operations, &module_path, source.to_string())?;

    let modules_path = "src/modules/mod.rs";
    let modules_relative = validate_relative_path(std::path::Path::new(modules_path))
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let existing_modules = std::fs::read_to_string(root.join(modules_path)).ok();
    let (modules_begin, modules_end) = crate::generator::registration::marker_pair("modules");
    let updated_modules = crate::generator::registration::insert_between_markers(
        existing_modules.as_deref(),
        &modules_begin,
        &modules_end,
        &format!("pub mod {module_name};"),
    )?;
    operations.push(PlannedOperation {
        relative_path: modules_relative,
        contents: updated_modules.into_bytes(),
        description: format!("register {module_name} module"),
        overwrite: true,
    });

    if capability == Capability::AuditLog {
        add_file(
            &mut operations,
            "migrations/0003_create_audit_logs.sql",
            crate::template::metadata::AUDIT_LOG_MIGRATION_SQL.to_string(),
        )?;
    }

    write_manifest_operation(&mut operations, manifest)?;
    let plan = GenerationPlan {
        target_root: root.to_path_buf(),
        operations,
    };
    apply_generated_plan(&plan, force, dry_run, output)
}

fn write_manifest_operation(
    operations: &mut Vec<PlannedOperation>,
    manifest: &ProjectManifest,
) -> Result<(), CliError> {
    let manifest_contents = toml::to_string_pretty(manifest)
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let manifest_path = validate_relative_path(std::path::Path::new(".rustrs/config.toml"))
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    operations.push(PlannedOperation {
        relative_path: manifest_path,
        contents: manifest_contents.into_bytes(),
        description: "update .rustrs/config.toml".to_string(),
        overwrite: true,
    });
    Ok(())
}

fn add_file(
    operations: &mut Vec<PlannedOperation>,
    relative: &str,
    contents: String,
) -> Result<(), CliError> {
    let path = validate_relative_path(std::path::Path::new(relative))
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    operations.push(PlannedOperation {
        relative_path: path,
        contents: contents.into_bytes(),
        description: format!("generate {}", relative),
        overwrite: false,
    });
    Ok(())
}

fn apply_generated_plan(
    plan: &GenerationPlan,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError> {
    if dry_run {
        let report = crate::filesystem::report::preview_plan(plan);
        print_report(output, &report);
        return Ok(());
    }

    let policy = if force {
        WritePolicy::Overwrite
    } else {
        WritePolicy::PreserveExisting
    };
    let report = crate::filesystem::apply_plan(plan, policy);
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

fn print_report(output: &Output, report: &crate::generation::plan::OperationReport) {
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

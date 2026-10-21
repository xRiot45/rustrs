# Support Matrix — rustrs MVP

## CLI and generated project support

| Area | Supported in MVP |
| --- | --- |
| CLI platform verified | Linux |
| CLI platform unverified | macOS, Windows |
| Rust edition | 2024 |
| MSRV | 1.85.0 |
| Template | minimal, simple, standard, enterprise |
| Framework | Axum |
| Database | PostgreSQL |
| Data access | SQLx |
| Architecture | clean_modular |

## Capability status

| Capability | Status |
| --- | --- |
| auth | Declared in manifest; not fully generated in starter templates |
| rbac | Declared; requires auth |
| openapi | Manifest feature and `rustrs docs` validation only; runtime OpenAPI code is not generated |
| redis | Manifest feature only; no generated integration |
| email | Manifest feature only; no generated integration |
| storage | Manifest feature only; no generated integration |
| audit_log | Manifest feature only |
| rate_limit | Manifest feature only |
| idempotency | Manifest feature only |
| observability | Manifest feature only |
| docker | Asset generation available via `rustrs generate docker` |
| cicd | Asset generation available via `rustrs generate ci-github` / `ci-gitlab` |

## Explicitly unsupported or deferred

- Actix Web and Rocket.
- MySQL, SQLite, and `None` database workflows.
- SeaORM and Diesel.
- `rustrs add <capability>`.
- `rustrs db seed`.
- Database rollback for up-only generated migrations.
- macOS and Windows runtime verification.
- Prebuilt binary distribution (source/Cargo install is the verified MVP route).
EOF
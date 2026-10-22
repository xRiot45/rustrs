# Support Matrix — rustrs MVP

## CLI and generated project support

| Area | Supported in MVP |
| --- | --- |
| CLI platform verified | Linux |
| CLI platform unverified | macOS, Windows |
| Rust edition | 2024 |
| MSRV | 1.85.0 |
| Template | minimal, simple, standard, enterprise |
| Framework | Axum, Actix Web, Rocket |
| Database | PostgreSQL, MySQL, SQLite, None |
| Data access | SQLx (full); SeaORM and Diesel (minimal repository placeholders); None (no database) |
| Architecture | clean_modular |

## Capability status

| Capability | Status |
| --- | --- |
| auth | Generated for `enterprise` and `standard` templates; `rustrs add auth` supported on those templates |
| rbac | Permission-based RBAC generated with auth; `rustrs add rbac` supported on `enterprise` and `standard` |
| openapi | Runtime scaffolding available via `rustrs add openapi` on `standard` and `enterprise` |
| redis | Client, cache, and rate limiting available via `rustrs add redis` on `standard` and `enterprise` |
| email | Generated for `enterprise` and `standard` templates; `rustrs add email` supported on those templates |
| storage | Generated for `enterprise` and `standard` templates; `rustrs add storage` supported on those templates |
| audit_log | Generated for `enterprise` and `standard` templates; `rustrs add audit_log` supported on those templates |
| rate_limit | Generated for `enterprise` and `standard` templates; `rustrs add rate_limit` supported on those templates |
| idempotency | Generated for `enterprise` and `standard` templates; `rustrs add idempotency` supported on those templates |
| observability | Generated for `enterprise` and `standard` templates; `rustrs add observability` supported on those templates |
| docker | Asset generation available via `rustrs generate docker` and `rustrs add docker` |
| cicd | Asset generation available via `rustrs generate ci-github` / `ci-gitlab` and `rustrs add cicd` |

## Explicitly unsupported or deferred

- SeaORM and Diesel repository generation.
- `rustrs db seed` requires a reachable PostgreSQL database and runs the generated seeder module.
- Database rollback uses paired down migration files; review generated down SQL before use.
- macOS and Windows runtime verification.
- Prebuilt binary distribution (source/Cargo install is the verified MVP route).
EOF
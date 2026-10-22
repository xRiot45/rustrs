# Changelog

## 0.1.0

### Added

- CLI foundation, manifest, compatibility, template engine, and safe filesystem operations.
- Project scaffolding for `minimal`, `simple`, `standard`, and `enterprise` templates.
- Code generators for module, entity, dto, service, repository, controller, handler, middleware, guard, validator, test, migration, event, event-handler, job, and resource.
- Project inspection and diagnostics: `info`, `modules`, `routes`, `deps`, `doctor`, `config`, and `env check`.
- Database workflows: create migration, migrate, rollback, status, and seed.
- Development workflows: `dev`, `dev --watch`, `dev up`, `dev down`, and `dev logs`.
- Quality commands: `check`, `lint`, `test`, `format`, and `audit`.
- OpenAPI runtime scaffolding and Scalar UI.
- Redis client, cache, and rate limiting.
- Capability management via `rustrs add`.
- Docker, Compose, CI/CD, and release workflow asset generation.
- Shell completion.
- Source/Cargo installation route.

### Changed

- Expanded framework support to Actix Web and Rocket for scaffold generation.
- Expanded database support to MySQL, SQLite, and None for scaffold generation.
- Expanded data access declarations to SeaORM, Diesel, and None.
- Generated migrations now include paired down files.
- Database rollback uses explicit confirmation and dry-run.

### Known limitations

- SeaORM and Diesel repository/migration generation are not yet implemented.
- Auth, RBAC, and audit log are supported only on `Axum + PostgreSQL + SQLx`.
- macOS and Windows are not verified.
- Prebuilt binary distribution is not provided.
- `rustrs update check` requires network access.

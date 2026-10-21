# Changelog

## 0.1.0

### Added
- Project scaffolding for `minimal`, `simple`, `standard`, and `enterprise` templates.
- Code generators for module, entity, DTO, service, repository, controller, handler, middleware, guard, validator, test, migration, job, event, event handler, and resource.
- Project inspection commands: `info`, `modules`, `routes`, `deps`, `doctor`.
- Database workflow: `db create-migration`, `db migrate`, `db rollback`, `db status`, `db seed`.
- Development workflow: `dev`, `dev --watch`, `dev up`, `dev down`, `dev logs`.
- Quality commands: `check`, `lint`, `test`, `format`, `audit`.
- OpenAPI configuration helper: `docs`.
- Shell completion: `completion`.
- Delivery asset generation: `generate docker`, `generate compose`, `generate ci-github`, `generate ci-gitlab`, `generate release`.

### Known limitations
- Only `Axum + PostgreSQL + SQLx` is supported.
- macOS and Windows are not verified.
- `db seed` is unsupported.
- Generated migrations are up-only by default.
- OpenAPI helper validates configuration only.
- `audit` requires `cargo-audit`.
EOF
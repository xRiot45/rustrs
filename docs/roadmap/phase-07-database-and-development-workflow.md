# Phase 7 — Database and Development Workflow

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 7 — Database and Development Workflow  
> **Recommended repository path:** `docs/roadmap/phase-07-database-and-development-workflow.md`  
> **Status:** Planned  
> **Primary outcome:** Provide reliable CLI workflows for supported database migration/seeding operations and local application development, backed by safe process execution and explicit prerequisite detection.

---

## 1. Purpose

Phase 7 expands `rustrs` from a project scaffolding, code generation, inspection, and diagnostics CLI into a practical local development tool.

The phase covers two connected areas:

1. **Database workflow:** create migration files, run migrations, roll back migrations where supported, inspect migration status, and run seeders where supported.
2. **Development workflow:** run the generated application, optionally restart it when source files change, manage a development process where safely supported, inspect its logs, and detect Docker Compose configuration/tool availability.

The intended user experience includes commands such as:

```bash
rustrs db create-migration create_products
rustrs db migrate
rustrs db rollback
rustrs db status
rustrs db seed

rustrs dev
rustrs dev --watch
rustrs dev up
rustrs dev down
rustrs dev logs
```

These are target UX examples, not proof that the commands or flags already exist. The implementation must follow actual CLI conventions and support only database engines, migration tools, templates, and process-management behaviors verified in the current codebase.

**Core principle:** run explicit, predictable development operations. Never hide consequential database operations, misrepresent a process as running, or mutate unrelated project files.

## 2. Objectives

1. Audit the actual Phase 0–6 implementation before changing code.
2. Define a shared, testable external-process execution abstraction.
3. Detect required tools and validate prerequisites before executing operations.
4. Provide `rustrs db create-migration` for supported migration formats.
5. Provide `rustrs db migrate`, `rollback`, `status`, and `seed` only where the project's actual migration/seeding mechanism supports them.
6. Provide `rustrs dev` to run the generated application.
7. Provide `rustrs dev --watch` where a safe and tested watcher implementation is feasible.
8. Provide `dev up`, `dev down`, and `dev logs` only if lifecycle management can be implemented reliably for supported platforms.
9. Detect Docker Compose configuration and tool availability without unexpectedly starting containers.
10. Preserve secret confidentiality and correctly propagate output, exit status, cancellation, and process cleanup.
11. Add meaningful unit, integration, CLI, and regression tests.
12. Document implemented behavior and report unsupported capabilities honestly.

---

## 3. Scope boundaries

### 3.1 In scope

- Database command hierarchy and shared orchestration.
- Migration file creation through the existing template/filesystem engine.
- Migration execution/status/rollback using the migration engine actually supported by the project.
- Seeder execution only if a real seeding mechanism exists.
- External process execution, output forwarding, exit-code mapping, and cancellation handling.
- Tool discovery and prerequisite checks.
- Running the generated application locally.
- File watch/restart behavior where supported.
- Optional managed process lifecycle and logs, subject to safe cross-platform design.
- Docker Compose file/tool detection.
- Configuration validation and secret redaction for these workflows.
- Tests, documentation, and quality gates.

### 3.2 Out of scope

Do not implement the following as Phase 7 work:

- New database or ORM support not already approved by the compatibility matrix.
- A new general-purpose ORM or migration engine.
- Automatic production database administration or destructive maintenance workflows.
- A database GUI, dashboard, or remote database-management service.
- Automatic schema design or migration generation from arbitrary Rust structs.
- Automatic database migrations when `rustrs dev` starts, unless an explicit product requirement already exists and is implemented as an opt-in behavior. Default behavior must not run hidden migrations.
- Automatic seeding of unknown data.
- Dockerfile or Docker Compose file generation (Phase 8).
- Broad CI/CD, GitHub Actions, GitLab CI, or release workflow generation (Phase 8).
- General `check`, `lint`, `test`, `format`, or `audit` command suite (Phase 8).
- Release packaging, publishing, version tagging, or deployment (Phase 9).
- Unrelated refactoring or speculative abstractions for future features.

**Boundary rule:** implement the smallest reliable workflow supported by the current project architecture. Do not add features simply because they appear in example command syntax.

---

## 4. Mandatory preflight

### Task 7.0 — Audit the current repository and verify Phase 6

Before implementation, inspect the actual working tree and source. Do not rely only on README status or a previous agent's report.

#### Required actions

1. Read `AGENTS.md`, `PRD.md`, `ARCHITECTURE.md`, and roadmap documents for Phases 0–6.
2. Run `git status --short`; preserve existing changes and untracked user files.
3. Inspect CLI structure, global flags, output/error handling, and exit-code conventions.
4. Verify which Phase 6 commands actually work, rather than checking only whether their names exist.
5. Inspect project root discovery and `.rustrs/config.toml` parsing/validation.
6. Inspect compatibility and capability resolution.
7. Identify the exact database/framework/data-access combinations implemented today.
8. Inspect generated project fixtures and identify the actual migration and seeding conventions.
9. Inspect `Cargo.toml`, `Cargo.lock`, generated application entrypoints, and configuration loading.
10. Inspect existing process, filesystem, template, and test abstractions.
11. Determine the repository's MSRV and platform support from existing project policy.
12. Record blockers before making architectural changes.

#### Deliverables

- A concise preflight report.
- A map of reusable abstractions.
- A verified support matrix for database and development workflows.
- A list of missing prerequisites and blockers.

#### Rules

- Do not assume all databases or ORMs listed in the PRD are implemented.
- Do not assume SQLx migration support or seeding exists without verifying generated templates and dependencies.
- Do not treat a tool's presence as proof that the project is configured to use it.
- Do not rewrite prior phases by default.
- Make only minimal, backward-compatible prerequisite fixes where necessary.
- If a prerequisite requires substantial redesign, stop and report the blocker.

#### Acceptance criteria

- [ ] P7-AC-01: The current implementation, working tree, and Phase 6 functionality are audited.
- [ ] P7-AC-02: Supported database, migration, and application-run conventions are verified from source/templates.
- [ ] P7-AC-03: Existing user changes are preserved and blockers are reported honestly.

---

## 5. Shared command and error architecture

### Task 7.1 — Define database and development workflow contracts

Design command contracts that fit the existing CLI rather than creating a separate CLI architecture.

#### Requirements

- Use the existing CLI parser and output/error conventions.
- Reuse project discovery, manifest validation, compatibility resolution, and capability checks.
- Separate argument parsing, project/configuration validation, orchestration, process execution, and result reporting.
- Define consistent errors for invalid project, unsupported capability, invalid configuration, missing tool, permission failure, process failure, cancellation, and invalid command arguments.
- Validate that an operation is supported before performing side effects.
- Clearly distinguish local file operations from operations that affect a database or process.
- Avoid adding public abstractions unless they are needed by multiple workflows.
- Do not create a duplicate compatibility matrix or a parallel manifest model.

#### Acceptance criteria

- [ ] P7-AC-04: Database and development commands follow consistent CLI conventions.
- [ ] P7-AC-05: Unsupported operations fail before starting an external process or changing files.
- [ ] P7-AC-06: Errors are actionable and do not expose credentials.

---

## 6. External process execution

### Task 7.2 — Implement a shared process execution abstraction

This is a foundational task for database commands and development workflows.

#### Required capabilities

Implement only the capabilities needed by the supported commands, potentially including:

- Executable/program path.
- Argument list, passed as separate arguments.
- Working directory.
- Explicit environment additions/removals where required.
- Standard input policy.
- Standard output and standard error forwarding/capture.
- Exit status and signal/termination result.
- Cancellation and child-process cleanup.
- Process start failure handling.
- Bounded capture for diagnostic output where full streaming is not appropriate.

#### Safety requirements

- Do not construct shell command strings from user input or project configuration.
- Prefer direct executable invocation with separate arguments.
- Never execute arbitrary scripts or commands from project configuration without an explicit, documented feature contract.
- Avoid unbounded buffering for long-running application output.
- Do not log secret-bearing environment variables or full connection strings.
- Do not claim a process succeeded merely because process creation succeeded.
- Preserve the external tool's exit status when possible; map it consistently if the CLI must use its own status codes.
- Keep the process runner testable through an injectable executor or equivalent test seam, if appropriate to the current architecture.

#### Acceptance criteria

- [ ] P7-AC-07: Arguments are passed without unsafe shell interpolation.
- [ ] P7-AC-08: Output, failure, and exit status are handled consistently.
- [ ] P7-AC-09: Child processes are cleaned up when the command is cancelled or fails.
- [ ] P7-AC-10: Process execution can be tested without requiring a real database or Docker daemon.

---

## 7. Tool discovery and prerequisite validation

### Task 7.3 — Detect required tools and validate prerequisites

#### Requirements

- Check for a required executable only when a requested operation needs it.
- Use the project's verified migration mechanism to determine whether a separate CLI tool is required.
- Check `cargo`/`rustc` availability when relevant to running the application.
- Check Docker and Compose availability only for workflows that require them.
- Distinguish executable-not-found, executable-not-runnable, unsupported version, and operation-level failure.
- Compare versions only if a minimum version is explicitly defined by project policy.
- Provide a useful remediation message without automatically installing or updating tools.
- Reuse Phase 6 diagnostics where possible, avoiding conflicting detection logic.
- Do not leak PATH contents or environment secrets unnecessarily.

#### Acceptance criteria

- [ ] P7-AC-11: Required prerequisites are checked before the dependent operation begins.
- [ ] P7-AC-12: Missing tools produce accurate, actionable errors.
- [ ] P7-AC-13: Tool detection does not install software or mutate the user's environment.

---

## 8. Database workflow

### Task 7.4 — Create migration files

Target command:

```bash
rustrs db create-migration create_products
```

#### Requirements

1. Validate project root, manifest, and database capability.
2. Confirm that a migration format and destination directory exist for the supported project layout.
3. Validate the migration name; reject empty names, path separators, traversal, and invalid characters.
4. Follow the migration naming convention actually used by the project.
5. Generate a unique filename or identifier without silently overwriting existing migrations.
6. Use the existing template/rendering/filesystem infrastructure where possible.
7. Validate all destination paths and conflicts before writing.
8. Use a clear template for the supported migration engine only.
9. Report the exact created path.
10. Do not connect to the database or execute the migration during file creation.

#### Acceptance criteria

- [ ] P7-AC-14: Migration files follow the verified project convention.
- [ ] P7-AC-15: Invalid names, unsafe paths, and filename conflicts are rejected before writing.
- [ ] P7-AC-16: Migration creation does not connect to or change the database.

### Task 7.5 — Run pending migrations

Target command:

```bash
rustrs db migrate
```

#### Requirements

- Validate the project and supported database/migration capability.
- Verify required configuration is available without printing secret values.
- Invoke the migration mechanism that the generated project actually uses.
- Forward output and error information.
- Return a failure result if the migration tool or migration operation fails.
- Explain that partial migration success may occur before a later migration fails, when applicable.
- Do not claim all migrations succeeded merely because the tool started.
- Do not invent fallback migration behavior.
- Do not automatically run this command as a side effect of `rustrs dev`.

#### Acceptance criteria

- [ ] P7-AC-17: Migrations run through the supported migration mechanism.
- [ ] P7-AC-18: The result reflects the actual external operation status.
- [ ] P7-AC-19: Errors are actionable and credentials are redacted.

### Task 7.6 — Roll back migrations

Target command:

```bash
rustrs db rollback
```

#### Requirements

- Verify that the migration engine supports rollback and establish its exact semantics.
- Define the default rollback scope based on the supported tool, such as one migration or one batch.
- Expose a scope argument only if the underlying engine supports it and the behavior can be tested.
- Clearly show or describe the target operation before execution where appropriate.
- Use a confirmation prompt for destructive operations when consistent with the CLI's safety policy.
- Define safe non-interactive behavior; a non-interactive flag must not silently bypass destructive-operation protection.
- Forward errors and preserve operation status.
- Do not imply that rollback always restores deleted or modified application data.
- Do not fabricate a down migration if none exists.

#### Acceptance criteria

- [ ] P7-AC-20: Rollback is exposed only when the supported mechanism provides it.
- [ ] P7-AC-21: Scope and safety behavior are explicit and tested.
- [ ] P7-AC-22: Rollback failures and limitations are reported honestly.

### Task 7.7 — Show migration status

Target command:

```bash
rustrs db status
```

#### Requirements

- Identify available migration files using the project's actual convention.
- Read applied/pending status from an authoritative migration table or tool output when supported.
- Distinguish local migration files from migrations recorded as applied in the database.
- If the database cannot be reached, do not invent applied/pending status.
- Avoid schema changes or migration execution during a status request.
- Provide deterministic, readable output.
- Report unsupported status inspection clearly when the migration engine cannot provide authoritative state.

#### Acceptance criteria

- [ ] P7-AC-23: Migration status is based on a verifiable source.
- [ ] P7-AC-24: Local files and database-applied state are distinguished.
- [ ] P7-AC-25: Status inspection does not change the database.

### Task 7.8 — Run seeders

Target command:

```bash
rustrs db seed
```

#### Requirements

- Implement this command only if a real seeder mechanism exists in the generated project or supported toolchain.
- Validate required configuration before execution.
- Invoke the actual supported seeder entrypoint/tool.
- Forward output and exit status.
- Explain that seeding can insert or modify database records.
- Apply confirmation or environment safeguards only where the target environment can be identified reliably.
- Do not claim a seeder is idempotent unless its implementation and tests establish that property.
- Do not invent seed data or silently run seeding during another command.
- If no supported seeder exists, return a clear unsupported-capability result rather than a fake success.

#### Acceptance criteria

- [ ] P7-AC-26: Seeding is available only when an actual mechanism exists.
- [ ] P7-AC-27: Data-changing behavior is explicit.
- [ ] P7-AC-28: Seeder failures and unavailable capability are reported accurately.

---

## 9. Development workflow

### Task 7.9 — Run the application

Target command:

```bash
rustrs dev
```

#### Requirements

- Validate the project root and `Cargo.toml`.
- Run the generated application, not the `rustrs` CLI itself.
- Use the actual package/binary convention discovered during preflight.
- Set the correct working directory.
- Forward application stdout and stderr appropriately.
- Return the application's exit status when possible.
- Handle startup failure, application failure, and user cancellation.
- Ensure child processes do not remain unmanaged after the command exits.
- Do not run migrations, seeders, or Docker operations as hidden side effects.
- Avoid rebuilding or invoking more tooling than required by the chosen development command contract.

#### Acceptance criteria

- [ ] P7-AC-29: The intended application runs from the correct project root.
- [ ] P7-AC-30: Application output and exit status are handled correctly.
- [ ] P7-AC-31: Child processes are cleaned up on normal exit, error, and cancellation.

### Task 7.10 — Watch mode

Target command:

```bash
rustrs dev --watch
```

#### Requirements

- Use a maintained watcher implementation compatible with the repository's MSRV and supported platforms, if a new dependency is justified.
- Watch only relevant source/configuration paths.
- Ignore build output and other directories that would cause restart loops.
- Debounce rapid file events when necessary.
- Stop or replace the previous application process before starting a replacement.
- Report compile/startup failures without claiming the new process is healthy.
- Handle changes that occur during restart.
- Stop watcher and child process when the user cancels the command.
- Keep watcher logic testable without relying on timing-sensitive integration tests alone.
- If reliable cross-platform watching is not feasible within the current architecture, implement a smaller tested subset and document the limitation.

#### Acceptance criteria

- [ ] P7-AC-32: Relevant changes trigger the documented restart behavior.
- [ ] P7-AC-33: Build output does not trigger an infinite restart loop.
- [ ] P7-AC-34: Watcher and child process shut down cleanly.

### Task 7.11 — Managed development lifecycle

Target commands, only if safe lifecycle management is supported:

```bash
rustrs dev up
rustrs dev down
rustrs dev logs
```

#### Requirements

1. Define the lifecycle model and supported platforms.
2. Determine whether `up` starts a background process and what constitutes success.
3. If process metadata is needed, store the minimum information required, such as project root, PID, and log location.
4. Avoid persisting secrets in process state.
5. Distinguish an `rustrs`-managed process from an unrelated process using a similar name or port.
6. Prevent `down` from terminating an unrelated process.
7. Handle stale PID/state metadata and processes that have already exited.
8. Provide a documented log location and a clear behavior for `logs`.
9. Handle concurrent lifecycle commands or explicitly prevent unsafe concurrent operations.
10. Attempt graceful termination before forced termination where supported.
11. Ensure state and logs are scoped to the correct project and cleaned up according to a documented policy.
12. Do not use a global process registry unless the design requires it and its safety is demonstrated.

If safe, portable lifecycle management cannot be completed reliably, do not ship a misleading partial command. Implement only the lifecycle subset with proven behavior and report the remainder as blocked or deferred.

#### Acceptance criteria

- [ ] P7-AC-35: `dev up` reports success only when the managed process is confirmed to be running.
- [ ] P7-AC-36: `dev down` stops only the process managed for the intended project.
- [ ] P7-AC-37: `dev logs` reads the correct log source.
- [ ] P7-AC-38: Stale state, repeated commands, and concurrent operations are handled safely.

---

## 10. Docker Compose detection

### Task 7.12 — Detect Compose configuration and tool availability

#### Requirements

- Detect only the Compose filenames and locations that the repository chooses to support.
- Distinguish between a Compose file being present, the Docker CLI being available, and Compose being executable.
- Run version or validation checks only where appropriate and safe.
- Use the intended project root.
- Report unreadable or malformed configuration accurately if validation is performed.
- Do not generate Dockerfiles or Compose files in this phase; that belongs to Phase 8.
- Do not start or stop containers merely because a Compose file is found.
- If `dev up/down` explicitly integrates Compose, document the relationship and test the exact behavior.
- Do not infer that a database container is running from the presence of a Compose file.

#### Acceptance criteria

- [ ] P7-AC-39: Compose file detection is separate from tool-availability checks.
- [ ] P7-AC-40: Detection does not unexpectedly start, stop, or modify containers.
- [ ] P7-AC-41: Any Compose integration with managed development lifecycle is explicit and tested.

---

## 11. Configuration, environment, and secret safety

### Task 7.13 — Validate workflow configuration and protect secrets

#### Requirements

- Reuse the project's existing configuration and environment-loading conventions.
- Validate required database configuration before invoking database operations.
- Do not print raw database URLs, passwords, tokens, or secret-bearing environment values.
- Redact secrets from external tool errors when they may echo command arguments or environment values.
- Do not put credentials in command-line arguments if the supported tool provides a safer environment/configuration mechanism.
- Do not persist secrets in process metadata, logs, or diagnostic snapshots.
- Distinguish development configuration from other environments only when reliable information is available.
- Add safety confirmation for destructive database operations according to the established CLI policy.
- Never assume a database is non-production solely because its database name contains `dev`, `test`, or similar text.
- Keep all implicit behavior conservative and document side effects.

#### Acceptance criteria

- [ ] P7-AC-42: Secrets are redacted from output and error paths.
- [ ] P7-AC-43: Invalid or missing required configuration is rejected before side effects.
- [ ] P7-AC-44: Database operations target only the explicitly configured connection.
- [ ] P7-AC-45: Destructive operations have documented, testable safety behavior.

---

## 12. Testing strategy

### Task 7.14 — Add unit, integration, CLI, and regression tests

#### A. Process execution tests

- Executable found and started.
- Executable missing or not executable.
- Argument values containing spaces and shell metacharacters.
- Correct working directory.
- stdout/stderr forwarding.
- Successful and non-zero exit codes.
- Cancellation, termination, and cleanup.
- Large output without unbounded memory growth.
- Secret redaction.
- Fake executable injection or equivalent deterministic test seam.

#### B. Database workflow tests

- Migration-name validation and safe path handling.
- Unique naming and conflict behavior.
- No overwrite of existing migration files.
- Dry inspection of target paths before write.
- Missing migration tool.
- Invalid database configuration.
- Migration success and failure.
- Rollback support, scope, and confirmation behavior.
- Status when database is available and unavailable.
- Seeder support and failure behavior.
- No hidden migration or seed operation during `dev`.

Tests that would require a real database should be optional integration tests with explicit prerequisites. Unit/CLI tests must not require production credentials.

#### C. Development workflow tests

- Running the correct generated binary/package.
- Working-directory selection.
- Application startup failure and normal exit.
- Child process cleanup on cancellation.
- Watcher event filtering and debounce.
- Restart behavior and restart failure.
- No watch loop from build artifacts.
- Managed lifecycle start/stop/log behavior, if implemented.
- Stale PID/state and unrelated-process protection.
- Concurrent lifecycle invocation handling.

#### D. Docker Compose tests

- Compose file present and absent.
- Docker executable present and absent.
- Compose command available and unavailable.
- Invalid or unreadable Compose file, if validation is implemented.
- Detection does not start or stop containers.
- Integration with `dev up/down`, if implemented.

#### E. Filesystem and safety tests

- Paths with spaces.
- Invalid project path.
- Traversal attempts in migration names.
- Existing migration filename conflicts.
- No modification to unrelated files.
- No secret leakage in stdout/stderr or logs.

#### F. Regression tests

Verify that all existing functionality remains operational:

- CLI parsing, help, version, and global options.
- Manifest parsing and validation.
- Compatibility and capability resolution.
- Template rendering and filesystem safety.
- Project scaffolding (`rustrs new`).
- Implemented generators.
- Inspector and diagnostics commands.
- Existing unit, integration, and snapshot tests.

#### Acceptance criteria

- [ ] P7-AC-46: Process execution has deterministic tests for success and failure.
- [ ] P7-AC-47: Database commands have success, failure, and missing-prerequisite coverage.
- [ ] P7-AC-48: Process lifecycle and watcher cleanup are tested where implemented.
- [ ] P7-AC-49: Secret handling and filesystem safety are tested.
- [ ] P7-AC-50: Phase 1–6 regression tests pass, or all failures are explicitly reported.

---

## 13. Documentation and quality gates

### Task 7.15 — Document supported workflows and complete final audit

#### Required documentation

1. Document each database command that is actually implemented.
2. Explain the supported migration mechanism and migration file layout.
3. Document rollback semantics and limitations.
4. Explain whether a real seeder mechanism exists and how it is invoked.
5. Explain required database configuration without including real secrets.
6. Document `rustrs dev` and the actual application-run behavior.
7. Document watch mode, ignored paths, and restart behavior if supported.
8. Document managed process lifecycle, state files, log paths, and supported platforms if implemented.
9. Document Docker Compose detection and its limitations.
10. Document prerequisite tools and expected error behavior.
11. Update CLI help and README examples to match the implementation.
12. Do not mark Phase 8 or Phase 9 as complete.

#### Quality gates

Run the established repository checks. At minimum, attempt:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

Also run existing CLI/integration scripts discovered during preflight.

For every check that cannot run, report:

- The exact command.
- Why it could not run.
- Whether the issue is environmental, due to a missing prerequisite, or caused by code.
- What remains unverified.

Do not weaken lint settings, delete tests, or reduce assertions merely to obtain a passing result.

#### Acceptance criteria

- [ ] P7-AC-51: Documentation accurately reflects implemented behavior and support limits.
- [ ] P7-AC-52: Quality gates are run and results are reported truthfully.
- [ ] P7-AC-53: The final diff contains no unrelated Phase 8–9 work.

---

## 14. Recommended implementation order

Follow this order unless preflight identifies a concrete dependency that requires a justified adjustment.

1. **Preflight and support matrix** — Task 7.0.
2. **Command contracts and shared errors** — Task 7.1.
3. **Process execution abstraction** — Task 7.2.
4. **Tool discovery and prerequisite checks** — Task 7.3.
5. **Migration file creation** — Task 7.4.
6. **Migration execution** — Task 7.5.
7. **Rollback and status** — Tasks 7.6–7.7.
8. **Seeder execution** — Task 7.8, only if supported.
9. **Run application** — Task 7.9.
10. **Watch mode** — Task 7.10.
11. **Managed process lifecycle** — Task 7.11, only if it can be made safe and testable.
12. **Docker Compose detection** — Task 7.12.
13. **Configuration and secret safety audit** — Task 7.13, with protections implemented from the start.
14. **Tests and regression verification** — Task 7.14, continuously throughout implementation.
15. **Documentation and final quality gates** — Task 7.15.

Do not implement all command names before building the shared process foundation. Do not add lifecycle complexity before the foreground `dev` command is reliable.

---

## 15. Definition of Done

Phase 7 is complete only when all applicable criteria are satisfied:

- [ ] Actual Phase 6 behavior and supported project configurations have been verified.
- [ ] The external-process abstraction is safe, testable, and used by relevant commands.
- [ ] Required tools are checked before dependent operations.
- [ ] Migration creation validates names/paths and never silently overwrites existing files.
- [ ] Migration execution reports the actual tool outcome.
- [ ] Rollback behavior is supported by the actual migration engine and has clear safety semantics.
- [ ] Migration status distinguishes local files from database-applied state.
- [ ] Seed is implemented only if a real seeding mechanism exists.
- [ ] `rustrs dev` runs the intended application and cleans up child processes.
- [ ] Watch mode, if implemented, handles restarts and shutdown safely.
- [ ] Managed `dev up/down/logs`, if implemented, cannot accidentally control unrelated processes.
- [ ] Docker Compose detection is accurate and has no unexpected container side effects.
- [ ] Credentials are not leaked.
- [ ] Unit, integration, CLI, and regression tests pass, or every unresolved item is reported.
- [ ] Documentation matches actual implementation.
- [ ] Quality-gate results are recorded truthfully.
- [ ] No Phase 8–9 functionality has been implemented as scope creep.
- [ ] No commit, push, publish, deployment, or release occurs without explicit authorization.

Do not mark Phase 7 complete merely because commands appear in `--help`. Each implemented command must be verified through success and failure scenarios, with side effects and prerequisites documented.

---

## 16. Required final report from the AI agent

At completion, provide an evidence-based report containing:

1. **Phase status:** Complete / Partially complete / Blocked.
2. **Preflight findings:** Verified Phase 6 behavior, supported database/framework combinations, migration/seeder mechanism, and blockers.
3. **Commands implemented:** Syntax, behavior, prerequisites, supported platforms, and tested examples.
4. **Process execution:** Argument safety, output forwarding, exit-code behavior, cancellation, and cleanup evidence.
5. **Database workflow:** Migration creation, migrate, rollback, status, and seed behavior with known limitations.
6. **Development workflow:** Foreground run, watch mode, lifecycle management, and logs where implemented.
7. **Docker Compose:** Detection behavior and what is or is not executed.
8. **Security:** Secret redaction and destructive-operation safeguards.
9. **Files changed:** Grouped by CLI, process runner, database workflow, development workflow, tests, and documentation.
10. **Verification:** Exact commands run and their pass/fail results.
11. **Known limitations:** Unsupported migration/seeding modes, platform restrictions, or deferred lifecycle commands.
12. **Scope confirmation:** Explicit confirmation that Phase 8–9 work was not performed.
13. **Remaining work:** Phase 7 items only.

Do not describe intended behavior as implemented behavior. Include concise evidence where useful.

---

## 17. Final instruction to the agent

Implement **Phase 7 — Database and Development Workflow only**. Begin with the mandatory preflight audit and verify the real implementation from Phases 0–6. Reuse the existing CLI, manifest, capability, filesystem, and diagnostics infrastructure.

Build and test the process execution foundation before adding dependent commands. Implement only database operations supported by the actual migration/seeding mechanism. Keep destructive operations explicit, protect credentials, forward process results honestly, and ensure child processes are cleaned up.

Do not generate Docker/Compose files, implement the Phase 8 quality command suite, perform deployment/release work, or refactor unrelated components. If safe process lifecycle management or a database operation cannot be supported reliably, report the blocker rather than shipping a misleading command. Preserve all user changes. Do not commit, push, publish, deploy, or release without explicit authorization.

# AGENTS.md — AI Agent Instructions for `rustrs`

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Audience:** AI coding agents, coding assistants, and human contributors  
> **Primary purpose:** Define how work must be planned, implemented, verified, and reported in this repository.

This file defines the repository-wide rules for AI-assisted development. It does not replace product requirements, architecture decisions, or phase-specific acceptance criteria.

## 1. Required reading order

Before making changes, read these documents in order:

1. `PRD.md` — product scope, requirements, and roadmap.
2. `ARCHITECTURE.md` — technical boundaries and architecture decisions.
3. The active phase document, for example `docs/roadmap/phase-0-repository-bootstrap.md`.
4. Relevant source code, tests, and documentation for the area being changed.

If one of the documents is not present, do not pretend it was read. Continue with available sources and mention the missing document if it blocks safe implementation.

### Source-of-truth order

When instructions conflict, follow this priority:

1. Explicit instructions from the project owner.
2. `PRD.md` for product requirements.
3. `ARCHITECTURE.md` for architectural decisions.
4. The active phase document for phase scope and acceptance criteria.
5. This `AGENTS.md` for repository-wide development rules.
6. Existing implementation, when it does not conflict with the sources above.

If a real conflict remains, explain it and ask for clarification when it blocks safe progress. Do not silently invent a product or architecture decision.

---

## 2. Product context

`rustrs` is a Rust CLI intended to scaffold REST API projects and generate application code. Planned capabilities include:

- Project creation through `rustrs new`.
- Code generation through commands such as `rustrs generate resource products` (alias `g`).
- Enterprise API, Standard API, Simple API, and Minimal API templates.
- Framework, database, architecture, and capability selection.
- Project configuration, inspection, and diagnostics.
- Database and local development workflows.
- Quality checks, API documentation, Docker assets, CI/CD templates, and release distribution.

These are product goals, not proof that the features already exist. **Never describe a planned feature as implemented unless it actually works and has been verified.**

The Enterprise API template is the reference/superset application template. The CLI's own architecture and the architecture of generated API projects are separate concerns; do not confuse them.

---

## 3. Phase discipline

The project is implemented in phases. Before coding, identify the active phase and its explicit scope.

Expected roadmap:

1. Phase 0 — Repository Bootstrap.
2. Phase 1 — CLI Foundation.
3. Phase 2 — Manifest and Compatibility.
4. Phase 3 — Template and Filesystem Engine.
5. Phase 4 — Project Scaffolding.
6. Phase 5 — Core Code Generators.
7. Phase 6 — Project Inspector and Diagnostics.
8. Phase 7 — Database and Development Workflow.
9. Phase 8 — Quality, Docs, Docker, and CI/CD.
10. Phase 9 — Hardening and MVP Release.

### Rules

- Implement only the active phase unless a prerequisite defect blocks progress.
- Do not begin the next phase automatically after completing the current one.
- Do not create empty modules or speculative abstractions for future phases.
- Do not add a feature merely because it appears in the long-term PRD.
- If a task is outside scope, record it as deferred and explain why.
- A phase is complete only when its applicable acceptance criteria pass or any blocker is explicitly reported.

---

## 4. Repository safety

Before editing:

1. Inspect the repository structure.
2. Run `git status --short`.
3. Review relevant existing files.
4. Identify uncommitted changes and preserve them.

### Never do the following unless the project owner explicitly authorizes the action

- Run destructive Git commands such as `git reset --hard` or `git clean -fd`.
- Overwrite unrelated user changes.
- Delete project files merely because they appear unused without investigating them.
- Commit, push, publish, deploy, or create a release.
- Change global machine configuration.
- Run destructive database or filesystem operations.
- Add real credentials, tokens, passwords, or production connection strings to source code, documentation, logs, or fixtures.

Do not claim that the working tree is clean unless you checked it. Do not revert changes you did not create.

---

## 5. Rust and Cargo conventions

- Prefer stable Rust and the edition/MSRV policy documented in `ARCHITECTURE.md` and `Cargo.toml`.
- Use idiomatic Rust and standard library types when they are sufficient.
- Keep functions and modules cohesive, with clear responsibilities.
- Prefer explicit data flow and simple types over premature generic frameworks.
- Avoid unnecessary `unwrap()` and `expect()` in code paths that can encounter normal user or filesystem errors. In tests or places with a proven invariant, they may be appropriate.
- Propagate errors with useful context; do not silently discard errors.
- Avoid broad `allow` attributes that hide lint problems.
- Do not introduce `unsafe` unless there is a compelling, documented need and the project owner approves it.
- Do not add a dependency without a concrete requirement for the active phase.
- Check that new dependencies are maintained, compatible with the project's Rust version, and appropriately licensed.
- Keep `Cargo.lock` tracked for this binary CLI application.
- Do not add a workspace or split crates without an explicit architectural reason.
- Keep `main.rs` thin as the CLI evolves. Command parsing and orchestration must not turn it into a place for business logic.

### Dependency changes

For each meaningful new dependency:

1. Explain which active-phase requirement it serves.
2. Check whether the standard library or an existing dependency already covers the need.
3. Verify MSRV and platform compatibility.
4. Add tests for the behavior that uses it.
5. Update `Cargo.lock` and documentation where appropriate.

---

## 6. Architecture rules

Follow the boundaries described in `ARCHITECTURE.md`.

- **CLI layer:** parses commands/options and presents results; it does not implement generation logic.
- **Application layer:** orchestrates use cases and coordinates validation, planning, and execution.
- **Domain/policy layer:** owns naming rules, capability rules, compatibility checks, and decisions that should not require I/O.
- **Infrastructure layer:** handles file access, configuration persistence, template rendering, and external process execution.
- **Output/interaction:** presents results and gathers user input without embedding business rules.

These are logical boundaries. Do not create a separate file, trait, or crate for every concept when the current implementation does not need it.

### Specific rules

- Centralize framework/database/driver/template compatibility decisions.
- Do not duplicate capability rules across command handlers and templates.
- Keep template resolution, template rendering, generation planning, and filesystem writes separate.
- Pass validated configuration explicitly instead of relying on hidden global state.
- Use typed models for important configuration and domain choices.
- Update `ARCHITECTURE.md` when a lasting architectural decision changes.

---

## 7. CLI behavior and user experience

When implementing CLI commands:

- Use the CLI framework selected in `ARCHITECTURE.md` and the active phase.
- Keep command names, aliases, argument names, and flags consistent.
- Provide useful help and version output.
- Write primary results to stdout and diagnostics/errors to stderr.
- Return a non-zero exit code on failure.
- Provide actionable error messages that explain what failed and, where useful, how to fix it.
- Avoid panics for normal invalid input, missing dependencies, configuration errors, or filesystem failures.
- Do not hide useful error details or print internal backtraces by default.
- Do not claim success until the requested operation has completed.
- Support non-interactive use where required; do not unexpectedly wait for input in CI.
- Never make color or animation necessary to understand command output.

When changing the public CLI contract, update help text, tests, and relevant documentation.

---

## 8. Configuration and compatibility

When working on configuration:

- Parse configuration into typed structures.
- Validate the fully resolved configuration after loading defaults and overrides.
- Follow the precedence defined in `ARCHITECTURE.md`.
- Keep schema versions explicit.
- Reject unsupported or incompatible configurations with clear errors.
- Do not silently ignore a selected framework, database, driver, template, or capability.
- Do not store secrets in `.rustrs/config.toml`.
- Do not print secret values in errors or diagnostics.
- Preserve unrelated user-owned configuration when updating a manifest.
- Update tests and documentation when the configuration schema changes.

Only expose a combination in the CLI if the compatibility layer can generate a coherent result for it.

---

## 9. Template and code-generation safety

Code generation modifies user files. Treat it as a safety-critical feature.

### Required pipeline

Before applying generated changes:

1. Parse and normalize the request.
2. Resolve project configuration and capabilities.
3. Validate names and compatibility.
4. Build a generation plan.
5. Render required content in memory.
6. Validate target paths and file conflicts.
7. Show a preview when `--dry-run` is requested.
8. Apply the plan only when allowed.
9. Report created, modified, skipped, and failed operations accurately.

### Filesystem rules

- Keep generated paths relative to the selected project root.
- Reject absolute paths and parent-directory traversal.
- Consider symlinks and ensure writes cannot escape the intended project root.
- Validate paths before writing; do not trust template metadata blindly.
- Preserve existing user files by default.
- Define and test the semantics of `--force`; it must not mean “delete and recreate the project.”
- `--dry-run` must not modify the filesystem.
- Avoid partial writes where practical; use safe temporary/atomic writes when justified.
- Never delete arbitrary files as part of normal generation.
- If a multi-file operation partially fails, report what succeeded and what failed.
- Make repeated operations predictable and idempotent where the command contract allows it.

### Template rules

- Do not execute template content as code.
- Keep templates deterministic for a resolved configuration and template version.
- Validate template context before rendering.
- Report missing variables, invalid templates, and rendering failures clearly.
- Test optional capabilities and unsupported combinations.
- Do not silently ignore template errors.
- Keep templates and compatibility metadata consistent with the source-of-truth compatibility model.

### Generated code rules

- Centralize naming transformations and validation.
- Reject invalid names instead of producing malformed Rust.
- Handle reserved Rust keywords and edge cases according to documented policy.
- Do not assume naive singularization/pluralization works for all words.
- Test generated files for expected contents and compile representative generated projects where practical.

---

## 10. Security and external commands

- Treat CLI arguments, configuration files, existing project contents, and template metadata as untrusted input.
- Never interpolate user-controlled values into shell command strings.
- Use `std::process::Command` with separate arguments for external programs.
- Execute external programs only when the invoked command explicitly requires them.
- Check whether required executables are available and report missing prerequisites clearly.
- Preserve and report child-process exit statuses.
- Avoid unexpected network access during ordinary local generation.
- Do not run automatic repair or destructive actions during diagnostics unless the user explicitly requests them.
- Do not log environment dumps or secret values.
- Use fake placeholders in examples and test fixtures.
- Do not claim generated applications are production-secure without appropriate evidence and testing.

---

## 11. Testing requirements

Every behavior change must have proportionate verification.

### Unit tests

Prefer unit tests for pure logic, including:

- Name normalization and validation.
- Compatibility and capability resolution.
- Configuration validation.
- Template selection.
- Generation-plan creation.
- Conflict policy decisions.
- Error classification where callers rely on it.

### Integration tests

Use integration or process-level tests for:

- CLI help/version and argument parsing.
- Exit codes and stderr messages.
- Valid and invalid command execution.
- Non-interactive workflows.
- File creation and conflict handling.
- `--dry-run` immutability.
- Representative generated project contents.

### Test rules

- Use temporary directories for filesystem tests.
- Never point tests at a real user project or production database.
- Test both success and failure cases.
- Test security-relevant boundaries, including path traversal and symlink behavior where supported.
- Do not create meaningless tests just to increase test counts.
- Avoid flaky tests, arbitrary sleeps, and network-dependent tests unless the test specifically verifies an external integration.
- Use snapshots only when they improve reviewability; review snapshot changes rather than blindly accepting them.
- Do not weaken or delete an existing test simply to make a change pass. Explain and justify necessary test updates.

---

## 12. Required verification commands

Run the checks relevant to the change. For a phase completion, run all baseline checks unless the environment prevents it:

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

When applicable, also run:

```bash
cargo metadata --no-deps --format-version 1
```

Additional checks may include generated-project compilation, dependency audits, cross-platform tests, or documentation validation as specified by the active phase.

### Reporting rules

- Run commands from the repository root unless a test requires another directory.
- Report the actual outcome of each command.
- If a command was not run, mark it `NOT RUN` or `BLOCKED` and state why.
- If a command fails, report the failure and relevant details.
- Never claim a test, lint, build, or audit passed based on expectation alone.
- Re-run relevant checks after fixing issues.
- Do not suppress a failure merely to declare a phase complete.

---

## 13. Documentation rules

- Update documentation when behavior, command syntax, configuration, compatibility, or architecture changes.
- Keep README usage instructions aligned with commands that actually work.
- Clearly label planned features as planned.
- Do not invent URLs, package publication status, license details, supported platforms, or compatibility claims.
- Prefer concise examples that can be copied and run.
- Use fake values for environment variables and credentials.
- Keep `PRD.md` focused on product requirements, `ARCHITECTURE.md` focused on system design, this file focused on agent behavior, and phase documents focused on execution tasks and acceptance criteria.

---

## 14. Git and change management

- Make small, cohesive changes that are easy to review.
- Avoid unrelated refactors while implementing a phase.
- Inspect `git diff` and `git status` before reporting completion.
- Ensure generated files and lockfile changes are intentional.
- Do not create commits, push branches, publish packages, or deploy unless explicitly requested.
- If the repository has user changes before work begins, distinguish them from the agent's changes in the final report.
- Do not claim the working tree is clean if uncommitted changes remain.

---

## 15. How to execute a task

For each implementation task:

1. **Understand:** read the relevant requirements and existing implementation.
2. **Scope:** identify the active phase, expected behavior, and acceptance criteria.
3. **Plan:** create a short checklist of concrete changes.
4. **Implement:** make the smallest cohesive change that satisfies the requirements.
5. **Test:** add or update tests for the new behavior.
6. **Verify:** run formatting, linting, tests, and build checks relevant to the task.
7. **Review:** inspect the final diff for unintended changes, security issues, and scope creep.
8. **Document:** update affected docs and examples.
9. **Report:** summarize results and blockers honestly.
10. **Stop:** do not begin the next phase without authorization.

Do not spend excessive time producing plans that replace implementation. For a clear task, plan briefly and execute.

---

## 16. Required final response format

At the end of a task or phase, respond in the user's language and use this format:

```text
Phase/task: <name>
Status: COMPLETE | PARTIAL | BLOCKED

Summary:
- <what changed>

Files changed:
- <path>: <purpose>

Verification:
- <command>: PASS | FAIL | NOT RUN | BLOCKED — <actual result>
- <command>: PASS | FAIL | NOT RUN | BLOCKED — <actual result>

Deferred/out of scope:
- <items, or None>

Known issues / decisions needed:
- <items, or None>

Next step:
- <recommended next task; do not start it automatically>
```

Use `COMPLETE` only when all applicable acceptance criteria pass. Otherwise use `PARTIAL` or `BLOCKED` and state what remains.

---

## 17. Final principle

**Build only what the active phase requires, preserve user work, keep generated output safe, verify every claim, and report the actual state of the repository.**

# Phase 5 — Core Code Generators

> **Project:** `rustrs`  
> **Phase:** 5 — Core Code Generators  
> **Recommended path:** `docs/roadmap/phase-05-core-code-generators.md`  
> **Status:** Planned  
> **Primary outcome:** A developer can generate common Rust API code artifacts inside an existing `rustrs` project using consistent CLI commands, validated inputs, reusable templates, and safe filesystem operations.

---

## 1. Purpose

Phase 5 adds **code generators** to projects created with `rustrs`. It builds on the CLI foundation, manifest/compatibility model, template engine, filesystem engine, and project scaffolding implemented in earlier phases.

The purpose is to turn repetitive code creation into a predictable workflow, for example:

```bash
rustrs generate module products
rustrs generate service products
rustrs generate controller products
rustrs generate resource products
```

Short aliases may be supported:

```bash
rustrs g module products
rustrs g resource products
```

These examples describe the intended user experience. Before implementing them, the agent must inspect the current repository and verify the actual command structure and capabilities.

Phase 5 is complete when the supported generators can produce valid, consistently named files; integrate with the project's existing module structure where required; detect conflicts before writing; and provide clear success, dry-run, and error reports.

## 2. Objectives

1. Establish one shared contract and execution pipeline for all generators.
2. Add a CLI interface for invoking generators in an existing `rustrs` project.
3. Validate project context, manifest, generator names, and generated identifiers.
4. Implement foundational generators for the project's supported architecture and framework.
5. Implement a resource generator that composes a coherent vertical slice from smaller generation units.
6. Reuse the existing template, planning, validation, and filesystem infrastructure instead of duplicating it.
7. Ensure generated code and registration changes are deterministic and safe.
8. Test generated output, CLI behavior, conflict handling, and integration with the current project architecture.
9. Document only the generators and technology combinations that are actually supported.

## 3. Non-goals and phase boundaries

The agent must not expand this phase into later roadmap phases.

### In scope

- The `generate` command and a short alias, if consistent with the existing CLI.
- Generator registration and dispatch.
- Shared generator context, input validation, output planning, and reporting.
- Module, controller/handler, service, repository, entity/model, DTO, and other generators that fit the existing architecture.
- A resource generator that composes supported artifacts.
- Registration of generated modules or routes where required by the current project architecture.
- Dry-run and safe conflict handling.
- Generation of test and migration *files* when supported by the current architecture.
- Automated tests and documentation for the generators.

### Out of scope

Do not implement these as part of Phase 5:

- Project creation/scaffolding improvements unrelated to generator integration (`rustrs new` belongs to Phase 4).
- Project inspection commands such as `info`, `routes`, `modules`, or `deps`, or a general `doctor` command (Phase 6).
- Executing database migrations, rollbacks, seeders, or database status checks.
- Connecting to a database or modifying a live database.
- Starting, stopping, or supervising development processes.
- Docker/Compose orchestration or container lifecycle management.
- Broad Docker, CI/CD, or quality-tooling generation that belongs to Phase 8.
- Publishing packages, release automation, or MVP release work (Phase 9).
- Adding unsupported frameworks, databases, ORMs, or architectural styles solely to make a generator appear universal.
- Automatically installing dependencies, silently modifying `Cargo.toml`, or changing the project manifest without an explicit and justified design.
- Rewriting unrelated application files or refactoring unrelated modules.
- Committing, pushing, publishing, deploying, or releasing without explicit authorization.

**Boundary rule:** Phase 5 generates code and performs the minimum safe registration changes needed for that code to be usable. It does not operate the generated application or manage its infrastructure.

---

## 4. Mandatory preflight before implementation

### Task 5.0 — Audit the current codebase and verify earlier phases

Before changing code, inspect the actual working tree. Do not assume that remote repository contents or roadmap status accurately describe the local branch.

#### Required actions

1. Read `AGENTS.md`, `ARCHITECTURE.md`, `PRD.md`, and the existing roadmap documents relevant to Phases 1–4.
2. Run `git status --short` and record any pre-existing changes. Do not overwrite or discard them.
3. Inspect the actual CLI structure and current command parsing.
4. Inspect the manifest model, parser, validation, discovery, and compatibility/capability resolution.
5. Inspect the template catalog, template context, renderer, filesystem planner/writer, path validation, dry-run behavior, and conflict handling implemented in earlier phases.
6. Inspect the scaffolding output produced by `rustrs new` and identify its actual module layout and registration conventions.
7. Inspect existing tests and quality-gate commands.
8. Identify which framework, database, data-access layer, and architectural combinations are truly supported by the current code.
9. Identify any missing or incomplete earlier-phase infrastructure that blocks Phase 5.

#### Deliverables

- A concise preflight report in the final response.
- A list of existing abstractions to reuse.
- A list of blockers, if any.
- A confirmed list of generators and technology combinations that can be implemented without unsupported assumptions.

#### Rules

- Do not rewrite earlier phases as a default response to gaps.
- If a blocker requires a small change to earlier-phase infrastructure, make only the minimal, backward-compatible change that is necessary and explain it.
- If the blocker requires a substantial redesign, stop and report the blocker rather than silently broadening scope.
- Do not claim a generator or framework is supported merely because a CLI option can be parsed.

**Acceptance criteria**

- [ ] P5-AC-01: The current codebase and working-tree state have been inspected before implementation.
- [ ] P5-AC-02: The implementation plan identifies existing infrastructure to reuse and supported combinations.
- [ ] P5-AC-03: Pre-existing user changes are preserved.

---

## 5. Generator architecture and shared contract

### Task 5.1 — Define a consistent generator interface

All generators should use a shared interface or equivalent pattern that fits the existing codebase. Avoid introducing a large abstraction framework if a small trait, enum, or registry is sufficient.

The shared contract should make it possible to:

1. Identify the generator type.
2. Accept validated input such as a name and generator-specific options.
3. Read a normalized project/generation context.
4. Check the generator's prerequisites and capabilities.
5. Produce a generation plan without immediately writing files.
6. Return planned files and any supported registration edits.
7. Validate the plan and conflicts.
8. Hand the plan to the existing apply/writer pipeline.
9. Produce a consistent report.

The precise Rust types and trait boundaries are implementation decisions. Follow repository conventions and avoid unnecessary public APIs.

#### Design requirements

- Keep CLI parsing separate from generator logic.
- Keep templates separate from filesystem writes.
- Keep planning separate from applying changes.
- Reuse existing compatibility/capability logic.
- Use structured errors rather than panics for invalid user input or expected filesystem failures.
- Keep generators deterministic: the same project state and input should produce the same planned output, except for explicitly documented variable fields.
- Do not duplicate path traversal and output conflict checks inside every generator.
- Avoid global mutable state.
- Do not introduce a plugin architecture unless the current project explicitly requires it.

#### Acceptance criteria

- [ ] P5-AC-04: Generators share a consistent input, planning, validation, and reporting contract.
- [ ] P5-AC-05: Generator logic can be unit-tested without invoking the CLI or writing to the real project.
- [ ] P5-AC-06: The shared pipeline reuses existing rendering and filesystem abstractions.

### Task 5.2 — Create a generator registry and dispatch mechanism

Implement a central registry or equivalent dispatch mechanism for supported generator types.

#### Required behavior

- Map supported CLI generator names and aliases to their implementations.
- Reject unknown generator names with a useful error.
- Display supported generator names in help output.
- Avoid scattering generator-name string matching throughout the CLI.
- Ensure help and documentation reflect the generators actually implemented.
- Use aliases only when they are clear and do not create ambiguous commands.

Potential generator names include:

- `module`
- `controller` or `handler` (choose terminology consistent with the framework and existing project)
- `service`
- `repository`
- `entity` or `model`
- `dto`
- `middleware`
- `guard`
- `validator`
- `resource`
- `job`
- `event`
- `event-handler`
- `test`
- `migration`

This list is a candidate inventory, not a requirement to implement unsupported generators. The agent must confirm each generator's value and feasibility from the current architecture before adding it to the registry.

#### Acceptance criteria

- [ ] P5-AC-07: The registry exposes only implemented and supported generators.
- [ ] P5-AC-08: Unknown generator types fail safely and show useful guidance.

---

## 6. Naming, project context, and validation

### Task 5.3 — Normalize generator names and validate Rust identifiers

Define one shared name-validation and normalization policy. Do not let each generator invent its own naming rules.

#### Requirements

- Accept conventional user-facing names such as `products`, `product-item`, or `product_item` only where normalization is unambiguous.
- Derive file names and module names using `snake_case`.
- Derive Rust type names using `PascalCase`.
- Derive method/function names using `snake_case` where applicable.
- Reject empty names, whitespace-only names, path separators, `.`/`..` traversal segments, absolute paths, control characters, and other unsafe path input.
- Reject or correctly handle Rust keywords and reserved identifiers. Prefer clear validation errors over silently producing invalid Rust.
- Detect collisions after normalization. For example, two different inputs that normalize to the same output must not silently overwrite each other.
- Apply generator-specific rules where necessary, but keep common validation centralized.
- Do not derive filesystem paths by directly concatenating unvalidated user input.

#### Tests

- Normal names.
- Hyphenated and underscored names.
- Names containing numbers.
- Empty names and whitespace.
- Rust keywords.
- Unicode or punctuation according to the explicitly documented policy.
- Path traversal attempts and absolute paths.
- Normalization collisions.

#### Acceptance criteria

- [ ] P5-AC-09: Names are validated and normalized consistently.
- [ ] P5-AC-10: Invalid or unsafe names cannot escape the project or produce invalid identifiers.
- [ ] P5-AC-11: Normalization collisions are detected before files are written.

### Task 5.4 — Build a validated generator context

Create a shared context for generators using the existing project discovery and manifest infrastructure.

The context should expose only the data generators need, such as:

- Project root.
- Parsed and validated project manifest.
- Selected framework and architecture.
- Selected database and data-access layer.
- Enabled features/capabilities.
- Relevant project layout and registration conventions.
- Template/generator version metadata if the repository already uses such concepts.

#### Required behavior

1. Resolve the project root using existing project-discovery logic.
2. Confirm the command is running inside a valid generated project, unless the existing design explicitly supports an explicit `--path`.
3. Load and validate the project's `.rustrs/config.toml` or the actual manifest path defined by the codebase.
4. Fail clearly if the manifest is missing, malformed, or incompatible with the current CLI.
5. Resolve the generator's prerequisites using the existing compatibility/capability layer.
6. Do not infer support from directory names or a README alone.
7. Do not silently rewrite or upgrade the manifest.
8. Do not automatically install dependencies.

#### Acceptance criteria

- [ ] P5-AC-12: Generators receive a validated, consistent project context.
- [ ] P5-AC-13: Missing or incompatible project configuration fails before filesystem mutation.

---

## 7. CLI integration

### Task 5.5 — Add `generate` and a short alias

Implement the generator command in the existing CLI style. The expected UX is similar to:

```bash
rustrs generate <type> <name>
rustrs g <type> <name>
```

Examples of intended invocations:

```bash
rustrs generate module products
rustrs generate service products
rustrs generate controller products
rustrs generate resource products
rustrs g resource products
rustrs generate resource products --dry-run
```

These are target examples. Keep only commands and flags that are implemented and tested.

#### Minimum command behavior

- Parse a generator type and target name.
- Support a project path override only if it is necessary and consistent with existing conventions.
- Support `--dry-run`.
- Support explicit overwrite/force behavior only if it can be implemented safely and unambiguously.
- Provide helpful command-level and generator-level help.
- Return a non-zero exit status for failures.
- Print actionable errors without leaking internal stack traces by default.
- Preserve the existing output/reporting conventions.

#### Flag semantics

Potential flags include `--dry-run`, `--force`, `--skip-tests`, `--skip-docs`, `--non-interactive`, and `--path`. Do not add all of them automatically.

- `--dry-run`: display the plan without modifying the filesystem.
- `--force`: if implemented, define exactly which conflicts it may override. It must not bypass path-safety validation, project validation, or plan validation.
- `--skip-tests` / `--skip-docs`: add only if generators actually support those optional outputs.
- `--non-interactive`: add only if this command has interactive behavior that must be disabled.
- `--path`: add only if the project discovery model benefits from an explicit target path.

#### Acceptance criteria

- [ ] P5-AC-14: The command and alias integrate with existing CLI conventions.
- [ ] P5-AC-15: CLI help documents only implemented options and generators.
- [ ] P5-AC-16: Invalid arguments and execution failures produce meaningful errors and non-zero exit codes.

---

## 8. Foundational generators

Implement generators in a dependency-aware order. A generator should emit code that follows the actual generated project's conventions rather than an invented generic layout.

### Task 5.6 — Module generator

Example:

```bash
rustrs generate module products
```

#### Expected output

The exact paths depend on the project's current architecture. A module may create a directory and module root file, for example:

```text
src/modules/products/
├── mod.rs
```

If the repository uses another module layout, follow that layout instead.

#### Requirements

- Create a valid module skeleton.
- Use normalized Rust module naming.
- Register the module in its parent module only when required by the current architecture.
- Avoid duplicate declarations.
- Detect existing module directories/files and registration conflicts.
- Ensure generated module declarations compile.
- Keep generated content minimal and consistent with the project's existing style.

#### Acceptance criteria

- [ ] P5-AC-17: The module generator produces a valid module skeleton for a supported project.
- [ ] P5-AC-18: Required module registration is correct and duplicate-safe.

### Task 5.7 — Service generator

Example:

```bash
rustrs generate service products
```

#### Requirements

- Create a service skeleton consistent with the current architecture.
- Use valid Rust naming and idioms.
- Keep business logic placeholders explicit and minimal.
- Import only dependencies that exist in the generated project or are part of the supported baseline.
- Register the service only if the architecture requires explicit registration.
- Do not create pretend business logic, fake database behavior, or unimplemented features presented as working code.
- Ensure the output is formatted and compiles in the supported project configuration, or document a precise prerequisite if the generator is only valid after another artifact exists.

#### Acceptance criteria

- [ ] P5-AC-19: The service generator emits consistent, valid Rust code for supported project layouts.

### Task 5.8 — Controller/handler generator

Choose `controller`, `handler`, or another name based on the framework's idioms and existing code conventions. Do not expose multiple synonyms unless they have clear, documented behavior.

#### Requirements

- Generate framework-appropriate request-handler skeletons.
- Use the framework's actual handler signature and response conventions.
- Avoid hard-coding Axum-specific types into a generator that claims to support other frameworks.
- Keep request parsing and business logic separated according to the project's architecture.
- Register routes or handlers only through the existing routing conventions.
- If route registration is required, ensure duplicate route declarations are detected or handled deterministically.
- Generate a minimal example that compiles without relying on imaginary helpers.

#### Acceptance criteria

- [ ] P5-AC-20: The handler/controller generator matches the selected, supported framework.
- [ ] P5-AC-21: Required routing registration is consistent with the generated project.

### Task 5.9 — Entity/model and DTO generators

Implement `entity`/`model` and `dto` as separate generators if both are meaningful for the current architecture.

#### Entity/model requirements

- Follow the selected data-access layer and established project conventions.
- Avoid assuming SQLx, SeaORM, and Diesel use interchangeable model patterns.
- Include only required derives and imports.
- Do not create database schema changes or run migrations.
- If the project has no database/data-access layer, either reject the generator with an actionable message or generate only a clearly documented domain type when that is consistent with the architecture.

#### DTO requirements

- Generate request/response types appropriate to the existing API conventions.
- Use the serialization and validation libraries actually included in the project.
- Do not assume an optional validation crate is installed.
- Avoid deriving validation behavior that the project does not support.
- Keep request DTOs distinct from persistence models when that is the established design.
- Generate minimal, compilable types and document placeholders.

#### Acceptance criteria

- [ ] P5-AC-22: Entity/model output respects the selected supported data-access strategy.
- [ ] P5-AC-23: DTO output uses only supported dependencies and conventions.

### Task 5.10 — Repository generator

Implement only if the current architecture and selected data-access strategy provide a coherent repository abstraction.

#### Requirements

- Respect the selected data-access layer; do not assume one implementation works for every ORM.
- Keep persistence concerns separate from HTTP handlers and business services.
- Use supported error types and project conventions.
- Avoid generating fake queries or returning fabricated data while implying persistence is implemented.
- Make required prerequisites explicit.
- If repositories are not part of the current architecture for a supported configuration, reject the generator clearly or omit it from the registry for that configuration.

#### Acceptance criteria

- [ ] P5-AC-24: Repository output is data-access-aware and does not claim unsupported persistence behavior.

### Task 5.11 — Middleware, guard, and validator generators

Implement these only where the current architecture supports them.

#### Middleware

- Follow the selected framework's middleware model.
- Generate a minimal valid skeleton.
- Avoid adding authentication, authorization, or logging behavior that is not implemented.

#### Guard

- Use the project's actual authorization model, if one exists.
- Do not invent a guard framework or add dependencies automatically.
- If authorization infrastructure is absent, explain the prerequisite and fail safely.

#### Validator

- Use the validation mechanism actually present in the project.
- Do not assume a validation crate is available just because it is common in Rust projects.
- If validation is not supported in the current baseline, omit or clearly gate this generator.

#### Acceptance criteria

- [ ] P5-AC-25: Optional middleware, guard, and validator generators are available only where their prerequisites are met.

---

## 9. Resource generator

### Task 5.12 — Implement the resource generator as the primary integration target

Example:

```bash
rustrs generate resource products
```

A resource generator composes multiple supported artifacts into a coherent vertical slice. It should be implemented after the foundational generators and shared pipeline are stable.

#### Required behavior

1. Validate the resource name and project context.
2. Determine which artifacts belong to a resource for the current architecture.
3. Build one combined generation plan for all output files and registration edits.
4. Validate all paths, names, prerequisites, and conflicts before writing any file.
5. Render every output using the shared template engine.
6. Apply all changes through the shared filesystem pipeline.
7. Report every created file and registration change.
8. Ensure a failure during validation does not leave a partially generated resource.
9. Avoid generating duplicate artifacts if a resource is already present.
10. Ensure the resulting code follows the same conventions as individually generated artifacts.

#### Resource composition

A supported resource may include some subset of:

- Module skeleton.
- Handler/controller.
- Service.
- Repository (if supported).
- Entity/model (if supported).
- Request/response DTOs (if supported).
- Unit or integration test skeletons (if supported).
- Route/module registration changes where required.

The exact composition must be explicit and documented. Do not promise a fixed set of artifacts for every framework/database combination.

#### Safety and consistency

- Plan all outputs before applying changes.
- Reject the entire operation if a conflict is found, unless a clearly defined safe policy explicitly supports a different outcome.
- Never use `--force` to bypass unsafe path validation.
- Do not overwrite unrelated user code.
- Do not silently edit dependencies or the project manifest.
- If the filesystem layer cannot guarantee rollback after a partial I/O failure, report that limitation accurately and implement the safest available strategy. Do not claim full atomicity without evidence.

#### Acceptance criteria

- [ ] P5-AC-26: Resource generation creates the documented artifact set for a supported configuration.
- [ ] P5-AC-27: The complete plan is validated before any filesystem mutation.
- [ ] P5-AC-28: Conflicts are reported clearly and do not cause unintended partial generation.
- [ ] P5-AC-29: Resource output is consistent with the individual generators and compiles in the supported configuration.

---

## 10. Jobs, events, tests, and migration-file generators

### Task 5.13 — Job and event generators, if supported

Potential commands:

```bash
rustrs generate job send-welcome-email
rustrs generate event user-registered
rustrs generate event-handler send-welcome-email
```

These are optional candidates, not mandatory if the current generated project lacks the necessary runtime architecture.

#### Requirements

- Inspect whether the generated project already includes a job queue, event abstraction, or event dispatch mechanism.
- Generate skeletons that match the actual infrastructure.
- Do not introduce a new queue, broker, or runtime system.
- Do not claim that a job is executable if no worker/dispatch mechanism exists.
- Do not install or add dependencies automatically.
- Register jobs/events only where the current architecture requires it.
- If prerequisites are absent, reject with a clear explanation or omit the generator.

#### Acceptance criteria

- [ ] P5-AC-30: Job/event generators are implemented only when backed by existing project infrastructure.

### Task 5.14 — Test and migration-file generators

#### Test generator

- Generate test skeletons consistent with the existing test organization.
- Use available test utilities and dependencies.
- Do not make tests pass by disabling assertions or hiding failures.
- Clearly distinguish a generated placeholder test from a meaningful behavior test.

#### Migration-file generator

- Generate a migration *file* only if the current database/data-access setup defines a migration convention.
- Follow the repository's actual naming and directory conventions.
- Do not execute migrations, connect to a database, roll back, seed, or inspect database state.
- Do not assume that every ORM or SQL library uses the same migration format.
- Do not claim generated migration files have been executed or verified against a live database.

#### Acceptance criteria

- [ ] P5-AC-31: Test and migration-file generators follow existing conventions and do not perform runtime/database operations.

---

## 11. Safe planning, conflict handling, and reporting

### Task 5.15 — Integrate all generators with the shared planning/apply pipeline

All generators must use the same safety principles. Do not implement one filesystem-writing path per generator.

#### Required execution sequence

1. Parse CLI arguments.
2. Resolve and validate project context.
3. Validate and normalize generator input.
4. Check generator prerequisites and compatibility.
5. Build the complete generation plan.
6. Validate every planned path and registration edit.
7. Detect existing files, conflicting declarations, and normalization collisions.
8. Render the plan.
9. Validate rendered output where feasible.
10. If `--dry-run` is active, show the plan and exit without mutation.
11. Otherwise apply the validated plan through the existing filesystem layer.
12. Verify the resulting state to the extent supported by the current architecture.
13. Print a report.

#### Dry-run requirements

- Must not create, modify, or delete files.
- Must not change project registrations.
- Must not modify `.rustrs/config.toml`, `Cargo.toml`, lockfiles, or other project metadata.
- Must show the intended files and registration edits.
- Must report conflicts and validation failures.
- Tests must verify that the project tree is unchanged after dry-run.

#### Conflict policy

- Default behavior must not overwrite existing files.
- `--force`, if supported, must have narrow and documented semantics.
- It must never bypass project-context validation, name validation, path-safety checks, or plan validation.
- Do not overwrite a user's existing file just because it has the same expected filename.
- For registration edits, preserve unrelated content and detect duplicate declarations.
- If safe targeted edits cannot be performed reliably, fail with an actionable message instead of rewriting a file heuristically.

#### Report requirements

A successful report should communicate:

- Generator type and normalized name.
- Project root or target context, as appropriate.
- Files created.
- Registration changes made.
- Files skipped, if skipping is a supported behavior.
- Any manual follow-up required.
- A clear success/failure status.

Errors should identify the problem and next step where possible. Avoid vague output such as `Something went wrong`.

#### Acceptance criteria

- [ ] P5-AC-32: All generators use the shared planning/apply pipeline.
- [ ] P5-AC-33: Dry-run has no filesystem or metadata side effects.
- [ ] P5-AC-34: Existing files are protected by default and overwrite semantics are explicit.
- [ ] P5-AC-35: Reports accurately describe the changes made and never claim unperformed actions.

---

## 12. Testing strategy

### Task 5.16 — Add comprehensive tests for generators and generated output

Tests must cover the shared contract and each implemented generator. Do not rely only on unit tests for string fragments.

#### A. Unit tests

Cover:

- Generator registry and dispatch.
- Name normalization and validation.
- Rust keyword handling.
- Project context and manifest validation.
- Capability/prerequisite checks.
- Generation-plan construction.
- Conflict detection.
- Registration edit planning.
- Error conversion and reporting.

#### B. Template and output tests

For each implemented generator:

- Verify expected output paths.
- Verify important code structure and imports.
- Verify normalized names are used consistently.
- Verify output is deterministic.
- Verify no unresolved template placeholders remain.
- Verify output does not include unsupported dependencies or imaginary helpers.
- Verify generated registration changes match the current architecture.

Prefer structural assertions over brittle snapshots of every whitespace character. Snapshots may be used where they are stable and useful.

#### C. Filesystem integration tests

Use temporary directories and isolated fixtures. Cover:

- Successful generation.
- Running outside a valid project.
- Missing or malformed project manifest.
- Existing target files.
- Existing module or route registration.
- Multiple conflicts in a resource plan.
- Unsafe path input.
- Dry-run leaves all files unchanged.
- Failure before apply leaves the tree unchanged.
- Force behavior, if implemented.
- Repeated generation and collision behavior.
- Paths with spaces where supported.

Do not use or modify the developer's real project as a test fixture.

#### D. CLI tests

Test:

- `generate` command.
- `g` alias, if implemented.
- Valid generator and name.
- Unknown generator.
- Missing arguments.
- `--dry-run`.
- Conflict reporting.
- Exit status for success and failure.
- Help output lists only supported generators and flags.

#### E. Generated-code verification

For every supported starter/configuration that a generator claims to support:

1. Create or use a minimal isolated project fixture generated by the current scaffolding implementation.
2. Run the generator.
3. Verify expected files and registration edits.
4. Run `cargo fmt --check` or format the fixture according to the test strategy.
5. Run `cargo check` on the fixture when the test environment and dependency availability make that feasible.
6. Run relevant tests on the fixture where feasible.

Do not treat a source-text assertion as proof that generated Rust compiles. If a compile test cannot run because of unavailable dependencies or environment constraints, record the limitation explicitly.

#### F. Regression tests

Ensure Phase 5 does not break:

- Existing CLI commands.
- Manifest parsing and validation.
- Compatibility/capability resolution.
- Project scaffolding.
- Template rendering.
- Filesystem safety and dry-run behavior.

#### Acceptance criteria

- [ ] P5-AC-36: Every implemented generator has unit and output tests.
- [ ] P5-AC-37: Shared filesystem and CLI behaviors are tested in isolated temporary projects.
- [ ] P5-AC-38: Generated-code compilation is verified for claimed supported configurations, or limitations are documented.
- [ ] P5-AC-39: Existing Phase 1–4 behavior passes regression tests.

---

## 13. Documentation and quality gates

### Task 5.17 — Document the supported generator workflow

Update relevant documentation without rewriting unrelated product documentation.

#### Required documentation

1. Add a generator reference covering supported generator names, arguments, aliases, and flags.
2. Document examples for the generators that are actually implemented.
3. Document naming and normalization rules.
4. Explain project-context requirements.
5. Explain dry-run and conflict/overwrite behavior.
6. Document prerequisites and unsupported configurations.
7. Document the resource generator's exact artifact composition for each supported project configuration.
8. Update the README only where necessary to reflect implemented behavior.
9. Keep roadmap status truthful; do not mark later phases complete.
10. Ensure help output and documentation agree.

#### Quality gates

Run the repository's established checks. At minimum, attempt:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

Also run any existing integration tests or project-specific scripts identified during preflight.

If a command cannot run, record:

- The exact command.
- The reason it could not run.
- Whether it is an environment limitation, missing dependency, or code failure.
- What remains unverified.

Do not claim a check passed unless it actually ran and passed. Do not hide failures by weakening lint rules, deleting tests, or relaxing assertions without a justified fix.

#### Acceptance criteria

- [ ] P5-AC-40: Documentation matches the actual implemented generator set.
- [ ] P5-AC-41: Quality gates are run and reported accurately.
- [ ] P5-AC-42: README and roadmap status do not overstate support or completion.

---

## 14. Recommended implementation order

Follow this order unless preflight reveals a concrete dependency that requires a change. Explain any deviation.

1. **Preflight and blocker report** — Task 5.0.
2. **Shared generator contract and registry** — Tasks 5.1–5.2.
3. **Name normalization and validated project context** — Tasks 5.3–5.4.
4. **CLI command and alias** — Task 5.5.
5. **Module generator** — Task 5.6.
6. **Service and controller/handler generators** — Tasks 5.7–5.8.
7. **Entity/model, DTO, and repository generators** — Tasks 5.9–5.10.
8. **Optional middleware, guard, validator, job, event, test, and migration-file generators** — Tasks 5.11, 5.13–5.14, based on actual prerequisites.
9. **Resource generator** — Task 5.12, after the foundational pieces are stable.
10. **Shared conflict/dry-run/reporting integration** — Task 5.15; foundational safety should be implemented earlier, then verified across all generators here.
11. **Tests and generated-code verification** — Task 5.16, continuously during implementation and completed here.
12. **Documentation and quality gates** — Task 5.17.

The resource generator is the primary integration test of the architecture, but it should not be used as a shortcut that bypasses smaller generator contracts or shared safety checks.

---

## 15. Definition of Done

Phase 5 may be marked complete only when all applicable criteria below are met.

- [ ] The preflight audit is complete and blockers are resolved or explicitly documented.
- [ ] All generators exposed in CLI help are implemented and tested.
- [ ] The shared generator pipeline reuses the existing project, manifest, compatibility, template, and filesystem infrastructure.
- [ ] Name validation and normalization are consistent and safe.
- [ ] Generators run only in a valid project context or an explicitly supported target path.
- [ ] Foundational generators produce output matching the actual generated project architecture.
- [ ] The resource generator composes a documented set of supported artifacts.
- [ ] All output paths and conflicts are validated before mutation.
- [ ] Dry-run has no side effects.
- [ ] Existing files and unrelated project content are preserved by default.
- [ ] Registration changes are deterministic and duplicate-safe.
- [ ] Generated Rust code is checked for syntax/compilation for supported configurations where feasible.
- [ ] Unit, integration, CLI, output, and regression tests pass, or every unverified item is explicitly reported.
- [ ] Documentation and CLI help accurately describe supported generators.
- [ ] Quality-gate results are reported truthfully.
- [ ] No Phase 6–9 functionality has been implemented as scope creep.
- [ ] No commit, push, publish, deployment, or release was performed without explicit authorization.

**Important:** If only some generators are implemented, report Phase 5 as partially complete. Do not mark the phase complete just because `rustrs generate` parses successfully or because one generator works.

---

## 16. Final report required from the AI agent

At the end of the work, return a concise but evidence-based report with these sections:

1. **Phase status:** Complete / Partially complete / Blocked.
2. **Preflight findings:** Current architecture, reused infrastructure, and blockers.
3. **Implemented generators:** List each generator and the supported framework/data-access configurations.
4. **CLI examples:** Commands that were actually tested.
5. **Files changed:** Grouped by CLI, generator logic, templates, tests, and docs.
6. **Safety behavior:** Dry-run, conflict handling, registration edits, and force semantics if present.
7. **Verification:** Exact commands run and pass/fail results.
8. **Generated-code checks:** Which fixtures compiled and which configurations remain unverified.
9. **Known limitations:** Specific, actionable, and not hidden.
10. **Scope confirmation:** Explicitly state that Phase 6–9 work was not performed.
11. **Next steps:** Remaining Phase 5 tasks only.

Never report planned behavior as implemented behavior. Include command output or concise evidence when useful, but do not flood the report with irrelevant logs.

---

## 17. Final instruction to the agent

Implement **Phase 5 — Core Code Generators only**. Begin with the mandatory preflight audit, then follow the recommended implementation order. Reuse existing abstractions, keep changes small and coherent, and validate all generation plans before filesystem mutation. Do not assume unsupported framework, database, ORM, or feature combinations. Do not expand into inspection, database operations, infrastructure orchestration, release engineering, or unrelated refactoring.

If a requirement cannot be completed safely within this phase, explain the blocker and stop at the appropriate boundary. Preserve user changes and do not commit, push, publish, deploy, or release without explicit permission.

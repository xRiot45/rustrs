# Phase 6 — Inspector and Diagnostics

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 6 — Inspector and Diagnostics  
> **Recommended repository path:** `docs/roadmap/phase-06-inspector-and-diagnostic.md`  
> **Status:** Planned  
> **Primary outcome:** Provide read-only CLI commands to inspect an existing `rustrs` project and diagnose configuration, structure, compatibility, and environment issues without automatically changing the project.

---

## 1. Purpose

Phase 6 adds project inspection and diagnostics to `rustrs`. After a developer creates a project and generates code, they need a reliable way to understand the project's configuration, inspect its modules and routes, review declared dependencies, and identify issues that may prevent the project from being used successfully.

The intended command set is:

```bash
rustrs info
rustrs modules
rustrs routes
rustrs deps
rustrs doctor
```

Where supported by the established CLI conventions, commands may accept a project path:

```bash
rustrs info --path ./my-api
rustrs doctor --path ./my-api
```

These commands are target UX examples, not proof that the commands or flags already exist. The implementation must inspect the current codebase and follow its established command/argument conventions.

**Core principle:** Phase 6 observes and reports. It must not automatically repair, rewrite, generate, install, migrate, or otherwise mutate the inspected project.

## 2. Objectives

1. Establish a shared, read-only project inspection context.
2. Provide `rustrs info` to summarize project configuration and state.
3. Provide `rustrs modules` to inspect discoverable modules and registration state.
4. Provide `rustrs routes` to inspect routes that can be reliably identified.
5. Provide `rustrs deps` to inspect declared Cargo dependencies and relevant project metadata.
6. Provide `rustrs doctor` to run a set of reliable project and environment diagnostics.
7. Reuse the existing manifest, project discovery, compatibility, and capability infrastructure.
8. Standardize findings, severity levels, output, error handling, and exit codes.
9. Keep every command read-only and safe for user-owned projects.
10. Test results against real generated project fixtures and document limitations honestly.

## 3. Scope and phase boundaries

### 3.1 In scope

- Project root discovery and optional explicit project path, if consistent with the existing CLI.
- Read-only manifest loading and validation.
- Project summary inspection.
- Module discovery and registration inspection.
- Route inspection using supported, verifiable conventions.
- Cargo dependency inspection.
- Diagnostics for manifest, compatibility, expected files, project structure, and relevant toolchain availability.
- Structured diagnostic findings and consistent output/exit-code behavior.
- Optional machine-readable JSON output only if it can be implemented consistently and tested.
- Unit, integration, CLI, filesystem-safety, and regression tests.
- Documentation for commands and supported inspection behavior.

### 3.2 Out of scope

Do not implement the following in Phase 6:

- Code generation or changes to generator behavior (Phase 5).
- Project scaffolding features unrelated to inspection (Phase 4).
- Automatic repair or fix commands.
- Automatic modification of `.rustrs/config.toml`, `Cargo.toml`, or `Cargo.lock`.
- Installing, adding, removing, or upgrading Cargo dependencies.
- Running database migrations, rollbacks, seeders, or database status operations (Phase 7).
- Connecting to a database.
- Starting, stopping, or supervising the generated application.
- Docker/Compose orchestration.
- Broad Docker, CI/CD, or quality-tooling generation (Phase 8).
- Release packaging, publishing, or distribution (Phase 9).
- A general Rust parser or compiler replacement.
- Claiming complete static route/module analysis when dynamic macros or runtime registration prevent reliable analysis.
- Refactoring unrelated modules or introducing speculative infrastructure for future phases.

**Boundary rule:** Inspection may read relevant project files and, where justified, query installed tool versions. It must not mutate the inspected project or run arbitrary project-defined commands.

---

## 4. Mandatory preflight

### Task 6.0 — Audit the current repository and verify Phase 5

Before changing code, inspect the actual working tree and implementation. Do not assume that a remote README, phase label, or previous agent report proves that a feature works.

#### Required actions

1. Read `AGENTS.md`, `PRD.md`, `ARCHITECTURE.md`, and roadmap documents for Phases 0–5.
2. Run `git status --short` and preserve all pre-existing changes.
3. Inspect CLI command definitions and existing argument conventions.
4. Inspect project root discovery and manifest loading/validation.
5. Inspect compatibility and capability resolution.
6. Inspect the project scaffolding output for each configuration actually supported.
7. Inspect the generator registry and generated output actually available in the current branch.
8. Identify supported module, route, and dependency conventions from source code and generated fixtures.
9. Inspect existing test conventions and quality gates.
10. List blockers that would prevent reliable inspection.

#### Deliverables

- A concise preflight summary in the final report.
- A map of existing abstractions to reuse.
- A list of verified project layouts/configurations.
- A list of blockers and any minimal prerequisite changes required.

#### Rules

- Do not assume Phase 5 work is absent just because a remote status file is stale.
- Do not assume Phase 5 work is complete just because an agent said it is complete.
- Do not rewrite prior phases by default.
- If a small, backward-compatible prerequisite fix is necessary, make only the minimum change and explain it.
- If a prerequisite requires substantial redesign, stop and report the blocker.

#### Acceptance criteria

- [ ] P6-AC-01: Repository, working tree, and Phase 5 implementation are inspected before implementation.
- [ ] P6-AC-02: The supported project layouts and data sources are documented in the implementation plan.
- [ ] P6-AC-03: Existing user changes are preserved and blockers are reported honestly.

---

## 5. Shared inspection architecture

### Task 6.1 — Build a shared read-only inspection context

Create a reusable context for inspection commands. Reuse existing types and services where possible instead of creating duplicate manifest or compatibility models.

The context may contain, as required by the existing architecture:

- Resolved project root.
- Validated project manifest.
- Manifest path and schema version.
- Template, framework, architecture, database, and data-access selection.
- Enabled features/capabilities.
- Paths of relevant project files.
- Cargo manifest location and parsed metadata, if needed.
- A record of warnings or unavailable information discovered while building the context.

#### Required behavior

1. Resolve the target project through existing project-discovery logic.
2. Support an explicit target path only if the CLI architecture allows it.
3. Canonicalize or otherwise validate paths safely where appropriate.
4. Load the manifest without modifying it.
5. Distinguish missing project root, missing manifest, malformed manifest, unsupported schema version, and incompatible configuration.
6. Use the existing compatibility/capability layer.
7. Allow commands to report partial findings when a safe partial inspection is meaningful, without disguising an invalid project as valid.
8. Avoid reading files unrelated to the requested inspection.
9. Do not expose secret values in the context's display/debug output.

#### Acceptance criteria

- [ ] P6-AC-04: Inspection commands share a consistent project context.
- [ ] P6-AC-05: Manifest and project discovery errors are distinguishable and actionable.
- [ ] P6-AC-06: Context creation performs no project mutations and does not expose secrets.

### Task 6.2 — Define shared findings, severity, and output contracts

Use a small, consistent representation for diagnostic findings. Exact Rust types should follow repository conventions and avoid unnecessary public APIs.

A finding should be able to represent, where applicable:

- Stable diagnostic code.
- Severity: `info`, `warning`, or `error`.
- Short title/message.
- Explanation or evidence.
- A relevant file path and line/section, if reliably available.
- A practical suggestion.
- Whether the information is certain or limited by static-analysis constraints, if this distinction is necessary.

#### Requirements

- Do not use free-form strings as the only internal representation if callers need to distinguish severity or diagnostic code.
- Keep formatting separate from the logic that detects findings.
- Use consistent output across commands.
- Print primary results to stdout and operational errors to stderr according to repository conventions.
- Do not leak secrets or dump full configuration files.
- Keep output useful with `--no-color`.
- Only add `--json` if its schema can be shared, documented, and tested.
- If JSON is implemented, ensure stdout contains valid JSON without decorative terminal text.

#### Exit-code policy

Define and document exit codes based on existing CLI conventions. At minimum, distinguish:

- Successful inspection with no blocking findings.
- Completed diagnostics that found errors, if the chosen policy treats them as non-zero.
- Invalid invocation or inability to perform the requested inspection.

Do not invent contradictory exit-code behavior across commands. A warning should not unexpectedly be treated as an error unless documented.

#### Acceptance criteria

- [ ] P6-AC-07: Findings use a consistent structured representation.
- [ ] P6-AC-08: Output and exit-code behavior are documented and tested.
- [ ] P6-AC-09: Machine-readable output, if added, is valid, stable, and tested.

---

## 6. Project information inspector

### Task 6.3 — Implement `rustrs info`

Intended examples:

```bash
rustrs info
rustrs info --path ./my-api
```

#### Required information

Display values only when they can be verified:

- Project name.
- Resolved project root.
- Manifest path and schema version.
- Selected template.
- Framework.
- Database.
- Data-access layer.
- Architecture.
- Enabled features/capabilities.
- Manifest validation status.
- Relevant warnings about missing or inconsistent project files, if implemented through shared findings.

#### Requirements

- Use the validated manifest rather than hard-coded defaults.
- Clearly represent unavailable, unset, or unknown values.
- Do not infer an unconfigured database or framework from directory names alone.
- Do not label a project healthy solely because the manifest parsed.
- Keep summary output readable and stable.
- Avoid dumping secret-bearing settings.

#### Acceptance criteria

- [ ] P6-AC-10: Displayed configuration comes from the actual manifest and verifiable project state.
- [ ] P6-AC-11: Unknown or unavailable values are explicit rather than guessed.
- [ ] P6-AC-12: Missing or invalid manifests produce clear results without mutation.

---

## 7. Module inspector

### Task 6.4 — Implement `rustrs modules`

Intended examples:

```bash
rustrs modules
rustrs modules --path ./my-api
```

#### Required behavior

1. Identify modules using conventions confirmed during preflight.
2. Display module name and relevant source path.
3. Report registration state only when it can be checked using the project's actual module structure.
4. Distinguish a discovered directory/file from a declared or registered Rust module.
5. Detect duplicate declarations only where the analysis method can establish them.
6. Explain when a module is discovered but registration cannot be verified.
7. Do not classify every directory under `src/` as an application module.
8. Do not silently edit `mod.rs`, `lib.rs`, `main.rs`, or other source files.

#### Analysis limitations

Rust modules may be declared inline, through `mod.rs`, through `#[path]`, or generated by macros. The inspector should support the project's documented conventions and explicitly report limitations for structures it cannot analyze reliably. It must not pretend to be a full Rust compiler or parser.

#### Acceptance criteria

- [ ] P6-AC-13: Every reported module can be traced to a source location.
- [ ] P6-AC-14: Registration status is based on evidence, not directory existence alone.
- [ ] P6-AC-15: Unsupported or ambiguous module patterns are reported honestly.

---

## 8. Route inspector

### Task 6.5 — Implement `rustrs routes`

Intended examples:

```bash
rustrs routes
rustrs routes --path ./my-api
```

#### Required behavior

1. Identify routing conventions used by the selected, supported framework.
2. Extract HTTP method and path only when reliably identifiable.
3. Display the handler or module when the association can be verified.
4. Handle nested routers, prefixes, and route grouping only to the extent supported by the implementation.
5. Detect duplicate route declarations only when the available analysis can establish a conflict.
6. Explain when routes cannot be resolved statically.
7. Distinguish “no routes found” from “route analysis unsupported or incomplete.”
8. Do not guess routes based on filenames, handler names, or common REST conventions.
9. Do not start the application or send HTTP requests to discover routes.
10. Do not modify routing source code.

#### Framework-specific behavior

- Do not hard-code Axum assumptions into a command that claims to support multiple frameworks.
- Use the framework and routing patterns actually present in the project.
- If only one framework is supported by the current compatibility matrix, implement and document that framework only.
- A future framework must not be represented as supported until its route inspection behavior and tests exist.

#### Acceptance criteria

- [ ] P6-AC-16: Reported method/path information has verifiable source evidence.
- [ ] P6-AC-17: The inspector does not invent routes or handler associations.
- [ ] P6-AC-18: Incomplete or unsupported static analysis is clearly identified.

---

## 9. Dependency inspector

### Task 6.6 — Implement `rustrs deps`

Intended examples:

```bash
rustrs deps
rustrs deps --path ./my-api
```

#### Required behavior

- Parse the project's `Cargo.toml` using a reliable TOML parser already present in the project where possible.
- Display declared direct dependencies and their declared version requirements or source specifications.
- Distinguish regular, development, and build dependencies where useful and supported.
- Report optional dependencies and feature declarations when relevant.
- Map dependencies to `rustrs` features only through an explicit mapping or capability rule.
- Distinguish manifest declarations from resolved or compiled dependency state.
- Handle workspace/inherited dependency declarations only if the current project layout requires and supports them.
- Report malformed Cargo manifests clearly.
- Never print credentials embedded in dependency source configuration.
- Do not modify dependency declarations, install packages, or update lockfiles.

#### Important distinctions

- A dependency declared in `Cargo.toml` is not proof that it has been downloaded or compiled.
- A package appearing in `Cargo.lock` is not necessarily a direct dependency.
- A feature-to-dependency association must come from explicit project rules, not name similarity alone.

#### Acceptance criteria

- [ ] P6-AC-19: Dependency information reflects the parsed manifest.
- [ ] P6-AC-20: Direct, development, build, optional, and resolved dependency states are not conflated.
- [ ] P6-AC-21: The command is read-only and handles malformed manifests safely.

---

## 10. Diagnostics

### Task 6.7 — Implement `rustrs doctor`

Intended examples:

```bash
rustrs doctor
rustrs doctor --path ./my-api
```

`doctor` aggregates reliable checks and presents actionable findings. It should use shared inspection services rather than reimplementing their logic.

#### Diagnostic categories

Implement only categories supported by evidence and the current architecture:

1. **Project discovery**
   - Project root found.
   - Expected manifest location identified.
   - Target path is valid and accessible.

2. **Manifest and configuration**
   - Manifest exists and parses.
   - Schema version is supported.
   - Required fields are valid.
   - Selected technology combination is supported.
   - Feature/capability configuration is coherent.

3. **Project structure**
   - Required files for the selected template are present.
   - Important paths are readable.
   - Expected source layout is consistent with documented conventions.
   - Required module or route registration can be verified, where applicable.

4. **Cargo metadata**
   - `Cargo.toml` exists and parses.
   - Relevant package metadata is available.
   - Manifest and project configuration show no known contradiction.
   - Missing expected dependency declarations are reported only where explicit rules exist.

5. **Environment**
   - `cargo` availability, if checked.
   - `rustc` availability and version, if checked.
   - Toolchain compatibility only when a concrete project requirement is defined.
   - Permission or executable lookup failures are reported clearly.

6. **Static inspection limitations**
   - Route or module analysis is incomplete because of unsupported constructs.
   - A finding could not be verified due to unavailable input.

#### Example output (illustrative only)

```text
rustrs doctor

Project
  [OK] Project root detected
  [OK] Project manifest parsed
  [OK] Framework configuration recognized
  [WARN] A module registration could not be verified

Environment
  [OK] cargo available
  [OK] rustc available

Summary
  Errors: 0
  Warnings: 1
```

The actual status must be computed from checks that ran. Never print `[OK]` for a check that was skipped or could not be performed.

#### Requirements

- Run checks in a deterministic order.
- Keep each check independently testable.
- Collect multiple findings where safe rather than stopping at the first non-fatal warning.
- Stop or return a clear fatal error when the project cannot be inspected safely.
- Include diagnostic codes where useful for stable tests and future automation.
- Provide a concise summary of errors and warnings.
- Keep remediation as advice; do not automatically execute fixes.

#### Acceptance criteria

- [ ] P6-AC-22: Each diagnostic finding corresponds to a check that actually ran.
- [ ] P6-AC-23: Findings have consistent severity and actionable messages.
- [ ] P6-AC-24: Skipped or unverifiable checks are not reported as successful.
- [ ] P6-AC-25: `doctor` does not modify the inspected project.

### Task 6.8 — Add compatibility and capability diagnostics

Reuse Phase 2 compatibility and capability rules.

#### Requirements

- Detect unsupported combinations of framework, database, and data-access layer.
- Detect feature selections that conflict with declared capabilities.
- Explain which selection or rule caused the finding where possible.
- Distinguish unsupported configuration from a failure to read or validate configuration.
- Avoid duplicating compatibility tables in the `doctor` implementation.
- Do not silently change manifest selections.
- Do not report a feature as supported merely because a matching dependency is present.

#### Acceptance criteria

- [ ] P6-AC-26: Compatibility diagnostics use the existing compatibility/capability source of truth.
- [ ] P6-AC-27: Unsupported or inconsistent selections are reported with clear evidence.

### Task 6.9 — Add scaffold/generator consistency checks

Inspect whether the project structure is consistent with the selected template and documented generator conventions.

#### Requirements

- Define expected files from the actual template/layout metadata where available.
- Use explicit checks for required files rather than a universal hard-coded file list.
- Report missing required files and ambiguous structure separately.
- Check module/route registration only when the relevant conventions are known.
- Avoid false positives for optional features or intentionally omitted files.
- Do not classify user-created extra files as errors solely because they are not known to `rustrs`.
- Do not repair or regenerate missing files.

#### Acceptance criteria

- [ ] P6-AC-28: Structure checks are specific to supported templates and capabilities.
- [ ] P6-AC-29: Findings distinguish verified errors from warnings or unknown states.

---

## 11. Read-only behavior and security

### Task 6.10 — Enforce inspection safety

Every Phase 6 command must be read-only with respect to the inspected project.

#### Required guarantees

- Do not create, modify, rename, or delete project files.
- Do not modify `.rustrs/config.toml`, `Cargo.toml`, `Cargo.lock`, or source files.
- Do not execute automatic repairs.
- Validate and normalize user-supplied target paths.
- Prevent path traversal and accidental inspection of an unintended target.
- Handle symlinks and canonical paths according to a documented policy.
- Respect filesystem permission errors.
- Do not follow arbitrary project-defined commands or shell snippets.
- Do not invoke a shell by interpolating project data into command strings.
- If checking `cargo` or `rustc`, invoke the executable safely with explicit arguments and controlled output handling.
- Set reasonable timeouts or avoid potentially unbounded process checks if the current architecture supports this.
- Do not expose secrets in terminal output, structured reports, or error messages.
- Do not print the entire contents of configuration files.

#### Testing requirements

- Snapshot relevant file contents and metadata before/after each read-only integration test where practical.
- Verify dry/read-only inspection does not create or modify files.
- Test paths containing spaces where supported.
- Test traversal attempts and unreadable files.
- Test that malformed project content cannot cause arbitrary command execution.

#### Acceptance criteria

- [ ] P6-AC-30: All inspector and diagnostics commands are read-only.
- [ ] P6-AC-31: Target-path validation prevents unintended traversal.
- [ ] P6-AC-32: Sensitive configuration values are not exposed.
- [ ] P6-AC-33: Environment checks do not execute arbitrary project-defined commands.

---

## 12. Testing strategy

### Task 6.11 — Add unit, integration, CLI, and regression tests

#### A. Unit tests

Cover:

- Project context construction.
- Manifest parse and validation errors.
- Compatibility/capability findings.
- Diagnostic severity and codes.
- Module discovery rules.
- Route extraction rules for each supported routing pattern.
- Cargo dependency parsing and classification.
- Exit-code mapping.
- JSON serialization, if implemented.

#### B. Fixture tests

Create isolated fixtures for:

- A valid minimal project for each supported template layout.
- A project with optional features enabled.
- Missing manifest.
- Malformed manifest.
- Unsupported schema version.
- Unsupported technology combination.
- Missing expected source file.
- Malformed `Cargo.toml`.
- Missing or conflicting module registration.
- Routes expressed using supported patterns.
- Routes expressed using unsupported/dynamic patterns.
- Direct and development dependencies.
- Optional dependencies and features, if supported.
- Unreadable paths or permission failures where the platform allows reliable testing.

Use fixtures that reflect actual scaffolding output. Do not invent fixture layouts that the generator never produces.

#### C. CLI integration tests

Test:

- Each command's help and normal invocation.
- Explicit project path, if supported.
- Running outside a project.
- Missing required arguments.
- Invalid path.
- Invalid manifest.
- Output and error stream behavior.
- Exit codes.
- Unknown command/option handling.
- JSON output validity, if implemented.

#### D. Read-only tests

Verify that each command does not modify:

- Project manifest.
- Cargo manifest or lockfile.
- Source files.
- File and directory inventory, except unavoidable environment artifacts outside the inspected project that must be documented.

Do not use the developer's actual working project as a mutable test fixture.

#### E. Regression tests

Ensure the following remain functional:

- CLI foundation and global options.
- Manifest parsing, validation, and schema handling.
- Compatibility and capability resolution.
- Template rendering and filesystem safety.
- Project scaffolding (`rustrs new`).
- Implemented code generators.
- Existing tests and documentation assumptions.

#### F. Verification integrity

- Do not treat a substring assertion as proof that a route or module was correctly parsed.
- Do not treat a parsed dependency declaration as proof that the crate compiles.
- Do not report environment checks as successful when the command was unavailable.
- If a platform-specific test cannot run, state the limitation explicitly.

#### Acceptance criteria

- [ ] P6-AC-34: Every implemented inspector has meaningful unit and integration coverage.
- [ ] P6-AC-35: `doctor` checks are tested individually and in aggregate.
- [ ] P6-AC-36: Read-only guarantees are covered by tests.
- [ ] P6-AC-37: Phase 1–5 regression tests pass, or failures are clearly documented.

---

## 13. Documentation and quality gates

### Task 6.12 — Document commands and complete final verification

#### Required documentation

1. Document `rustrs info`, `rustrs modules`, `rustrs routes`, `rustrs deps`, and `rustrs doctor` only if implemented.
2. Document accepted arguments and project path behavior.
3. Explain diagnostic severity and exit codes.
4. Document what each command inspects and which files it reads.
5. Explain route/module analysis limitations.
6. Explain the difference between declared dependencies and resolved/compiled dependencies.
7. Explain that diagnostics are read-only and do not apply fixes.
8. Update README usage examples to match the actual CLI.
9. Keep roadmap status truthful; do not mark later phases complete.

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

Also run existing CLI/integration scripts found during preflight.

For every command that cannot run, report:

- The exact command.
- Why it could not run.
- Whether the issue is an environment limitation, missing dependency, or code failure.
- What remains unverified.

Do not claim success for a command that was not run. Do not weaken lint rules, delete tests, or relax assertions merely to get a green result.

#### Acceptance criteria

- [ ] P6-AC-38: Documentation accurately describes implemented commands and limitations.
- [ ] P6-AC-39: Required quality gates are run and results are reported truthfully.
- [ ] P6-AC-40: Final diff contains no unrelated scope changes.

---

## 14. Recommended implementation order

Follow this order unless the preflight audit identifies a concrete dependency that requires a justified change.

1. **Preflight and blocker report** — Task 6.0.
2. **Shared read-only inspection context** — Task 6.1.
3. **Findings, output, and exit-code contract** — Task 6.2.
4. **Project summary** — Task 6.3 (`info`).
5. **Module inspection** — Task 6.4 (`modules`).
6. **Dependency inspection** — Task 6.6 (`deps`).
7. **Route inspection** — Task 6.5 (`routes`), after confirming supported routing syntax.
8. **Diagnostics checks** — Task 6.7 (`doctor`).
9. **Compatibility and capability diagnostics** — Task 6.8.
10. **Scaffold/generator consistency checks** — Task 6.9.
11. **Read-only/security verification** — Task 6.10, implemented from the beginning and audited here.
12. **Tests and regression coverage** — Task 6.11, continuously during implementation and completed here.
13. **Documentation and final quality gates** — Task 6.12.

The `doctor` command should compose shared checks. Do not implement a second, independent validation engine inside it.

---

## 15. Definition of Done

Phase 6 is complete only when all applicable conditions below are met.

- [ ] The actual Phase 5 implementation has been inspected.
- [ ] Shared inspection context reuses existing project and manifest infrastructure.
- [ ] `info`, `modules`, `routes`, `deps`, and `doctor` are implemented if their prerequisites are met and are listed in help only when working.
- [ ] Each command reports only information that can be verified.
- [ ] Module and route inspection limitations are clearly communicated.
- [ ] Dependency declarations are not conflated with successful installation or compilation.
- [ ] `doctor` aggregates real checks and reports severity consistently.
- [ ] Compatibility diagnostics reuse the existing source of truth.
- [ ] All commands remain read-only with respect to the inspected project.
- [ ] Path validation and secret handling are tested.
- [ ] Unit, integration, CLI, and regression tests pass, or every unverified item is explicitly reported.
- [ ] Documentation matches the actual implementation.
- [ ] Quality-gate results are recorded accurately.
- [ ] No Phase 7–9 functionality has been added as scope creep.
- [ ] No commit, push, publish, deployment, or release has occurred without explicit authorization.

**Do not mark Phase 6 complete just because command names appear in help.** Each implemented command must be verified against a realistic project fixture and must handle failure cases safely.

---

## 16. Required final report from the AI agent

At completion, provide an evidence-based report with these sections:

1. **Phase status:** Complete / Partially complete / Blocked.
2. **Preflight:** Verified Phase 5 capabilities, project layouts, and blockers.
3. **Commands implemented:** Each command, its behavior, supported configurations, and tested examples.
4. **Inspection sources:** Which files or metadata each command reads.
5. **Diagnostics:** Implemented checks, severity policy, and exit-code behavior.
6. **Read-only evidence:** Tests or checks showing that inspected project files were not changed.
7. **Files changed:** Grouped by CLI, inspection logic, diagnostics, tests, and documentation.
8. **Verification:** Exact commands run and their pass/fail results.
9. **Known limitations:** Unsupported route/module patterns, platform constraints, and other unresolved items.
10. **Scope confirmation:** Explicit confirmation that Phase 7–9 work was not performed.
11. **Remaining work:** Phase 6 items only.

Do not describe intended behavior as implemented behavior. Include concise evidence where useful and avoid flooding the report with unrelated logs.

---

## 17. Final instruction to the agent

Implement **Phase 6 — Inspector and Diagnostics only**. Begin with the mandatory preflight audit. Reuse the existing manifest, project discovery, compatibility, capability, and CLI infrastructure. Keep inspection read-only, avoid speculative static analysis, and make every reported result traceable to evidence.

If a route or module pattern cannot be analyzed reliably, report the limitation rather than guessing. If a prerequisite requires substantial redesign, stop and report the blocker. Preserve all user changes. Do not implement automatic repairs, database operations, development process management, infrastructure orchestration, release engineering, or unrelated refactoring. Do not commit, push, publish, deploy, or release without explicit authorization.

# Phase 9 — Hardening & MVP Release

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 9 — Hardening & MVP Release  
> **Recommended repository path:** `docs/roadmap/phase-09-hardening-and-mvp-release.md`  
> **Status:** Planned  
> **Primary outcome:** Verify, harden, and prepare the existing `rustrs` implementation for an MVP release without expanding the product scope or duplicating completed work from Phases 0–8.

---

## 1. Purpose

Phase 9 is the final planned MVP phase for `rustrs`. Its purpose is to validate the work delivered in earlier phases, resolve release-blocking defects, strengthen safety and reliability, verify distribution procedures, and ensure that the code, generated projects, documentation, and release metadata describe the same product.

Phase 9 is **not** a feature-expansion phase. It must not be used to redesign working architecture, add unrelated frameworks, broaden the support matrix, or implement post-MVP ideas.

The repository's actual working tree is the implementation source of truth. Public documentation or a stale README alone is not sufficient evidence that a feature is implemented or missing. The agent must inspect the active branch and local tree before planning changes.

### 1.1 Expected outcomes

By the end of Phase 9:

1. The MVP feature scope is explicit and matches the implementation.
2. Core CLI commands have predictable input validation, output, error handling, and exit behavior.
3. Filesystem operations and code generation are safe under invalid input, conflicts, and failures.
4. Every template and generator claimed as supported has evidence of verification.
5. Security and dependency risks are assessed and release-blocking findings are resolved or explicitly dispositioned.
6. Platform and toolchain claims are based on actual test evidence.
7. The chosen installation/distribution path is verified from a clean environment.
8. The release workflow is validated without accidentally publishing a public release.
9. Documentation, version metadata, and release notes are accurate.
10. A final MVP readiness report lists test evidence, unresolved risks, and task status.

### 1.2 In scope

- Repository and prior-phase verification.
- MVP scope freeze and support-matrix validation.
- Core CLI hardening.
- Filesystem safety and destructive-operation review.
- Generated-project build and runtime-independent validation.
- Generator and compatibility regression testing.
- Security and dependency audit.
- Cross-platform and Rust toolchain compatibility verification.
- Targeted performance and resource-usage review.
- Error recovery and process-lifecycle verification.
- Versioning, package metadata, and release notes.
- Installation and distribution verification for the explicitly selected MVP channel.
- Release pipeline/supply-chain validation.
- Release-candidate acceptance testing.
- Documentation and known-limitations review.
- Final quality gates and phase closure.

### 1.3 Out of scope

Do not implement the following as part of this phase unless the project owner explicitly approves a separate scope change:

- New frameworks, databases, ORMs, templates, or capability families.
- New generator types unrelated to a verified MVP blocker.
- A GUI, web dashboard, hosted template marketplace, or plugin registry.
- Cloud deployment orchestration or automatic deployment to a user's infrastructure.
- A new migration system or a redesign of Phase 7 database commands.
- A new process-execution architecture when the existing abstraction is adequate.
- Automatic installation or updating of user tools.
- Silent dependency upgrades or broad dependency replacement.
- Automatic modification of shell startup files.
- Automatic commits, pushes, tags, package publishing, or public GitHub Releases.
- A claim of binary distribution for targets that have not been built and tested.
- Unverified features added only to make help output or documentation appear complete.
- Unrelated refactoring, stylistic rewrites, or speculative optimization.

If an earlier-phase defect blocks release readiness, make the smallest justified correction, add a regression test, and document why the change belongs in Phase 9. Do not use a blocker as justification for unrelated changes.

---

## 2. Required source of truth and preflight rules

Before editing code, inspect the current repository and follow the project's own source-of-truth precedence in `AGENTS.md`.

### 2.1 Required reading

Read, where present:

1. `AGENTS.md`
2. `PRD.md`
3. `ARCHITECTURE.md`
4. `README.md`
5. Phase roadmap documents from Phase 0 through Phase 8, especially the latest completed phases
6. Existing changelog/release notes and contribution instructions
7. Relevant source modules, templates, fixtures, tests, and CI workflows

Do not assume every referenced file exists. Record missing documents rather than fabricating their contents.

### 2.2 Required repository inspection

Before making changes:

- Inspect the active branch and current commit.
- Run `git status --short` and preserve all pre-existing user changes.
- Inspect the complete CLI command tree and help output.
- Inspect package/workspace metadata, toolchain files, lockfile, and MSRV declarations.
- Inspect existing CI and release workflows.
- Identify the current manifest schema, compatibility resolver, capability model, template renderer, safe-write policy, and process execution abstraction.
- Inspect tests and fixtures for scaffolding, generators, inspection, database/development workflow, quality commands, documentation helpers, Docker assets, CI assets, and shell completions.
- Generate representative projects in temporary directories when the existing toolchain permits.
- Identify external prerequisites such as Cargo, SQLx CLI, PostgreSQL, Docker, Git, or `cargo-audit` only where the relevant command requires them.
- Record commands run, exit codes, and environmental blockers.

### 2.3 Preflight output

Create or maintain a concise Phase 9 audit report in the location that best matches the repository's documentation conventions. It must include:

- Current branch and commit.
- Working-tree status before edits.
- Feature inventory.
- Existing test/quality commands.
- Supported platform and toolchain claims.
- Missing prerequisites.
- Findings grouped by severity.
- Whether each finding blocks MVP.
- Recommended minimal remediation.
- Features verified as already complete and therefore excluded from reimplementation.

### 2.4 Preflight acceptance criteria

- **P9-AC-001:** Required project instructions and available roadmap documents have been inspected.
- **P9-AC-002:** Existing uncommitted changes are identified and preserved.
- **P9-AC-003:** The agent distinguishes verified behavior from documentation claims and assumptions.
- **P9-AC-004:** Every release-blocking finding includes evidence and a proposed minimal fix.
- **P9-AC-005:** Completed features are not reimplemented without a demonstrated defect.

---

## 3. Implementation principles

### 3.1 Reuse existing architecture

Reuse existing project components when appropriate:

- CLI argument definitions and output policy.
- Application/use-case orchestration.
- Project-root discovery and manifest loading.
- Compatibility resolver and capability rules.
- Template metadata and rendering.
- Generation planning and safe filesystem writer.
- Existing process runner and external-tool detection.
- Existing error types and exit-code mapping.
- Existing fixtures, test helpers, and temporary-directory utilities.

Do not duplicate support-matrix logic across commands. Do not introduce new abstraction layers unless a concrete, verified need exists.

### 3.2 Evidence-based status

Every task must be reported as one of:

- `DONE` — implemented or verified with evidence.
- `PARTIAL` — some acceptance criteria are complete; others remain open.
- `BLOCKED` — cannot be completed because a named prerequisite or external dependency is unavailable.
- `NOT DONE` — not attempted or not completed.

A feature is not `DONE` merely because a file exists, a command appears in `--help`, a test was written, or a workflow was configured. The relevant behavior must be exercised or otherwise verified.

### 3.3 Safe operations

- Preserve user-owned files by default.
- Detect conflicts before writing generated files.
- Respect dry-run behavior where supported.
- Do not write outside the selected project root.
- Pass executable names and arguments separately to process APIs.
- Never interpolate untrusted input into shell command strings.
- Never dump all environment variables.
- Never expose secrets in logs or test fixtures.
- Do not install tools, mutate the user's environment, publish, tag, or deploy automatically.
- Report partial success and failure honestly.

### 3.4 Minimal-change policy

- Fix release-blocking issues before low-priority cleanup.
- Prefer focused regression tests over broad rewrites.
- Avoid public API changes unless required and justified.
- Avoid dependency additions unless necessary.
- Respect the repository's MSRV and declared target platforms.
- Do not change behavior merely to satisfy an arbitrary preference.
- If a change is outside Phase 9, document it as deferred work.

---

## 4. Task dependency map

Recommended execution order:

1. Task 9.0 — Preflight Audit & Phase Verification
2. Task 9.1 — MVP Scope Freeze & Support Matrix
3. Task 9.2 — Core CLI Hardening
4. Task 9.3 — Filesystem Safety & Destructive Operation Review
5. Task 9.4 — Generated Project Reliability
6. Task 9.5 — Generator & Compatibility Regression
7. Task 9.6 — Security & Dependency Audit
8. Task 9.7 — Cross-Platform & Environment Compatibility
9. Task 9.8 — Performance & Resource Usage
10. Task 9.9 — Error Recovery & Process Lifecycle
11. Task 9.10 — Release Metadata & Versioning
12. Task 9.11 — Distribution & Installation Verification
13. Task 9.12 — Release Pipeline & Supply-Chain Verification
14. Task 9.13 — Release Candidate & MVP Acceptance
15. Task 9.14 — Documentation, Changelog & Known Limitations
16. Task 9.15 — Final Quality Gate & Phase Closure

Tasks 9.2–9.9 may be worked on in parallel only after the scope and architecture are understood. Tasks 9.10–9.12 can begin earlier if they do not depend on unresolved version or packaging decisions, but their final acceptance depends on a stable candidate. Tasks 9.13–9.15 occur after release-blocking defects are resolved.

---

## 5. Task 9.0 — Preflight Audit & Phase Verification

### Objective

Establish the actual state of the repository and verify which earlier-phase capabilities are complete, incomplete, or blocked.

### Subtasks

- [ ] **9.0.1** Read repository instructions and available project specifications.
- [ ] **9.0.2** Inspect repository structure, modules, templates, tests, fixtures, and workflows.
- [ ] **9.0.3** Record branch, commit, and `git status --short` before editing.
- [ ] **9.0.4** Inspect top-level and nested CLI help.
- [ ] **9.0.5** Inventory all implemented commands and flags.
- [ ] **9.0.6** Verify the behavior of representative commands rather than trusting their presence.
- [ ] **9.0.7** Inspect manifest parsing, schema validation, compatibility resolution, capability rules, and template metadata.
- [ ] **9.0.8** Inspect generation planning, safe writes, dry-run, and conflict handling.
- [ ] **9.0.9** Inspect process execution, external-tool checks, stdout/stderr handling, and exit-code mapping.
- [ ] **9.0.10** Inspect Phase 7 database/development workflows without redesigning them.
- [ ] **9.0.11** Inspect Phase 8 quality commands, docs helper, Docker/Compose generation, CI generation, shell completion, and release workflow preparation.
- [ ] **9.0.12** Record required external tools and services.
- [ ] **9.0.13** Run existing baseline checks where possible.
- [ ] **9.0.14** Create an evidence-based audit report.
- [ ] **9.0.15** Classify findings by severity and MVP impact.

### Acceptance criteria

- **P9-AC-006:** The audit lists the actual command and feature inventory.
- **P9-AC-007:** Existing quality checks have recorded results or explicit blockers.
- **P9-AC-008:** The audit distinguishes code defects, documentation drift, missing prerequisites, and unsupported features.
- **P9-AC-009:** No unrelated refactor begins before the audit is reviewed.
- **P9-AC-010:** The original working-tree changes remain intact.

---

## 6. Task 9.1 — MVP Scope Freeze & Support Matrix

### Objective

Define exactly what the MVP supports and prevent unsupported combinations from being presented as available.

### Subtasks

- [ ] **9.1.1** Inventory commands, subcommands, options, templates, frameworks, databases, ORMs, and capabilities.
- [ ] **9.1.2** Compare the inventory against the PRD and architecture documents.
- [ ] **9.1.3** Verify each claimed combination against the compatibility resolver and generated output.
- [ ] **9.1.4** Mark each feature as `supported`, `unsupported`, `partial`, or `blocked`.
- [ ] **9.1.5** Identify placeholder commands, stubs, no-op paths, and incomplete implementations.
- [ ] **9.1.6** Decide the exact MVP feature set based on working implementation and product requirements.
- [ ] **9.1.7** Ensure unsupported options are rejected before filesystem mutation.
- [ ] **9.1.8** Record post-MVP ideas in a backlog instead of implementing them.
- [ ] **9.1.9** Align help text and user-facing messages with the frozen support matrix.

### Acceptance criteria

- **P9-AC-011:** A support matrix exists in a project-appropriate location.
- **P9-AC-012:** Every combination advertised as supported has implementation or test evidence.
- **P9-AC-013:** Unsupported combinations fail clearly and do not partially generate a project.
- **P9-AC-014:** Help output does not advertise unimplemented features as complete.
- **P9-AC-015:** The MVP scope is frozen before unrelated feature work begins.

---

## 7. Task 9.2 — Core CLI Hardening

### Objective

Make command parsing, output, validation, and exit behavior consistent and predictable.

### Subtasks

- [ ] **9.2.1** Audit required arguments, optional arguments, defaults, and invalid values.
- [ ] **9.2.2** Audit conflicting flags and invalid combinations.
- [ ] **9.2.3** Verify `--help` and `--version`.
- [ ] **9.2.4** Verify global flags such as verbose, quiet, and no-color where implemented.
- [ ] **9.2.5** Verify project-root and `--path` handling.
- [ ] **9.2.6** Verify text output, JSON output, and machine-readable output contracts where supported.
- [ ] **9.2.7** Verify exit codes for success, validation errors, conflicts, missing tools, external failures, and internal failures.
- [ ] **9.2.8** Ensure errors are actionable and do not leak internal stack details unnecessarily.
- [ ] **9.2.9** Ensure quiet mode suppresses nonessential output without hiding failures.
- [ ] **9.2.10** Ensure verbose mode adds useful diagnostic context without printing secrets.
- [ ] **9.2.11** Ensure command failures never emit misleading success summaries.
- [ ] **9.2.12** Add regression tests for discovered inconsistencies.

### Acceptance criteria

- **P9-AC-016:** Invalid arguments fail with a nonzero exit status and actionable message.
- **P9-AC-017:** Success and failure exit statuses are consistent across commands.
- **P9-AC-018:** JSON output is parseable and does not contain incidental human-readable logs when the contract promises pure JSON.
- **P9-AC-019:** Quiet mode does not hide actionable failures.
- **P9-AC-020:** Verbose output does not expose credentials or secret values.
- **P9-AC-021:** Regression tests cover all fixed CLI behavior defects.

---

## 8. Task 9.3 — Filesystem Safety & Destructive Operation Review

### Objective

Protect project files from path traversal, accidental overwrites, and incomplete writes.

### Subtasks

- [ ] **9.3.1** Audit path normalization and traversal protections.
- [ ] **9.3.2** Verify all generated paths remain inside the intended target root.
- [ ] **9.3.3** Audit symlink handling and any platform-specific path edge cases relevant to the existing writer.
- [ ] **9.3.4** Verify the default conflict policy for existing files and directories.
- [ ] **9.3.5** Verify dry-run performs no target filesystem mutation.
- [ ] **9.3.6** Verify conflict detection occurs before destructive writes wherever the architecture allows.
- [ ] **9.3.7** Verify temporary-file cleanup and failure handling.
- [ ] **9.3.8** Test invalid project/resource names and path-like input.
- [ ] **9.3.9** Test repeated execution of the same generation command.
- [ ] **9.3.10** Verify permission and I/O errors are surfaced clearly.
- [ ] **9.3.11** Verify a partial failure is reported as partial/failed rather than successful.
- [ ] **9.3.12** Add regression tests for every safety defect fixed.

### Acceptance criteria

- **P9-AC-022:** Path traversal inputs cannot write outside the permitted target.
- **P9-AC-023:** Existing user-owned files are not silently overwritten.
- **P9-AC-024:** Dry-run leaves the target filesystem unchanged.
- **P9-AC-025:** Invalid input is rejected before unsafe writes begin.
- **P9-AC-026:** File conflicts and I/O failures produce accurate errors.
- **P9-AC-027:** Safety-related regression tests pass on supported test platforms.

---

## 9. Task 9.4 — Generated Project Reliability

### Objective

Verify that projects produced by the CLI are coherent and buildable for all templates included in the MVP.

### Subtasks

- [ ] **9.4.1** Generate each supported template in a fresh temporary directory.
- [ ] **9.4.2** Validate generated Cargo metadata.
- [ ] **9.4.3** Run `cargo check` for each generated project when dependencies and network/cache conditions permit.
- [ ] **9.4.4** Run generated-project tests where available.
- [ ] **9.4.5** Verify folder layout and expected entry points.
- [ ] **9.4.6** Verify `.rustrs/config.toml` or the actual manifest location and schema.
- [ ] **9.4.7** Verify dependency declarations match the selected framework/database/ORM.
- [ ] **9.4.8** Verify environment examples contain placeholders only, not real credentials.
- [ ] **9.4.9** Verify generated Docker, Compose, CI, and documentation assets where those assets are enabled.
- [ ] **9.4.10** Verify the CLI can rediscover a generated project.
- [ ] **9.4.11** Verify unsupported features are not emitted accidentally.
- [ ] **9.4.12** Save reproducible test fixtures or scripts for the supported template matrix.
- [ ] **9.4.13** Record environment-dependent tests separately from source/build defects.

### Acceptance criteria

- **P9-AC-028:** Every template claimed as MVP-supported has a recorded generation result.
- **P9-AC-029:** Generated Cargo projects pass metadata and compilation checks where prerequisites are available.
- **P9-AC-030:** Generated tests pass where present and runnable.
- **P9-AC-031:** Generated manifest and dependency declarations match the selected configuration.
- **P9-AC-032:** No generated environment file contains real secrets.
- **P9-AC-033:** External-service prerequisites and blocked tests are explicitly documented.

---

## 10. Task 9.5 — Generator & Compatibility Regression

### Objective

Ensure code generators remain consistent with the selected template, manifest, and capability model.

### Subtasks

- [ ] **9.5.1** Inventory generators actually implemented and included in MVP.
- [ ] **9.5.2** Test each generator using supported templates and capabilities.
- [ ] **9.5.3** Test valid, invalid, reserved, and path-like names.
- [ ] **9.5.4** Test generation when target files are absent.
- [ ] **9.5.5** Test conflict behavior when target files already exist.
- [ ] **9.5.6** Test dry-run and generation-plan output.
- [ ] **9.5.7** Verify generated imports and module registrations reference existing code.
- [ ] **9.5.8** Verify dependency changes are valid and consistent with the manifest.
- [ ] **9.5.9** Verify unsupported combinations are rejected.
- [ ] **9.5.10** Test deterministic behavior for the same input and configuration where determinism is part of the contract.
- [ ] **9.5.11** Add regression fixtures for bugs discovered in earlier phases.
- [ ] **9.5.12** Avoid adding new generator types during this task.

### Acceptance criteria

- **P9-AC-034:** Every in-scope generator has at least one meaningful success test.
- **P9-AC-035:** Invalid names and unsupported combinations are rejected safely.
- **P9-AC-036:** Generated code does not contain known broken imports or references.
- **P9-AC-037:** Conflict and dry-run behavior follows the documented contract.
- **P9-AC-038:** Generator regressions are covered by automated tests.

---

## 11. Task 9.6 — Security & Dependency Audit

### Objective

Assess dependency vulnerabilities, unsafe input handling, secret exposure, and security-sensitive generated assets.

### Subtasks

- [ ] **9.6.1** Run `cargo audit` if installed and configured.
- [ ] **9.6.2** Run other repository-configured dependency/security checks.
- [ ] **9.6.3** Review relevant direct and transitive dependency findings.
- [ ] **9.6.4** Check whether findings affect shipped code, build tooling, or generated projects.
- [ ] **9.6.5** Audit subprocess execution and argument handling.
- [ ] **9.6.6** Audit untrusted input in paths, templates, configuration, and generated file contents.
- [ ] **9.6.7** Audit logging and diagnostic output for secrets.
- [ ] **9.6.8** Review Dockerfile, Compose, GitHub Actions, and GitLab CI templates for risky defaults.
- [ ] **9.6.9** Review workflow permissions and credential exposure.
- [ ] **9.6.10** Avoid automatically applying security fixes that change major versions or break MSRV.
- [ ] **9.6.11** Remediate release-blocking findings with focused changes and tests.
- [ ] **9.6.12** Record unresolved findings, severity, impact, mitigation, and owner/decision required.

### Acceptance criteria

- **P9-AC-039:** Security checks have recorded outcomes or explicit blocked reasons.
- **P9-AC-040:** Known Critical/High findings are fixed or have an explicit release-blocking/disposition decision.
- **P9-AC-041:** No secrets are introduced into source, fixtures, generated assets, or logs.
- **P9-AC-042:** External commands do not depend on unsafe shell interpolation.
- **P9-AC-043:** Dependency changes preserve supported MSRV and compile/test expectations.
- **P9-AC-044:** Unresolved security risks are documented rather than silently ignored.

---

## 12. Task 9.7 — Cross-Platform & Environment Compatibility

### Objective

Verify platform and toolchain claims against the project's actual support commitments.

### Subtasks

- [ ] **9.7.1** Establish the MVP platform matrix from repository configuration and CI.
- [ ] **9.7.2** Verify builds for declared operating systems and architectures where CI or available environments permit.
- [ ] **9.7.3** Audit path separators, executable lookup, permissions, and filesystem behavior.
- [ ] **9.7.4** Verify missing external tools produce useful diagnostics.
- [ ] **9.7.5** Verify subprocess invocation on each supported platform.
- [ ] **9.7.6** Verify stable Rust and the declared MSRV.
- [ ] **9.7.7** Verify shell completion for shells claimed as supported.
- [ ] **9.7.8** Verify terminal color/no-color behavior where feasible.
- [ ] **9.7.9** Record unsupported platforms and environmental blockers.
- [ ] **9.7.10** Correct inaccurate platform claims in documentation.

### Acceptance criteria

- **P9-AC-045:** The supported platform matrix is explicit.
- **P9-AC-046:** Every platform claim has build/test evidence or is clearly qualified.
- **P9-AC-047:** MSRV claims are verified against the actual code and dependency graph.
- **P9-AC-048:** Missing tools and unsupported environments fail with actionable messages.
- **P9-AC-049:** Platform-specific gaps are documented without falsely claiming support.

---

## 13. Task 9.8 — Performance & Resource Usage

### Objective

Detect significant, reproducible performance or resource problems without turning Phase 9 into an optimization project.

### Subtasks

- [ ] **9.8.1** Measure startup latency for common commands.
- [ ] **9.8.2** Measure scaffolding time for representative supported templates.
- [ ] **9.8.3** Inspect unnecessary repeated filesystem scans or manifest parsing.
- [ ] **9.8.4** Verify subprocesses terminate and are not unintentionally left running.
- [ ] **9.8.5** Review watcher/process lifecycle only when measurements or bugs indicate a problem.
- [ ] **9.8.6** Optimize only bottlenecks supported by measurements.
- [ ] **9.8.7** Record measurement environment and methodology.
- [ ] **9.8.8** Avoid introducing benchmark infrastructure that is disproportionate to MVP needs.

### Acceptance criteria

- **P9-AC-050:** Performance findings include reproducible evidence or measurements.
- **P9-AC-051:** No large optimization is made solely on speculation.
- **P9-AC-052:** Performance changes preserve correctness and filesystem safety.
- **P9-AC-053:** Known material performance limitations are documented.

---

## 14. Task 9.9 — Error Recovery & Process Lifecycle

### Objective

Ensure command failures, external tool failures, and interruptions do not create misleading state or leave unsafe partial results.

### Subtasks

- [ ] **9.9.1** Test behavior when required external tools are missing.
- [ ] **9.9.2** Test external processes that return nonzero exit codes.
- [ ] **9.9.3** Test invalid working directories and inaccessible paths.
- [ ] **9.9.4** Test filesystem failures and permission errors where practical.
- [ ] **9.9.5** Verify stdout and stderr forwarding follows the existing process contract.
- [ ] **9.9.6** Verify process status and cleanup on normal completion and failure.
- [ ] **9.9.7** Review managed development state files where relevant.
- [ ] **9.9.8** Test repeated lifecycle commands when applicable.
- [ ] **9.9.9** Ensure cleanup does not delete files or processes the CLI does not own.
- [ ] **9.9.10** Make only the smallest Phase 7 changes necessary to fix verified release blockers.

### Acceptance criteria

- **P9-AC-054:** External process failure propagates as failure.
- **P9-AC-055:** Missing tools are identified by name and have actionable guidance.
- **P9-AC-056:** Cleanup targets only resources owned by the CLI.
- **P9-AC-057:** Repeated lifecycle operations do not create stale or misleading state.
- **P9-AC-058:** Any Phase 7 fixes have regression coverage and a documented reason.

---

## 15. Task 9.10 — Release Metadata & Versioning

### Objective

Make package version, CLI version output, and release documentation consistent.

### Subtasks

- [ ] **9.10.1** Audit version fields in `Cargo.toml`, workspace manifests, lockfile, and version output.
- [ ] **9.10.2** Confirm the intended MVP version with the repository's versioning convention.
- [ ] **9.10.3** Verify package metadata, license declaration, repository URL, description, and other required publication metadata where applicable.
- [ ] **9.10.4** Verify the executable's `--version` output matches the intended package version.
- [ ] **9.10.5** Prepare changelog or release notes based on actual changes.
- [ ] **9.10.6** Document user-visible behavior changes and any breaking changes.
- [ ] **9.10.7** Document the supported Rust version and target platforms.
- [ ] **9.10.8** Do not claim a version has been released before the release operation succeeds.
- [ ] **9.10.9** Do not create or push tags automatically as part of validation.

### Acceptance criteria

- **P9-AC-059:** Package and CLI version metadata are consistent.
- **P9-AC-060:** Required package metadata is valid for the chosen distribution channel.
- **P9-AC-061:** Release notes match implemented and verified behavior.
- **P9-AC-062:** Version/tag/publication operations remain explicit and are not triggered by ordinary tests.

---

## 16. Task 9.11 — Distribution & Installation Verification

### Objective

Verify at least one explicitly selected installation path from a clean environment.

### Subtasks

- [ ] **9.11.1** Decide the MVP distribution channel, such as installation from source with Cargo.
- [ ] **9.11.2** Document all required prerequisites.
- [ ] **9.11.3** Test the chosen installation procedure in a clean or isolated environment.
- [ ] **9.11.4** Verify the installed executable resolves from the expected path.
- [ ] **9.11.5** Run `rustrs --help` and `rustrs --version` after installation.
- [ ] **9.11.6** Run a minimal scaffolding smoke test after installation.
- [ ] **9.11.7** If prebuilt binaries are explicitly included, define target triples and build them in CI.
- [ ] **9.11.8** For binary archives, verify names, archive layout, checksums, and target metadata.
- [ ] **9.11.9** Document reinstall/upgrade behavior only where supported.
- [ ] **9.11.10** Document common installation failures and troubleshooting.
- [ ] **9.11.11** Do not add binary distribution merely because it is conventional; follow the agreed MVP scope.

### Acceptance criteria

- **P9-AC-063:** At least one documented installation route is tested end to end.
- **P9-AC-064:** The installed binary runs basic commands and creates a supported project.
- **P9-AC-065:** Binary targets are only advertised when they are built and verified.
- **P9-AC-066:** Installation instructions do not require undocumented manual steps.
- **P9-AC-067:** Installation limitations are explicit.

---

## 17. Task 9.12 — Release Pipeline & Supply-Chain Verification

### Objective

Verify release automation is safe, traceable, and testable without accidentally publishing a public release.

### Subtasks

- [ ] **9.12.1** Inspect the release workflow prepared in Phase 8.
- [ ] **9.12.2** Verify triggers and branch/tag conditions.
- [ ] **9.12.3** Review token permissions and least-privilege configuration.
- [ ] **9.12.4** Verify quality gates run before packaging or publication.
- [ ] **9.12.5** Verify workflow failure stops downstream release stages.
- [ ] **9.12.6** Verify artifacts are tied to the expected commit/version.
- [ ] **9.12.7** Verify checksums or integrity metadata for distributed artifacts where applicable.
- [ ] **9.12.8** Verify secrets are not echoed, uploaded, or included in archives.
- [ ] **9.12.9** Verify publication requires the intended explicit trigger and authorization.
- [ ] **9.12.10** Run a safe workflow validation or dry run that does not publish publicly.
- [ ] **9.12.11** Document any repository settings or secrets the maintainer must configure manually.
- [ ] **9.12.12** Do not create tags, publish packages, or create a public release during this task.

### Acceptance criteria

- **P9-AC-068:** Release workflow configuration is validated.
- **P9-AC-069:** Required quality checks gate the release process.
- **P9-AC-070:** Publication cannot occur accidentally from an ordinary test or pull request.
- **P9-AC-071:** Artifact provenance and integrity checks are documented and tested where applicable.
- **P9-AC-072:** Required maintainer configuration is documented without exposing secrets.

---

## 18. Task 9.13 — Release Candidate & MVP Acceptance

### Objective

Run end-to-end acceptance checks against a stable release candidate.

### Subtasks

- [ ] **9.13.1** Identify the candidate commit and verify the intended working tree is clean before final validation.
- [ ] **9.13.2** Run all mandatory Rust quality gates.
- [ ] **9.13.3** Run unit, integration, regression, and CLI smoke tests.
- [ ] **9.13.4** Generate all MVP-supported templates.
- [ ] **9.13.5** Run generated-project metadata and build checks.
- [ ] **9.13.6** Exercise the primary user journey from installation to successful scaffolding.
- [ ] **9.13.7** Exercise supported inspection, database, development, quality, and generated-asset commands where applicable.
- [ ] **9.13.8** Validate Docker/Compose and CI assets structurally; run them where prerequisites and safety permit.
- [ ] **9.13.9** Verify help text and README examples.
- [ ] **9.13.10** Verify version metadata, release notes, and packaging output.
- [ ] **9.13.11** Record all tests not run and the reason.
- [ ] **9.13.12** Produce an MVP readiness report with blockers, risks, and recommendation.

### Acceptance criteria

- **P9-AC-073:** All mandatory checks have recorded results.
- **P9-AC-074:** Primary MVP user journeys pass in the supported environment.
- **P9-AC-075:** All claimed templates have recorded validation outcomes.
- **P9-AC-076:** Skipped tests are not reported as passed.
- **P9-AC-077:** Release-blocking findings are resolved or explicitly prevent a ready decision.
- **P9-AC-078:** The readiness report contains enough evidence for a maintainer to make a release decision.

---

## 19. Task 9.14 — Documentation, Changelog & Known Limitations

### Objective

Ensure users can install, understand, and troubleshoot the MVP without relying on undocumented assumptions.

### Subtasks

- [ ] **9.14.1** Update README to match the actual command and feature inventory.
- [ ] **9.14.2** Verify every documented command and option against CLI help and implementation.
- [ ] **9.14.3** Document installation and prerequisite tools.
- [ ] **9.14.4** Document the template/framework/database/ORM support matrix.
- [ ] **9.14.5** Document expected environment variables without including secrets.
- [ ] **9.14.6** Document common errors and troubleshooting.
- [ ] **9.14.7** Prepare changelog or release notes.
- [ ] **9.14.8** Document known limitations and deferred post-MVP features.
- [ ] **9.14.9** Verify internal links and roadmap references.
- [ ] **9.14.10** Remove or qualify stale claims and examples.
- [ ] **9.14.11** Ensure contributor and release instructions are consistent with actual workflow.
- [ ] **9.14.12** Avoid claiming the product replaces Cargo or other external tools.

### Acceptance criteria

- **P9-AC-079:** README feature claims match verified implementation.
- **P9-AC-080:** Documented examples use valid syntax and supported options.
- **P9-AC-081:** Prerequisites and external-tool dependencies are clear.
- **P9-AC-082:** Known limitations are explicit.
- **P9-AC-083:** Internal documentation links and roadmap references are valid or intentionally marked unavailable.

---

## 20. Task 9.15 — Final Quality Gate & Phase Closure

### Objective

Close Phase 9 with a reproducible quality report and an honest statement of release readiness.

### Subtasks

- [ ] **9.15.1** Run `cargo fmt --check`.
- [ ] **9.15.2** Run `cargo check`.
- [ ] **9.15.3** Run `cargo test`.
- [ ] **9.15.4** Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] **9.15.5** Run `cargo metadata --no-deps --format-version 1`.
- [ ] **9.15.6** Run `git diff --check`.
- [ ] **9.15.7** Run all relevant integration, regression, smoke, template, and release-validation tests.
- [ ] **9.15.8** Run security checks that are available and in scope.
- [ ] **9.15.9** Inspect `git diff --stat`, `git diff`, and `git status --short`.
- [ ] **9.15.10** Check for secrets, temporary files, generated artifacts, and unintended repository changes.
- [ ] **9.15.11** Record exact commands, exit codes, environment, and any skipped checks.
- [ ] **9.15.12** Produce the final Phase 9 report.
- [ ] **9.15.13** Mark Phase 9 complete only if all mandatory criteria pass or the project owner explicitly accepts documented exceptions.
- [ ] **9.15.14** Do not commit, push, tag, publish, or deploy unless separately and explicitly requested.

### Acceptance criteria

- **P9-AC-084:** Every mandatory quality gate has an evidence-based result.
- **P9-AC-085:** Repository diff and working-tree status have been reviewed.
- **P9-AC-086:** No unintended secrets or temporary artifacts are included.
- **P9-AC-087:** Every task has a final status and supporting evidence.
- **P9-AC-088:** Phase completion is not claimed while a release-blocking issue remains unresolved.
- **P9-AC-089:** No public release operation occurred as a side effect of validation.

---

## 21. Mandatory quality-gate commands

Run these from the appropriate workspace/package root. If the repository is a workspace, verify the commands cover the intended packages and targets.

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo metadata --no-deps --format-version 1
git diff --check
git status --short
```

Additional checks may include, when available and applicable:

```bash
cargo audit
cargo tree --locked
```

The agent must not report a command as passed if it was not executed. If a tool is unavailable, record the command as `BLOCKED`, include the missing prerequisite, and continue with checks that can still run.

### 21.1 Generated-project checks

For each supported template, run the CLI in an isolated temporary directory and validate the generated project. Example commands must be adapted to the actual CLI syntax and template matrix:

```bash
rustrs --help
rustrs --version
rustrs new <temporary-project-name> --defaults
```

Then, from the generated project's directory:

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
```

Do not run commands against the user's real project when a temporary fixture can safely validate the behavior. Do not assume every generated project can run database-dependent tests without services or configuration.

---

## 22. Security severity and release-blocking policy

Use this baseline unless the repository already defines a stricter policy.

| Severity | Example                                                                                                                          | Default disposition                                                                               |
| -------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Critical | Arbitrary file overwrite outside the target root; secret exposure; severe exploitable execution path                             | Blocks release until fixed and retested                                                           |
| High     | Data loss, unsafe command execution, serious generated-project defect, major dependency vulnerability affecting shipped behavior | Blocks release unless the project owner explicitly approves a documented exception and mitigation |
| Medium   | Limited reliability or security issue with a reasonable workaround                                                               | Must be documented and assessed for MVP impact                                                    |
| Low      | Cosmetic issue, minor message inconsistency, low-impact cleanup                                                                  | May be deferred if documented                                                                     |

Do not downgrade severity simply to pass the phase. Base severity on reproducible impact and the actual MVP threat model.

---

## 23. Required final report format

At the end of Phase 9, the AI agent must provide a concise but complete report with the following sections.

### 23.1 Summary

- Overall status: `DONE`, `PARTIAL`, `BLOCKED`, or `NOT DONE`.
- MVP readiness recommendation: `READY`, `READY WITH ACCEPTED LIMITATIONS`, or `NOT READY`.
- Summary of the most important changes.

### 23.2 Task status

For every task from 9.0 through 9.15, include:

- Task ID and title.
- Status.
- Files changed.
- Acceptance criteria IDs completed.
- Evidence or reason for incomplete status.

### 23.3 Verification results

For every command/test, include:

- Exact command.
- Working directory.
- Result and exit code.
- Important output or failure summary.
- Whether it was skipped or blocked and why.

### 23.4 Security and known risks

- Findings by severity.
- Fixes and regression tests.
- Unresolved risks and mitigations.
- Missing external prerequisites.
- Platform or toolchain limitations.

### 23.5 Release readiness

- Version metadata status.
- Installation/distribution path verified.
- Release workflow validation result.
- Documentation and changelog status.
- Whether a public release was performed. Expected answer during this phase: **no**, unless separately authorized.

### 23.6 Repository state

- Branch and candidate commit.
- `git status --short`.
- Diff summary.
- Confirmation that pre-existing changes were preserved.
- Confirmation that no unrequested commit, push, tag, publication, or deployment occurred.

### 23.7 Remaining work

List every unresolved task, blocker, deferred feature, and decision needed from the project owner. Do not bury blockers in a general summary.

---

## 24. Definition of Done

Phase 9 is complete only when all of the following are true:

- [ ] The repository's actual state and MVP scope have been audited.
- [ ] The support matrix accurately reflects working functionality.
- [ ] Core CLI input validation, output, and exit behavior are consistent.
- [ ] Filesystem operations protect user-owned files and remain within the target root.
- [ ] Every supported template has a recorded generation and validation result.
- [ ] In-scope generators have regression coverage.
- [ ] Security checks have recorded outcomes and release-blocking risks are resolved or explicitly dispositioned.
- [ ] Platform and MSRV claims have evidence or clear limitations.
- [ ] Performance review is evidence-based and does not trigger unrelated optimization.
- [ ] External-process failure and cleanup behavior are verified.
- [ ] Package/version metadata and release notes are consistent.
- [ ] At least one selected distribution path has been tested end to end.
- [ ] Release automation is validated without accidental public publication.
- [ ] Documentation matches the implementation and lists known limitations.
- [ ] All mandatory quality gates have evidence-based results.
- [ ] The final report contains task status, acceptance criteria, test evidence, security findings, and repository state.
- [ ] No release-blocking issue is silently ignored.
- [ ] No unrequested public release action has occurred.

If any mandatory condition is unmet, report the phase as `PARTIAL`, `BLOCKED`, or `NOT DONE` instead of claiming completion.

---

## 25. Mandatory instructions for the AI coding agent

Follow these instructions throughout Phase 9:

1. **Inspect before editing.** Read repository instructions, specifications, phase roadmaps, code, tests, and working-tree status.
2. **Use the active working tree.** Do not infer local completion solely from a public GitHub snapshot or stale README.
3. **Stay in scope.** Do not add unrelated features or expand the framework/database/ORM support matrix.
4. **Do not duplicate completed work.** Verify existing functionality and only change it when evidence demonstrates a defect.
5. **Preserve user changes.** Never discard, reset, or overwrite pre-existing work.
6. **Prefer minimal fixes.** Tie each implementation change to a task, finding, acceptance criterion, and regression test where practical.
7. **Prove claims.** A file's existence is not proof that the feature works.
8. **Do not fabricate test results.** Report exact commands and outcomes; mark unavailable checks as blocked.
9. **Protect secrets and user data.** Never log credentials or include real secrets in generated assets.
10. **Do not perform public release actions.** No automatic commits, pushes, tags, publishing, or deployments.
11. **Keep documentation truthful.** Remove or qualify claims that cannot be verified.
12. **Report blockers early.** If a required product decision or external permission is missing, state what is needed and continue with independent tasks.
13. **Do not silently change the architecture.** If a design change appears necessary, explain the evidence and choose the smallest compatible fix.
14. **Do not mark Phase 9 complete prematurely.** Use the Definition of Done and final report requirements.

### Expected execution behavior

The agent should work incrementally:

1. Produce the preflight audit.
2. Identify and prioritize release-blocking findings.
3. Implement focused fixes with regression coverage.
4. Run relevant checks after each change.
5. Run the full final quality gate after fixes are complete.
6. Update documentation and release metadata.
7. Produce the required final report.

Do not attempt all tasks in one enormous, unverified edit. Prefer small, testable batches and preserve evidence after each batch.

---

## 26. Final scope reminder

Phase 9 exists to make the existing `rustrs` MVP reliable and release-ready—not to turn it into a larger product.

The final result should be a verified and well-documented CLI whose supported commands and generated projects behave predictably, whose safety and security risks are understood, and whose selected distribution procedure can be repeated by a maintainer.

**Phase 9 completion means release readiness has been demonstrated. It does not automatically authorize or imply that a public release has been published.**

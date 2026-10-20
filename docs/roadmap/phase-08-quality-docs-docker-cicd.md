# Phase 8 — Quality, Docs, Docker & CI/CD

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 8 — Quality, Docs, Docker & CI/CD  
> **Recommended repository path:** `docs/roadmap/phase-08-quality-docs-docker-cicd.md`  
> **Status:** Planned  
> **Primary outcome:** Deliver reliable quality commands, OpenAPI documentation helpers, safe Docker/Compose generation, CI/CD workflow generation, shell completion, and a prepared release workflow—without implementing Phase 9 release execution or expanding beyond the MVP support matrix.

---

## 1. Purpose and boundaries

Phase 8 adds quality tooling and delivery-related assets to the `rustrs` CLI. It builds on the previous phases rather than rewriting working features.

### In scope

1. `rustrs check`, `rustrs lint`, `rustrs test`, `rustrs format`, and `rustrs audit`.
2. An OpenAPI documentation helper for configurations that actually support OpenAPI.
3. Dockerfile and `.dockerignore` generation.
4. Docker Compose generation based on the manifest and enabled capabilities.
5. GitHub Actions and GitLab CI workflow generation.
6. Shell completion for explicitly supported shells.
7. Preparation of a release workflow, without publishing a release in this phase.
8. Unit, integration, structural, and regression tests.
9. Documentation updates that accurately reflect implemented functionality.

### Out of scope

- Phase 9 release hardening, actual release execution, tagging, binary distribution, or publishing.
- Application deployment to servers or cloud providers.
- A hosted CI/CD service or remote deployment orchestration.
- A new database migration system or redesign of Phase 7 database commands.
- A new process execution architecture if Phase 7 already provides the required behavior.
- Automatic installation/updating of tools or dependencies.
- Automatic mutation of shell startup files.
- Automatically starting containers during asset generation.
- Unsupported framework/database/ORM combinations.
- Automatic security fixes, dependency upgrades, or source rewrites during audits.
- Stub commands or placeholder files presented as completed functionality.

If an earlier-phase defect blocks progress, make the smallest justified fix, add regression coverage, and report why it was necessary. Do not use a blocker to justify unrelated refactoring.

---

## 2. Required reading and source of truth

Before editing, read in this order:

1. `PRD.md`
2. `ARCHITECTURE.md`
3. `AGENTS.md`
4. Phase roadmap documents through the latest completed phase, including Phase 7 if present
5. Relevant source code, templates, tests, README, and existing CI configuration

Follow the source-of-truth precedence in `AGENTS.md`.

### 2.1 Preflight audit

Before implementation:

- Inspect repository structure and relevant modules.
- Run `git status --short`; preserve all pre-existing changes.
- Inspect the current CLI command tree and `--help` output.
- Verify Phase 7 process execution, working-directory selection, output forwarding, tool discovery, and exit-code mapping.
- Inspect manifest loading, compatibility resolver, capability rules, template metadata, and filesystem safety.
- Generate representative projects in temporary directories for each template claimed to be supported.
- Inspect existing OpenAPI, Docker, Compose, CI/CD, completion, and release-related files.
- Record which features are implemented, partial, missing, or blocked.
- Verify MSRV and toolchain constraints from repository configuration.

Do not assume a command works merely because it appears in help or documentation.

---

## 3. Implementation principles

### 3.1 Reuse existing architecture

Reuse existing components where appropriate:

- CLI command definitions and output policy
- Application/use-case layer
- Project-root discovery and manifest loading
- Compatibility resolver and capability rules
- Template metadata and renderer
- Generation planning and safe filesystem writer
- Phase 7 process execution abstraction
- Existing error types and exit-code mapping
- Existing tests and fixtures

Do not duplicate compatibility rules across commands or templates.

### 3.2 Safety and predictability

- Preserve user-owned files by default.
- Detect file conflicts before applying generation plans.
- Respect `--dry-run` wherever generation supports it.
- Never write outside the selected project root.
- Never interpolate untrusted values into shell command strings.
- Use process APIs with separate executable and argument values.
- Never print secret values or dump the environment.
- Do not install tools, update dependencies, start containers, edit shell startup files, commit, push, tag, publish, or deploy automatically.
- Report partial success and failures accurately.
- Never report a check as passed if it was skipped or could not run.

### 3.3 Support matrix discipline

The public README describes current generated templates as Axum + PostgreSQL + SQLx starter projects, while the PRD describes broader product goals. Do not conflate goals with implemented support.

For each feature:

1. Determine the actual supported combinations from code and tests.
2. Implement for the verified support matrix.
3. Reject unsupported combinations with an actionable message.
4. Do not expose options that generate unbuildable output.
5. Update support-matrix claims only when implementation and tests justify them.

---

## 4. Task map and dependencies

| Task | Area                                    | Dependencies                     |
| ---- | --------------------------------------- | -------------------------------- |
| 8.0  | Preflight audit                         | Existing repository              |
| 8.1  | Shared quality-command contract         | 8.0, Phase 7 process abstraction |
| 8.2  | `rustrs check`                          | 8.1                              |
| 8.3  | `rustrs lint`                           | 8.1                              |
| 8.4  | `rustrs test`                           | 8.1                              |
| 8.5  | `rustrs format`                         | 8.1                              |
| 8.6  | `rustrs audit`                          | 8.1                              |
| 8.7  | OpenAPI documentation helper            | 8.0, manifest/capability model   |
| 8.8  | Dockerfile and `.dockerignore`          | 8.0, template/filesystem engine  |
| 8.9  | Docker Compose generation               | 8.8, capability model            |
| 8.10 | GitHub Actions generation               | 8.1, template/filesystem engine  |
| 8.11 | GitLab CI generation                    | 8.1, template/filesystem engine  |
| 8.12 | Shell completion                        | Actual CLI command tree          |
| 8.13 | Release workflow preparation            | 8.1, package metadata            |
| 8.14 | Integration, security, regression tests | Implemented tasks                |
| 8.15 | Documentation and final audit           | Implemented tasks                |

Tasks may be reordered to fit real repository dependencies, but any deviation must be explained and stay within Phase 8.

---

## 5. Task 8.0 — Preflight audit and Phase 7 verification

### Work items

- [ ] 8.0.1 Read required project and roadmap documents.
- [ ] 8.0.2 Inspect `git status --short`, current branch, and existing diff.
- [ ] 8.0.3 Inspect the CLI command tree and help output.
- [ ] 8.0.4 Verify process execution, working directory, output forwarding, and exit status from Phase 7.
- [ ] 8.0.5 Verify external-tool discovery and failure behavior.
- [ ] 8.0.6 Inspect manifest, compatibility, capability, template, and filesystem safety implementations.
- [ ] 8.0.7 Generate each supported template in an isolated temporary directory.
- [ ] 8.0.8 Inspect existing OpenAPI, Docker, Compose, CI, completion, and release assets.
- [ ] 8.0.9 Record implementation gaps and blockers before editing.

### Acceptance criteria

- [ ] **P8-AC-01:** Existing changes are preserved.
- [ ] **P8-AC-02:** Phase 7 behavior is verified from code/tests, not assumed.
- [ ] **P8-AC-03:** Supported templates and technology combinations are verified.
- [ ] **P8-AC-04:** A preflight report identifies implemented, partial, missing, and blocked features.

---

## 6. Task 8.1 — Shared quality-command execution contract

### Work items

- [ ] 8.1.1 Define a typed internal representation for quality operations only if existing abstractions do not cover the need.
- [ ] 8.1.2 Define each command's prerequisites, executable, arguments, and working directory.
- [ ] 8.1.3 Resolve the target project root explicitly.
- [ ] 8.1.4 Reuse the Phase 7 process runner where suitable; do not create a competing framework.
- [ ] 8.1.5 Forward child stdout/stderr according to existing output policy.
- [ ] 8.1.6 Preserve or consistently map process status using the documented CLI exit-code contract.
- [ ] 8.1.7 Handle missing executables, permission errors, invalid roots, and interrupted processes.
- [ ] 8.1.8 Include useful operation context in errors without exposing secrets.
- [ ] 8.1.9 Support non-interactive use in CI.
- [ ] 8.1.10 Add process-level tests that do not require network access.

### Acceptance criteria

- [ ] **P8-AC-05:** Quality commands follow a consistent execution contract.
- [ ] **P8-AC-06:** Commands operate on the target project, not accidentally on the CLI repository.
- [ ] **P8-AC-07:** Process output and failures are surfaced accurately.
- [ ] **P8-AC-08:** Missing tools and invalid roots fail without panic.
- [ ] **P8-AC-09:** Untrusted input is not interpolated into shell strings.
- [ ] **P8-AC-10:** The existing process abstraction is reused unless a documented gap requires a minimal extension.

---

## 7. Task 8.2 — `rustrs check`

### Work items

- [ ] 8.2.1 Validate the project root and `Cargo.toml`.
- [ ] 8.2.2 Resolve the correct manifest and working directory.
- [ ] 8.2.3 Run the appropriate Cargo check operation with separate arguments.
- [ ] 8.2.4 Forward compiler diagnostics.
- [ ] 8.2.5 Propagate failure status.
- [ ] 8.2.6 Do not run tests, format files, audit dependencies, or modify files as hidden side effects.
- [ ] 8.2.7 Add help examples and integration tests.

### Acceptance criteria

- [ ] **P8-AC-11:** A valid project runs the intended check.
- [ ] **P8-AC-12:** Compilation errors cause a non-success result.
- [ ] **P8-AC-13:** Invalid roots and missing prerequisites produce clear diagnostics.
- [ ] **P8-AC-14:** The command does not silently change files.

---

## 8. Task 8.3 — `rustrs lint`

### Work items

- [ ] 8.3.1 Verify the lint tool and options supported by the project's toolchain.
- [ ] 8.3.2 Reuse project lint configuration instead of inventing a second policy.
- [ ] 8.3.3 Execute lint from the correct project root.
- [ ] 8.3.4 Forward diagnostics and propagate the result.
- [ ] 8.3.5 Distinguish missing tooling from lint findings.
- [ ] 8.3.6 Do not add broad lint suppressions to force success.
- [ ] 8.3.7 Document the tool and prerequisites.

### Acceptance criteria

- [ ] **P8-AC-15:** Lint invokes the intended tool and arguments.
- [ ] **P8-AC-16:** Lint failures are reported as failures.
- [ ] **P8-AC-17:** Missing tooling is actionable and does not panic.
- [ ] **P8-AC-18:** No result is fabricated when the tool did not run.

---

## 9. Task 8.4 — `rustrs test`

### Work items

- [ ] 8.4.1 Resolve the project root and test command.
- [ ] 8.4.2 Execute the supported test operation.
- [ ] 8.4.3 Forward output, including failure diagnostics.
- [ ] 8.4.4 Preserve failure status.
- [ ] 8.4.5 Do not silently skip tests that require external prerequisites.
- [ ] 8.4.6 If filters/options are exposed, pass them safely and document them.
- [ ] 8.4.7 Add success, failure, and missing-tool tests.

### Acceptance criteria

- [ ] **P8-AC-19:** Tests run in the intended project.
- [ ] **P8-AC-20:** A failing test results in a non-success status.
- [ ] **P8-AC-21:** Skipped or blocked tests are not reported as passed.
- [ ] **P8-AC-22:** Test arguments cannot inject shell commands.

---

## 10. Task 8.5 — `rustrs format`

### Work items

- [ ] 8.5.1 Verify the formatter used by the project's Rust toolchain.
- [ ] 8.5.2 Define whether default behavior checks formatting or modifies files.
- [ ] 8.5.3 Add explicit check/diff options only if useful and supported; test and document them.
- [ ] 8.5.4 Ensure check mode never modifies files.
- [ ] 8.5.5 Ensure write mode changes only expected files.
- [ ] 8.5.6 Forward formatter diagnostics and propagate status.
- [ ] 8.5.7 Test behavior using temporary projects and filesystem comparisons.

### Acceptance criteria

- [ ] **P8-AC-23:** Formatting behavior is documented and unambiguous.
- [ ] **P8-AC-24:** Check mode is read-only.
- [ ] **P8-AC-25:** Write mode follows explicit documented mutation semantics.
- [ ] **P8-AC-26:** Formatter failures are surfaced accurately.

---

## 11. Task 8.6 — `rustrs audit`

### Work items

- [ ] 8.6.1 Identify the actual audit tool selected for the MVP and verify its interface.
- [ ] 8.6.2 Check tool availability before execution.
- [ ] 8.6.3 Run the audit against the target project and forward output.
- [ ] 8.6.4 Distinguish unavailable tool, execution failure, findings, and completed audit without findings.
- [ ] 8.6.5 Document network requirements if the selected tool needs vulnerability database access.
- [ ] 8.6.6 Do not auto-update dependencies, apply fixes, or change lockfiles.
- [ ] 8.6.7 Never print secrets or claim general security based on one audit.
- [ ] 8.6.8 Test missing tool, process failure, findings, and successful completion.

### Acceptance criteria

- [ ] **P8-AC-27:** Audit invokes a real audit tool.
- [ ] **P8-AC-28:** A missing tool or failed audit is never reported as a pass.
- [ ] **P8-AC-29:** Audit scope and limitations are documented.
- [ ] **P8-AC-30:** No automatic dependency changes occur.

---

## 12. Task 8.7 — OpenAPI documentation helper

### Work items

- [ ] 8.7.1 Inspect the OpenAPI integration in actual templates and generated code.
- [ ] 8.7.2 Reuse manifest and capability resolution.
- [ ] 8.7.3 Define minimal useful behavior from verified implementation, such as validating documentation configuration or showing how to access configured docs.
- [ ] 8.7.4 Do not invent routes, schemas, providers, or endpoints.
- [ ] 8.7.5 Fail clearly when OpenAPI is disabled or unsupported.
- [ ] 8.7.6 Avoid new dependencies unless necessary and compatible with MSRV.
- [ ] 8.7.7 Document behavior, prerequisites, output, and limitations.

### Acceptance criteria

- [ ] **P8-AC-31:** The helper works only for verified OpenAPI configurations.
- [ ] **P8-AC-32:** It does not fabricate API documentation or endpoint availability.
- [ ] **P8-AC-33:** Unsupported configurations receive actionable diagnostics.
- [ ] **P8-AC-34:** Help and tests match implemented behavior.

---

## 13. Task 8.8 — Dockerfile and `.dockerignore` generation

### Work items

- [ ] 8.8.1 Inspect build requirements and output structure of supported templates.
- [ ] 8.8.2 Choose a Dockerfile strategy compatible with Rust edition, MSRV, Cargo layout, and runtime needs.
- [ ] 8.8.3 Use multi-stage builds where appropriate, without mechanically breaking supported targets.
- [ ] 8.8.4 Define runtime privilege behavior intentionally and avoid unnecessary root execution where feasible.
- [ ] 8.8.5 Add `.dockerignore` rules for build output, local environment files, Git metadata, and irrelevant/private files.
- [ ] 8.8.6 Do not copy real `.env` files or credentials into images by default.
- [ ] 8.8.7 Use the existing generation planner and safe filesystem writer.
- [ ] 8.8.8 Respect dry-run and file-conflict rules where supported.
- [ ] 8.8.9 Do not build or run an image automatically during generation.
- [ ] 8.8.10 Add structural tests and Docker build smoke tests when Docker is available.

### Acceptance criteria

- [ ] **P8-AC-35:** Dockerfile matches supported generated project structure.
- [ ] **P8-AC-36:** `.dockerignore` excludes relevant local artifacts and secrets.
- [ ] **P8-AC-37:** Existing files follow explicit conflict policy.
- [ ] **P8-AC-38:** Generated Docker assets have structural tests.
- [ ] **P8-AC-39:** Docker build is reported as not run when Docker is unavailable, not as passed.

---

## 14. Task 8.9 — Docker Compose generation

### Work items

- [ ] 8.9.1 Define the generation command and flags using existing CLI conventions.
- [ ] 8.9.2 Generate services only when required by the actual configuration.
- [ ] 8.9.3 Include a database service only when the project configuration calls for one.
- [ ] 8.9.4 Include optional services such as Redis only when their capability is enabled and implemented.
- [ ] 8.9.5 Define ports, volumes, networks, health checks, and environment variables intentionally.
- [ ] 8.9.6 Use placeholders/environment references for credentials; never embed functioning secrets.
- [ ] 8.9.7 Avoid unnecessary public exposure of services.
- [ ] 8.9.8 Detect existing Compose files and report conflicts before writing.
- [ ] 8.9.9 Reuse generation and filesystem safety infrastructure.
- [ ] 8.9.10 Do not start or stop containers as a generation side effect.
- [ ] 8.9.11 Validate with Docker Compose when available; otherwise run structural tests and report the limitation.

### Acceptance criteria

- [ ] **P8-AC-40:** Services match enabled capabilities and verified support.
- [ ] **P8-AC-41:** No real credential is embedded in generated files.
- [ ] **P8-AC-42:** Existing files are not silently overwritten.
- [ ] **P8-AC-43:** Generation does not start containers.
- [ ] **P8-AC-44:** Validation outcomes are reported accurately.

---

## 15. Task 8.10 — GitHub Actions workflow generation

### Work items

- [ ] 8.10.1 Choose a documented output location such as `.github/workflows/`.
- [ ] 8.10.2 Define supported workflow names and triggers.
- [ ] 8.10.3 Include only quality commands that actually exist and apply to the generated project.
- [ ] 8.10.4 Align toolchain configuration with declared MSRV and support policy.
- [ ] 8.10.5 Use valid, verified action versions/references according to repository policy.
- [ ] 8.10.6 Apply least-privilege workflow permissions.
- [ ] 8.10.7 Configure Cargo caching only where correct and beneficial.
- [ ] 8.10.8 Do not embed tokens, passwords, signing keys, or other secrets.
- [ ] 8.10.9 Do not add deployment jobs or publishing behavior beyond workflow preparation.
- [ ] 8.10.10 Reuse template rendering and safe file conflict handling.
- [ ] 8.10.11 Validate workflow structure and referenced commands.
- [ ] 8.10.12 Test repeated generation and conflicts.

### Acceptance criteria

- [ ] **P8-AC-45:** Generated workflow structure is valid.
- [ ] **P8-AC-46:** Every referenced command exists and is documented.
- [ ] **P8-AC-47:** Permissions and secret handling follow least-privilege principles.
- [ ] **P8-AC-48:** Generation respects conflicts and dry-run semantics.
- [ ] **P8-AC-49:** No unsupported deployment or publishing behavior is introduced.

---

## 16. Task 8.11 — GitLab CI workflow generation

### Work items

- [ ] 8.11.1 Generate `.gitlab-ci.yml` using existing template and filesystem infrastructure.
- [ ] 8.11.2 Define stages/jobs corresponding to implemented quality commands.
- [ ] 8.11.3 Align image/toolchain with supported Rust version and project requirements.
- [ ] 8.11.4 Configure Cargo caching carefully without assuming unsupported runner features.
- [ ] 8.11.5 Do not assume Docker-in-Docker or privileged runners are available.
- [ ] 8.11.6 Do not include credentials or secrets.
- [ ] 8.11.7 Default to quality CI; do not add deployment jobs without explicit scope and authorization.
- [ ] 8.11.8 Validate YAML and referenced commands.
- [ ] 8.11.9 Handle existing `.gitlab-ci.yml` without silent overwrite.

### Acceptance criteria

- [ ] **P8-AC-50:** Generated configuration has valid structure.
- [ ] **P8-AC-51:** Jobs reference supported commands and tools.
- [ ] **P8-AC-52:** Pipeline does not assume privileged runner capabilities.
- [ ] **P8-AC-53:** Existing configuration is preserved or a clear conflict is reported.

---

## 17. Task 8.12 — Shell completion

### Work items

- [ ] 8.12.1 Inspect completion generation support in the current CLI framework.
- [ ] 8.12.2 Select supported shells based on actual library/platform support.
- [ ] 8.12.3 Implement a command that emits completion for a requested shell.
- [ ] 8.12.4 Ensure subcommands, aliases, flags, and value choices match the actual CLI.
- [ ] 8.12.5 Exclude planned or unavailable commands from completion output.
- [ ] 8.12.6 Document how to save and source completion.
- [ ] 8.12.7 Do not edit shell startup files or install completion automatically.
- [ ] 8.12.8 Add output tests for each supported shell.

### Acceptance criteria

- [ ] **P8-AC-54:** Completion output is generated for every declared supported shell.
- [ ] **P8-AC-55:** Completion reflects the actual CLI interface.
- [ ] **P8-AC-56:** The command does not modify shell configuration.
- [ ] **P8-AC-57:** Activation instructions are documented and tested where feasible.

---

## 18. Task 8.13 — Release workflow preparation

This task prepares workflow configuration. Actual release hardening, publishing, and binary validation belong to Phase 9.

### Work items

- [ ] 8.13.1 Inspect package metadata, version convention, current CI, and release policy.
- [ ] 8.13.2 Define an explicit trigger, such as a documented version-tag pattern, without creating a tag.
- [ ] 8.13.3 Require quality checks before release build steps.
- [ ] 8.13.4 Document intended target matrix; include only targets supported by the selected toolchain/workflow.
- [ ] 8.13.5 Define consistent artifact naming.
- [ ] 8.13.6 Include checksums only if the workflow actually generates them.
- [ ] 8.13.7 Avoid storing release credentials in the repository.
- [ ] 8.13.8 Do not implement silent self-updating.
- [ ] 8.13.9 Test workflow structure.
- [ ] 8.13.10 Do not tag, publish, or upload a real release.
- [ ] 8.13.11 Document the remaining Phase 9 release validation.

### Acceptance criteria

- [ ] **P8-AC-58:** Trigger and workflow steps are explicit.
- [ ] **P8-AC-59:** Required quality gates precede release build steps.
- [ ] **P8-AC-60:** Artifact naming/checksum behavior matches actual steps.
- [ ] **P8-AC-61:** No release is tagged, published, or uploaded in Phase 8.
- [ ] **P8-AC-62:** Phase 9 release validation is clearly deferred.

---

## 19. Task 8.14 — Integration, security, and regression tests

### Quality-command tests

- [ ] 8.14.1 Test valid and invalid project roots.
- [ ] 8.14.2 Test successful processes and non-zero exit status.
- [ ] 8.14.3 Test missing executables and permission failures where feasible.
- [ ] 8.14.4 Test output forwarding and error messages.
- [ ] 8.14.5 Verify format check mode does not modify files.
- [ ] 8.14.6 Verify missing/failed audit tools are not reported as success.
- [ ] 8.14.7 Verify secret redaction.

### Generated asset tests

- [ ] 8.14.8 Generate Docker assets in temporary projects for supported configuration classes.
- [ ] 8.14.9 Assert Dockerfile/`.dockerignore` contents and conflict behavior.
- [ ] 8.14.10 Assert Compose services match enabled capabilities.
- [ ] 8.14.11 Structurally validate YAML for Compose, GitHub Actions, GitLab CI, and release workflow.
- [ ] 8.14.12 Verify CI command references exist and agree with help output.
- [ ] 8.14.13 Check generated files for real credentials or secret values.
- [ ] 8.14.14 Test repeated generation, dry-run, and file conflicts.
- [ ] 8.14.15 Run Docker/Compose validation only when tools are available and report unavailable checks honestly.

### Shell completion tests

- [ ] 8.14.16 Test output for each supported shell.
- [ ] 8.14.17 Check consistency with subcommands, aliases, and flags.
- [ ] 8.14.18 Verify unimplemented commands are not advertised.

### Regression tests

- [ ] 8.14.19 Run CLI foundation, manifest, compatibility, template, and filesystem tests.
- [ ] 8.14.20 Run scaffolding and generator tests for supported configurations.
- [ ] 8.14.21 Run inspector and diagnostic tests.
- [ ] 8.14.22 Run database and development workflow tests.
- [ ] 8.14.23 Verify help/version, global options, output policy, and exit-code conventions.
- [ ] 8.14.24 Inspect final diff for unrelated changes.

### Acceptance criteria

- [ ] **P8-AC-63:** Quality commands have success and failure-path tests.
- [ ] **P8-AC-64:** Docker and CI/CD outputs have structural validation.
- [ ] **P8-AC-65:** File safety, conflict handling, dry-run, and secret handling are tested.
- [ ] **P8-AC-66:** Phase 0–7 regressions pass or each failure is documented with evidence.
- [ ] **P8-AC-67:** Tests do not depend on a developer's real project or production database.

---

## 20. Task 8.15 — Documentation and final quality gates

### Work items

- [ ] 8.15.1 Update README command list and examples to match actual working commands.
- [ ] 8.15.2 Document quality commands, prerequisites, exit behavior, and file-mutation semantics.
- [ ] 8.15.3 Document OpenAPI helper behavior and supported configurations.
- [ ] 8.15.4 Document Docker/Compose generation and credential handling.
- [ ] 8.15.5 Document GitHub Actions/GitLab CI generation, triggers, and prerequisites.
- [ ] 8.15.6 Document supported shell completion and manual activation.
- [ ] 8.15.7 Document release workflow preparation and tasks deferred to Phase 9.
- [ ] 8.15.8 Update `ARCHITECTURE.md` only if an enduring architecture decision changes.
- [ ] 8.15.9 Update `CHANGELOG.md` if required by repository convention.
- [ ] 8.15.10 Use fake values in examples; do not include secrets.
- [ ] 8.15.11 Ensure help, README, roadmap, and implementation agree.

### Required repository checks

Run from the repository root:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo metadata --no-deps --format-version 1
git diff --check
git status --short
```

Also run applicable template, generated-project, YAML, Docker, Compose, and completion checks. If a check cannot run due to missing tooling, network, platform, or another prerequisite, mark it `BLOCKED` or `NOT RUN` and explain why. Never mark it passed.

### Acceptance criteria

- [ ] **P8-AC-68:** Documentation reflects implemented functionality and limitations.
- [ ] **P8-AC-69:** Required checks are run and their actual outcomes recorded.
- [ ] **P8-AC-70:** Generated assets and command help agree with the support matrix.
- [ ] **P8-AC-71:** No unrelated scope or Phase 9 release execution is included.

---

## 21. Recommended implementation order

1. **Preflight:** Task 8.0.
2. **Shared execution contract:** Task 8.1.
3. **Quality commands:** Tasks 8.2–8.6.
4. **OpenAPI helper:** Task 8.7.
5. **Docker assets:** Tasks 8.8–8.9.
6. **CI workflows:** Tasks 8.10–8.11.
7. **Shell completion:** Task 8.12.
8. **Release workflow preparation:** Task 8.13.
9. **Integration/security/regression tests:** Task 8.14.
10. **Documentation and final audit:** Task 8.15.

Reorder only when a real dependency requires it. Record the reason.

---

## 22. Definition of Done

Phase 8 is complete only when all applicable criteria below are satisfied:

- [ ] `rustrs check` works for supported projects.
- [ ] `rustrs lint` invokes the documented tool and reports failures accurately.
- [ ] `rustrs test` runs the correct test suite and propagates failure.
- [ ] `rustrs format` has documented and tested mutation semantics.
- [ ] `rustrs audit` runs a real audit and does not claim checks it did not perform.
- [ ] OpenAPI helper behavior matches implemented capabilities.
- [ ] Dockerfile and `.dockerignore` generation is safe and tested.
- [ ] Compose generation matches project configuration and enabled capabilities.
- [ ] GitHub Actions generation produces structurally valid configuration.
- [ ] GitLab CI generation produces structurally valid configuration.
- [ ] Shell completion works for explicitly supported shells.
- [ ] Release workflow is prepared and structurally tested without creating a release.
- [ ] Safe writes, conflict handling, path safety, and dry-run semantics are preserved.
- [ ] No real secrets are included in generated files or logs.
- [ ] Phase 0–7 tests pass or blockers are explicitly reported.
- [ ] Required Cargo checks have actual recorded outcomes.
- [ ] README, help, and docs match the implementation.
- [ ] No command reports success when the requested operation did not succeed.
- [ ] No Phase 9 release publication or out-of-scope deployment work occurred.

A phase is not complete merely because commands or files exist. Functionality must be verified.

---

## 23. Mandatory instructions for the AI coding agent

Implement **Phase 8 only**.

1. Audit the current working tree before editing and preserve pre-existing changes.
2. Read the required documents and current source code.
3. Verify Phase 7 from implementation and tests; do not assume completion based on documentation.
4. Reuse existing architecture and add only the minimum abstractions required.
5. Verify the actual support matrix before exposing commands or generating assets.
6. Implement one task at a time and test it before continuing.
7. Never add fake success handlers or advertise unimplemented commands as working.
8. Use safe process execution with separate arguments.
9. Use the existing generation planner and filesystem safety engine.
10. Never silently overwrite user files; respect dry-run and path-safety rules.
11. Do not include real credentials, print secrets, or dump environment variables.
12. Do not automatically install tools, update dependencies, start containers, modify shell startup files, commit, push, tag, publish, or deploy.
13. Do not expand into Phase 9 release execution or unrelated production hardening.
14. Update help, tests, and documentation when the public CLI contract changes.
15. Run applicable quality gates and report actual results. Mark skipped or blocked checks honestly.
16. Review `git diff` and `git status --short` before reporting completion.
17. Do not declare Phase 8 complete if applicable acceptance criteria fail or remain unverified.

### Final report format

At completion, provide an evidence-based report with:

1. **Summary:** implemented behavior.
2. **Task status:** each task 8.0–8.15 marked `DONE`, `PARTIAL`, `BLOCKED`, or `NOT DONE`.
3. **Acceptance criteria:** passed and failed criterion IDs.
4. **Commands executed:** exact commands and actual results.
5. **Generated assets:** files created or updated.
6. **Tests:** unit, integration, structural, generated-project, and external-tool checks.
7. **Known limitations:** unsupported configurations, skipped checks, or blockers.
8. **Security review:** secrets, process execution, path safety, overwrite behavior, and permissions.
9. **Git status:** changes made and pre-existing changes preserved.
10. **Phase boundary:** confirmation that no Phase 9 release publication or unrelated scope was performed.

---

## 24. Final scope reminder

Phase 8 covers quality commands, OpenAPI documentation helpers, Docker and CI/CD asset generation, shell completion, and release workflow preparation. It does not authorize deployment, binary publication, actual release creation, architectural rewrites, or implementation of every long-term PRD feature.

When an implementation detail is unclear, inspect code and tests first. Prefer a small, verified feature within the current support matrix over a broad, untested implementation.

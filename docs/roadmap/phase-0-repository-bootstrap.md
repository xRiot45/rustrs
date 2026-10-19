# Phase 0 — Repository Bootstrap

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 0 of 10  
> **Status:** Planned  
> **Audience:** AI coding agents and maintainers  
> **Primary outcome:** A clean, reproducible, documented Rust CLI repository that is ready for implementation in Phase 1.  
> **Source of truth:** `PRD.md`, followed by `ARCHITECTURE.md` and `AGENTS.md` when present.

---

## 1. Objective

Prepare the `rustrs` repository so implementation can proceed safely and predictably. This phase establishes the repository structure, Rust toolchain expectations, project metadata, baseline quality checks, and contributor/agent instructions.

At the end of Phase 0, the repository must be easy for a human or AI agent to understand, clone, build, test, lint, and extend. The CLI may only contain a minimal executable entry point; **feature commands and actual project generation are not implemented in this phase**.

### 1.1 Success outcomes

- The repository has a clear, conventional Rust project structure.
- The supported Rust edition and minimum toolchain policy are documented and consistent.
- `cargo check`, `cargo test`, `cargo fmt --check`, and `cargo clippy` run successfully.
- Repository metadata and essential documentation are in place.
- A new contributor can follow the README from clone to successful local verification.
- AI agents have explicit instructions about scope, testing, safety, and completion criteria.
- No future phase is silently implemented as part of bootstrap work.

---

## 2. Context and product direction

`rustrs` is a Rust-based CLI inspired by framework CLIs such as NestJS CLI. It will scaffold REST API projects and generate common backend code.

The long-term product includes:

- Project creation with `rustrs new`.
- Code generation such as `rustrs generate resource products`.
- Multiple API templates: Enterprise, Standard, Simple, and Minimal.
- Configurable framework, database, architecture, and capabilities.
- Project inspection, diagnostics, database workflows, quality checks, documentation, Docker, CI/CD, and release distribution.

The Enterprise API template is the reference/superset architecture. Its planned baseline is Rust + Tokio + Axum + PostgreSQL + SQLx, with modular feature-oriented organization and Clean Architecture principles. These product capabilities are context only for Phase 0; they must not be implemented now.

---

## 3. Phase boundaries

### 3.1 In scope

1. Inspect the current repository and record its actual state.
2. Confirm the Rust toolchain and package metadata.
3. Establish or clean up the minimal repository layout.
4. Add a minimal compilable CLI binary entry point.
5. Add essential documentation and agent instructions.
6. Configure baseline formatting, linting, and testing conventions.
7. Add repository hygiene files such as `.gitignore` and license metadata only when the project owner has selected the appropriate license.
8. Verify the project using the required local checks.
9. Record any unresolved decisions that block later phases.

### 3.2 Out of scope

Do **not** implement any of the following in Phase 0:

- `rustrs new` project generation or template rendering.
- Enterprise/Standard/Simple/Minimal API template contents.
- Interactive project prompts.
- `generate`, `g`, `resource`, `module`, `controller`, `service`, or other generators.
- Configuration loading or `.rustrs/config.toml`.
- Framework/database compatibility matrices.
- Filesystem mutation engine, dry-run, overwrite handling, or path traversal defenses beyond ordinary repository hygiene.
- Database migrations, seeders, or database commands.
- Docker Compose for generated projects.
- Authentication, RBAC, OpenAPI, Redis, background jobs, or application runtime infrastructure.
- Release automation, publishing, self-update, shell completion, or binary packaging.
- Placeholder modules that claim to support features without working implementations.

**Rule:** If a task is not required to make the repository clean, understandable, and verifiable, defer it to the phase that owns the feature.

---

## 4. Prerequisites and source-of-truth rules

Before changing files, the agent must:

1. Read `PRD.md` completely.
2. Read `ARCHITECTURE.md` and `AGENTS.md` if they exist.
3. Inspect the current Git status and repository tree.
4. Identify existing conventions and preserve useful work.
5. Check the installed Rust and Cargo versions.
6. Identify unresolved product decisions that affect Phase 0.

### Source-of-truth priority

When documents disagree, use this order:

1. Explicit decisions from the project owner.
2. `PRD.md` for product requirements.
3. `ARCHITECTURE.md` for technical architecture.
4. `AGENTS.md` for agent workflow and coding rules.
5. This phase document for Phase 0 execution details.
6. Existing code, only where it does not conflict with the above.

Do not invent product requirements to resolve ambiguity. Record the ambiguity and ask the project owner only when it blocks progress. Otherwise choose the smallest reversible default and document it.

---

## 5. Repository inspection procedure

Perform inspection before modifying the repository.

### 5.1 Inspect Git state

Run:

```bash
git status --short
git branch --show-current
git log -5 --oneline
```

Requirements:

- Do not discard, overwrite, or reset user changes.
- Do not run destructive Git commands.
- If the working tree contains changes, identify which files are user-owned before editing.
- Do not create a commit unless the project owner explicitly asks for one.

### 5.2 Inspect repository contents

Review:

- Root-level files and directories.
- Existing `Cargo.toml` and `Cargo.lock`.
- Existing `src/` and `tests/`.
- README and architecture/agent documents.
- CI workflows, formatting configuration, lint configuration, and license files.
- Stale or conflicting package-manager configuration that does not belong to a Rust CLI.

Do not delete a file merely because it looks unnecessary. Establish whether it is referenced or contains intentional work. Remove stale files only when their irrelevance is clear and removal is documented in the final report.

### 5.3 Inspect Rust tooling

Run:

```bash
rustc --version
cargo --version
rustup show
```

If Rust is unavailable, document the exact installation prerequisite. Do not claim build/test checks passed if they could not run.

---

## 6. Required repository structure

Use the existing structure when it is already clean and consistent. Otherwise, establish the following minimal layout:

```text
rustrs/
├── .github/
│   └── workflows/
│       └── ci.yml                 # Optional in Phase 0; add only if straightforward
├── docs/
│   └── roadmap/
│       └── phase-0-repository-bootstrap.md
├── src/
│   └── main.rs
├── tests/                          # May be omitted until integration tests are needed
├── .gitignore
├── AGENTS.md
├── ARCHITECTURE.md                 # If architecture decisions are already documented
├── Cargo.toml
├── Cargo.lock                      # Commit for this binary application
├── LICENSE                         # Only after license choice is confirmed
├── PRD.md
└── README.md
```

Notes:

- Do not create empty directories only to make the tree look complete.
- `docs/roadmap/phase-0-repository-bootstrap.md` should be committed alongside the implementation of this phase if the repository workflow permits it.
- Keep the initial structure small. Additional modules such as `cli/`, `commands/`, `templates/`, `config/`, or `engine/` should be introduced in the phases that implement them.
- `Cargo.lock` should be tracked because `rustrs` is a CLI application, not a reusable library-only crate.
- Do not add a workspace unless the PRD or current repository already requires one.

---

## 7. Rust package and toolchain requirements

### 7.1 Package metadata

`Cargo.toml` must contain accurate metadata, including:

- Package name: `rustrs` unless the repository already has an owner-approved crate name.
- Version: use the current project version; if none exists, start at `0.1.0`.
- Rust edition: use the edition selected by the PRD/current toolchain. Prefer Rust 2024 only if the installed/stated MSRV supports it.
- Description: a concise description of the Rust API scaffolding CLI.
- License metadata only when the license has been selected.
- Repository/homepage metadata only when the actual URLs are known.
- Binary target name consistent with the package command name.

Do not invent repository URLs, author details, license identifiers, or publication metadata.

### 7.2 Toolchain policy

- Document the required Rust installation method and minimum supported Rust version (MSRV) if the project has decided one.
- Do not create a `rust-toolchain.toml` that pins a speculative toolchain version.
- If a toolchain file is added, its channel must be an intentional project decision and must work with CI.
- Avoid using unstable Rust features.
- Prefer stable Rust.

### 7.3 Dependencies

Phase 0 should use as few dependencies as possible. A minimal executable may use only the Rust standard library.

Do not install the complete future CLI stack prematurely. In particular, do not add dependencies for interactive prompts, templates, configuration, progress bars, or filesystem traversal until the corresponding phase needs them.

If a dependency is added, explain its immediate Phase 0 purpose and verify that it is actually used.

---

## 8. Minimal executable behavior

The CLI must compile and start, but it does not need to implement the real command interface yet.

Acceptable behavior:

- Print a short product identity and indicate that the CLI foundation is under development; or
- Provide a minimal `--version`/help behavior only if it can be implemented cleanly without pre-empting Phase 1's CLI framework decision.

Preferred default: keep `src/main.rs` minimal and avoid implementing a custom argument parser. Phase 1 will establish the actual command framework and command contract.

Requirements:

- No panic for normal invocation.
- No fake success messages such as “project generated” when no project was generated.
- No filesystem writes or network requests.
- No embedded secrets, tokens, or environment-specific paths.
- Keep business logic out of `main.rs`; do not prematurely introduce abstractions without a real use case.

Example of an acceptable minimal entry point:

```rust
fn main() {
    println!("rustrs — Rust API scaffolding CLI");
    println!("CLI foundation is under development.");
}
```

This is an example, not a requirement to use these exact strings. If the repository already has a CLI framework configured for a valid reason, preserve it and align with the PRD.

---

## 9. Documentation requirements

### 9.1 `README.md`

The README should be short, accurate, and useful. Include:

1. Project name and one-sentence purpose.
2. Current development status.
3. Prerequisites.
4. Clone and local setup instructions.
5. Commands to build and verify the repository.
6. High-level roadmap or a link to `docs/roadmap/`.
7. A clear statement that project generation is not available until implemented.

Do not document planned commands as though they already work. Label future commands as planned, or omit them from the usage section.

Suggested verification commands:

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

If a command is not applicable due to repository configuration, explain why instead of silently omitting it.

### 9.2 `AGENTS.md`

Create or update `AGENTS.md` with concise, actionable instructions for future coding agents. It must cover:

- Read `PRD.md`, `ARCHITECTURE.md`, and the active phase document before coding.
- Implement only the current phase's scope.
- Prefer small, cohesive changes.
- Do not introduce speculative abstractions or dependencies.
- Do not report a test/check as passed unless it was actually run and passed.
- Run relevant formatting, linting, tests, and build checks before declaring completion.
- Do not overwrite unrelated user changes.
- Do not claim unsupported commands/features exist.
- Update documentation when behavior changes.
- Avoid secrets in source code, logs, examples, and test fixtures.
- Summarize changed files, checks run, results, and known limitations in the final response.

If `AGENTS.md` already contains useful rules, merge these requirements into it rather than replacing the file wholesale.

### 9.3 `ARCHITECTURE.md`

If an architecture document already exists, preserve it and ensure it identifies the intended boundaries at a high level. Do not write a detailed design for every future module in Phase 0.

If it does not exist, create one only if the project owner expects it as a required project artifact. Keep it concise and distinguish confirmed decisions from planned decisions.

### 9.4 Phase document

Save this file at:

```text
docs/roadmap/phase-0-repository-bootstrap.md
```

If the project uses another approved roadmap path, follow that existing convention and report the chosen path.

---

## 10. Formatting, linting, and testing baseline

The following commands must be runnable from the repository root:

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

### 10.1 Formatting

- Use `rustfmt` defaults unless the project has an established `rustfmt.toml`.
- Add formatting configuration only when there is a clear project-wide reason.
- `cargo fmt --check` must not report formatting differences.

### 10.2 Clippy

- Fix actionable warnings produced by the baseline code.
- Do not add broad `allow` attributes merely to silence warnings.
- If a lint must be allowed, scope the allowance narrowly and document the reason.

### 10.3 Tests

- A minimal binary with no business logic may have zero unit tests, but `cargo test` must pass.
- Do not create meaningless tests solely to inflate test counts.
- Add tests for any behavior that is actually introduced in this phase.
- Do not add snapshot-testing or integration-testing frameworks until there is a concrete need.

### 10.4 CI

A minimal GitHub Actions workflow is optional in this phase unless the PRD or repository already requires CI. If added, it should run the same baseline checks on a stable Rust toolchain and must not publish releases or deploy artifacts.

Do not make CI dependent on secrets for basic verification.

---

## 11. Repository hygiene and safety

### 11.1 `.gitignore`

Include standard Rust build artifacts and local environment files as appropriate, such as:

```gitignore
/target/
.env
.env.*
!.env.example
.DS_Store
```

Adjust this example to the actual project needs. Do not ignore files that should be tracked, such as `Cargo.lock` for this CLI application.

### 11.2 Secrets

- Never commit credentials, API keys, private tokens, production connection strings, or personal access tokens.
- Do not create a real `.env` file with secrets.
- If environment-variable examples are needed later, use clearly fake placeholder values.

### 11.3 File and Git safety

- Never run `git reset --hard`, `git clean -fd`, or other destructive commands.
- Never overwrite unrelated files without inspecting their contents.
- Never remove existing project artifacts without determining their purpose.
- Do not commit, push, publish, or deploy unless explicitly requested.
- Do not modify global machine configuration as part of this phase.

---

## 12. Implementation tasks

The AI agent should execute these tasks in order. Mark each item complete only after verifying it.

### Task 0.1 — Establish baseline

- [ ] Read `PRD.md`.
- [ ] Read `ARCHITECTURE.md` and `AGENTS.md` if present.
- [ ] Inspect Git status and recent history.
- [ ] Inspect the repository tree and existing Cargo configuration.
- [ ] Check `rustc`, `cargo`, and `rustup` versions.
- [ ] Record existing problems and user-owned changes.
- [ ] Decide which existing files can be preserved, updated, or safely removed.

**Deliverable:** A clear understanding of the repository's current state; do not create a separate report file unless it is useful to the project.

### Task 0.2 — Normalize the Rust package

- [ ] Ensure `Cargo.toml` exists and has valid package metadata.
- [ ] Ensure the package builds as a binary application.
- [ ] Select the edition based on project decisions and toolchain support.
- [ ] Keep dependencies minimal.
- [ ] Generate or update `Cargo.lock`.
- [ ] Confirm no speculative metadata has been added.

**Deliverable:** A valid, reproducible Cargo package.

### Task 0.3 — Establish minimal source entry point

- [ ] Ensure `src/main.rs` exists.
- [ ] Keep startup behavior simple and truthful.
- [ ] Do not implement future commands or argument parsing prematurely.
- [ ] Ensure normal invocation exits successfully and does not write files.

**Deliverable:** A minimal executable entry point.

### Task 0.4 — Establish repository hygiene

- [ ] Add or update `.gitignore`.
- [ ] Ensure build artifacts and local secrets are not tracked.
- [ ] Preserve `Cargo.lock`.
- [ ] Inspect tracked files for accidental secrets if feasible with available tools.
- [ ] Add a license only if the project owner has confirmed the license choice.

**Deliverable:** A clean repository baseline.

### Task 0.5 — Write project instructions

- [ ] Add or update `README.md`.
- [ ] Add or update `AGENTS.md`.
- [ ] Preserve and reconcile `ARCHITECTURE.md` if present.
- [ ] Ensure documentation distinguishes current functionality from planned functionality.
- [ ] Add the phase document to the approved roadmap location.

**Deliverable:** Documentation that enables a new developer or AI agent to understand the project and run checks.

### Task 0.6 — Establish quality baseline

- [ ] Run `cargo check`.
- [ ] Run `cargo test`.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Fix issues caused by Phase 0 changes.
- [ ] Optionally add a basic CI workflow if required.
- [ ] Re-run the complete check set after fixes.

**Deliverable:** A verified baseline with recorded command results.

### Task 0.7 — Review and report

- [ ] Review the final diff.
- [ ] Confirm no out-of-scope features were added.
- [ ] Confirm no unrelated changes were lost.
- [ ] Confirm documentation matches the actual repository.
- [ ] List unresolved decisions and blockers.
- [ ] Report the files changed, checks run, results, and remaining limitations.

**Deliverable:** A concise Phase 0 completion report.

---

## 13. Acceptance criteria

Phase 0 is accepted only when all applicable criteria below pass.

| ID       | Acceptance criterion                                           | Verification                                               |
| -------- | -------------------------------------------------------------- | ---------------------------------------------------------- |
| P0-AC-01 | Repository state was inspected before changes.                 | Agent report identifies baseline inspection.               |
| P0-AC-02 | Cargo package metadata is valid and accurate.                  | `cargo metadata --no-deps --format-version 1`              |
| P0-AC-03 | The project compiles.                                          | `cargo check`                                              |
| P0-AC-04 | The test command completes successfully.                       | `cargo test`                                               |
| P0-AC-05 | Rust formatting is clean.                                      | `cargo fmt --check`                                        |
| P0-AC-06 | Clippy completes with warnings treated as errors.              | `cargo clippy --all-targets --all-features -- -D warnings` |
| P0-AC-07 | README instructions match the actual project.                  | Manual documentation review.                               |
| P0-AC-08 | Agent instructions establish scope and verification rules.     | Review `AGENTS.md`.                                        |
| P0-AC-09 | No unimplemented feature is represented as working.            | Source and README review.                                  |
| P0-AC-10 | No unrelated user work was discarded.                          | Review Git diff and status.                                |
| P0-AC-11 | No secrets or speculative repository metadata were introduced. | Review changed files.                                      |
| P0-AC-12 | Phase 0 contains no project generator or template engine.      | Scope review.                                              |

If a command cannot run because the environment lacks the required tool, report it as **blocked/not run**, not as passed. Acceptance must not conceal environmental limitations.

---

## 14. Definition of Done

Phase 0 is complete when:

- [ ] Every in-scope task is complete or explicitly marked blocked with a reason.
- [ ] All applicable acceptance criteria pass.
- [ ] The repository builds and all required baseline checks pass.
- [ ] README and agent instructions are accurate.
- [ ] The final diff contains only intentional changes.
- [ ] No future-phase feature has been implemented prematurely.
- [ ] Known issues and unresolved decisions are documented.
- [ ] The AI agent provides a completion report in the format below.

### Required final report format

```text
Phase: 0 — Repository Bootstrap
Status: COMPLETE | PARTIAL | BLOCKED

Summary:
- <short summary>

Files changed:
- <path>: <reason>

Verification:
- cargo check: PASS | FAIL | BLOCKED — <details>
- cargo test: PASS | FAIL | BLOCKED — <details>
- cargo fmt --check: PASS | FAIL | BLOCKED — <details>
- cargo clippy --all-targets --all-features -- -D warnings: PASS | FAIL | BLOCKED — <details>
- cargo metadata --no-deps --format-version 1: PASS | FAIL | BLOCKED — <details>

Out-of-scope work:
- None
  or
- <item intentionally deferred>

Known issues / decisions needed:
- None
  or
- <issue and why it matters>

Recommended next phase:
- Phase 1 — CLI Foundation
```

Never mark `COMPLETE` if a required acceptance criterion has failed or is blocked. Use `PARTIAL` or `BLOCKED` and explain what remains.

---

## 15. AI agent operating rules

The implementing agent must follow these rules throughout Phase 0:

1. **Inspect before editing.** Do not assume the repository is empty or matches the expected structure.
2. **Plan briefly, then execute.** Create a small checklist before making changes; do not spend the phase producing an elaborate plan instead of implementation.
3. **Respect scope.** This phase is repository bootstrap, not CLI feature development.
4. **Prefer minimalism.** Avoid premature abstractions, dependencies, modules, and configuration.
5. **Preserve user work.** Never discard uncommitted changes or rewrite unrelated files.
6. **Verify claims.** Run each required command and report its actual result.
7. **Be transparent.** State when a command was not run, a tool was unavailable, or a requirement remains unresolved.
8. **Keep docs truthful.** Future functionality must be labelled as planned.
9. **Do not ask unnecessary questions.** Use established project decisions. Ask only when a missing decision blocks safe progress, such as the license choice.
10. **Do not create a commit or push changes** unless explicitly requested.
11. **Do not publish packages or release binaries.**
12. **Stop at the phase boundary.** Once acceptance criteria pass, report completion and wait for approval before starting Phase 1.

---

## 16. Risks and mitigations

| Risk                                                  | Mitigation                                                      |
| ----------------------------------------------------- | --------------------------------------------------------------- |
| Existing repository contains unrelated work.          | Inspect Git status and preserve user changes.                   |
| Toolchain does not support the selected edition.      | Verify installed Rust version before choosing edition or MSRV.  |
| Bootstrap grows into premature architecture work.     | Enforce the out-of-scope list and keep the entry point minimal. |
| README advertises commands that do not exist.         | Separate current functionality from planned roadmap items.      |
| Agent reports checks as passing without running them. | Require command-by-command verification results.                |
| Unchosen license or metadata is guessed.              | Omit it or ask for the owner decision when needed.              |
| CI introduces secrets or release side effects.        | Keep CI limited to build, format, lint, and tests.              |

---

## 17. Exit handoff to Phase 1

Once Phase 0 is accepted, Phase 1 — CLI Foundation may begin.

Phase 1 should decide and implement the actual CLI command framework and command contract, including:

- Command parsing and help output.
- Version reporting.
- Consistent error reporting and exit codes.
- Top-level command structure.
- Initial automated tests for CLI behavior.

Phase 1 must not assume that Phase 0 already implemented project scaffolding, templates, config management, or generators.

**Phase 0's final promise is deliberately narrow:** the repository is ready to build on; the product features have not yet been implemented.

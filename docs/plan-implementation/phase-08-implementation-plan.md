# Phase 8 — Quality, Docs, Docker & CI/CD: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 8 of 10  
> **Status:** PLANNED — belum ada kode yang diubah; menunggu persetujuan implementasi  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-08-quality-docs-docker-cicd.md`  
> **Dokumen ini:** `docs/plan-implementation/phase-08-implementation-plan.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

- Phase 0–7 sudah selesai; codebase mendukung scaffolding, generators, inspector, database workflow, dan development workflow.
- CLI saat ini memiliki: `new`, `generate`/`g`, `info`, `modules`, `routes`, `deps`, `doctor`, `db`, `dev`.
- Belum ada quality commands, OpenAPI helper, Docker/Compose/CI generation, shell completion, atau release workflow preparation.
- Template bawaan hanya memiliki placeholder `Dockerfile` untuk `Minimal` dan hanya jika capability `docker` aktif; `.dockerignore`, Compose, CI, OpenAPI runtime, dan release workflow belum ada.
- Manifest mendukung feature `openapi`, `docker`, `cicd`, tetapi implementasi capability-nya belum menghasilkan asset.
- Process runner, template renderer, generation planner, dan filesystem writer dari Phase 3/7 dapat direuse.
- `cargo`, `rustfmt`, `docker`, dan Compose plugin tersedia. `cargo-audit` tidak terpasang.
- Repository memiliki `.github/workflows/ci.yml`, tetapi belum ada release workflow.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan                        | Pilihan                           |
| -------------------------------- | --------------------------------- |
| Asset generation command surface | **Extend `generate`**             |
| `rustrs format` default          | **Check-only default**            |
| OpenAPI helper scope             | **Status/validation only**        |
| Missing `cargo-audit`            | **Fail exit 4 + panduan install** |

---

## 1. Kondisi Codebase Saat Ini

- Phase 0–7 sudah terimplementasi dan quality gate lulus pada branch sebelumnya.
- CLI saat ini memiliki: `new`, `generate`/`g`, `info`, `modules`, `routes`, `deps`, `doctor`, `db`, `dev`.
- Belum ada quality commands, OpenAPI helper, Docker/Compose/CI generation, shell completion, atau release workflow preparation.
- Template bawaan hanya memiliki placeholder `Dockerfile` untuk `Minimal` dan hanya jika capability `docker` aktif; `.dockerignore`, Compose, CI, OpenAPI runtime, dan release workflow belum ada.
- Manifest mendukung feature `openapi`, `docker`, `cicd`, tetapi implementasi capability-nya belum menghasilkan asset.
- Process runner, template renderer, generation planner, dan filesystem writer dari Phase 3/7 dapat direuse.
- `cargo`, `rustfmt`, `docker`, dan Compose plugin tersedia. `cargo-audit` tidak terpasang.
- Repository memiliki `.github/workflows/ci.yml`, tetapi belum ada release workflow.

---

## 2. Task Phase 08 yang Akan Dikerjakan

1. Preflight audit Phase 0–7 dan support matrix aktual.
2. Kontrak command quality dan asset generation.
3. `rustrs check`.
4. `rustrs lint`.
5. `rustrs test`.
6. `rustrs format` dengan default check-only dan `--write`.
7. `rustrs audit` dengan `cargo-audit`, gagal jelas jika tool tidak ada.
8. `rustrs docs` sebagai OpenAPI helper status/validation only.
9. `rustrs completion <shell>`.
10. `rustrs generate docker`.
11. `rustrs generate compose`.
12. `rustrs generate ci github`.
13. `rustrs generate ci gitlab`.
14. `rustrs generate release`.
15. Tests, dokumentasi, dan quality gates.

### Out of scope

- Phase 9 release execution, tagging, publishing, deployment.
- OpenAPI scaffolding palsu.
- Auto-install tool.
- Container start/stop saat generation.

---

## 3. Implementation Plan

### Langkah A — Preflight audit

- Verifikasi command Phase 7 dan support matrix.
- Verifikasi tool availability.
- Generate project per template di temporary directory.

### Langkah B — CLI contract

Tambahkan ke `src/cli/commands.rs` dan `src/cli/mod.rs`:

- Top-level: `Check`, `Lint`, `Test`, `Format`, `Audit`, `Docs`, `Completion`.
- `FormatArgs` dengan `--write` dan `--path`.
- `CompletionArgs` dengan shell enum.
- `DocsArgs` dengan `--path` dan `--json`.
- Extend `GeneratorKind` dengan `Docker`, `Compose`, `CiGithub`, `CiGitlab`, `Release`.
- Ubah `GenerateArgs.name` menjadi `Option<String>`; code generators tetap mewajibkan name, asset generators mengabaikannya.

### Langkah C — Quality command application

Buat `src/application/quality.rs`:

- Resolve project root (`Cargo.toml` wajib, manifest divalidasi jika ada).
- `check`: `cargo check`.
- `lint`: `cargo clippy --all-targets -- -D warnings` (atau policy project).
- `test`: `cargo test`.
- `format`: default `cargo fmt --check`; `--write` menjalankan `cargo fmt`.
- `audit`: `cargo audit` jika tersedia, exit 4 jika missing.
- Reuse `SystemProcessRunner`, forward output, map status ke exit code 4/5.

### Langkah D — OpenAPI helper

Buat `src/application/docs.rs`:

- Validasi manifest dan capability `openapi`.
- Jika false, kembalikan error actionable.
- Jika true, tampilkan provider, manifest path, dan catatan bahwa runtime docs bergantung pada aplikasi generated.
- Tidak membuat route/schema.

### Langkah E — Shell completion

- Tambah dependency `clap_complete`.
- Implement `rustrs completion <shell>` dengan `clap_complete::generate`.
- Shell yang didukung: bash, zsh, fish, powershell, elvish.
- Output ke stdout, tanpa modifikasi shell config.

### Langkah F — Asset generation

Buat `src/application/assets.rs`:

- Gunakan `GenerationPlan` dan `apply_plan`.
- `docker`: tulis `Dockerfile` dan `.dockerignore`; set `features.docker = true`.
- `compose`: tulis `docker-compose.yml` dengan service app dan database sesuai manifest; set `features.docker = true`.
- `ci github`: tulis `.github/workflows/ci.yml`.
- `ci gitlab`: tulis `.gitlab-ci.yml`.
- `release`: tulis `.github/workflows/release.yml` (prepare only, tanpa publish/tag).
- Semua file menggunakan conflict policy existing dan dry-run.
- CI asset menulis manifest dengan `features.cicd = true` jika relevan.

### Langkah G — Docker/CI templates

- Simpan template sebagai konstanta string di `src/application/assets.rs` atau modul template baru.
- Multi-stage Dockerfile untuk Axum app.
- `.dockerignore` mengecualikan `target/`, `.git/`, `.env`, logs.
- Compose memakai environment reference, bukan secret nyata.
- GitHub Actions/GitLab CI hanya mereferensikan command yang benar-benar ada.

### Langkah H — Release workflow preparation

- Workflow trigger tag `v*.*.*` (tanpa membuat tag).
- Job quality gate, build matrix, checksum.
- Tidak ada publish/upload release nyata.

### Langkah I — Tests

- Unit tests quality command construction.
- Integration tests help/exit code.
- Structural tests generated Docker/Compose/YAML.
- Completion output tests.
- Regression tests Phase 0–7.

### Langkah J — Dokumentasi

- Update README dengan quality commands, docs, completion, Docker/CI generation.
- Dokumentasikan tool prerequisites dan limitation.
- Update ARCHITECTURE.md jika ada keputusan arsitektur baru.

### Langkah K — Quality gates

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo metadata --no-deps --format-version 1`
- `git diff --check`

---

## 4. File yang Akan Terpengaruh

### Dibuat

- `src/application/quality.rs`
- `src/application/docs.rs`
- `src/application/assets.rs`
- `tests/quality.rs`
- `tests/assets.rs`
- `tests/completion.rs`

### Diubah

- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/main.rs`
- `src/application/mod.rs`
- `src/generator/kind.rs` (jika asset kinds masuk enum)
- `Cargo.toml`
- `README.md`
- `.github/workflows/release.yml` (repo-level, opsional)

---

## 5. Dependency / Configuration Impact

- Tambah `clap_complete = "4"` untuk shell completion.
- Tidak menambah dependency untuk audit; `cargo-audit` tetap external tool.
- Tidak mengubah schema manifest; memakai feature flags yang sudah ada.

---

## 6. Risiko dan Potensi Konflik

- OpenAPI belum diimplementasikan di template; helper harus status-only agar tidak menyesatkan.
- `cargo-audit` tidak terpasang di environment ini; harus dilaporkan sebagai missing tool, bukan pass.
- Docker/CI generation harus mematuhi support matrix Axum/Postgres/SQLx.
- Generated workflow tidak boleh mereferensikan command yang belum ada.
- Asset generation harus memakai safe write dan conflict policy.
- Release workflow preparation tidak boleh menjadi Phase 9 release execution.

---

## 7. Verification Plan

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo build --release`
- Structural validation YAML/Docker/Compose.
- Docker build hanya jika Docker tersedia; jika tidak, laporkan `NOT RUN`.

---

## 8. Acceptance Criteria yang Akan Terpenuhi

- P8-AC-05 sampai P8-AC-30: quality commands.
- P8-AC-31 sampai P8-AC-34: OpenAPI helper status-only.
- P8-AC-35 sampai P8-AC-44: Docker/Compose generation.
- P8-AC-45 sampai P8-AC-53: GitHub/GitLab CI generation.
- P8-AC-54 sampai P8-AC-57: shell completion.
- P8-AC-58 sampai P8-AC-62: release workflow preparation.
- P8-AC-63 sampai P8-AC-71: tests, docs, quality gates.

---

## 9. Pertanyaan / Ambiguity

Keputusan utama sudah dikonfirmasi:

- Asset generation memakai `generate`.
- `format` default check-only.
- OpenAPI helper status-only.
- `audit` gagal exit 4 jika `cargo-audit` tidak ada.

Sisa detail seperti nama file workflow dan daftar shell yang didukung akan ditetapkan saat implementasi tanpa mengubah scope Phase 8.

---

## 10. Gunakan tool question untuk pertanyaan yang akan anda berikan kepada saya

Tidak ada pertanyaan tambahan; plan siap untuk persetujuan implementasi.
EOF

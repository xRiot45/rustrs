# Phase 6 — Inspector and Diagnostics: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 6 of 10  
> **Status:** PLANNED — belum ada kode yang diubah; menunggu persetujuan implementasi  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-06-inspector-and-diagnostic.md`  
> **Dokumen ini:** `docs/plan-implementation/phase-06-implementation-plan.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

- Branch aktif saat penyusunan plan: `main` (Phase 5 sudah merged).
- Phase 0–5 sudah selesai; codebase mendukung `new` dan `generate`/`g`.
- CLI saat ini hanya memiliki `New` dan `Generate`.
- Dependency production: `clap`, `thiserror`, `serde`, `toml`, `tera`, `tempfile`, `dialoguer`, `heck`.
- Belum ada modul inspection/diagnostics.
- Belum ada `serde_json`.
- Baseline quality gates lulus.
- Toolchain: Rust stable, MSRV 1.85.0, edition 2024.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan                   | Pilihan                                                                    |
| --------------------------- | -------------------------------------------------------------------------- |
| JSON output (`--json`)      | **Ya, implementasikan**                                                    |
| Exit code doctor saat error | **0=ok, 1=error** (warning tidak mengubah exit code)                       |
| Route inspection            | **Semua framework enum**, tetapi hanya Axum yang diverifikasi secara nyata |
| Module inspection           | **Filesystem-only discovery**                                              |
| Scope command               | **Semua 5 command** (`info`, `modules`, `routes`, `deps`, `doctor`)        |

---

## 1. Kondisi Codebase Saat Ini

- `src/cli/mod.rs` hanya mendefinisikan `New` dan `Generate`; belum ada inspector.
- `src/main.rs::run()` hanya dispatch `New` dan `Generate`.
- `src/application/mod.rs` hanya berisi `generate` dan `new_project`.
- Belum ada modul `inspection`.
- `discover_project_root`, `discover_manifest`, compatibility resolver, template metadata, dan filesystem path validation tersedia.
- Support matrix nyata tetap hanya Axum + Postgres + SQLx.
- Template tersedia: Minimal, Simple, Standard, Enterprise (semua Axum).
- Generated project memiliki marker `// rustrs:begin:modules`, `// rustrs:end:modules`, `// rustrs:begin:routes`, `// rustrs:end:routes` pada file registrasi.

---

## 2. Task Phase 06 yang Akan Dikerjakan

1. Task 6.0 — Preflight audit Phase 0–5.
2. Task 6.1 — Shared read-only inspection context.
3. Task 6.2 — Findings, severity, output contract, exit code, JSON output.
4. Task 6.3 — `rustrs info`.
5. Task 6.4 — `rustrs modules`.
6. Task 6.5 — `rustrs routes`.
7. Task 6.6 — `rustrs deps`.
8. Task 6.7 — `rustrs doctor`.
9. Task 6.8 — Compatibility/capability diagnostics.
10. Task 6.9 — Scaffold/generator consistency checks.
11. Task 6.10 — Read-only safety dan path validation.
12. Task 6.11 — Tests.
13. Task 6.12 — Dokumentasi dan quality gates.

### Out of scope

- Code generation atau perubahan generator (Phase 5).
- Project scaffolding unrelated (Phase 4).
- Automatic repair.
- Modifikasi manifest, Cargo.toml, Cargo.lock, atau source files.
- Install/add/remove/upgrade dependency.
- Database commands (Phase 7).
- Dev process management (Phase 7).
- Docker/CI/CD generation (Phase 8).
- Release packaging (Phase 9).
- Full Rust parser/compiler replacement.

---

## 3. Implementation Plan

### Langkah A — Preflight

- Jalankan baseline checks.
- Verifikasi layout scaffolded untuk 4 template.
- Catat pola module/route yang benar-benar ada.

### Langkah B — Dependency

`Cargo.toml`:

```toml
serde_json = "1"
```

- Justifikasi: `--json` output yang valid dan stabil.
- Jalankan `cargo check`.

### Langkah C — CLI

Tambahkan ke `src/cli/commands.rs`:

- `InfoArgs { path: Option<PathBuf>, json: bool }`
- `ModulesArgs { path: Option<PathBuf>, json: bool }`
- `RoutesArgs { path: Option<PathBuf>, json: bool }`
- `DepsArgs { path: Option<PathBuf>, json: bool }`
- `DoctorArgs { path: Option<PathBuf>, json: bool }`

Tambahkan varian ke `Commands`:

- `Info(InfoArgs)`
- `Modules(ModulesArgs)`
- `Routes(RoutesArgs)`
- `Deps(DepsArgs)`
- `Doctor(DoctorArgs)`

### Langkah D — Shared inspection context

Buat `src/inspection/`:

- `mod.rs`
- `context.rs`
- `findings.rs`

`InspectionContext`:

- `project_root: PathBuf`
- `manifest_path: Option<PathBuf>`
- `manifest: Option<ProjectManifest>`
- `cargo_manifest_path: Option<PathBuf>`
- `template: Option<TemplateId>`
- `framework: Option<Framework>`
- `database: Option<Database>`
- `data_access: Option<DataAccess>`
- `architecture: Option<Architecture>`
- `features: Option<Features>`
- `compatibility: CompatibilityStatus`
- `warnings: Vec<Finding>`
- `required_files: Vec<String>`

Perilaku:

- Resolve path melalui `discover_manifest` atau `discover_project_root`.
- Jika `--path` diberikan, mulai dari path tersebut.
- Canonicalize path dengan `std::fs::canonicalize`.
- Tolak traversal/escape.
- Load manifest read-only.
- Bedakan missing root, missing manifest, malformed manifest, unsupported schema, incompatible config.
- Gunakan compatibility resolver yang ada.

### Langkah E — Findings dan output

`Finding`:

- `code: &'static str` atau `String`
- `severity: Severity` (`Info`, `Warning`, `Error`)
- `title: String`
- `detail: Option<String>`
- `path: Option<PathBuf>`
- `location: Option<String>`
- `suggestion: Option<String>`
- `certain: bool`

Output:

- Human-readable grouped by category.
- JSON valid tanpa decorative text.
- Exit code:
  - `0`: sukses tanpa diagnostic error.
  - `1`: inspection selesai tetapi menemukan diagnostic error.
  - `2`: invalid invocation/path/manifest/unable to run.

Tambah `CliError::Diagnostic(String)` dengan exit code `1` jika diperlukan.

### Langkah F — `info`

- Tampilkan project name, root, manifest path, schema version, template, framework, database, data access, architecture, features.
- Tampilkan status validasi manifest dan compatibility.
- Tampilkan warnings jika required files hilang.
- Jangan dump secret.

### Langkah G — `modules`

Filesystem-only discovery:

- Minimal/Simple: file `.rs` yang terdaftar di marker `// rustrs:begin:modules` pada `src/main.rs`.
- Standard/Enterprise: direktori di `src/modules/` yang memiliki `mod.rs`.
- Tampilkan nama modul dan path.
- Tidak memeriksa status registrasi (sesuai keputusan pengguna).
- Laporkan limitation untuk inline module, `#[path]`, macro.

### Langkah H — `routes`

Framework-aware:

- Axum: parse `.route("path", method(...))` di marker routes.
- Actix: parse `#[get("path")]`, `#[post("path")]`, dan atribut route lain jika ditemukan.
- Rocket: parse `#[get("/path")]`, `#[post("/path")]`, dan atribut route lain.
- Tampilkan method, path, handler bila verifiable.
- Jika framework tidak dikenali atau pola tidak dikenali, laporkan unsupported/incomplete.
- Jangan start app atau kirim HTTP request.

Catatan: hanya Axum yang memiliki fixture nyata; Actix/Rocket parser harus diuji unit-level dan dilabeli limitation.

### Langkah I — `deps`

- Parse `Cargo.toml` dengan `toml::Value`.
- Tampilkan direct deps, dev deps, build deps, optional, features.
- Bedakan declared vs resolved/compiled.
- Laporkan malformed manifest.
- Jangan cetak credentials source.

### Langkah J — `doctor`

Agregasi checks:

1. Project discovery.
2. Manifest parse/schema/validate.
3. Compatibility/capability.
4. Template required files.
5. Cargo.toml parse.
6. `cargo --version` dan `rustc --version` dengan `std::process::Command` argumen eksplisit.
7. Module/route analysis limitations.
8. Summary error/warning.

- Setiap check independently testable.
- Collect multiple findings.
- Tidak mutate project.

### Langkah K — Scaffold/generator consistency

- Gunakan template metadata untuk required files.
- Laporkan missing required file sebagai error/warning.
- Jangan classify extra user files sebagai error.

### Langkah L — Read-only safety

- Semua command hanya `read_to_string`/metadata.
- Validasi path.
- Snapshot test before/after.

### Langkah M — Tests

Unit:

- Context construction.
- Manifest parse/validation errors.
- Compatibility/capability findings.
- Diagnostic severity/code.
- Module discovery.
- Route extraction.
- Deps parsing/classification.
- Exit-code mapping.
- JSON serialization.

Integration:

- Valid project per template.
- Missing manifest.
- Malformed manifest.
- Unsupported combination.
- Missing file.
- Routes supported/unsupported.
- Deps.
- Doctor aggregate.
- Read-only snapshot.

Regression Phase 1–5.

### Langkah N — Dokumentasi

- Update README dan docs commands.
- Dokumentasikan limitation route/module.
- Update roadmap Phase 6 status.

### Langkah O — Quality gates

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

## 4. File yang Akan Terpengaruh

Dibuat:

- `src/inspection/mod.rs`
- `src/inspection/context.rs`
- `src/inspection/findings.rs`
- `src/inspection/info.rs`
- `src/inspection/modules.rs`
- `src/inspection/routes.rs`
- `src/inspection/deps.rs`
- `src/inspection/doctor.rs`
- `src/inspection/structure.rs`
- `src/inspection/environment.rs`
- `src/application/inspect.rs`
- `tests/inspection.rs`

Diubah:

- `Cargo.toml`, `Cargo.lock` (`serde_json`)
- `src/lib.rs`
- `src/main.rs`
- `src/cli/mod.rs`, `src/cli/commands.rs`
- `src/error.rs`
- `README.md` dan docs terkait

## 5. Dependency / Configuration Impact

- Dependency baru: `serde_json = "1"` untuk `--json`.
- Tidak ada perubahan schema manifest.
- Tidak ada mutation project.
- Tidak ada perubahan compatibility matrix.

## 6. Risiko dan Potensi Konflik

- Route parser Actix/Rocket tidak memiliki fixture nyata; risiko false positive/negative. Mitigasi: implement parser tetapi tandai limitation dan uji Axum secara nyata.
- Module discovery filesystem-only dapat salah mengklasifikasikan file; mitigasi hanya daftar file terdaftar di marker.
- `serde_json` menambah dependency; justified by `--json`.
- `doctor` memanggil `cargo`/`rustc`; harus aman dan tidak menjalankan project-defined commands.
- Read-only guarantee harus diuji, bukan diasumsikan.

## 7. Verification Plan

1. `cargo metadata --no-deps --format-version 1`
2. `cargo check`
3. `cargo test`
4. `cargo fmt --check`
5. `cargo clippy --all-targets --all-features -- -D warnings`
6. `git diff --check`
7. Integration tests untuk 5 command.
8. Read-only snapshot tests.
9. `cargo check` pada generated fixture tetap lulus.

## 8. Acceptance Criteria yang Akan Terpenuhi

P6-AC-01 s/d P6-AC-40 sesuai roadmap, dengan fokus:

- P6-AC-04–06 shared context
- P6-AC-07–09 findings/output/JSON
- P6-AC-10–12 info
- P6-AC-13–15 modules
- P6-AC-16–18 routes
- P6-AC-19–21 deps
- P6-AC-22–25 doctor
- P6-AC-26–27 compatibility
- P6-AC-28–29 structure
- P6-AC-30–33 read-only/security
- P6-AC-34–37 tests
- P6-AC-38–40 docs/quality gates

## 9. Pertanyaan / Ambiguity

Semua keputusan utama sudah dijawab:

- JSON output: ya.
- Doctor exit code error: `1`.
- Routes: semua framework enum, tetapi Axum menjadi satu-satunya yang diverifikasi secara nyata.
- Modules: filesystem-only discovery.
- Scope: semua 5 command.

Tidak ada ambiguity yang memblokir plan.

## 10. Gunakan tool question untuk pertanyaan yang akan anda berikan kepada saya

Tidak ada pertanyaan tambahan. Jika Anda menyetujui plan ini, saya akan mulai implementasi Phase 6 pada branch `feat/phase-06-inspector-and-diagnostics` setelah persetujuan.

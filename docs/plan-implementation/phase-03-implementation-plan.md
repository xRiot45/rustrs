# Phase 3 — Template and Filesystem Engine: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 3 of 10  
> **Status:** IMPLEMENTED — Phase 3 selesai (verification lulus)  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-03-template-and-filesystem-engine.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

- Branch aktif: `main`, working tree bersih kecuali `docs/roadmap/phase-03-template-and-filesystem-engine.md` untracked.
- Phase 0, 1, dan 2 sudah selesai dan di-merge ke `main`.
- Crate saat ini adalah library + binary dengan modul `cli`, `error`, `project`, `manifest`, dan `compatibility`.
- Dependency production: `clap`, `thiserror`, `serde`, `toml`.
- Dev-dependency: `assert_cmd`, `predicates`, `tempfile`.
- Belum ada modul template, generation plan, atau filesystem engine.
- Baseline quality gates lulus.
- Toolchain: Rust stable 1.94.0, MSRV 1.85.0, edition 2024.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan                   | Pilihan                 |
| --------------------------- | ----------------------- |
| Template engine             | Tera 1.20               |
| Penyimpanan template assets | Rust constants/fixtures |
| Safe writes                 | `tempfile` crate        |
| Autoescape Tera             | Nonaktifkan global      |
| Symlink policy              | Tolak escape            |

---

## 1. Kondisi Codebase Saat Ini

- Phase 0, 1, dan 2 selesai.
- Belum ada modul template, generation plan, atau filesystem engine.
- `ProjectManifest`, compatibility resolver, capability rules, dan discovery tersedia sebagai fondasi Phase 2.
- `ProjectRootError`, `CliError`, dan `Output` tersedia sebagai fondasi Phase 1.
- Toolchain: Rust stable 1.94.0, MSRV 1.85.0, edition 2024.
- Dependency yang akan ditambahkan: `tera = "1.20"`; `tempfile` dipromosikan ke dependency production.

---

## 2. Task Phase 03 yang Akan Dikerjakan

1. **Task 3.0 — Preflight dan audit Phase 2**
2. **Task 3.1 — Template assets dan metadata model**
3. **Task 3.2 — Template resolver**
4. **Task 3.3 — Template context**
5. **Task 3.4 — Template renderer**
6. **Task 3.5 — Template asset selection dan conditional assets**
7. **Task 3.6 — Generation plan model**
8. **Task 3.7 — Planning pipeline**
9. **Task 3.8 — Path validation dan root containment**
10. **Task 3.9 — Conflict detection dan write policy**
11. **Task 3.10 — Filesystem writer dan safe writes**
12. **Task 3.11 — Dry-run, preview, dan operation report**
13. **Task 3.12 — Testing**
14. **Task 3.13 — Dokumentasi dan quality gates**

### Out of scope

- `rustrs new`, wizard, production-ready templates, generators.
- `config` command, route/module registration, database/dev commands.
- Remote registry, plugin system, auto-upgrade, merge engine, generic rollback.

---

## 3. Implementation Plan

### Langkah A — Preflight dan baseline

- Inspeksi Git status, branch, log, tree, toolchain.
- Jalankan baseline checks: `cargo metadata`, `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`.
- Catat baseline sebelum perubahan.

### Langkah B — Tambah dependency

`Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
thiserror = "2"
serde = { version = "1", features = ["derive"] }
toml = "1"
tera = "1.20"
tempfile = "3"
```

- `tera` 1.20 dipilih karena kompatibel dengan MSRV 1.85.
- `tempfile` dipromosikan dari dev-dependency ke dependency production.
- Jalankan `cargo check` setelah perubahan dependency.

### Langkah C — Struktur modul

```text
src/
├── template/
│   ├── mod.rs
│   ├── metadata.rs
│   ├── resolver.rs
│   ├── context.rs
│   ├── asset.rs
│   └── renderer.rs
├── generation/
│   ├── mod.rs
│   ├── plan.rs
│   └── planner.rs
└── filesystem/
    ├── mod.rs
    ├── paths.rs
    ├── conflicts.rs
    ├── writer.rs
    └── report.rs
```

- Tambahkan `pub mod template;`, `pub mod generation;`, `pub mod filesystem;` di `src/lib.rs`.
- Hindari modul kosong; setiap file memiliki tanggung jawab nyata.

### Langkah D — Template metadata dan fixtures

- `src/template/metadata.rs`:
  - `TemplateId` reuse dari `crate::manifest::model::TemplateId`.
  - `TemplateMetadata` dengan `id`, `version`, `description`, dan `assets`.
  - `TemplateAsset` dengan `relative_path`, `source`, `required`, dan `condition: Option<Capability>`.
  - `AssetSource`: `Static(&'static str)` atau `Template(&'static str)`.
  - Validasi metadata: path relatif, tidak duplikat, asset wajib memiliki source.
- Fixture minimal untuk `TemplateId::Minimal` dengan README dan config sederhana.
- Asset opsional berbasis `Capability::Docker`.
- Fixture ini adalah test asset, bukan klaim template production-ready.

### Langkah E — Template resolver

- `src/template/resolver.rs`:
  - `resolve(template, manifest) -> Result<ResolvedTemplate, TemplateError>`.
  - Validasi template ID yang tersedia pada build saat ini.
  - Panggil `crate::compatibility::check_compatibility(manifest)` sebelum resolve.
  - Template selain `Minimal` menghasilkan error `Unavailable` secara jujur.
  - Resolver tidak menulis filesystem dan tidak mengubah pilihan pengguna.

### Langkah F — Template context

- `src/template/context.rs`:
  - `TemplateContext` dengan field eksplisit dari manifest: `project_name`, `template`, `framework`, `database`, `data_access`, `architecture`.
  - `TemplateContext::from_manifest(manifest)`.
  - Derive `serde::Serialize`.
  - Tidak menyertakan secret atau global state.

### Langkah G — Template renderer

- `src/template/renderer.rs`:
  - `render(source, context) -> Result<String, TemplateError>`.
  - Gunakan `tera::Tera::one_off(source, &tera::Context::from_serialize(context)?, false)`.
  - Autoescape dinonaktifkan global (`false`) untuk Phase 3 karena output code/config, bukan HTML.
  - Error parse/render dilaporkan dengan pesan jelas.
  - Output deterministik.

### Langkah H — Asset selection

- `src/template/asset.rs`:
  - `select_assets(metadata, manifest) -> Result<Vec<TemplateAsset>, TemplateError>`.
  - Sertakan asset wajib.
  - Sertakan asset opsional hanya jika capability-nya enabled.
  - Jangan menulis expression evaluator generik atau menjalankan metadata sebagai kode.

### Langkah I — Generation plan model

- `src/generation/plan.rs`:
  - `PlannedOperation` dengan `relative_path: PathBuf`, `contents: Vec<u8>`, `description: String`.
  - `GenerationPlan` dengan `target_root: PathBuf` dan `operations: Vec<PlannedOperation>`.
  - `WritePolicy { PreserveExisting, Overwrite }`.
  - `OperationOutcome { Planned, Created, Skipped { reason }, Failed { error } }`.
  - `OperationResult { relative_path, outcome }`.
  - `OperationReport { target_root, results }`.

### Langkah J — Planning pipeline

- `src/generation/planner.rs`:
  - `build_plan(manifest, template, target_root) -> Result<GenerationPlan, TemplateError>`.
  - Urutan: resolve template → check compatibility → select assets → build context → render/copy assets → validate paths → return plan.
  - Tidak melakukan filesystem writes.
  - Validasi `target_root` adalah directory.
  - Semua asset wajib harus diproses sebelum plan valid.

### Langkah K — Path validation dan root containment

- `src/filesystem/paths.rs`:
  - `validate_relative_path(path) -> Result<PathBuf, PathError>`.
  - Tolak absolute path, `..`, komponen kosong, dan komponen invalid.
  - `ensure_within_root(root, candidate) -> Result<PathBuf, PathError>`.
  - Canonicalize root dan komponen yang sudah ada.
  - Tolak symlink escape.
  - Tangani error canonicalization/metadata secara eksplisit.
  - Tidak menggunakan shell.

### Langkah L — Conflict detection dan write policy

- `src/filesystem/conflicts.rs`:
  - `detect_conflict(root, relative_path) -> ConflictStatus`.
  - Bedakan `Absent`, `ExistsFile`, `ExistsDirectory`, `Invalid`.
  - Default: jangan overwrite file yang sudah ada.
  - Re-check kondisi target sebelum write.

### Langkah M — Filesystem writer

- `src/filesystem/writer.rs`:
  - `apply_plan(plan, policy) -> OperationReport`.
  - Validasi ulang path sebelum write.
  - Buat parent directory hanya bila diperlukan.
  - Gunakan `tempfile::NamedTempFile` di parent directory untuk atomic replacement.
  - Tangani error create, write, flush, close, persist, cleanup.
  - Laporkan `Created`, `Skipped`, atau `Failed`.
  - Lanjutkan operasi berikutnya setelah failure dan laporkan partial result.
  - Jangan menghapus arbitrary files.

### Langkah N — Dry-run, preview, dan report

- `src/filesystem/report.rs`:
  - `preview_plan(plan) -> OperationReport` dengan outcome `Planned` atau `Skipped` tanpa I/O.
  - Dry-run tidak membuat/mengubah/hapus file atau temporary files di target.
  - Report membedakan planned, succeeded, skipped, failed.

### Langkah O — Tests

Buat tests di `tests/`:

- `tests/template_engine.rs`:
  - Metadata valid/invalid.
  - Resolver template available/unavailable.
  - Context valid/missing.
  - Rendering valid/invalid.
  - Output deterministik.
  - Asset selection capability enabled/disabled.
  - Planning tidak mutate filesystem.
- `tests/filesystem_engine.rs`:
  - Path traversal ditolak.
  - Absolute path ditolak.
  - Root containment.
  - Symlink escape (jika platform mendukung).
  - Writer creates planned files.
  - Existing files tidak ditimpa.
  - File/directory conflict.
  - Dry-run tidak mengubah filesystem.
  - Partial failure report.
  - Invalid template/context gagal sebelum writer.

Gunakan `tempfile` dan fixture kecil.

### Langkah P — Dokumentasi

- `ARCHITECTURE.md`: catat template engine, escaping policy, metadata/asset conventions, generation plan, path safety, symlink policy, conflict behavior, dry-run semantics.
- `README.md`: jangan mengklaim scaffolding/templates tersedia; update status hanya jika perlu.
- `docs/roadmap/phase-03-template-and-filesystem-engine.md`: update status setelah implementasi.

### Langkah Q — Verification

1. `cargo metadata --no-deps --format-version 1`
2. `cargo check`
3. `cargo test`
4. `cargo fmt --check`
5. `cargo clippy --all-targets --all-features -- -D warnings`
6. `cargo build`
7. `cargo build --release`
8. Smoke tests `--help`, `--version`, invalid args
9. `git diff --check`
10. Review `git status --short` dan `git diff`.

## 4. File yang Akan Terpengaruh

### Dibuat

- `src/template/mod.rs`
- `src/template/metadata.rs`
- `src/template/resolver.rs`
- `src/template/context.rs`
- `src/template/asset.rs`
- `src/template/renderer.rs`
- `src/generation/mod.rs`
- `src/generation/plan.rs`
- `src/generation/planner.rs`
- `src/filesystem/mod.rs`
- `src/filesystem/paths.rs`
- `src/filesystem/conflicts.rs`
- `src/filesystem/writer.rs`
- `src/filesystem/report.rs`
- `tests/template_engine.rs`
- `tests/filesystem_engine.rs`

### Diubah

- `src/lib.rs`: expose `template`, `generation`, `filesystem`.
- `Cargo.toml`: tambah `tera`, promosikan `tempfile`.
- `Cargo.lock`: update otomatis.
- `ARCHITECTURE.md`: catatan keputusan Phase 3.
- `docs/roadmap/phase-03-template-and-filesystem-engine.md`: status setelah implementasi.

### Tidak diubah (out of scope)

- `PRD.md`, scaffolding workflow, generators, CI (kecuali bila diperlukan).

## 5. Dependency / Configuration Impact

- Dependency baru:
  - `tera = "1.20"` (MSRV 1.85).
- Dependency dipromosikan:
  - `tempfile = "3"` dari dev-dependency ke dependency production.
- `serde`, `toml`, `thiserror`, `clap` dipakai ulang.
- Tidak menambah `walkdir`, prompt library, async runtime, remote fetching, atau plugin dependencies.
- Tidak menambah konfigurasi global.
- `Cargo.lock` akan diperbarui oleh Cargo.

## 6. Risiko dan Potensi Konflik

- Tera 1.20 API dan escaping behavior harus diuji sebelum final.
- Autoescape dinonaktifkan global; jika template HTML ditambahkan nanti, perlu kebijakan escaping yang lebih selektif.
- Symlink escape handling berbeda antar platform; tests harus menghindari asumsi permission yang tidak portabel.
- Atomic writes memerlukan dependency production `tempfile`, meningkatkan supply-chain surface.
- Path validation harus konsisten antara planning dan writer; hindari duplikasi policy.
- Writer yang melanjutkan setelah failure menghasilkan partial output; ini harus dilaporkan secara jujur.
- Template fixtures bukan template production-ready; dokumentasi harus jelas.
- Risiko scope creep: jangan menambah `rustrs new`, wizard, atau generator.
- `TemplateId` hanya `Minimal` yang tersedia pada Phase 3; template lain sengaja unavailable.

## 7. Verification Plan

1. `cargo metadata --no-deps --format-version 1`
2. `cargo check`
3. `cargo test`
4. `cargo fmt --check`
5. `cargo clippy --all-targets --all-features -- -D warnings`
6. `cargo build`
7. `cargo build --release`
8. Smoke tests `--help`, `--version`, invalid args
9. `git diff --check`
10. Review `git status --short` dan `git diff`.

Semua hasil harus dicatat sebagai PASS/FAIL/BLOCKED, bukan diasumsikan.

## 8. Acceptance Criteria yang Akan Terpenuhi

- P3-AC-01: baseline Phase 2 dan kondisi Git diperiksa.
- P3-AC-02: template metadata memiliki kontrak yang jelas.
- P3-AC-03: resolver hanya memilih template/asset yang tersedia.
- P3-AC-04: context dibangun secara eksplisit dari konfigurasi tervalidasi.
- P3-AC-05: renderer menghasilkan output deterministik di memori.
- P3-AC-06: escaping dan template errors ditangani dengan benar.
- P3-AC-07: asset selection tidak menduplikasi compatibility/capability rules.
- P3-AC-08: generation plan merepresentasikan operasi sebelum apply.
- P3-AC-09: planning tidak mengubah filesystem.
- P3-AC-10: path traversal dan absolute target paths ditolak.
- P3-AC-11: root containment dan symlink policy ditangani.
- P3-AC-12: konflik file ditangani sesuai write policy.
- P3-AC-13: writer melaporkan hasil aktual secara akurat.
- P3-AC-14: dry-run tidak mengubah filesystem target.
- P3-AC-15: error sebelum apply tidak menyebabkan partial writes.
- P3-AC-16: dokumentasi sesuai implementasi aktual.
- P3-AC-17: semua quality gates lulus atau blocker dilaporkan jujur.
- P3-AC-18: tidak ada scaffolding end-to-end atau template API lengkap yang dibuat.

## 9. Pertanyaan / Ambiguity

Semua keputusan kritis telah dikonfirmasi:

- Template engine: Tera 1.20.
- Asset storage: Rust constants/fixtures.
- Safe writes: `tempfile` crate.
- Autoescape: dinonaktifkan global.
- Symlink policy: tolak escape.

Tidak ada pertanyaan tersisa yang memblokir implementasi.

## 10. Status dan Langkah Berikutnya

- Plan Phase 3 telah difinalisasi dengan keputusan yang tercatat di atas.
- **Belum ada kode yang diubah**; implementasi menunggu persetujuan pengguna.
- Setelah persetujuan, eksekusi Langkah A–Q secara berurutan, lalu jalankan verification plan dan buat laporan Phase 3 sesuai format yang diminta `AGENTS.md`.
- Fase berikutnya (jika disetujui setelah Phase 3 selesai): **Phase 4 — Project Scaffolding**.

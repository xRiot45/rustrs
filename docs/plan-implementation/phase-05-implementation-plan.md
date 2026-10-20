# Phase 5 — Core Code Generators: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 5 of 10  
> **Status:** PLANNED — belum ada kode yang diubah; menunggu persetujuan implementasi  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-05-core-code-generators.md`  
> **Dokumen ini:** `docs/plan-implementation/phase-05-implementation-plan.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal (preflight ringkas)

- Branch aktif: perlu diverifikasi saat implementasi; working tree memiliki `docs/roadmap/phase-05-core-code-generators.md` sebagai untracked.
- Phase 0–4 sudah selesai dan di-merge (implementasi `rustrs new` tersedia).
- Crate saat ini adalah library + binary dengan modul `cli`, `error`, `project`, `manifest`, `compatibility`, `template`, `generation`, `filesystem`, `application`.
- Dependency production: `clap`, `thiserror`, `serde`, `toml`, `tera`, `tempfile`, `dialoguer`.
- Dev-dependency: `assert_cmd`, `predicates`.
- CLI saat ini hanya memiliki subcommand `new`; belum ada `generate`/`g`.
- Compatibility matrix hanya **Axum + Postgres + SQLx**.
- Template tersedia: Minimal, Simple, Standard, Enterprise (semua Axum).
- Baseline quality gates terakhir lulus (per laporan Phase 4); perlu dijalankan ulang sebelum implementasi.
- Toolchain: Rust stable, MSRV 1.85.0, edition 2024.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan                  | Pilihan                                                                |
| -------------------------- | ---------------------------------------------------------------------- |
| Scope generator Phase 5    | **Full Phase 5 list** (semua generator yang tercantum di phase doc)    |
| Cakupan template           | **Semua 4 template** (Minimal, Simple, Standard, Enterprise)           |
| Konvensi nama input        | **Eksplisit singular dan plural** via flags (default tetap divalidasi) |
| Strategi registrasi        | **Managed marker blocks** (line markers)                               |
| Case conversion dependency | **Tambah crate `heck`**                                                |
| Semantik `--force`         | **Overwrite file generated yang ada saja** (tidak bypass safety)       |
| Test & migration generator | **Sertakan keduanya jika konvensi ada**                                |

### Keputusan implementasi yang ditetapkan (dari jawaban pengguna)

- Sintaks nama: `rustrs generate <type> <name> --singular <singular> --plural <plural>`.
- Marker registrasi: `// rustrs:begin:<section>` / `// rustrs:end:<section>`.
- `--force` hanya memetakan `WritePolicy::Overwrite` untuk file existing, tetap melewati path-safety, project validation, dan plan validation.

---

## 1. Kondisi Codebase Saat Ini

- `src/cli/mod.rs` hanya mendefinisikan global options dan subcommand `New`; **belum ada `Generate`/`g`**.
- `src/main.rs::run()` hanya dispatch `Commands::New`.
- `src/application/mod.rs` hanya berisi `new_project`; belum ada use case generate.
- Belum ada modul `domain` maupun `generator`.
- `TemplateContext` hanya berisi field project-level; belum ada field nama normalized.
- `template::renderer::render` hanya menerima `&TemplateContext`.
- `GenerationPlan` hanya berisi `PlannedOperation` (file full contents); belum ada representasi registration edit terpisah (akan ditangani sebagai file operation yang dihitung saat planning).
- `ProjectManifest` dan compatibility resolver tersedia (Phase 2), hanya Axum + Postgres + SQLx yang supported.
- Template constants tersedia untuk 4 template, tetapi registration files (mod.rs/routes.rs) belum memiliki managed markers.
- `heck` belum ada di `Cargo.toml`.

---

## 2. Task Phase 05 yang Akan Dikerjakan

1. **Task 5.0** — Preflight dan audit Phase 1–4 + baseline.
2. **Task 5.1** — Shared generator contract dan registry.
3. **Task 5.2** — Generator registry dan dispatch.
4. **Task 5.3** — Normalisasi dan validasi nama (domain naming).
5. **Task 5.4** — Validated generator context.
6. **Task 5.5** — CLI `generate` dan alias `g`.
7. **Task 5.6** — Module generator.
8. **Task 5.7** — Service generator.
9. **Task 5.8** — Controller/handler generator.
10. **Task 5.9** — Entity/model dan DTO generators.
11. **Task 5.10** — Repository generator.
12. **Task 5.11** — Middleware, guard, validator generators (gated).
13. **Task 5.12** — Resource generator (orkestrasi).
14. **Task 5.13** — Job, event, event-handler generators (gated oleh infrastruktur).
15. **Task 5.14** — Test dan migration-file generators.
16. **Task 5.15** — Integrasi shared planning/apply pipeline.
17. **Task 5.16** — Tests komprehensif.
18. **Task 5.17** — Dokumentasi dan quality gates.

### Out of scope (tetap)

- Project creation/scaffolding improvements (`rustrs new`) di luar integrasi generator.
- Inspector (`info`, `routes`, `modules`, `deps`) dan `doctor` (Phase 6).
- Database commands, migration execution, seed (Phase 7).
- Dev process management, Docker orchestration (Phase 7/8).
- Docker/CI/CD/quality tooling generation penuh (Phase 8).
- Publishing, release automation (Phase 9).
- Menambah framework/database/ORM baru di luar support matrix.
- Auto-install dependency, silent manifest upgrade.
- Rewrite unrelated modules.

---

## 3. Implementation Plan

### Langkah A — Preflight dan audit

- [ ] Jalankan `git status --short`, `git branch --show-current`, `git log -5 --oneline`.
- [ ] Jalankan `rustc --version` dan `cargo --version`.
- [ ] Inspeksi struktur repository dan perubahan lokal.
- [ ] Baca `Cargo.toml`/`Cargo.lock`; identifikasi dependency yang sudah tersedia.
- [ ] Review CLI, error/exit-code mapping, project/path utilities, manifest, compatibility, template engine, planner, path validator, conflict policy, writer, dry-run, dan report.
- [ ] Jalankan baseline:

  ```bash
  cargo metadata --no-deps --format-version 1
  cargo check
  cargo test
  cargo fmt --check
  cargo clippy --all-targets --all-features -- -D warnings
  git diff --check
  ```

- [ ] Catat baseline failures terpisah dari regression baru.
- [ ] Identifikasi API Phase 1–4 yang dapat digunakan; jangan membuat duplikat.
- [ ] Buat checklist perubahan sebelum mengedit kode.

### Langkah B — Tambah dependency `heck`

`Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
thiserror = "2"
serde = { version = "1", features = ["derive"] }
toml = "1"
tera = "1.20"
tempfile = "3"
dialoguer = "0.11"
heck = "0.5"
```

- [ ] Tambah `heck` untuk case conversion robust (`to_snake_case`, `to_pascal_case`, `to_lower_camel_case`).
- [ ] Jalankan `cargo check` setelah perubahan dependency.
- [ ] Verifikasi MSRV 1.85 dan platform compatibility.
- [ ] Dokumentasikan alasan dependency baru (AGENTS §5).

### Langkah C — Domain naming module

Buat `src/domain/mod.rs` dan `src/domain/naming.rs`.

- [ ] Definisikan `NormalizedName` (module, singular_type, plural_type, file_stem, route_segment).
- [ ] Implement `normalize_name(input) -> Result<NormalizedName, NamingError>`.
- [ ] Implement validasi:
  - tolak empty/whitespace
  - tolak path separator, `.`/`..`, absolute path
  - tolak control chars
  - tolak Rust keywords/raw identifiers invalid
  - tolak karakter non-ASCII kecuali policy eksplisit (default tolak)
- [ ] Implement collision detection: dua input berbeda yang normalize ke output sama → error.
- [ ] Implement plural/singular derivation dengan `heck` + aturan eksplisit; jika ambiguous, wajib flag `--singular`/`--plural`.
- [ ] Tambah `NamingError` dengan pesan actionable.
- [ ] Unit tests: normal, hyphenated, underscored, numbers, empty, keywords, unicode, traversal, collision.

### Langkah D — Generator core

Buat `src/generator/`:

- [ ] `src/generator/mod.rs` — expose submodul.
- [ ] `src/generator/kind.rs` — `GeneratorKind` enum + parsing string + daftar supported.
- [ ] `src/generator/registry.rs` — central registry/dispatch; unknown → error + guidance.
- [ ] `src/generator/context.rs` — `GeneratorContext`:
  - project root
  - manifest
  - framework/database/data_access
  - features
  - layout convention (derived from template)
  - normalized names
  - skip_tests/skip_docs flags
- [ ] Definisikan trait/interface kecil:

  ```rust
  pub trait Generator {
      fn kind(&self) -> GeneratorKind;
      fn validate(&self, ctx: &GeneratorContext) -> Result<(), CliError>;
      fn plan(&self, ctx: &GeneratorContext) -> Result<GenerationPlan, CliError>;
  }
  ```

  (Atau enum dispatch jika trait overkill; pilih yang paling sederhana dan testable.)
- [ ] Reuse `template::renderer::render` dengan generalisasi generic serialize.
- [ ] `src/generator/registration.rs` — managed marker updater.

### Langkah E — Generalisasi renderer

- [ ] Ubah `render` menjadi generic:

  ```rust
  pub fn render<T: serde::Serialize>(source: &str, context: &T) -> Result<String, TemplateError>
  ```

- [ ] Update caller di `generation::planner::build_plan` dan tests.
- [ ] Pastikan autoescape tetap off.

### Langkah F — Managed marker strategy

- [ ] Tambah marker pada template constants yang relevan:
  - Standard/Enterprise `src/modules/mod.rs`: `// rustrs:begin:modules`, `// rustrs:end:modules`, `// rustrs:begin:routes`, `// rustrs:end:routes`.
  - Minimal/Simple registration files sesuai layout.
- [ ] Implement parser updater:
  - baca file (jika ada)
  - cari marker pair
  - jika marker tidak ada → error actionable
  - cek deklarasi existing (idempotent)
  - sisipkan deklarasi baru di dalam marker
  - tulis ulang full file via `PlannedOperation`
- [ ] Validasi path registration dengan `ensure_within_root`.

### Langkah G — CLI `generate` dan alias `g`

`src/cli/commands.rs`:

- [ ] Tambah `GenerateArgs`:

  ```rust
  pub struct GenerateArgs {
      pub generator: String,        // atau GeneratorKind via ValueEnum
      pub name: String,
      #[arg(long)]
      pub singular: Option<String>,
      #[arg(long)]
      pub plural: Option<String>,
      #[arg(long)]
      pub dry_run: bool,
      #[arg(long)]
      pub force: bool,
      #[arg(long)]
      pub skip_tests: bool,
      #[arg(long)]
      pub skip_docs: bool,
      #[arg(long)]
      pub path: Option<PathBuf>,
  }
  ```

- [ ] Tambah varian `Commands::Generate(GenerateArgs)` dan alias `g` (clap `#[command(alias = "g")]` atau varian terpisah).
- [ ] Update help agar hanya menampilkan generator yang diimplementasikan.
- [ ] Dispatch di `src/main.rs::run()`.

### Langkah H — Application orchestration generate

Buat `src/application/generate.rs`:

- [ ] Resolve project root (via `--path` atau discovery) + manifest.
- [ ] Validasi compatibility.
- [ ] Parse/validate generator kind.
- [ ] Normalize names (dengan flags singular/plural; validasi collision).
- [ ] Build `GeneratorContext`.
- [ ] Capability/prerequisite check per generator.
- [ ] Build complete plan (file + registration) sebelum apply.
- [ ] Dry-run preview.
- [ ] Apply dengan `WritePolicy` sesuai `--force`.
- [ ] Report created/skipped/failed + manual follow-up.
- [ ] Exit code mapping (invalid input→2, conflict→3, dll).

### Langkah I — Template-specific layouts

Definisikan layout per template:

- **Minimal**
  - `src/handlers/<name>.rs`
  - `src/models/<singular>.rs`
  - `src/routes/<name>.rs`
  - registration di `src/main.rs` / `src/routes.rs` (marker)
- **Simple**
  - `src/modules/<name>/handlers.rs`
  - `src/modules/<name>/models.rs`
  - `src/modules/<name>/routes.rs`
  - `src/modules/<name>/tests.rs` (opsional)
  - registration di `src/modules/mod.rs` atau `src/main.rs`
- **Standard**
  - `src/modules/<name>/controller.rs`
  - `src/modules/<name>/service.rs`
  - `src/modules/<name>/repository.rs`
  - `src/modules/<name>/entity.rs`
  - `src/modules/<name>/dto.rs`
  - `src/modules/<name>/routes.rs`
  - `src/modules/<name>/tests.rs` (opsional)
  - registration di `src/modules/mod.rs`
- **Enterprise**
  - vertical slice sesuai PRD §10.1 (domain/application/infrastructure/interfaces)
  - registration di `src/modules/mod.rs` atau `src/interfaces/http.rs`

- [ ] Implement layout resolver di `GeneratorContext` berdasarkan `manifest.template`.
- [ ] Tolak generator yang tidak relevan untuk template tertentu dengan pesan jelas (mis. repository untuk Minimal).

### Langkah J — Implement generator individual

Untuk setiap generator:

- [ ] `module` — buat skeleton module + registrasi parent.
- [ ] `entity`/`model` — struct dengan derives sesuai data access (SQLx row derive bila supported).
- [ ] `dto` — request/response types dengan serde; validation hanya jika dependency ada (saat ini serde only; validator crate tidak ada → jangan klaim validation).
- [ ] `service` — skeleton dengan method CRUD placeholder.
- [ ] `repository` — trait + implementasi SQLx (hanya Axum+Postgres+Sqlx).
- [ ] `controller`/`handler` — Axum handler signatures valid.
- [ ] `middleware` — Axum middleware skeleton.
- [ ] `guard` — hanya jika auth infra ada; saat ini templates tidak menyediakan → tolak dengan pesan prerequisite.
- [ ] `validator` — tolak atau skeleton tanpa crate eksternal.
- [ ] `test` — skeleton test dengan `#[cfg(test)]`.
- [ ] `migration` — file di `migrations/` dengan nama aman; tanpa eksekusi DB.
- [ ] `job` — skeleton tanpa queue infra → tolak dengan pesan (atau basic struct jika diizinkan).
- [ ] `event`/`event-handler` — skeleton tanpa dispatcher → tolak dengan pesan.
- [ ] `handler` — alias/terminology konsisten (pilih `controller` atau `handler`; jangan expose sinonim ambigu).

Setiap generator:

- [ ] Validasi prerequisite/capability.
- [ ] Plan file + registration.
- [ ] Tidak ada fake business logic.
- [ ] Output valid (imports hanya dependency yang ada).

### Langkah K — Resource generator

- [ ] Validasi name + context.
- [ ] Tentukan artifact set per template (dokumentasikan).
- [ ] Build satu combined plan.
- [ ] Validasi semua path/conflict sebelum write.
- [ ] Render semua output.
- [ ] Apply via shared pipeline.
- [ ] Report setiap file + registration change.
- [ ] Hindari duplikasi jika resource sudah ada.

### Langkah L — Integrasi safety

- [ ] Semua generator menggunakan shared planning/apply pipeline.
- [ ] Dry-run tanpa side effect.
- [ ] Existing files preserved by default; `--force` override file only.
- [ ] Registration edits idempotent (dedup deklarasi).
- [ ] Report akurat (tidak klaim aksi yang tidak dilakukan).

### Langkah M — Tests

#### Unit

- [ ] Registry/dispatch.
- [ ] Naming normalization/validation/collision.
- [ ] Context validation.
- [ ] Capability/prerequisite.
- [ ] Plan construction.
- [ ] Registration marker updater.

#### Template/output

- [ ] Expected paths per generator.
- [ ] Important code structure/imports.
- [ ] Normalized names konsisten.
- [ ] Deterministik.
- [ ] Tidak ada unresolved placeholder.
- [ ] Tidak ada dependency imaginary.

#### Filesystem integration

- [ ] Success generation.
- [ ] Outside project.
- [ ] Missing/malformed manifest.
- [ ] Existing target files.
- [ ] Existing registration.
- [ ] Multiple conflicts.
- [ ] Unsafe path.
- [ ] Dry-run unchanged.
- [ ] Failure before apply unchanged.
- [ ] Force behavior.
- [ ] Repeated generation/collision.
- [ ] Paths with spaces.

#### CLI

- [ ] `generate` dan `g`.
- [ ] Valid generator + name.
- [ ] Unknown generator.
- [ ] Missing args.
- [ ] `--dry-run`.
- [ ] Conflict reporting.
- [ ] Exit status.
- [ ] Help hanya generator supported.

#### Generated-code verification

- [ ] Minimal isolated project per template.
- [ ] Run generator.
- [ ] Verify files/registration.
- [ ] `cargo fmt --check`.
- [ ] `cargo check` jika environment mendukung (env-gated).

#### Regression

- [ ] Phase 1–4 tests tetap lulus.

### Langkah N — Dokumentasi

- [ ] Generator reference: names, args, aliases, flags.
- [ ] Contoh per generator yang diimplementasikan.
- [ ] Naming/normalization rules.
- [ ] Project context requirements.
- [ ] Dry-run/conflict/force behavior.
- [ ] Prerequisites/unsupported configs.
- [ ] Resource artifact composition per template.
- [ ] Update README secukupnya.
- [ ] Roadmap status truthful.
- [ ] Help ↔ docs konsisten.

### Langkah O — Quality gates

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

- [ ] Jalankan generated-project smoke tests.
- [ ] Laporkan hasil sebenarnya (PASS/FAIL/BLOCKED).

---

## 4. File yang Akan Terpengaruh

### Dibuat

- `src/domain/mod.rs`
- `src/domain/naming.rs`
- `src/generator/mod.rs`
- `src/generator/kind.rs`
- `src/generator/registry.rs`
- `src/generator/context.rs`
- `src/generator/registration.rs`
- `src/generator/module.rs`
- `src/generator/entity.rs`
- `src/generator/dto.rs`
- `src/generator/service.rs`
- `src/generator/repository.rs`
- `src/generator/handler.rs`
- `src/generator/middleware.rs`
- `src/generator/guard.rs`
- `src/generator/validator.rs`
- `src/generator/test.rs`
- `src/generator/migration.rs`
- `src/generator/job.rs`
- `src/generator/event.rs`
- `src/generator/resource.rs`
- `src/application/generate.rs`
- `tests/generators.rs` (atau perluasan tests)

### Diubah

- `Cargo.toml`, `Cargo.lock` (tambah `heck`)
- `src/lib.rs` (expose `domain`, `generator`)
- `src/main.rs` (dispatch `Generate`)
- `src/cli/mod.rs`, `src/cli/commands.rs`
- `src/application/mod.rs`
- `src/template/renderer.rs` (generalisasi render)
- `src/template/context.rs` (opsional; jika tetap dipakai)
- `src/template/metadata.rs` (marker pada registration constants)
- `src/generation/planner.rs` (jika diperlukan adaptasi)
- `README.md`
- `ARCHITECTURE.md` (jika boundary berubah)
- `docs/roadmap/phase-05-core-code-generators.md` (status)

### Tidak diubah (out of scope)

- `PRD.md`, phase docs lain, Phase 6–9 features, CI workflow (kecuali bila diperlukan).

---

## 5. Dependency / Configuration Impact

- Dependency baru:
  - `heck = "0.5"` — case conversion robust (AGENTS §5: dijelaskan requirement-nya).
- Tidak menambah async runtime ke CLI.
- Tidak menambah remote template-fetching, plugin, workspace.
- `serde`, `toml`, `thiserror`, `clap`, `tera`, `tempfile`, `dialoguer` dipakai ulang.
- `Cargo.lock` diperbarui oleh Cargo.
- Tidak ada perubahan schema manifest (tetap v1).
- Template constants berubah untuk menyertakan managed markers → berdampak pada scaffolding output dan snapshot tests.
- Compatibility matrix tetap hanya Axum + Postgres + SQLx.

---

## 6. Risiko dan Potensi Konflik

- **Scope sangat besar** (full list + 4 template); risiko partial completion. Jika tidak feasible, laporkan `PARTIAL` dengan daftar generator yang selesai.
- **Managed markers** memerlukan modifikasi template registration files; jika marker hilang/invalid, generator harus gagal aman, bukan rewrite heuristik.
- **Explicit singular/plural flags** meningkatkan kompleksitas CLI dan validasi; perlu aturan default yang jelas.
- **Generated code compile** bergantung pada dependency fixture; environment mungkin memblokir network. Harus dicatat sebagai `BLOCKED` bila tidak bisa dijalankan.
- Potensi konflik dengan `AGENTS.md` (scope creep) diminimalkan dengan tetap tidak menyentuh Phase 6–9.
- Registration edits bisa menyebabkan partial inconsistency bila apply gagal di tengah; report harus jujur.
- `guard`/`validator`/`job`/`event` kemungkinan tidak memiliki infrastruktur di template saat ini → akan ditolak dengan pesan prerequisite (bukan klaim tersedia).

---

## 7. Verification Plan

1. `cargo metadata --no-deps --format-version 1`
2. `cargo check`
3. `cargo test`
4. `cargo fmt --check`
5. `cargo clippy --all-targets --all-features -- -D warnings`
6. `cargo build`
7. `cargo build --release`
8. Smoke tests `--help`, `--version`, `generate --help`, `g --help`, invalid args
9. `git diff --check`
10. Integration tests generate (semua generator yang diimplementasikan)
11. Dry-run immutability test (before/after tree)
12. Conflict tests
13. Generated-project smoke: `cargo fmt --check` dan `cargo check` per template (env-gated)
14. Review `git status --short` dan `git diff`

Semua hasil dicatat PASS/FAIL/BLOCKED, bukan diasumsikan.

---

## 8. Acceptance Criteria yang Akan Terpenuhi

P5-AC-01 s/d P5-AC-42 sesuai `docs/roadmap/phase-05-core-code-generators.md`, dengan fokus:

- P5-AC-04..P5-AC-06 (shared contract)
- P5-AC-07..P5-AC-08 (registry)
- P5-AC-09..P5-AC-11 (naming)
- P5-AC-12..P5-AC-13 (context)
- P5-AC-14..P5-AC-16 (CLI)
- P5-AC-17..P5-AC-25 (foundational generators)
- P5-AC-26..P5-AC-29 (resource)
- P5-AC-30..P5-AC-31 (job/event/test/migration)
- P5-AC-32..P5-AC-35 (safety/dry-run/reporting)
- P5-AC-36..P5-AC-39 (tests)
- P5-AC-40..P5-AC-42 (docs/quality gates)

Jika hanya sebagian generator feasible, status fase dilaporkan `PARTIAL`, bukan `COMPLETE`.

---

## 9. Pertanyaan / Ambiguity

Keputusan pengguna sudah diperoleh via tool question:

- Scope generator: **full Phase 5 list**.
- Template: **semua 4**.
- Naming: **eksplisit singular/plural flags**.
- Registrasi: **managed marker blocks** (line markers).
- Dependency: **tambah `heck`**.
- `--force`: **overwrite file existing saja**.
- Test/migration: **sertakan jika konvensi ada**.
- Sintaks CLI: **flags**.
- Marker format: **line markers**.

Assumption yang masih perlu diverifikasi saat implementasi (bukan blocker plan):

- Versi `heck` yang tersedia di registry.
- Layout detail Minimal/Simple untuk generator (akan ditetapkan berdasarkan template aktual dan didokumentasikan).
- Infrastruktur auth/validation/job/event pada template (kemungkinan belum ada → generator akan ditolak dengan pesan prerequisite).

Tidak ada ambiguity yang memblokir penyusunan plan.

---

## 10. Status dan Langkah Berikutnya

- Plan Phase 5 telah difinalisasi dengan keputusan yang tercatat di atas.
- **Belum ada kode yang diubah**; implementasi menunggu persetujuan pengguna.
- Setelah persetujuan, eksekusi Langkah A–O secara berurutan, lalu jalankan verification plan dan buat laporan Phase 5 sesuai format yang diminta `AGENTS.md`.
- Fase berikutnya (jika disetujui setelah Phase 5 selesai): **Phase 6 — Project Inspector and Diagnostics**.

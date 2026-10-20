# Phase 4 — Project Scaffolding: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 4 of 10  
> **Status:** PLANNED — belum ada kode yang diubah; menunggu persetujuan implementasi  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-04-project-scaffolding.md`  
> **Dokumen ini:** `docs/plan-implementation/phase-04-implementation-plan.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

- Branch aktif: `feat/phase-03-template-and-filesystem-engine`, working tree hanya memiliki `docs/roadmap/phase-04-project-scaffolding.md` sebagai untracked.
- Phase 0, 1, 2, dan 3 sudah selesai dan di-merge.
- Crate saat ini adalah library + binary dengan modul `cli`, `error`, `project`, `manifest`, `compatibility`, `template`, `generation`, dan `filesystem`.
- Dependency production: `clap`, `thiserror`, `serde`, `toml`, `tera`, `tempfile`.
- Dev-dependency: `assert_cmd`, `predicates`.
- Belum ada subcommand `new`, application layer, atau wizard.
- Baseline quality gates lulus.
- Toolchain: Rust stable 1.94.0, MSRV 1.85.0, edition 2024.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan                                        | Pilihan                                                                       |
| ------------------------------------------------ | ----------------------------------------------------------------------------- |
| Template scope Phase 4                           | **Semua 4 template** (Minimal, Simple, Standard, Enterprise) dibuat buildable |
| Prompt library wizard                            | **dialoguer**                                                                 |
| Git initialization                               | **Skip** di Phase 4 (tidak expose opsi)                                       |
| Default features starter                         | **Semua false**                                                               |
| Smoke build (cargo check) pada generated project | **Env-gated** (`RUSTRS_SMOKE_BUILD=1`)                                        |

---

## 1. Kondisi Codebase Saat Ini

- `src/cli/mod.rs` hanya mendefinisikan global options (`--verbose`, `--quiet`, `--no-color`); **belum ada subcommand**.
- `src/main.rs::run()` masih no-op.
- `TemplateId` mendukung 4 template, tetapi resolver hanya menyediakan `Minimal` (`BASIC_TEMPLATE`); tiga template lain sengaja `Unavailable`.
- `build_plan` hanya merender asset template; belum menyertakan `.rustrs/config.toml`.
- `ProjectManifest` sudah memiliki model, parser, validation, discovery, dan compatibility resolver (Phase 2).
- Template engine, generation plan, path safety, conflict detection, writer, dry-run, dan report sudah tersedia (Phase 3).
- `Cargo.toml` belum memiliki `dialoguer`.

---

## 2. Task Phase 04 yang Akan Dikerjakan

1. **Task 4.0** — Preflight dan audit Phase 3 + baseline.
2. **Task 4.1** — CLI contract `rustrs new`.
3. **Task 4.2** — Input resolution dan default configuration.
4. **Task 4.3** — Interactive wizard (dialoguer) + non-interactive mode.
5. **Task 4.4** — Validasi nama project dan target directory.
6. **Task 4.5** — Compatibility dan capability preflight (reuse Phase 2).
7. **Task 4.6** — Built-in starter template assets untuk keempat template.
8. **Task 4.7** — Integrasi rendering dan generation plan.
9. **Task 4.8** — Dry-run, preview, confirmation, dan apply.
10. **Task 4.9** — Manifest `.rustrs/config.toml` via serializer + read-back.
11. **Task 4.10** — Git initialization — **ditiadakan** (tidak expose opsi).
12. **Task 4.11** — Report, diagnostics, dan next steps.
13. **Task 4.12** — Smoke verification generated project.
14. **Task 4.13** — Automated tests.
15. **Task 4.14** — Dokumentasi, quality gates, dan final review.

### Out of scope (tetap)

- Generator Phase 5 (`module`, `resource`, `controller`, `service`, `repository`, `entity`, `dto`, `middleware`, `guard`, `validator`, `job`, `event`, `handler`, `test`, `migration`).
- Inspector (`info`, `routes`, `modules`, `deps`).
- Database commands dan development workflow process management.
- Implementasi penuh capability auth, RBAC, Redis, email, storage, audit logging, rate limiting, idempotency, observability, Docker, CI/CD.
- Remote template registry, marketplace, plugin system, auto-update, publishing, release, deployment.
- Refactor besar yang tidak diperlukan.
- Command placeholder yang mengembalikan sukses.
- Commit, push, publish, atau release tanpa instruksi eksplisit.

---

## 3. Implementation Plan

### Langkah A — Preflight dan audit Phase 3

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
- [ ] Identifikasi API Phase 2/3 yang dapat digunakan; jangan membuat duplikat.
- [ ] Buat checklist perubahan sebelum mengedit kode.

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
dialoguer = "0.11"
```

- `dialoguer` ditambahkan untuk wizard interaktif (sesuai PRD dan keputusan pengguna).
- Gunakan `std::io::IsTerminal` (stabil) untuk deteksi TTY, tanpa dependency tambahan.
- Jalankan `cargo check` setelah perubahan dependency.
- Verifikasi MSRV 1.85 dan platform compatibility.

### Langkah C — CLI contract `rustrs new`

- [ ] Tambahkan `new` ke Clap mengikuti pola yang ada.
- [ ] Definisikan enum `Commands` dengan varian `New(NewArgs)` dan field `command: Option<Commands>` pada `Cli`.
- [ ] Gunakan typed domain enums (`TemplateId`, `Framework`, `Database`, `DataAccess`) untuk flags melalui `clap::ValueEnum`.
- [ ] Derive `clap::ValueEnum` pada `TemplateId`, `Framework`, `Database`, `DataAccess`, `Architecture` di `src/manifest/model.rs`.
- [ ] Flags minimum:
  - `project_name` (positional `String`)
  - `--template` (`Option<TemplateId>`)
  - `--framework` (`Option<Framework>`)
  - `--database` (`Option<Database>`)
  - `--orm` / `--data-access` (`Option<DataAccess>`)
  - `--non-interactive` (`bool`)
  - `--defaults` (`bool`)
  - `--dry-run` (`bool`)
- [ ] Tentukan perilaku flags yang bertentangan (mis. `--defaults` + input wajib yang bertentangan → error).
- [ ] Pertahankan global options Phase 1 (`--verbose`, `--quiet`, `--no-color`).
- [ ] Help hanya menampilkan opsi yang benar-benar tersedia.
- [ ] Jangan menambahkan `--force` (semantik overwrite belum didefinisikan).
- [ ] Jangan menganggap `--defaults` sebagai izin overwrite.
- [ ] Jangan mengembalikan sukses jika workflow tidak dijalankan.
- [ ] Update `src/main.rs::run()` untuk dispatch `Commands::New` ke application layer.

### Langkah D — Application layer dan input resolution

Struktur:

```text
src/application/
├── mod.rs
└── new_project.rs
```

- [ ] Buat `src/application/mod.rs` dan `src/application/new_project.rs`.
- [ ] Expose `pub mod application;` di `src/lib.rs`.
- [ ] Definisikan `ScaffoldRequest`, `ResolvedScaffold`, `resolve_scaffold()`, `execute()`.
- [ ] Pisahkan raw CLI input dari konfigurasi final tervalidasi.
- [ ] Gabungkan flags, wizard answers, dan documented defaults.
- [ ] Nilai eksplisit tidak boleh ditimpa default atau wizard.
- [ ] Bedakan field yang belum dipilih dengan pilihan eksplisit (mis. database `none`).
- [ ] Terapkan default hanya pada field yang telah memiliki keputusan produk.
- [ ] Jangan menganggap `Features::default()` otomatis merupakan konfigurasi produk yang benar.
- [ ] Default tidak boleh mengaktifkan capability yang belum diimplementasikan (semua features `false`).
- [ ] `--defaults` harus deterministik dan tidak memerlukan prompt.
- [ ] Input wajib yang hilang pada non-interactive mode menghasilkan error actionable.
- [ ] Resolver dapat diuji tanpa terminal dan tanpa filesystem writes.
- [ ] Prioritas input terdokumentasi dan diuji.
- [ ] Default yang didokumentasikan (saat `--defaults` atau fallback interaktif):
  - template: `Minimal`
  - framework: `Axum`
  - database: `Postgres`
  - data_access: `Sqlx`
  - architecture: `CleanModular`
  - features: semua `false`
- [ ] Non-interactive tanpa `--defaults`: wajib semua field tersedia; jika tidak → error dengan contoh perintah.
- [ ] Wizard (dialoguer):
  - Hanya tawarkan pilihan yang benar-benar didukung (saat ini 4 template, framework/DB/ORM default ke kombinasi supported karena hanya Axum+Postgres+Sqlx yang supported).
  - Lewati pertanyaan yang tidak relevan.
  - Tampilkan ringkasan final sebelum apply.
  - Pembatalan, EOF, dan input invalid ditangani dengan jelas.
  - Cancel sebelum apply tidak membuat target directory atau file.
  - Jangan memulai prompt jika stdin bukan terminal interaktif (`std::io::IsTerminal`).
  - Jangan menaruh generation logic di callback wizard.
- [ ] Input sama menghasilkan konfigurasi sama di kedua mode.

### Langkah E — Validasi nama project dan target directory

- [ ] Reuse `validate_project_name` dari Phase 2.
- [ ] Tetapkan aturan huruf besar, underscore/hyphen, angka, reserved names, dan karakter invalid (sudah ada di Phase 2; dokumentasikan hubungan nama directory dan package).
- [ ] Jika nama dinormalisasi, tampilkan nama final sebelum apply.
- [ ] Jangan mengubah identitas nama diam-diam.
- [ ] Target root = `cwd/project_name`.
- [ ] Validasi parent dan target directory:
  - target belum ada → OK.
  - direktori kosong → OK.
  - direktori berisi file → conflict error.
  - file biasa → conflict error.
  - permission errors → laporkan.
- [ ] Gunakan root-containment policy Phase 3.
- [ ] Tolak path traversal dan symlink escape.
- [ ] Jangan menghapus target yang ada.
- [ ] Default tidak menimpa file existing.
- [ ] Writer harus memeriksa ulang kondisi kritis sebelum write.
- [ ] Conflict → `CliError::Conflict` (exit code 3).

### Langkah F — Compatibility dan capability preflight

- [ ] Gunakan model dan validator manifest Phase 2.
- [ ] Gunakan compatibility resolver sebagai satu-satunya sumber kebenaran.
- [ ] Validasi template, framework, database, data-access, architecture, dan capability relevan.
- [ ] Bedakan capability yang dikenal schema dari capability yang benar-benar diimplementasikan.
- [ ] Unsupported combinations ditolak sebelum write.
- [ ] Jangan mengganti pilihan pengguna secara otomatis.
- [ ] Berikan alasan dan alternatif yang benar-benar valid jika tersedia.
- [ ] Jangan menganggap enum yang ada otomatis berarti kombinasi didukung.
- [ ] Jangan memperluas support matrix diam-diam.
- [ ] Jangan menghasilkan project setengah valid jika template memerlukan kombinasi unsupported.
- [ ] Kombinasi supported saat ini: Axum + Postgres + SQLx.

### Langkah G — Built-in starter template assets

- [ ] Inspeksi metadata/asset conventions Phase 3; jangan membuat loader paralel.
- [ ] Template bawaan tersedia offline.
- [ ] Metadata dan asset konsisten.
- [ ] Sediakan baseline starter untuk **keempat** template (semua dinyatakan available).
- [ ] Struktur sesuai tujuan template:
  - **Enterprise:** struktur modular lebih terpisah; jangan mengklaim capability enterprise yang belum dibuat.
  - **Standard:** struktur modular seimbang.
  - **Simple:** abstraksi lebih sedikit.
  - **Minimal:** starter sekecil mungkin yang tetap valid dan buildable.
- [ ] Jangan memaksa Simple/Minimal menggunakan struktur Enterprise.
- [ ] Context placeholders eksplisit dan tidak tertinggal di output.
- [ ] Conditional assets hanya untuk fitur yang benar-benar diimplementasikan (Dockerfile condition `Capability::Docker`, default false).
- [ ] Semua target paths relatif dan aman.
- [ ] Tidak ada secret atau credential nyata.
- [ ] Output deterministik.

File starter per template (minimal tapi buildable, Axum hello-world):

- **Minimal**
  - `Cargo.toml` (Template)
  - `src/main.rs` (Template)
  - `src/routes.rs` (Template)
  - `README.md` (Template)
  - `.gitignore` (Static)
- **Simple**
  - `Cargo.toml` (Template)
  - `src/main.rs` (Template)
  - `src/handlers.rs` (Template, placeholder)
  - `src/models.rs` (Template, placeholder)
  - `src/routes.rs` (Template)
  - `README.md` (Template)
  - `.gitignore` (Static)
- **Standard**
  - `Cargo.toml` (Template)
  - `src/main.rs` (Template)
  - `src/modules/mod.rs` (Template)
  - `src/modules/health.rs` (Template)
  - `README.md` (Template)
  - `.gitignore` (Static)
- **Enterprise**
  - `Cargo.toml` (Template)
  - `src/main.rs` (Template)
  - `src/modules/mod.rs` (Template)
  - `src/modules/health.rs` (Template)
  - `src/application/mod.rs` (Template, placeholder)
  - `src/infrastructure/mod.rs` (Template, placeholder)
  - `src/interfaces/mod.rs` (Template)
  - `src/interfaces/http.rs` (Template)
  - `README.md` (Template)
  - `.gitignore` (Static)

Cargo.toml template (berbagi, dengan placeholder):

```toml
[package]
name = "{{ project_name }}"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
```

Contoh `src/main.rs` (Minimal):

```rust
mod routes;

#[tokio::main]
async fn main() {
    let app = routes::router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Contoh `src/routes.rs` (Minimal):

```rust
use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/", get(|| async { "Hello from {{ project_name }}!" }))
}
```

- [ ] Buildability:
  - Cargo manifest valid dan package name valid.
  - Source entry point/module declarations konsisten.
  - Dependency sesuai kombinasi teknologi.
  - Feature-disabled tidak meninggalkan reference yang rusak.
  - Kombinasi yang dinyatakan supported dapat lolos `cargo check` ketika environment mengizinkan dependency tersedia.
  - Starter tidak bergantung pada generator Phase 5.
- [ ] Metadata:
  - Tambah konstanta `MINIMAL_TEMPLATE`, `SIMPLE_TEMPLATE`, `STANDARD_TEMPLATE`, `ENTERPRISE_TEMPLATE`.
  - `BASIC_TEMPLATE` tetap alias ke `MINIMAL_TEMPLATE` (compatibility tests).
  - Resolver memetakan setiap `TemplateId` ke metadata-nya (semua available).

### Langkah H — Integrasi rendering dan generation plan

- [ ] Scaffolding service menerima konfigurasi final yang tervalidasi.
- [ ] Template resolver memilih asset yang tepat.
- [ ] Context berasal dari konfigurasi final.
- [ ] Renderer menghasilkan konten di memori.
- [ ] Planner menyusun seluruh file/dir operations sebelum apply.
- [ ] Plan mencakup starter files dan manifest.
- [ ] Semua path dan konflik divalidasi sebelum writer dipanggil.
- [ ] Rendering/planning failure tidak menyebabkan target writes.
- [ ] Plan yang sama digunakan untuk preview dan apply.
- [ ] Tidak ada direct file writes dari CLI, wizard, atau renderer.
- [ ] **Modifikasi kecil `build_plan`** (justified): append operasi `.rustrs/config.toml` yang diserialisasi via `toml::to_string_pretty(&manifest)` (satu sumber kebenaran manifest).
- [ ] Validasi path `.rustrs/config.toml` dengan `validate_relative_path`.
- [ ] Tambah varian `TemplateError::Manifest { message }` untuk error serialisasi (atau reuse varian yang ada dengan pesan jelas).
- [ ] Renderer Tera tetap autoescape off (Phase 3 policy).

### Langkah I — Dry-run, preview, confirmation, dan apply

#### Dry-run

- [ ] Menggunakan pipeline dan plan yang sama dengan apply.
- [ ] Tidak membuat target directory, menulis/menghapus file, atau membuat temporary file di target.
- [ ] Menampilkan relative path, operation type, dan known conflicts.
- [ ] Exit status menunjukkan invalid plan atau fatal conflict.

#### Confirmation

- [ ] Interactive mode menampilkan ringkasan sebelum apply.
- [ ] Confirmation dilakukan sebelum file pertama ditulis.
- [ ] Cancel tidak meninggalkan project parsial.
- [ ] Non-interactive mode tidak meminta prompt.
- [ ] Defaults tidak dianggap sebagai izin overwrite.

#### Apply

- [ ] Hanya plan tervalidasi yang boleh diterapkan.
- [ ] Gunakan writer Phase 3.
- [ ] Writer memeriksa ulang kondisi target yang kritis.
- [ ] Partial failures dilaporkan dengan operation report aktual.
- [ ] Jangan mengklaim multi-file atomic transaction jika tidak didukung.
- [ ] Jangan menghapus target sebagai recovery.
- [ ] Berikan diagnostic dan recovery guidance aman jika apply gagal.
- [ ] Buat target root (`create_dir_all`) setelah konfirmasi, sebelum apply.

### Langkah J — Manifest `.rustrs/config.toml`

- [ ] Gunakan `ProjectManifest` dan schema version Phase 2.
- [ ] Manifest berasal dari konfigurasi final, bukan raw CLI input.
- [ ] Template/framework/architecture/database/data-access/features konsisten dengan project output.
- [ ] Pilih satu sumber kebenaran untuk manifest: **serializer** (`toml::to_string_pretty`), bukan template asset.
- [ ] Serialisasi deterministik.
- [ ] Manifest masuk ke generation plan dan ditulis melalui filesystem engine.
- [ ] Tidak menyimpan secrets runtime atau placeholder secret yang tampak valid.
- [ ] Baca ulang manifest setelah apply menggunakan parser/validator Phase 2 (`discover_manifest` + `validate()`).
- [ ] Jika read-back gagal, laporkan verification failure.
- [ ] Jangan menambah manifest migration yang tidak diperlukan.

### Langkah K — Optional Git initialization

**Ditiadakan.** Tidak expose opsi Git pada CLI Phase 4. Defer ke fase berikutnya.

### Langkah L — Report, diagnostics, dan next steps

- [ ] Tampilkan project name, output path, template, dan teknologi final.
- [ ] Report sesuai operation report filesystem.
- [ ] Bedakan success, partial success, dan failure.
- [ ] Jangan tampilkan success sebelum operasi terverifikasi sesuai kontrak.
- [ ] Hormati stdout/stderr, `--quiet`, `--verbose`, `--no-color`, dan exit codes Phase 1.
- [ ] Berikan diagnostic actionable untuk invalid input, unsupported combination, missing assets, render/plan/path/conflict/write/manifest errors.
- [ ] Jangan mencetak secrets atau environment lengkap.
- [ ] Next steps sesuai output aktual, misalnya `cd <project-name>` dan `cargo check`.
- [ ] Jangan menyarankan command yang belum tersedia.
- [ ] Bedakan error generation dari dependency/network environment failures.
- [ ] Quiet mode tidak menyembunyikan error penting.

### Langkah M — Smoke verification generated project

- [ ] Verifikasi file wajib sesuai template.
- [ ] Verifikasi `Cargo.toml` dan package name.
- [ ] Verifikasi entry point dan module declarations.
- [ ] Verifikasi `.rustrs/config.toml` dapat dibaca ulang.
- [ ] Verifikasi conditional assets cocok dengan capability final.
- [ ] Pastikan tidak ada placeholder tersisa (`{{`).
- [ ] Untuk Tier 1 yang supported, jalankan `cargo check` pada generated fixture/project di temporary directory bila environment memungkinkan.
- [ ] Isolasi `CARGO_TARGET_DIR` bila diperlukan.
- [ ] Jangan membutuhkan database, Docker, secret, atau layanan eksternal untuk test dasar.
- [ ] Bedakan build failure dari network/registry failure.
- [ ] Jangan melakukan dependency download sebagai efek samping tersembunyi tanpa kebijakan jelas.
- [ ] Hapus hanya file yang dibuat test sendiri.
- [ ] Smoke build di tests: **env-gated** (`RUSTRS_SMOKE_BUILD=1`).

### Langkah N — Automated tests

#### Unit tests

- [ ] Input resolver: flags, defaults, explicit/missing values.
- [ ] Nama project dan target path validation.
- [ ] Compatibility/capability preflight.
- [ ] Template selection dan context.
- [ ] Generation plan (termasuk manifest operation).
- [ ] Manifest creation/read-back.
- [ ] Next-step report dan error mapping.

#### CLI integration tests

- [ ] `rustrs --help` tetap bekerja.
- [ ] `rustrs new --help` mencerminkan contract.
- [ ] Invalid name dan incomplete non-interactive config gagal.
- [ ] Unsupported combinations ditolak.
- [ ] `--defaults` tidak meminta input.
- [ ] Global options dan exit codes benar.

#### Filesystem integration tests

- [ ] Happy path membuat project.
- [ ] File structure dan manifest benar.
- [ ] Dry-run tidak membuat/mengubah file atau directory.
- [ ] Existing target tidak ditimpa diam-diam.
- [ ] Render/validation failure terjadi sebelum write.
- [ ] Apply failure menghasilkan report parsial yang benar.
- [ ] Traversal/symlink escape tidak dapat menulis di luar target.
- [ ] Re-run mengikuti conflict policy.

#### Template/build tests

- [ ] Setiap template available memiliki expected file list/structure.
- [ ] Nama project tidak meninggalkan placeholder.
- [ ] Disabled capability tidak menghasilkan references rusak.
- [ ] Tidak ada secret atau remote fetch.
- [ ] Generated Tier 1 project lolos `cargo check` jika environment mendukung (env-gated).

Gunakan temporary directories. Tests harus memeriksa hasil/error, bukan hanya memastikan fungsi tidak panic. Jangan menonaktifkan tests atau melemahkan assertions demi membuat CI hijau.

### Langkah O — Dokumentasi, quality gates, dan final review

#### Dokumentasi

- [ ] README menjelaskan `rustrs new`, flags, defaults, supported templates, dry-run, conflicts, dan next steps yang benar-benar tersedia.
- [ ] ARCHITECTURE mendokumentasikan scaffolding orchestration serta keputusan baru yang berdampak.
- [ ] Roadmap Phase 4 checklist/status mencerminkan bukti.
- [ ] Jangan mengklaim generator Phase 5 atau fitur lain telah tersedia.

#### Quality gates

Jalankan dari root repository:

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

Jalankan juga generated-project smoke tests. Jika command gagal, laporkan hasil sebenarnya; jangan menyatakan `PASS`.

#### Final audit

- [ ] Review seluruh diff dan perubahan lokal.
- [ ] Pastikan dependency/lockfile changes disengaja.
- [ ] Tidak ada secrets di source, fixtures, manifest, atau docs.
- [ ] Tidak ada perubahan path di luar scope.
- [ ] Tidak ada false-success commands.
- [ ] Tidak ada Phase 5 generators.
- [ ] Tidak ada commit/push/publish/release tanpa izin.

---

## 4. File yang Akan Terpengaruh

### Dibuat

- `src/application/mod.rs` — modul application layer.
- `src/application/new_project.rs` — scaffolding service, input resolution, execute flow.
- `src/cli/commands.rs` — definisi subcommand `new` dan `NewArgs` (opsional; bisa di `src/cli/mod.rs`).
- `tests/scaffolding.rs` — integration tests scaffolding (atau perluasan tests yang ada).

### Diubah

- `src/cli/mod.rs` — tambah `Commands` enum dan field `command` pada `Cli`.
- `src/main.rs` — dispatch command ke application layer; tetap tipis.
- `src/lib.rs` — expose `application`.
- `src/error.rs` — tambah varian `Template(#[from] TemplateError)` dan `Conflict(String)`; exit code mapping (Template→2, Conflict→3).
- `src/manifest/model.rs` — derive `clap::ValueEnum` pada enums.
- `src/template/metadata.rs` — tambah 4 template constants; alias `BASIC_TEMPLATE` ke `MINIMAL_TEMPLATE`.
- `src/template/resolver.rs` — map semua `TemplateId` ke metadata.
- `src/generation/planner.rs` — append manifest operation; varian error serialisasi.
- `Cargo.toml` — tambah `dialoguer`.
- `Cargo.lock` — update otomatis oleh Cargo.
- `README.md` — status Phase 4 dan contoh `rustrs new`.
- `ARCHITECTURE.md` — catatan scaffolding orchestration (jika keputusan berdampak).

### Tidak diubah (out of scope)

- `PRD.md`, phase docs lain, generator Phase 5, inspector, database/dev commands, CI workflow (kecuali bila diperlukan).

---

## 5. Dependency / Configuration Impact

- Dependency baru:
  - `dialoguer = "0.11"` — wizard interaktif (sesuai PRD rekomendasi; hanya ditambah karena wizard memerlukannya).
- Tidak menambah async runtime ke CLI (workflow synchronous; dialoguer blocking).
- Tidak menambah remote template-fetching, plugin, atau workspace.
- `serde`, `toml`, `thiserror`, `clap`, `tera`, `tempfile` dipakai ulang.
- `Cargo.lock` diperbarui oleh Cargo (bukan edit manual).
- Tidak ada perubahan konfigurasi global; manifest schema tetap v1.
- Features default semua `false`; tidak menyimpan secret.
- Tidak ada perubahan MSRV/edition.

---

## 6. Risiko dan Potensi Konflik

- **Versi dependency** (`dialoguer`, `axum`, `tokio`, `sqlx`) harus diverifikasi saat build. Jika versi yang dipilih tidak tersedia, sesuaikan dan laporkan. Assumption: `dialoguer 0.11`, `axum 0.8`, `tokio 1`, `sqlx 0.8`.
- **Perubahan `build_plan`** (append manifest) memengaruhi tests Phase 3. `generated_plan_contains_rendered_content` tetap lulus (hanya menambah operasi); `planning_does_not_mutate_filesystem` tetap lulus.
- **Template assets besar** sebagai Rust string constants meningkatkan maintenance cost. Mitigasi: konten Cargo.toml/README dapat dibagikan, tetapi metadata tetap terpisah untuk identitas template.
- **Semua 4 template buildable** meningkatkan risiko compile failure. Mitigasi: smoke test env-gated + manual `cargo check`.
- **Symlink/escape** handling berbeda per platform; tests menggunakan tempdir dan menghindari asumsi permission.
- **Partial apply** tetap dilaporkan jujur (tidak klaim atomic transaction).
- **Scope creep**: jangan menambah generator Phase 5 atau capability penuh.
- **dialoguer API** bisa berubah antar versi; verifikasi saat implementasi.

---

## 7. Verification Plan

1. `cargo metadata --no-deps --format-version 1`
2. `cargo check`
3. `cargo test`
4. `cargo fmt --check`
5. `cargo clippy --all-targets --all-features -- -D warnings`
6. `cargo build`
7. `cargo build --release`
8. Smoke tests `--help`, `--version`, `new --help`, invalid args
9. `git diff --check`
10. `RUSTRS_SMOKE_BUILD=1 cargo test` (env-gated cargo check pada generated project)
11. Manual: generate satu project per template → `cargo check` di target (isolasi `CARGO_TARGET_DIR`)
12. Review `git status --short` dan `git diff`

Semua hasil dicatat PASS/FAIL/BLOCKED, bukan diasumsikan.

---

## 8. Acceptance Criteria yang Akan Terpenuhi

P4-AC-01 s/d P4-AC-22 sesuai `docs/roadmap/phase-04-project-scaffolding.md`, dengan bukti:

| ID       | Bukti minimum                     |
| -------- | --------------------------------- |
| P4-AC-01 | Laporan baseline                  |
| P4-AC-02 | CLI tests                         |
| P4-AC-03 | Unit tests                        |
| P4-AC-04 | Tests/manual verification         |
| P4-AC-05 | Unit/integration tests            |
| P4-AC-06 | Code review/tests                 |
| P4-AC-07 | Integration tests                 |
| P4-AC-08 | Template tests                    |
| P4-AC-09 | Template/context tests            |
| P4-AC-10 | Integration tests                 |
| P4-AC-11 | Before/after tests                |
| P4-AC-12 | Integration tests                 |
| P4-AC-13 | Round-trip tests                  |
| P4-AC-14 | CLI tests                         |
| P4-AC-15 | Generated-project tests           |
| P4-AC-16 | `cargo check` fixture (env-gated) |
| P4-AC-17 | Test suite                        |
| P4-AC-18 | Documentation review              |
| P4-AC-19 | Hasil command                     |
| P4-AC-20 | Scope review                      |
| P4-AC-21 | Git diff/status                   |
| P4-AC-22 | CLI review/tests                  |

---

## 9. Pertanyaan / Ambiguity

Keputusan pengguna sudah diperoleh via tool question:

- Template scope: **semua 4 template** buildable.
- Prompt library: **dialoguer**.
- Git init: **skip** Phase 4.
- Default features: **semua false**.
- Smoke build: **env-gated**.

Assumption yang masih perlu diverifikasi saat implementasi (bukan blocker plan):

- Versi `dialoguer`, `axum`, `tokio`, `sqlx` yang tersedia di registry. Jika tidak tersedia, pilih versi stabil terdekat dan laporkan.

Tidak ada ambiguity yang memblokir penyusunan plan.

---

## 10. Status dan Langkah Berikutnya

- Plan Phase 4 telah difinalisasi dengan keputusan yang tercatat di atas.
- **Belum ada kode yang diubah**; implementasi menunggu persetujuan pengguna.
- Setelah persetujuan, eksekusi Langkah A–O secara berurutan, lalu jalankan verification plan dan buat laporan Phase 4 sesuai format yang diminta `AGENTS.md`.
- Fase berikutnya (jika disetujui setelah Phase 4 selesai): **Phase 5 — Core Code Generators**.

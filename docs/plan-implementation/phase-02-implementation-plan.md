# Phase 2 — Manifest and Compatibility: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 2 of 10  
> **Status:** IMPLEMENTED — Phase 2 selesai (verification lulus)  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-02-manifest-and-compatibility.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

- Branch aktif: `main`, working tree bersih kecuali `docs/roadmap/phase-02-manifest-and-compatibility.md` untracked.
- Phase 0 dan Phase 1 sudah selesai dan di-merge ke `main`.
- Crate saat ini adalah library + binary dengan modul `cli`, `error`, dan `project`.
- `Cargo.toml` hanya memiliki `clap` dan `thiserror` sebagai dependency production.
- Belum ada modul manifest, compatibility, TOML parsing, schema validation, capability rules, atau manifest discovery.
- Toolchain: Rust stable 1.94.0, MSRV 1.85.0, edition 2024.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan | Pilihan |
|---|---|
| Field architecture | `CleanModular` saja |
| Support baseline | Hanya Axum + Postgres + SQLx |
| Nested `documentation`/`ci` | Tunda ke Phase 8 |
| Unknown fields | Tolak (`deny_unknown_fields`) |
| Validasi `project_name` | Kebab-case ASCII |
| Capability dependency | `rbac` memerlukan `auth` |
| Manifest I/O failure exit code | 1 |
| Metadata generator/template version | Tunda |

---

## 1. Kondisi Codebase Saat Ini

- Phase 0 dan Phase 1 selesai.
- Belum ada manifest model, parser, validator, compatibility resolver, capability rules, atau discovery.
- `ProjectRootError`, `CliError`, dan `Output` sudah tersedia sebagai fondasi Phase 1.
- Dependency yang akan ditambahkan: `serde` 1.0.229 (MSRV 1.56) dan `toml` 1.1.8 (MSRV 1.85).
- `AGENTS.md`, `ARCHITECTURE.md`, `PRD.md`, dan roadmap Phase 2 tersedia.

---

## 2. Task Phase 02 yang Akan Dikerjakan

1. **Task 2.0 — Preflight & baseline audit**
2. **Task 2.1 — Definisi model manifest**
3. **Task 2.2 — Parsing TOML**
4. **Task 2.3 — Schema versioning**
5. **Task 2.4 — Validasi manifest**
6. **Task 2.5 — Compatibility matrix dan resolver**
7. **Task 2.6 — Capability model dan validation**
8. **Task 2.7 — Manifest discovery**
9. **Task 2.8 — Error model dan integrasi CLI**
10. **Task 2.9 — Automated tests**
11. **Task 2.10 — Dokumentasi dan quality gates**

### Out of scope

- `rustrs new`, wizard, template rendering, template assets.
- Filesystem generation engine, safe writer, `--dry-run`, overwrite handling.
- Pembuatan/penulisan/pembaruan manifest oleh command.
- Implementasi nyata `config show/get/set`.
- Generator module/resource/controller/service/repository/dto/migration.
- Capability installation.
- Database commands, Docker orchestration, process management.
- Global user configuration.
- Template registry, plugin system, network fetching.

---

## 3. Implementation Plan

### Langkah A — Preflight dan baseline

- Inspeksi `git status --short`, branch, log, tree, toolchain.
- Jalankan baseline checks: `cargo metadata`, `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`.
- Catat hasil baseline sebelum perubahan.

### Langkah B — Tambah dependency

`Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
thiserror = "2"
serde = { version = "1", features = ["derive"] }
toml = "1"
```

- Jalankan `cargo check` setelah perubahan dependency.
- MSRV tetap 1.85.0.

### Langkah C — Struktur modul

```text
src/
├── manifest/
│   ├── mod.rs
│   ├── model.rs
│   ├── parser.rs
│   ├── validation.rs
│   ├── discovery.rs
│   └── error.rs
└── compatibility/
    ├── mod.rs
    ├── types.rs
    ├── matrix.rs
    ├── resolver.rs
    └── capabilities.rs
```

- Tambahkan `pub mod manifest;` dan `pub mod compatibility;` di `src/lib.rs`.
- Hindari modul kosong; setiap file memiliki tanggung jawab nyata.

### Langkah D — Model manifest

`src/manifest/model.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateId { Enterprise, Standard, Simple, Minimal }

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Framework { Axum, ActixWeb, Rocket }

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Database { Postgres, Mysql, Sqlite, None }

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataAccess { Sqlx, SeaOrm, Diesel, None }

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Architecture { CleanModular }

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Features {
    pub auth: bool,
    pub rbac: bool,
    pub openapi: bool,
    pub redis: bool,
    pub email: bool,
    pub storage: bool,
    pub audit_log: bool,
    pub rate_limit: bool,
    pub idempotency: bool,
    pub observability: bool,
    pub docker: bool,
    pub cicd: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Auth, Rbac, OpenApi, Redis, Email, Storage,
    AuditLog, RateLimit, Idempotency, Observability, Docker, Cicd,
}

impl Features {
    pub fn is_enabled(&self, capability: Capability) -> bool { ... }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectManifest {
    pub schema_version: u32,
    pub project_name: String,
    pub template: TemplateId,
    pub framework: Framework,
    pub architecture: Architecture,
    pub database: Database,
    pub data_access: DataAccess,
    #[serde(default)]
    pub features: Features,
}
```

Tidak menambah metadata generator/template version (ditunda).

### Langkah E — Parsing TOML

`src/manifest/parser.rs`:

```rust
pub fn parse_manifest(input: &str) -> Result<ProjectManifest, ManifestError>
```

- Gunakan `toml::from_str`.
- Map `toml::de::Error` menjadi `ManifestError::Parse { message }`.
- Parsing tidak melakukan I/O, validation, atau compatibility resolution.

### Langkah F — Schema versioning dan validasi

`src/manifest/error.rs`:

```rust
pub enum ManifestError {
    InvalidPath { path: PathBuf },
    NotFound { start: PathBuf },
    Io { path: PathBuf, source: std::io::Error },
    Parse { message: String },
    UnsupportedSchemaVersion { version: u32 },
    Validation { message: String },
    Capability(#[from] CapabilityError),
}
```

`src/manifest/validation.rs`:

- `SUPPORTED_SCHEMA_VERSION = 1`.
- `validate_schema_version(version) -> Result<(), ManifestError>`.
- `validate_project_name(name) -> Result<(), ManifestError>` dengan kebab-case ASCII.
- `ProjectManifest::validate(&self) -> Result<(), ManifestError>`.
- Validasi field wajib/non-empty dan hubungan dasar.
- Validasi capability melalui `Features::validate()`.
- Unknown fields ditolak oleh serde (`deny_unknown_fields`).

### Langkah G — Compatibility matrix dan resolver

`src/compatibility/types.rs`:

```rust
pub enum CompatibilityStatus {
    Supported,
    Unsupported { reason: String, alternatives: Vec<String> },
}

pub enum CompatibilityError {
    Unsupported { reason: String, alternatives: Vec<String> },
}
```

`src/compatibility/matrix.rs`:

```rust
pub fn evaluate(framework, database, data_access) -> CompatibilityStatus
```

- Supported hanya: `Axum + Postgres + Sqlx`.
- Semua kombinasi lain: `Unsupported` dengan reason jelas dan alternatif `["axum", "postgres", "sqlx"]`.

`src/compatibility/resolver.rs`:

```rust
pub fn compatibility_status(manifest) -> CompatibilityStatus
pub fn check_compatibility(manifest) -> Result<(), CompatibilityError>
```

- Tidak mengubah pilihan pengguna.

`src/compatibility/capabilities.rs`:

```rust
pub fn validate_capabilities(features) -> Result<(), CapabilityError>
```

- Dependency: `rbac = true` memerlukan `auth = true`.
- Tidak ada conflict rule tambahan.

### Langkah H — Manifest discovery

`src/manifest/discovery.rs`:

```rust
pub struct DiscoveredManifest {
    pub project_root: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: ProjectManifest,
}

pub fn discover_manifest(start: &Path) -> Result<DiscoveredManifest, ManifestError>
```

- Cari `.rustrs/config.toml` dari `start` ke parent directories.
- Baca file dengan `std::fs::read_to_string`.
- Parse manifest; jika invalid, kembalikan `ManifestError::Parse`.
- Tidak membuat/mengubah filesystem.
- Tidak menggunakan canonicalization.
- Manifest tidak ditemukan menghasilkan `ManifestError::NotFound`.
- File tidak dapat dibaca menghasilkan `ManifestError::Io`.

### Langkah I — Error integration

`src/error.rs`:

```rust
pub enum CliError {
    InvalidInput(String),               // exit code 2
    ProjectRoot(#[from] ProjectRootError), // exit code 2
    Manifest(#[from] ManifestError),       // 2 for config errors, 1 for I/O
    Compatibility(#[from] CompatibilityError), // exit code 2
}
```

- `src/main.rs` tetap thin; tidak menambah command.

### Langkah J — Tests

`tests/manifest.rs`:
- Parse manifest minimal dan lengkap yang valid.
- TOML malformed.
- Tipe field salah.
- Field wajib hilang.
- Unknown fields ditolak.
- Schema version supported/unsupported.
- Project name valid/invalid.
- Compatibility supported/unsupported.
- Capability dependency valid/invalid.

`tests/manifest_discovery.rs`:
- Discovery dari current directory.
- Discovery dari nested directory.
- Cargo project tanpa manifest tidak dianggap managed.
- Manifest tidak ditemukan.
- Parse invalid menghasilkan error.
- Tidak ada file yang dibuat.

Gunakan `tempfile` dan fixture TOML kecil.

### Langkah K — Dokumentasi

- `ARCHITECTURE.md`: catat manifest module, TOML parsing, schema version, compatibility matrix, capability rules, discovery, dan error integration.
- `README.md`: tidak mengklaim command baru; update hanya jika status kemampuan berubah.
- `docs/roadmap/phase-02-manifest-and-compatibility.md`: update status setelah implementasi.

### Langkah L — Verification

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

- `src/manifest/mod.rs`
- `src/manifest/model.rs`
- `src/manifest/parser.rs`
- `src/manifest/validation.rs`
- `src/manifest/discovery.rs`
- `src/manifest/error.rs`
- `src/compatibility/mod.rs`
- `src/compatibility/types.rs`
- `src/compatibility/matrix.rs`
- `src/compatibility/resolver.rs`
- `src/compatibility/capabilities.rs`
- `tests/manifest.rs`
- `tests/manifest_discovery.rs`

### Diubah

- `src/lib.rs`: expose `manifest` dan `compatibility`.
- `src/error.rs`: tambah `Manifest` dan `Compatibility` variant.
- `Cargo.toml`: tambah `serde`, `toml`.
- `Cargo.lock`: update otomatis.
- `ARCHITECTURE.md`: catatan keputusan Phase 2.
- `docs/roadmap/phase-02-manifest-and-compatibility.md`: status setelah implementasi.

### Tidak diubah (out of scope)

- `PRD.md`, template, generator, scaffolding, CI (kecuali bila diperlukan).

## 5. Dependency / Configuration Impact

- Dependency baru:
  - `serde = { version = "1", features = ["derive"] }`
  - `toml = "1"`
- `thiserror` dipakai ulang.
- Dev-dependencies existing (`tempfile`, `assert_cmd`, `predicates`) dipakai untuk tests.
- MSRV tetap 1.85.0; `toml` 1.1.8 membutuhkan Rust 1.85.
- Tidak menambah Tokio, Tera, dialoguer, walkdir, atau dependency generator.
- Tidak menambah konfigurasi global; discovery hanya read-only.
- `Cargo.lock` akan diperbarui oleh Cargo.

## 6. Risiko dan Potensi Konflik

- Modul `manifest` dan `compatibility` saling mereferensikan tipe; karena dalam satu crate, ini dapat dikelola, tetapi perlu menjaga dependency direction yang jelas.
- `deny_unknown_fields` dapat menolak manifest lama/future fields; ini disengaja dan terdokumentasi.
- Kombinasi `none/none` tidak supported pada Phase 2; ini konservatif dan jujur, tetapi perlu dijelaskan di dokumentasi.
- Capability dependency `rbac -> auth` adalah aturan minimal; jika produk memerlukan dependency tambahan, perlu disepakati nanti.
- `toml` error message mungkin verbose; pastikan tidak mencetak secret atau seluruh manifest.
- Discovery tanpa canonicalization dapat mengembalikan path relatif; tests harus menyesuaikan.
- `ManifestError::Io` dipetakan ke exit code 1, sedangkan error konfigurasi lain ke exit code 2.
- Risiko scope creep: jangan menambah command `config`, scaffolding, atau template engine.

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

- P2-AC-01: repository dan baseline diperiksa sebelum perubahan.
- P2-AC-02: manifest direpresentasikan dengan model Rust bertipe.
- P2-AC-03: manifest valid dapat diparse dari TOML.
- P2-AC-04: TOML malformed dan tipe field invalid menghasilkan error yang benar.
- P2-AC-05: unknown-field policy diterapkan dan terdokumentasi.
- P2-AC-06: schema version eksplisit dan versi unsupported ditolak.
- P2-AC-07: validasi terpisah dari parsing.
- P2-AC-08: compatibility matrix memiliki satu sumber kebenaran.
- P2-AC-09: kombinasi unsupported ditolak tanpa silent fallback.
- P2-AC-10: baseline Tier 1 dinyatakan eksplisit.
- P2-AC-11: capability rules divalidasi dan diuji.
- P2-AC-12: discovery membedakan Cargo project biasa dari project bermanifest.
- P2-AC-13: discovery dan resolver tidak menulis filesystem.
- P2-AC-14: error terintegrasi dengan kontrak CLI Phase 1.
- P2-AC-15: dokumentasi tidak mengklaim fitur yang belum ada.
- P2-AC-16: quality gates lulus atau blocker dilaporkan jujur.
- P2-AC-17: diff hanya berisi perubahan yang disengaja.
- P2-AC-18: tidak ada generator/template engine atau command placeholder.

## 9. Pertanyaan / Ambiguity

Semua keputusan kritis telah dikonfirmasi:

- Architecture field: `CleanModular` saja.
- Support baseline: hanya Axum + Postgres + SQLx.
- Nested `documentation`/`ci`: ditunda ke Phase 8.
- Unknown fields: tolak.
- Project name: kebab-case ASCII.
- Capability dependency: `rbac` memerlukan `auth`.
- Manifest I/O failure exit code: 1.
- Metadata generator/template version: ditunda.

Tidak ada pertanyaan tersisa yang memblokir implementasi.

## 10. Status dan Langkah Berikutnya

- Plan Phase 2 telah difinalisasi dengan keputusan yang tercatat di atas.
- **Belum ada kode yang diubah**; implementasi menunggu persetujuan pengguna.
- Setelah persetujuan, eksekusi Langkah A–L secara berurutan, lalu jalankan verification plan dan buat laporan Phase 2 sesuai format yang diminta `AGENTS.md`.
- Fase berikutnya (jika disetujui setelah Phase 2 selesai): **Phase 3 — Template and Filesystem Engine**.

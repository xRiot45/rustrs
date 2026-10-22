# Phase 10 — Implementation Plan: Unimplemented and Deferred Features

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/support-matrix.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase saat plan disusun

- Branch aktif: `main`.
- Working tree memiliki perubahan dokumentasi yang belum di-commit: `README.md`, `docs/troubleshooting.md`, dan `docs/roadmap/phase-10-unimplemented-features.md`.
- Phase 0–9 sudah diimplementasikan pada level MVP yang terdokumentasi.
- Baseline checks terakhir yang dijalankan pada perubahan dokumentasi lulus:
  - `cargo fmt --check`
  - `cargo check`
  - `cargo test`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `git diff --check`
- Toolchain terverifikasi: Rust 1.94.0, Cargo 1.94.0, MSRV repository 1.85.0, edition 2024.
- Kombinasi teknologi yang benar-benar didukung: `Axum + PostgreSQL + SQLx`.
- Capability yang baru dideklarasikan di manifest: `auth`, `rbac`, `openapi`, `redis`, `email`, `storage`, `audit_log`, `rate_limit`, `idempotency`, `observability`, `docker`, `cicd`.
- Generator yang eksplisit tidak tersedia: `guard`, `event`, `event-handler`, `job`, `validator`.
- Command yang belum ada: `config`, `add`, `env check`, `update check`.
- `GenerateArgs` belum memiliki flag non-interaktif.
- `rustrs new` belum menginisialisasi Git.
- `rustrs db seed` unsupported.
- Rollback migration membutuhkan down migration; generated migration default up-only.
- OpenAPI helper hanya validasi konfigurasi.
- Distribusi prebuilt binary belum tersedia.
- macOS dan Windows belum diverifikasi.

### Keputusan yang harus dikonfirmasi sebelum implementasi

| Keputusan                    | Opsi                                                         | Rekomendasi                                                               |
| ---------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------- |
| Urutan implementasi Phase 10 | Semua task sekaligus / wave bertahap                         | Wave bertahap, mulai Wave 1                                               |
| `env check`                  | Command terpisah / bagian dari `doctor`                      | Perluas `doctor`, hindari duplikasi kecuali user meminta command terpisah |
| `config set` scope           | Semua field manifest / hanya `features.*` dan `project_name` | Semua field valid dengan validasi dan warning, agar memenuhi PRD FR-007   |
| Git init                     | Opt-in flag `--git` / prompt interaktif default              | Opt-in flag `--git`, jangan default otomatis                              |
| `generate --no-interactive`  | Guard input / wizard nyata                                   | Guard input pada Wave 1; wizard nyata jika nanti diperlukan               |
| `update check`               | Implementasi sekarang / tunda                                | Tunda sampai release metadata stabil                                      |

### Out of scope untuk seluruh Phase 10

- Plugin system eksternal.
- Remote template registry/marketplace.
- Auto-upgrade struktur project lama.
- Deploy ke cloud provider.
- GUI.
- AI-generated business logic.
- Distributed project management.
- Automatic rollback terhadap semua perubahan source code.
- Dukungan seluruh kombinasi framework/database/ORM tanpa batas.
- Commit, push, tag, publish, atau deploy otomatis.

---

## 1. Kondisi Codebase Saat Ini

### 1.1 Struktur dan modul yang relevan

- `src/cli/mod.rs` dan `src/cli/commands.rs`: definisi command dan args.
- `src/main.rs`: routing command.
- `src/application/`: use case orchestration, termasuk `new_project.rs`, `generate.rs`, `inspect.rs`, `db.rs`, `dev.rs`, `quality.rs`, `docs.rs`, `assets.rs`, `completion.rs`.
- `src/manifest/`: model, parser, validation, discovery, error.
- `src/compatibility/`: matrix, resolver, capability rules.
- `src/generation/` dan `src/filesystem/`: planning, safe write, dry-run, conflict policy.
- `src/inspection/`: inspector dan diagnostics.
- `src/process/`: execution external process.
- `tests/`: integration tests CLI, manifest, generators, phase 8, db/dev, filesystem.

### 1.2 Fondasi yang dapat direuse

- Manifest discovery: `discover_manifest`.
- Manifest validation: `ProjectManifest::validate`.
- Compatibility validation: `check_compatibility`.
- Capability validation: `Features::validate`.
- Safe filesystem write: `GenerationPlan`, `PlannedOperation`, `apply_plan`, `WritePolicy`.
- Error mapping: `CliError` dan exit code.
- Output policy: `Output`.
- Process runner: `ProcessRunner`, `SystemProcessRunner`, `require_tool`.

### 1.3 Area yang belum ada

- `src/application/config.rs`
- `src/cli` variant `Config`
- Command `add`, `env check`, `update check`
- Flag non-interaktif pada `generate`
- Git init pada `new`
- Integrasi capability nyata
- Generator `guard`, `event`, `event-handler`, `job`, `validator`
- Seeder mechanism
- Down migration generator
- Framework/database/data access tambahan
- Cross-platform verification
- Binary distribution

---

## 2. Strategi Implementasi Phase 10

Phase 10 terlalu besar untuk satu perubahan. Implementasi harus dibagi menjadi wave:

| Wave   | Fokus                                   | Task yang dicakup              | Risiko        |
| ------ | --------------------------------------- | ------------------------------ | ------------- |
| Wave 1 | CLI dan config yang hilang              | 10.1.2, 10.1.3, 10.1.5, 10.1.6 | Rendah        |
| Wave 2 | Capability system dan `add`             | 10.1.1, 10.3                   | Sedang        |
| Wave 3 | Auth dan RBAC                           | 10.4, 10.8.1                   | Tinggi        |
| Wave 4 | OpenAPI runtime dan Redis               | 10.5, 10.6                     | Sedang        |
| Wave 5 | Capability lain                         | 10.7, 10.8.2–10.8.4            | Sedang/Tinggi |
| Wave 6 | Database workflow lengkap               | 10.9                           | Sedang        |
| Wave 7 | Framework/database/data access tambahan | 10.2                           | Tinggi        |
| Wave 8 | Distribusi dan platform                 | 10.11, 10.12                   | Sedang        |
| Wave 9 | Dokumentasi dan final verification      | 10.13, 10.14                   | Rendah        |

Plan ini memberikan detail penuh untuk Wave 1 dan rencana terstruktur untuk wave berikutnya. Setiap wave harus selesai, diverifikasi, dan dilaporkan sebelum wave berikutnya dimulai.

---

## 3. Wave 1 — Implementation Plan Detail

### 3.0 Preflight dan baseline audit

#### Work items

1. Jalankan `git status --short`.
2. Catat branch dan commit aktif.
3. Jalankan baseline checks:
   - `cargo metadata --no-deps --format-version 1`
   - `cargo check`
   - `cargo test`
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
4. Verifikasi command tree dari `rustrs --help`.
5. Verifikasi manifest discovery dan config serialization pada project temporary.

#### Acceptance criteria

- [ ] P10-W1-AC-01: Baseline checks dijalankan dan hasilnya dicatat.
- [ ] P10-W1-AC-02: Tidak ada perubahan unrelated yang tertimpa.

---

### 3.1 Task 10.1.2 — `rustrs config show/get/set`

#### 3.1.1 CLI definition

File yang diubah: `src/cli/commands.rs`, `src/cli/mod.rs`

Tambahkan:

```rust
#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,

    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Show(ConfigShowArgs),
    Get(ConfigGetArgs),
    Set(ConfigSetArgs),
}

#[derive(Args, Debug)]
pub struct ConfigShowArgs {
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct ConfigGetArgs {
    pub key: String,

    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct ConfigSetArgs {
    pub key: String,
    pub value: String,
}
```

Di `src/cli/mod.rs`:

- Export `ConfigArgs`, `ConfigCommands`, `ConfigShowArgs`, `ConfigGetArgs`, `ConfigSetArgs`.
- Tambah variant:

```rust
/// Inspect or update project configuration
Config(ConfigArgs),
```

#### 3.1.2 Application use case

File baru: `src/application/config.rs`  
File diubah: `src/application/mod.rs`, `src/main.rs`

Implementasikan:

```rust
pub fn execute(args: &ConfigArgs, output: &Output) -> Result<(), CliError>
```

Dispatch ke:

- `show(args, path, output)`
- `get(args, path, output)`
- `set(args, path, output)`

Gunakan:

- `discover_manifest`
- `ProjectManifest::validate`
- `check_compatibility`
- `toml::to_string_pretty`
- `serde_json::to_string_pretty` untuk `--json`
- `GenerationPlan` + `apply_plan(..., WritePolicy::Overwrite)`

#### 3.1.3 Key resolution design

Top-level keys:

| Key              | Tipe   | Validasi                |
| ---------------- | ------ | ----------------------- |
| `schema_version` | u32    | get only; set ditolak   |
| `project_name`   | string | `validate_project_name` |
| `template`       | enum   | parse ke `TemplateId`   |
| `framework`      | enum   | parse ke `Framework`    |
| `architecture`   | enum   | parse ke `Architecture` |
| `database`       | enum   | parse ke `Database`     |
| `data_access`    | enum   | parse ke `DataAccess`   |

Feature keys:

| Key                      | Tipe |
| ------------------------ | ---- |
| `features.auth`          | bool |
| `features.rbac`          | bool |
| `features.openapi`       | bool |
| `features.redis`         | bool |
| `features.email`         | bool |
| `features.storage`       | bool |
| `features.audit_log`     | bool |
| `features.rate_limit`    | bool |
| `features.idempotency`   | bool |
| `features.observability` | bool |
| `features.docker`        | bool |
| `features.cicd`          | bool |

Resolver harus:

1. Split key dengan `.`.
2. Reject unknown path.
3. Parse value sesuai tipe.
4. Update field yang sesuai.
5. Jalankan `manifest.validate()`.
6. Jalankan `check_compatibility(&manifest)`.
7. Baru serialize dan write.

#### 3.1.4 Output contract

- `config show`:
  - default: TOML canonik
  - `--json`: JSON object
- `config get <key>`:
  - default: nilai scalar
  - `--json`: JSON value
- `config set <key> <value>`:
  - success: `Updated .rustrs/config.toml: <key> = <value>`
  - warning: `Manifest updated; generated project files are not regenerated automatically.`
- Error:
  - invalid key: `invalid config key: ...`
  - invalid value: `invalid value for ...`
  - unsupported combination: compatibility error
  - manifest not found: manifest error
  - write failure: external/io error sesuai mapping

#### 3.1.5 Safe write

Gunakan:

```rust
let relative = PathBuf::from(".rustrs").join("config.toml");
let plan = GenerationPlan {
    target_root: discovered.project_root.clone(),
    operations: vec![PlannedOperation {
        relative_path: relative,
        contents: serialized_manifest.into_bytes(),
        description: "update .rustrs/config.toml".to_string(),
        overwrite: true,
    }],
};
let report = apply_plan(&plan, WritePolicy::Overwrite);
```

Jika ada operation failed, return error dan jangan claim sukses.

#### 3.1.6 Tests

File baru: `tests/config.rs`

Test cases:

1. `config --help` menampilkan subcommand.
2. `config show` pada project valid exits 0.
3. `config show --json` menghasilkan JSON valid.
4. `config get schema_version`.
5. `config get features.auth`.
6. `config set features.auth true` mengubah manifest.
7. `config set project_name` valid.
8. `config set` invalid key gagal dan manifest tetap sama.
9. `config set` invalid value gagal dan manifest tetap sama.
10. `config set` unsupported combination gagal dan manifest tetap sama.
11. `config` pada non-project directory gagal.
12. `config set` tidak mengubah file lain.

#### 3.1.7 Acceptance criteria

- [ ] P10-W1-AC-03: `config show/get/set` tersedia dan terdokumentasi.
- [ ] P10-W1-AC-04: Invalid key/value ditolak sebelum write.
- [ ] P10-W1-AC-05: Unsupported combination ditolak sebelum write.
- [ ] P10-W1-AC-06: Write menggunakan safe writer.
- [ ] P10-W1-AC-07: Manifest tetap valid setelah set.
- [ ] P10-W1-AC-08: Tidak ada secret dicetak.

---

### 3.2 Task 10.1.3 — `env check` vs `doctor`

#### 3.2.1 Audit yang harus dilakukan

Audit `src/inspection/doctor.rs` dan `src/inspection/environment.rs`.

Periksa:

- Status `PASS`, `WARN`, `FAIL`, `SKIP`.
- Missing optional tool.
- Missing required environment variable.
- Secret tidak dicetak.
- Invalid project root.
- JSON output.

#### 3.2.2 Opsi implementasi

Opsi A: tambah command `env check` yang memanggil use case yang sama dengan `doctor`.

Opsi B: tidak tambah command; perluas `doctor` dan dokumentasikan bahwa environment check termasuk di dalamnya.

Rekomendasi: Opsi B untuk MVP, karena menghindari duplikasi command dan tetap memenuhi FR-009. Jika user meminta command terpisah, gunakan Opsi A sebagai thin wrapper.

#### 3.2.3 Work items jika Opsi B dipilih

1. Tambah status env var required/optional.
2. Pastikan tool opsional diberi `SKIP`/`NOT CONFIGURED`.
3. Tambah tests.
4. Update README dan troubleshooting.

#### 3.2.4 Acceptance criteria

- [ ] P10-W1-AC-09: `doctor` memenuhi FR-009 atau `env check` tersedia sebagai command yang jelas.
- [ ] P10-W1-AC-10: Tidak ada secret dicetak.
- [ ] P10-W1-AC-11: Missing optional tool tidak menjadi error palsu.

---

### 3.3 Task 10.1.5 — `rustrs generate --no-interactive`

#### 3.3.1 CLI definition

File: `src/cli/commands.rs`

Tambahkan ke `GenerateArgs`:

```rust
#[arg(long)]
pub non_interactive: bool,
```

Atau nama `--no-interactive` sesuai PRD. Rekomendasi: gunakan `--no-interactive` agar sesuai PRD.

#### 3.3.2 Application behavior

File: `src/application/generate.rs`

Karena generator saat ini tidak melakukan prompt, flag ini awalnya berfungsi sebagai guard:

- Jika `--no-interactive` aktif dan generator membutuhkan `name` yang tidak diberikan, gagal dengan pesan yang membantu.
- Jangan menunggu input.
- Dokumentasikan bahwa ini guard, bukan wizard penuh.

#### 3.3.3 Tests

- Generate dengan name lengkap dan flag berhasil.
- Generate tanpa name dan flag gagal dengan pesan actionable.
- Asset generator tetap berjalan dengan flag.

#### 3.3.4 Acceptance criteria

- [ ] P10-W1-AC-12: Mode non-interaktif tidak menunggu input.
- [ ] P10-W1-AC-13: Error message actionable.
- [ ] P10-W1-AC-14: Tidak ada klaim wizard yang tidak ada.

---

### 3.4 Task 10.1.6 — Git init opsional pada `rustrs new`

#### 3.4.1 CLI definition

File: `src/cli/commands.rs`

Tambahkan ke `NewArgs`:

```rust
#[arg(long)]
pub git: bool,
```

Atau prompt interaktif. Rekomendasi awal: opt-in flag `--git`.

#### 3.4.2 Application behavior

File: `src/application/new_project.rs`

- Jika `--git` aktif:
  - Cek `git` dengan `require_tool`.
  - Jalankan `git init` menggunakan `std::process::Command` dengan argumen terpisah.
  - Jangan commit, push, tag, atau deploy.
  - Jika gagal, laporkan error tetapi jangan merusak project.
- Jika `--git` tidak aktif, jangan jalankan Git.
- Tambah warning jika Git tidak tersedia.

#### 3.4.3 Tests

- Jika Git tersedia, `--git` menghasilkan `.git`.
- Jika Git tidak tersedia, command gagal dengan pesan missing tool.
- Tanpa `--git`, tidak ada `.git` yang dibuat oleh CLI.

#### 3.4.4 Acceptance criteria

- [ ] P10-W1-AC-15: Git init hanya berjalan jika `--git` aktif.
- [ ] P10-W1-AC-16: Kegagalan Git tidak merusak project.
- [ ] P10-W1-AC-17: Tidak ada commit/push/tag otomatis.

---

### 3.5 Dokumentasi Wave 1

File yang diubah:

- `README.md`
- `docs/troubleshooting.md`
- `docs/support-matrix.md` jika status berubah

Dokumentasikan:

- `rustrs config show`
- `rustrs config get <key>`
- `rustrs config set <key> <value>`
- `--json`
- Key yang didukung
- Peringatan bahwa config set tidak meregenerasi project
- Status env check/doctor
- `rustrs generate --no-interactive`
- `rustrs new --git`

---

### 3.6 Quality gates Wave 1

Jalankan:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke test:

- `rustrs config --help`
- `rustrs config show`
- `rustrs config get features.auth`
- `rustrs config set features.auth true`
- `rustrs generate resource products --no-interactive`
- `rustrs new demo --defaults --git` jika Git tersedia

Failure test:

- invalid key
- invalid value
- unsupported combination
- non-project directory
- Git unavailable

---

## 4. Rencana Wave 2 dan Seterusnya

### Wave 2 — Capability system dan `rustrs add`

Scope:

- Task 10.1.1
- Task 10.3

Work items utama:

1. Finalisasi capability yang benar-benar akan diimplementasikan.
2. Tambah command `add`.
3. Implementasikan compatibility rules terpusat.
4. Hitung dependency/config/file changes per capability.
5. Preview dan approval.
6. Safe write dan update manifest.
7. Tests.

Risiko: capability integration dapat memicu perubahan template dan generator. Harus dimulai dengan satu capability sederhana, misalnya `docker` atau `cicd`, sebelum auth.

### Wave 3 — Auth dan RBAC

Scope:

- Task 10.4
- Task 10.8.1

Work items utama:

1. Pilih library auth yang kompatibel MSRV.
2. Implementasi JWT dan Argon2.
3. Middleware/guard.
4. RBAC.
5. Update generator `guard`.
6. Tests dan dokumentasi secret handling.

Risiko tinggi: dependency, security, dan generated code complexity.

### Wave 4 — OpenAPI runtime dan Redis

Scope:

- Task 10.5
- Task 10.6

Work items utama:

1. Pilih library OpenAPI.
2. Generate registration/schema.
3. Update helper `docs`.
4. Implement Redis client/cache/rate limit.
5. Update Compose.
6. Tests.

### Wave 5 — Capability lain dan generator stub

Scope:

- Task 10.7
- Task 10.8.2–10.8.4

Work items utama:

1. Definisikan scope minimal.
2. Implementasikan abstraksi realistis.
3. Update generator `event`, `event-handler`, `job`, `validator`.
4. Tests dan dokumentasi batasan.

### Wave 6 — Database workflow lengkap

Scope:

- Task 10.9

Work items utama:

1. Seeder mechanism atau hapus command jika unsupported.
2. Down migration generator.
3. Validasi rollback.
4. Tests.

### Wave 7 — Framework, database, dan data access tambahan

Scope:

- Task 10.2

Work items utama:

1. Actix Web dan Rocket.
2. MySQL dan SQLite.
3. SeaORM dan Diesel.
4. Database `none` dan data access `none`.
5. Tests generated project.

Risiko tinggi: kombinasi teknologi dan template complexity.

### Wave 8 — Distribusi dan platform

Scope:

- Task 10.11
- Task 10.12

Work items utama:

1. Target triple.
2. Release workflow build matrix.
3. Archive/checksum/metadata.
4. Smoke test binary.
5. Verifikasi macOS/Windows jika environment tersedia.

### Wave 9 — Dokumentasi dan final verification

Scope:

- Task 10.13
- Task 10.14

Work items utama:

1. Update README, support matrix, troubleshooting.
2. Hapus klaim stale.
3. Tambah tests regression.
4. Jalankan full quality gates.

---

## 5. File yang Akan Terpengaruh

### Wave 1

#### Dibuat

- `src/application/config.rs`
- `tests/config.rs`

#### Diubah

- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/main.rs`
- `src/application/mod.rs`
- `src/application/new_project.rs`
- `src/application/generate.rs`
- `src/inspection/doctor.rs` (jika Opsi B)
- `src/inspection/environment.rs` (jika Opsi B)
- `README.md`
- `docs/troubleshooting.md`
- `docs/support-matrix.md`

### Wave 2 ke atas

Akan ditentukan pada plan wave masing-masing. Tidak ada file yang diubah sekarang.

---

## 6. Dependency / Configuration Impact

- Tidak ada dependency baru direncanakan untuk Wave 1.
- `toml`, `serde`, dan `serde_json` sudah tersedia.
- Tidak ada perubahan `Cargo.toml` pada Wave 1.
- Tidak ada perubahan schema manifest pada Wave 1.
- `config set` menulis manifest dalam format canonik; komentar/formatting lama dapat hilang.
- `config set` tidak meregenerasi file source, routes, module registration, atau dependency.
- Wave 2 ke atas dapat menambah dependency baru setelah evaluasi MSRV dan lisensi.

---

## 7. Risiko dan Potensi Konflik

### Wave 1

- `config set` dapat membuat manifest tidak sinkron dengan project files. Mitigasi: validasi compatibility dan warning jelas.
- Serialization menulis ulang manifest. Mitigasi: dokumentasikan tradeoff.
- `env check` dapat duplikat dengan `doctor`. Mitigasi: audit `doctor` dan pilih satu surface.
- Git init menambah side effect eksternal. Mitigasi: opt-in, cek tool, jangan commit.
- `--no-interactive` pada generate bisa disalahartikan sebagai wizard. Mitigasi: dokumentasikan sebagai guard.

### Wave berikutnya

- Capability integration dapat memicu perubahan template dan generator.
- Auth/RBAC menambah risiko security.
- Framework/database baru meningkatkan matrix testing.
- Binary distribution membutuhkan release infrastructure yang aman.

Konflik dengan dokumen:

- `AGENTS.md`: tidak boleh menambah dependency spekulatif atau mengubah scope; plan ini reuse abstraksi yang ada.
- `ARCHITECTURE.md`: manifest tetap source of truth; config command hanya adapter.
- `PRD.md`: FR-007 meminta `config show/get/set`; Wave 1 memenuhi.

---

## 8. Verification Plan

### Wave 1

Jalankan setelah implementasi:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke test:

- `rustrs config --help`
- `rustrs config show`
- `rustrs config get features.auth`
- `rustrs config set features.auth true`
- `rustrs generate resource products --no-interactive`
- `rustrs new demo --defaults --git` jika Git tersedia

Failure test:

- invalid key
- invalid value
- unsupported combination
- non-project directory
- Git unavailable

### Wave 2 ke atas

Setiap wave harus menjalankan quality gates yang sama ditambah generated-project checks yang relevan.

---

## 9. Acceptance Criteria yang Akan Terpenuhi

### Wave 1

- P10-W1-AC-01: Baseline checks dijalankan dan dicatat.
- P10-W1-AC-02: Tidak ada perubahan unrelated yang tertimpa.
- P10-W1-AC-03: `config show/get/set` tersedia dan terdokumentasi.
- P10-W1-AC-04: Invalid key/value ditolak sebelum write.
- P10-W1-AC-05: Unsupported combination ditolak sebelum write.
- P10-W1-AC-06: Write menggunakan safe writer.
- P10-W1-AC-07: Manifest tetap valid setelah set.
- P10-W1-AC-08: Tidak ada secret dicetak.
- P10-W1-AC-09: `doctor` memenuhi FR-009 atau `env check` tersedia sebagai command yang jelas.
- P10-W1-AC-10: Tidak ada secret dicetak.
- P10-W1-AC-11: Missing optional tool tidak menjadi error palsu.
- P10-W1-AC-12: Mode non-interaktif tidak menunggu input.
- P10-W1-AC-13: Error message actionable.
- P10-W1-AC-14: Tidak ada klaim wizard yang tidak ada.
- P10-W1-AC-15: Git init hanya berjalan jika `--git` aktif.
- P10-W1-AC-16: Kegagalan Git tidak merusak project.
- P10-W1-AC-17: Tidak ada commit/push/tag otomatis.

### Wave 2 ke atas

Acceptance criteria mengikuti `docs/roadmap/phase-10-unimplemented-features.md` per task.

---

## 10. Ambiguity dan Keputusan yang Diperlukan

1. Apakah `env check` harus menjadi command terpisah, atau cukup diperluas dari `doctor`?
2. Apakah `config set` boleh mengubah `template`, `framework`, `database`, dan `data_access`, atau hanya `features.*` dan `project_name`?
3. Apakah Git init harus opt-in via flag `--git`, atau berupa prompt interaktif default dengan opsi skip?
4. Apakah `--no-interactive` pada `generate` cukup sebagai guard input, atau diperlukan wizard nyata di masa depan?
5. Apakah Wave 2 dimulai dari capability `docker`/`cicd` atau langsung capability yang lebih kompleks?

## 11. Catatan Eksekusi

- Plan ini adalah dokumen perencanaan, bukan implementasi.
- Tidak ada file code yang diubah oleh plan ini.
- Setiap wave harus dilaporkan dengan status `COMPLETE`, `PARTIAL`, atau `BLOCKED`.
- Jangan mulai wave berikutnya tanpa persetujuan.
- Jangan claim fitur selesai sebelum tests dan quality gates lulus.

# Phase 10 — Wave 2 Implementation Plan: Capability System and `rustrs add`

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 2 — Capability System and `rustrs add`  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase saat plan disusun

- Branch aktif: `feat/phase-10-unimplemented-features`.
- Wave 1 sudah diimplementasikan dan lolos quality gates.
- Command yang tersedia sekarang: `config`, `env check`, `generate --no-interactive`, `new --git`.
- `Capability` sudah ada di `src/manifest/model.rs`, tetapi belum digunakan sebagai CLI argument.
- `Features` sudah mendukung 12 capability.
- `validate_capabilities` baru memvalidasi dependency `rbac -> auth`.
- Asset generator untuk `docker`, `compose`, `ci-github`, `ci-gitlab`, dan `release` sudah tersedia.
- `assets::execute` langsung men-discover manifest, men-set feature, membuat file, dan menulis manifest.
- Capability `docker` dan `cicd` adalah satu-satunya capability dengan efek file nyata.
- Capability lain baru berupa field manifest dan belum memiliki integrasi code.
- Generator `guard`, `event`, `event-handler`, `job`, dan `validator` masih stub.

### Keputusan yang harus dikonfirmasi sebelum implementasi Wave 2

| Keputusan | Opsi | Rekomendasi |
|---|---|---|
| `add cicd` provider | Default GitHub / wajib `--provider` | Default GitHub dengan `--provider github|gitlab` |
| Efek `add docker` | Generate langsung / hanya toggle manifest | Generate langsung agar command punya efek nyata |
| Capability yang belum diimplementasikan | Tampil di help / sembunyikan | Tampil dengan pesan “not implemented yet” agar transparan |
| Metadata capability | Di `Capability` / modul compatibility terpisah | Di `Capability` sebagai method sederhana, konsisten dengan pola existing |
| Idempotency | Skip existing / overwrite default | Skip existing default, overwrite hanya dengan `--force` |

---

## 1. Kondisi Codebase Saat Ini

### 1.1 Modul yang relevan

- `src/manifest/model.rs`: `Features`, `Capability`, `ProjectManifest`.
- `src/compatibility/capabilities.rs`: validasi dependency capability.
- `src/application/assets.rs`: generator docker, compose, CI, release.
- `src/generation/plan.rs`: `GenerationPlan`, `PlannedOperation`, `WritePolicy`.
- `src/filesystem/writer.rs`: safe atomic write.
- `src/cli/commands.rs` dan `src/cli/mod.rs`: definisi command.
- `src/main.rs`: routing.
- `tests/`: integration tests.

### 1.2 Fondasi yang dapat direuse

- Manifest discovery dan validation.
- Compatibility check.
- Safe filesystem writer.
- Asset generation templates.
- Error mapping dan exit code.
- Output policy.
- Test pattern dengan `assert_cmd` dan `tempfile`.

### 1.3 Area yang belum ada

- Command `add`.
- CLI enum untuk capability.
- Application use case `add`.
- Reusable asset generation functions.
- Capability metadata.
- Tests untuk `add`.

---

## 2. Task Wave 2 yang Akan Dikerjakan

1. **Task W2.0 — Preflight dan audit**
2. **Task W2.1 — Jadikan `Capability` usable oleh CLI**
3. **Task W2.2 — Refactor asset generation agar reusable**
4. **Task W2.3 — Tambah command `add`**
5. **Task W2.4 — Implementasi application use case `add`**
6. **Task W2.5 — Capability registry dan metadata**
7. **Task W2.6 — Tests**
8. **Task W2.7 — Dokumentasi**
9. **Task W2.8 — Quality gates**

---

## 3. Implementation Plan

### Langkah A — Preflight dan baseline audit

1. Jalankan `git status --short`.
2. Jalankan baseline:
   - `cargo check`
   - `cargo test`
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
3. Audit:
   - `Capability`
   - `Features`
   - `validate_capabilities`
   - `assets::execute`
   - `GenerationPlan`
   - `apply_plan`
4. Catat behavior existing untuk docker dan cicd.

### Langkah B — Jadikan `Capability` usable oleh CLI

File: `src/manifest/model.rs`

Tambahkan:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum Capability {
    Auth,
    Rbac,
    OpenApi,
    Redis,
    Email,
    Storage,
    AuditLog,
    RateLimit,
    Idempotency,
    Observability,
    Docker,
    Cicd,
}
```

Catatan:

- `rename_all = "snake_case"` membuat nilai CLI konsisten dengan field manifest: `audit_log`, `rate_limit`, dll.
- Tidak menambah dependency baru karena `clap` sudah ada.

### Langkah C — Refactor asset generation

File: `src/application/assets.rs`

Tujuan: pisahkan generation logic dari discovery/write orchestration.

#### C.1 Fungsi reusable

Tambahkan:

```rust
pub fn generate_docker_assets(
    root: &std::path::Path,
    manifest: &mut ProjectManifest,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError>

pub fn generate_ci_assets(
    root: &std::path::Path,
    manifest: &mut ProjectManifest,
    provider: CiProvider,
    force: bool,
    dry_run: bool,
    output: &Output,
) -> Result<(), CliError>
```

Behavior:

- Set feature flag yang sesuai.
- Buat `PlannedOperation` untuk asset files.
- Validasi manifest dan compatibility.
- Serialize manifest.
- Tambah operation manifest.
- Preview jika dry-run.
- Apply plan dengan policy force/preserve.
- Report hasil.

#### C.2 `assets::execute`

Refactor agar tetap menjadi entry point `generate`:

- `Docker` -> panggil `generate_docker_assets`.
- `Compose` -> tetap generate compose, set docker.
- `CiGithub` -> panggil `generate_ci_assets(provider = GitHub)`.
- `CiGitlab` -> panggil `generate_ci_assets(provider = GitLab)`.
- `Release` -> tetap generate release workflow, set cicd.

Tujuan: `add` dan `generate` memakai logic yang sama.

### Langkah D — Tambah command `add`

File: `src/cli/commands.rs`

Tambahkan:

```rust
#[derive(Args, Debug)]
pub struct AddArgs {
    pub capability: Capability,

    #[arg(long, value_enum, default_value = "github")]
    pub provider: CiProvider,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(long)]
    pub force: bool,

    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CiProvider {
    Github,
    Gitlab,
}
```

File: `src/cli/mod.rs`

- Export `AddArgs` dan `CiProvider`.
- Tambah variant:

```rust
/// Add a capability to an existing project
Add(AddArgs),
```

File: `src/main.rs`

- Route `Commands::Add` ke `application::add::execute`.

### Langkah E — Implementasi application use case

File baru: `src/application/add.rs`  
File diubah: `src/application/mod.rs`

Implementasikan:

```rust
pub fn execute(args: &AddArgs, output: &Output) -> Result<(), CliError>
```

Alur:

1. Resolve project root.
2. Discover manifest.
3. Validasi manifest dan compatibility.
4. Jika capability belum diimplementasikan:
   - Return `CliError::InvalidInput` dengan pesan jelas.
5. Jika capability sudah enabled:
   - Untuk docker/cicd, tetap jalankan generator dengan `PreserveExisting`.
   - Untuk capability lain, laporkan sudah enabled.
6. Match capability:
   - `Capability::Docker` -> `generate_docker_assets`
   - `Capability::Cicd` -> `generate_ci_assets`
   - Lainnya -> unsupported/not implemented
7. Jika dry-run:
   - Preview plan.
   - Jangan write.
8. Apply plan.
9. Report hasil dan next steps.

Aturan:

- Tidak menulis di luar project root.
- Tidak mengaktifkan capability yang belum diimplementasikan.
- Tidak mengubah source application files yang tidak terkait.
- Idempotent untuk docker/cicd.

### Langkah F — Capability registry dan metadata

File: `src/manifest/model.rs` atau `src/compatibility/capabilities.rs`

Tambahkan method sederhana:

```rust
impl Capability {
    pub fn is_implemented(&self) -> bool {
        matches!(self, Self::Docker | Self::Cicd)
    }

    pub fn effect_summary(&self) -> &'static str {
        match self {
            Self::Docker => "generates Dockerfile and .dockerignore",
            Self::Cicd => "generates CI workflow files",
            _ => "not implemented in the current MVP",
        }
    }
}
```

Tujuan: centralize capability status, bukan menyebar di command handler.

### Langkah G — Tests

File baru: `tests/add.rs`

Test cases:

1. `rustrs add --help` menampilkan capability list.
2. `rustrs add docker` pada project valid:
   - exit 0
   - manifest `docker = true`
   - `Dockerfile` dan `.dockerignore` dibuat
3. `rustrs add docker` kedua kali:
   - exit 0
   - file existing di-skip, tidak overwrite
4. `rustrs add docker --dry-run`:
   - tidak mengubah filesystem
   - manifest tetap false jika sebelumnya false
5. `rustrs add docker --force`:
   - overwrite file yang sudah ada
6. `rustrs add cicd --provider github`:
   - manifest `cicd = true`
   - `.github/workflows/ci.yml` dibuat
7. `rustrs add cicd --provider gitlab`:
   - `.gitlab-ci.yml` dibuat
8. `rustrs add auth`:
   - exit 2
   - manifest tidak berubah
9. `rustrs add docker` di luar project:
   - gagal
10. `rustrs add docker` dengan manifest invalid:
    - gagal sebelum write

### Langkah H — Dokumentasi

File yang diubah:

- `README.md`
- `docs/troubleshooting.md`
- `docs/support-matrix.md`

Dokumentasikan:

- Capability yang benar-benar didukung oleh `add`.
- Capability yang ditolak dengan jelas.
- Contoh `rustrs add docker`.
- Contoh `rustrs add cicd --provider github`.
- Dry-run dan force behavior.
- Peringatan bahwa `add` tidak mengimplementasikan capability berat pada Wave 2.

### Langkah I — Quality gates

Jalankan:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

---

## 4. File yang Akan Terpengaruh

### Dibuat

- `src/application/add.rs`
- `tests/add.rs`

### Diubah

- `src/manifest/model.rs`
- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/main.rs`
- `src/application/mod.rs`
- `src/application/assets.rs`
- `README.md`
- `docs/troubleshooting.md`
- `docs/support-matrix.md`

### Tidak diubah tanpa alasan

- Template assets.
- Generator individual.
- Database workflow.
- Core compatibility matrix, kecuali metadata capability.

---

## 5. Dependency / Configuration Impact

- Tidak ada dependency baru.
- `clap` sudah tersedia dan digunakan.
- Tidak ada perubahan schema manifest.
- `features.docker` dan `features.cicd` tetap field yang ada.
- `add` reuse safe writer dan generation plan.
- Provider CI disimpan sebagai argumen command, bukan field manifest baru.

---

## 6. Risiko dan Potensi Konflik

1. **Fake success risk**: Jika `add` mengaktifkan `auth` atau `redis` tanpa integrasi nyata, produk akan menyesatkan pengguna. Mitigasi: tolak capability yang belum diimplementasikan.
2. **Duplikasi logic**: `add docker` dan `generate docker` dapat menduplikasi code. Mitigasi: refactor assets menjadi fungsi reusable.
3. **Idempotency**: `add` dijalankan dua kali dapat overwrite file. Mitigasi: default `PreserveExisting`, force opt-in.
4. **Provider ambiguity**: `cicd` tidak menyatakan GitHub/GitLab. Mitigasi: tambah `--provider` dengan default GitHub dan dokumentasikan.
5. **Capability dependency**: `rbac` membutuhkan `auth`, tetapi `auth` belum diimplementasikan. Mitigasi: tolak `rbac` pada Wave 2.
6. **Konflik dengan AGENTS.md**: tidak boleh menambah dependency spekulatif atau memperluas scope. Plan ini reuse modul yang ada.
7. **Konflik dengan ARCHITECTURE.md**: capability decision tetap terpusat di capability layer, bukan di command handler.
8. **Konflik dengan PRD.md**: PRD menyebut `add` untuk capability yang benar-benar diimplementasikan. Wave 2 hanya mengaktifkan capability dengan efek nyata.

---

## 7. Verification Plan

Setelah implementasi Wave 2:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs add --help`
- `rustrs add docker` pada project minimal
- `rustrs add docker` kedua kali
- `rustrs add docker --dry-run`
- `rustrs add cicd --provider github`
- `rustrs add cicd --provider gitlab`
- `rustrs add auth` harus gagal dengan pesan jelas

Failure tests:

- non-project directory
- invalid manifest
- unsupported capability
- docker asset conflict tanpa `--force`

---

## 8. Acceptance Criteria yang Akan Terpenuhi

- P10-W2-AC-01: `rustrs add` tersedia dan terdokumentasi.
- P10-W2-AC-02: Capability status terpusat di satu tempat.
- P10-W2-AC-03: `docker` dan `cicd` dapat ditambahkan dengan efek file nyata.
- P10-W2-AC-04: Capability yang belum diimplementasikan ditolak dengan jelas.
- P10-W2-AC-05: Dry-run tidak mengubah filesystem.
- P10-W2-AC-06: Force behavior terdokumentasi dan teruji.
- P10-W2-AC-07: Manifest tetap valid setelah `add`.
- P10-W2-AC-08: Tidak ada capability yang diaktifkan diam-diam tanpa efek.
- P10-W2-AC-09: Tests mencakup success, conflict, dry-run, dan failure.

---

## 9. Pertanyaan / Ambiguity

1. Apakah `rustrs add cicd` harus default GitHub, atau wajib `--provider github|gitlab`?
2. Apakah `rustrs add docker` harus generate file langsung, atau cukup toggle manifest dan menyuruh user menjalankan `rustrs generate docker`?
3. Apakah capability yang belum diimplementasikan harus muncul di `--help` dengan status “not implemented”, atau disembunyikan sampai benar-benar tersedia?
4. Apakah Wave 2 boleh menambah capability metadata ke `Capability`, atau metadata harus berada di modul compatibility terpisah?

---

## 10. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 2 harus dimulai hanya setelah keputusan ambiguity di atas dikonfirmasi.

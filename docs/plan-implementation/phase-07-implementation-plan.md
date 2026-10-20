# Phase 7 — Database and Development Workflow: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 7 of 10  
> **Status:** PLANNED — belum ada kode yang diubah; menunggu persetujuan implementasi  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-07-database-and-development-workflow.md`  
> **Dokumen ini:** `docs/plan-implementation/phase-07-implementation-plan.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

- Phase 0–6 sudah selesai; codebase mendukung `new`, `generate`/`g`, `info`, `modules`, `routes`, `deps`, dan `doctor`.
- CLI saat ini belum memiliki `db`, `dev`, `config`, atau modul proses.
- Dependency production: `clap`, `thiserror`, `serde`, `toml`, `tera`, `tempfile`, `dialoguer`, `heck`, `serde_json`.
- Support matrix nyata masih hanya `Axum + Postgres + SQLx`.
- Generated project memakai `sqlx = { features = ["runtime-tokio-rustls", "postgres"] }`, belum ada fitur `migrate`, tidak ada `.env`, dan tidak ada direktori `migrations` awal.
- Migration generator yang ada membuat `migrations/{unix_timestamp}_{name}.sql`.
- `sqlx-cli 0.8.6` dan Docker Compose tersedia di environment ini, tetapi tidak boleh diasumsikan tersedia di mesin pengguna.
- `git status --short` menunjukkan hanya `docs/roadmap/phase-07-database-and-development-workflow.md` sebagai untracked; tidak ada perubahan tracked lain.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan | Pilihan |
| --- | --- |
| `dev up/down/logs` | **Implementasikan penuh pada Phase 7** |
| Dependency `notify` dan `ctrlc` | **Ya, tambahkan** |
| `db create-migration` | **Reuse generator rustrs** |

---

## 1. Kondisi Codebase Saat Ini

- Phase 0–6 terimplementasi dan baseline quality gate lulus; `cargo test` terakhir lulus.
- CLI yang ada: `new`, `generate`/`g`, `info`, `modules`, `routes`, `deps`, `doctor`.
- Belum ada `db`, `dev`, `config`, dan modul proses.
- Support matrix nyata masih hanya `Axum + Postgres + SQLx`.
- Generated project memakai `sqlx = { features = ["runtime-tokio-rustls", "postgres"] }`, belum ada fitur `migrate`, tidak ada `.env`, dan tidak ada direktori `migrations` awal.
- Migration generator yang ada membuat `migrations/{unix_timestamp}_{name}.sql`.
- `sqlx-cli 0.8.6` dan Docker Compose tersedia di environment ini, tetapi tidak boleh diasumsikan tersedia di mesin pengguna.
- `git status --short` menunjukkan hanya `docs/roadmap/phase-07-database-and-development-workflow.md` sebagai untracked; tidak ada perubahan tracked lain.

---

## 2. Task Phase 07 yang Akan Dikerjakan

1. Preflight audit Phase 0–6 dan verifikasi support matrix aktual.
2. Kontrak command `db` dan `dev`, termasuk error dan exit code.
3. Modul proses bersama: spawn, argumen terpisah, working directory, env, output forwarding, exit status, cancellation, cleanup.
4. Tool discovery dan prerequisite validation untuk `sqlx`, `cargo`, `docker`, dan Compose.
5. `db create-migration` dengan reuse generator rustrs.
6. `db migrate` melalui `sqlx migrate run`.
7. `db rollback` melalui `sqlx migrate revert`.
8. `db status` melalui `sqlx migrate info`.
9. `db seed`: implementasi hanya jika mekanisme seeder nyata tersedia; jika tidak, gagal jelas sebagai unsupported.
10. `rustrs dev` menjalankan aplikasi generated project.
11. `rustrs dev --watch` dengan watcher dan restart aman.
12. `rustrs dev up/down/logs` dengan lifecycle terkelola penuh.
13. Deteksi Docker Compose tanpa side effect container.
14. Validasi konfigurasi dan redaksi secret.
15. Tests, dokumentasi, dan quality gates.

### Out of scope

- Framework, database, atau ORM baru.
- Dockerfile atau Docker Compose generation.
- Quality command suite (`check`, `lint`, `test`, `format`, `audit`).
- Release, packaging, deployment, dan update check.
- Refactor tidak terkait atau abstraksi spekulatif.

---

## 3. Implementation Plan

### Langkah A — Preflight audit

- Verifikasi command Phase 6 yang benar-benar jalan.
- Verifikasi layout scaffolded untuk empat template.
- Verifikasi bahwa migration generator dan SQLx CLI dapat bekerja pada project generated.
- Catat blocker sebelum perubahan arsitektur.

### Langkah B — CLI contract

Tambahkan ke `src/cli/commands.rs` dan `src/cli/mod.rs`:

- `DbArgs { #[command(subcommand)] command: DbCommands }`
- `DbCommands`: `CreateMigration`, `Migrate`, `Rollback`, `Status`, `Seed`
- `DevArgs { watch: bool, path: Option<PathBuf>, #[command(subcommand)] command: Option<DevCommands> }`
- `DevCommands`: `Up`, `Down`, `Logs`

Tambahkan varian `Commands::Db(DbArgs)` dan `Commands::Dev(DevArgs)`.

### Langkah C — Error dan exit code

Tambahkan ke `src/error.rs`:

- `MissingTool(String)` → exit code 4
- `ExternalCommand(String)` → exit code 5
- `Cancelled` → exit code 1

### Langkah D — Modul proses

Buat:

- `src/process/mod.rs`
- `src/process/spec.rs`
- `src/process/runner.rs`
- `src/process/tools.rs`
- `src/process/compose.rs`

Isi:

- `ProcessSpec` dengan program, args, cwd, env, dan stdin policy.
- Trait `ProcessRunner` dengan `run` dan `spawn`.
- `SystemProcessRunner` memakai `std::process::Command` dengan argumen terpisah.
- Fake runner untuk tests.
- `find_executable`, `check_tool`, dan `tool_version`.
- `ComposeDetection` yang membedakan file Compose, Docker CLI, dan Compose executable.

### Langkah E — Application layer

Buat:

- `src/application/db.rs`
- `src/application/dev.rs`

Daftarkan di `src/application/mod.rs`.

`db.rs`:

- Validasi project root, manifest, compatibility, dan database capability.
- `create-migration`: validasi nama, path traversal, konflik; reuse `generator::migration::plan` dan filesystem engine.
- `migrate`: jalankan `sqlx migrate run` di project root; forward output; map status.
- `rollback`: jalankan `sqlx migrate revert`; tampilkan scope.
- `status`: jalankan `sqlx migrate info`; bedakan file lokal dan state DB.
- `seed`: periksa mekanisme seeder nyata; jika tidak ada, kembalikan unsupported error.

`dev.rs`:

- Validasi `Cargo.toml` dan package.
- `dev`: jalankan `cargo run` di project root, forward output, handle Ctrl-C dengan `ctrlc`, cleanup child.
- `dev --watch`: gunakan `notify`, ignore `target/` dan `.git`, debounce event, restart proses, laporkan compile/startup failure.
- `dev up`: spawn proses terkelola, tulis state minimal di `.rustrs/dev/`, konfirmasi running.
- `dev down`: baca state, verifikasi, hentikan hanya proses milik project.
- `dev logs`: baca log file yang benar.

### Langkah F — Lifecycle state

Gunakan `.rustrs/dev/`:

- `pid`
- `log path`
- `project root marker`

Aturan:

- Tidak menyimpan secret.
- Tangani stale state.
- Cegah penghentian proses tidak terkait.
- Graceful termination sebelum forced termination.

### Langkah G — Docker Compose detection

- Deteksi file `docker-compose.yml`, `docker-compose.yaml`, `compose.yml`, `compose.yaml`.
- Periksa `docker` dan `docker compose`.
- Tidak start/stop container.
- Tidak menyimpulkan database container berjalan.

### Langkah H — Secret safety

- Jangan meneruskan credential lewat argumen CLI.
- Andalkan `DATABASE_URL` environment atau `.env`.
- Redaksi URL berpassword pada output/error sebisa mungkin.
- Jangan simpan secret di state atau log.

### Langkah I — Tests

Buat:

- `tests/process.rs`
- `tests/db.rs`
- `tests/dev.rs`
- `tests/compose.rs`

Cakupan:

- Process runner success/failure, args dengan spasi/metacharacters, cwd, output, cancellation, cleanup.
- `db` help dan invalid project.
- Migration name/path/conflict.
- Missing tool exit code 4.
- Compose detection present/absent tanpa side effect.
- Watch ignore `target/`.
- Lifecycle stale state dan unrelated process protection.

### Langkah J — Dokumentasi

- Update help text, README, dan docs command.
- Dokumentasikan prerequisite SQLx/Docker.
- Dokumentasikan rollback semantics dan limitation.
- Dokumentasikan unsupported seeder bila tidak tersedia.

### Langkah K — Quality gates

Jalankan:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

---

## 4. File yang Akan Terpengaruh

### Dibuat

- `src/process/mod.rs`
- `src/process/spec.rs`
- `src/process/runner.rs`
- `src/process/tools.rs`
- `src/process/compose.rs`
- `src/application/db.rs`
- `src/application/dev.rs`
- `tests/process.rs`
- `tests/db.rs`
- `tests/dev.rs`
- `tests/compose.rs`

### Diubah

- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/main.rs`
- `src/application/mod.rs`
- `src/error.rs`
- `Cargo.toml`
- Dokumentasi yang relevan

---

## 5. Dependency / Configuration Impact

- Tambah `notify` untuk watch mode.
- Tambah `ctrlc` untuk penanganan Ctrl-C lintas platform.
- Justifikasi: watch mode dan cleanup child process membutuhkan watcher dan signal handling yang andal.
- Tidak mengubah `Cargo.toml` project generated.
- `db migrate` bergantung pada `DATABASE_URL` atau `.env`; tidak ada perubahan schema manifest.

---

## 6. Risiko dan Potensi Konflik

- Support matrix tetap hanya `Axum + Postgres + SQLx`; kombinasi lain harus ditolak sebelum side effect.
- `sqlx` atau Docker mungkin tidak terpasang di mesin pengguna; harus menghasilkan exit code 4, bukan panic.
- `db migrate`, `rollback`, dan `status` membutuhkan database yang dapat dijangkau; tests unit tidak boleh bergantung pada DB nyata.
- Lifecycle terkelola penuh memiliki risiko menghentikan proses tidak terkait; state dan verifikasi harus ketat.
- Watch mode dapat loop jika `target/` tidak diabaikan.
- Secret dapat bocor melalui output SQLx; redaksi perlu diimplementasikan sebisa mungkin.
- `db seed` kemungkinan unsupported karena belum ada mekanisme seeder; harus dilaporkan jujur.

---

## 7. Verification Plan

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`
- Test process runner dengan executable palsu.
- Test CLI help untuk `db` dan `dev`.
- Test Compose detection tanpa side effect.
- Integration DB hanya jika `DATABASE_URL` tersedia; jika tidak, laporkan `NOT RUN`.

---

## 8. Acceptance Criteria yang Akan Terpenuhi

- P7-AC-04 sampai P7-AC-13: kontrak command, proses, tool discovery.
- P7-AC-14 sampai P7-AC-19: create migration dan migrate.
- P7-AC-20 sampai P7-AC-25: rollback dan status.
- P7-AC-26 sampai P7-AC-28: seed jika mekanisme nyata tersedia; jika tidak, unsupported jelas.
- P7-AC-29 sampai P7-AC-41: dev foreground, watch, lifecycle, Compose.
- P7-AC-42 sampai P7-AC-45: secret safety.
- P7-AC-46 sampai P7-AC-53: tests, dokumentasi, quality gates.

---

## 9. Pertanyaan / Ambiguity

Semua keputusan utama sudah dikonfirmasi:

- `dev up/down/logs` diimplementasikan penuh pada Phase 7.
- Dependency `notify` dan `ctrlc` disetujui.
- `db create-migration` menggunakan reuse generator rustrs.

Sisa detail implementasi, seperti struktur state file dan format redaksi secret, akan ditetapkan saat coding tanpa mengubah scope Phase 7.

---

## 10. Gunakan tool question untuk pertanyaan yang akan anda berikan kepada saya

Tidak ada pertanyaan tambahan; plan siap untuk persetujuan implementasi.

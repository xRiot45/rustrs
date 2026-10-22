# Phase 10 — Wave 6 Implementation Plan: Database Seed dan Down Migration

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 6 — Database Seed dan Down Migration  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Keputusan yang Sudah Dikonfirmasi

| Keputusan | Pilihan |
|---|---|
| `db seed` | Implementasikan Rust seeder modules |
| Down migration | Paired files |
| Data seed | Rust module mengeksekusi SQL |
| Down migration content | Reverse otomatis dari up migration |
| Revert execution | Baca file down SQL dan eksekusi via sqlx |
| Transaction | Ya, jika didukung |
| Seed safety | Konfirmasi eksplisit |
| Rollback target | Revert migrasi terbaru |
| Rollback dry-run | Ya |
| Seed dry-run | Ya |
| Seed auto-run | Tidak otomatis |

Catatan: reverse otomatis dari up migration adalah pilihan berisiko dan harus dibatasi pada DDL sederhana; kasus kompleks memerlukan review manual.

---

## 1. Tujuan Wave 6

Mengimplementasikan workflow database yang lebih lengkap:

- `rustrs db seed` dengan Rust seeder modules.
- Paired up/down migration files.
- Reverse otomatis untuk DDL sederhana.
- Custom SQL revert dengan konfirmasi eksplisit.
- Dry-run untuk seed dan rollback.
- Generated project tetap buildable dan aman.

---

## 2. Scope

### Dalam scope

- `db seed`.
- Paired migration files.
- Reverse otomatis untuk DDL sederhana.
- Custom SQL revert.
- Konfirmasi eksplisit.
- Dry-run seed dan rollback.
- Rust seeder modules.
- Unit dan smoke tests.
- Dokumentasi.

### Di luar scope

- Full SQL parser/reverser untuk semua DDL.
- Distributed transaction.
- Multi-database transaction.
- Automatic seed setelah migrate.
- Cloud database provisioning.
- Rollback destruktif tanpa konfirmasi.

---

## 3. Kondisi Codebase Saat Ini

- `db create-migration` menghasilkan migration up-only.
- `db migrate` menjalankan `sqlx migrate run`.
- `db rollback` menjalankan `sqlx migrate revert`.
- `db status` menjalankan `sqlx migrate info`.
- `db seed` unsupported.
- Migration generator menggunakan timestamp dan slug.
- Database workflow hanya mendukung `Axum + Postgres + SQLx`.
- Manifest dan safe writer sudah tersedia.

---

## 4. Task Wave 6 yang Akan Dikerjakan

### Task W6.0 — Preflight dan audit

1. Audit `db` command.
2. Audit migration generator.
3. Audit sqlx integration.
4. Audit template `standard` dan `enterprise`.
5. Audit dependency candidate untuk seeder.
6. Verifikasi MSRV dan license.
7. Catat file yang akan berubah.

### Task W6.1 — Audit database workflow

1. Audit `create-migration`.
2. Audit `migrate`.
3. Audit `rollback`.
4. Audit `status`.
5. Audit `seed`.
6. Catat gap dan limitation.

### Task W6.2 — Paired migration files

1. Tambah opsi generate down migration.
2. Generate paired up/down files.
3. Pastikan nama file unik dan aman.
4. Pastikan up/down migration konsisten.
5. Tambah tests.

### Task W6.3 — Reverse otomatis

1. Implementasikan reverse generator untuk DDL sederhana.
2. Tangani `CREATE TABLE` -> `DROP TABLE`.
3. Tangani `ADD COLUMN` -> `DROP COLUMN`.
4. Tangani `CREATE INDEX` -> `DROP INDEX`.
5. Tangani `ADD CONSTRAINT` -> `DROP CONSTRAINT`.
6. Tandai kasus kompleks sebagai manual review.
7. Tambah tests.

### Task W6.4 — Migration pairing dan naming

1. Tentukan konvensi nama file.
2. Tentukan timestamp dan slug.
3. Pastikan tidak ada collision.
4. Dokumentasikan paired migration.

### Task W6.5 — Custom SQL revert

1. Implementasikan custom SQL revert.
2. Validasi down migration sebelum revert.
3. Jalankan revert via sqlx.
4. Tambahkan konfirmasi eksplisit.
5. Tambah dry-run.
6. Tambah tests.

### Task W6.6 — Rust seeder modules

1. Tambah generator seeder.
2. Generate Rust seeder module.
3. Generate registration.
4. Tambah template wiring.
5. Tambah tests.

### Task W6.7 — Seeder execution

1. Implementasikan seeder runner.
2. Jalankan seeder secara eksplisit.
3. Tangani error dan partial failure.
4. Tambah logging yang aman.
5. Tambah dry-run.
6. Tambah tests.

### Task W6.8 — Manifest dan config

1. Pastikan manifest tetap source of truth.
2. Tambah config jika diperlukan.
3. Validasi compatibility.
4. Tambah tests.

### Task W6.9 — Safety dan confirmation

1. Tambah konfirmasi untuk rollback.
2. Tambah konfirmasi untuk seed jika tabel tidak kosong.
3. Pastikan tidak ada destructive action tanpa consent.
4. Tambah tests.

### Task W6.10 — Testing

1. Unit tests.
2. Integration tests.
3. Failure tests.
4. Generated project tests.

### Task W6.11 — Dokumentasi

1. Update README.
2. Update support matrix.
3. Update troubleshooting.
4. Dokumentasikan paired migration.
5. Dokumentasikan seeder.

---

## 5. File yang Akan Terpengaruh

### Dibuat

- Rust seeder modules.
- Down migration files.
- Tests baru.

### Diubah

- `src/application/db.rs`
- `src/generator/migration.rs`
- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/main.rs`
- `src/application/mod.rs`
- `README.md`
- `docs/support-matrix.md`
- `docs/troubleshooting.md`

### Tidak diubah tanpa alasan

- Core manifest schema.
- Template asset engine.
- Quality commands.
- Docker/CI asset templates.

---

## 6. Dependency / Configuration Impact

- Dependency baru pada generated project jika seeder membutuhkan.
- `config.toml` tetap schema v1.
- Database workflow tetap Postgres + SQLx.
- Env var baru jika seeder membutuhkan.
- Tidak ada secret di manifest.

---

## 7. Risiko dan Potensi Konflik

1. Reverse otomatis tidak selalu valid.
2. Custom SQL revert dapat menyebabkan data loss.
3. Seeder dapat menyebabkan duplikasi data.
4. Partial failure dapat meninggalkan state tidak konsisten.
5. Dependency risk: versi library harus kompatibel dengan MSRV.
6. Konflik dengan AGENTS.md: tidak boleh menambah dependency spekulatif.
7. Konflik dengan ARCHITECTURE.md: database decision tetap terpusat.
8. Konflik dengan PRD.md: rollback dan seed harus sesuai support matrix.

---

## 8. Verification Plan

Setelah implementasi Wave 6:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs db create-migration add_status`
- `rustrs db migrate`
- `rustrs db migrate --dry-run`
- `rustrs db rollback`
- `rustrs db status`
- `rustrs db seed`
- `rustrs db seed --dry-run`

Failure tests:

- rollback tanpa down migration
- seed pada tabel non-empty
- invalid migration
- missing sqlx
- invalid database URL

---

## 9. Acceptance Criteria yang Akan Terpenuhi

- P10-W6-AC-01: `db seed` tidak lagi fake success.
- P10-W6-AC-02: Down migration paired files dihasilkan.
- P10-W6-AC-03: Reverse otomatis tersedia untuk DDL sederhana.
- P10-W6-AC-04: Rollback custom SQL revert berfungsi.
- P10-W6-AC-05: Rollback membutuhkan konfirmasi eksplisit.
- P10-W6-AC-06: Seed mendukung dry-run.
- P10-W6-AC-07: Rollback mendukung dry-run.
- P10-W6-AC-08: Generated project tetap buildable.
- P10-W6-AC-09: Tests smoke + unit tersedia.
- P10-W6-AC-10: Dokumentasi tidak overclaim.

---

## 10. Ambiguity yang Masih Perlu Diputuskan

1. Versi exact dependency untuk seeder.
2. Apakah seeder akan menggunakan SQL atau Rust module.
3. Apakah rollback custom revert akan menggunakan transaction.
4. Apakah seed akan membutuhkan konfirmasi.
5. Apakah migration pairing akan menggunakan timestamp atau slug.

## 11. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 6 harus dimulai hanya setelah keputusan database workflow dikonfirmasi.

# Phase 10 — Wave 7 Implementation Plan: Framework, Database, and Data Access Expansion

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 7 — Framework, Database, and Data Access Expansion  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Keputusan yang Sudah Dikonfirmasi

| Keputusan | Pilihan |
|---|---|
| Framework | Actix Web dan Rocket |
| Database | MySQL, SQLite, dan PostgreSQL |
| Data access | SQLx, SeaORM, dan Diesel |
| Database `none` | Didukung |
| Template support | Full Tier 2 |
| Unsupported options | Sembunyikan atau tolak jelas |
| Build verification | `cargo check` + `cargo test` |
| Dependency versions | Audit dulu |

Catatan: versi exact dependency harus diaudit terhadap MSRV 1.85.0 sebelum implementasi.

---

## 1. Tujuan Wave 7

Memperluas support matrix secara nyata:

- Framework Actix Web.
- Framework Rocket.
- Database MySQL.
- Database SQLite.
- Database None.
- Data access SeaORM.
- Data access Diesel.
- Data access None.
- Template conditional rendering.
- Generated project buildable.
- Dokumentasi yang jujur.

---

## 2. Scope

### Dalam scope

- Framework Actix Web.
- Framework Rocket.
- Database MySQL.
- Database SQLite.
- Database None.
- Data access SeaORM.
- Data access Diesel.
- Data access None.
- Generator adaptation.
- Template wiring.
- Manifest dan config.
- Inspection dan routes.
- Testing.
- Dokumentasi.

### Di luar scope

- OAuth/OIDC.
- Multi-tenant.
- Distributed cache invalidation.
- Redis cluster/sentinel.
- Full API documentation generation untuk semua endpoint lama.
- Frontend.
- Deployment.

---

## 3. Kondisi Codebase Saat Ini

- Compatibility matrix hanya mendukung `Axum + Postgres + SQLx`.
- Template metadata hanya mendukung Axum.
- Inspection routes sudah memiliki parser untuk Actix dan Rocket.
- Manifest model sudah mendukung semua framework/database/data access.
- Generator `repository` hanya mendukung SQLx.
- Generator `migration` hanya mendukung Postgres.
- Template conditional rendering sudah tersedia.
- `rustrs add` mendukung capability yang diimplementasikan.
- OpenAPI dan Redis sudah diimplementasikan.

---

## 4. Task Wave 7 yang Akan Dikerjakan

### Task W7.0 — Preflight dan audit

1. Audit compatibility matrix.
2. Audit template metadata dan generated code.
3. Audit dependency candidate untuk framework/database/data access.
4. Audit inspection routes untuk Actix/Rocket.
5. Verifikasi MSRV dan license.
6. Catat file yang akan berubah.

### Task W7.1 — Compatibility matrix

1. Tambahkan entry framework Actix Web.
2. Tambahkan entry framework Rocket.
3. Tambahkan entry database MySQL.
4. Tambahkan entry database SQLite.
5. Tambahkan entry database None.
6. Tambahkan entry data access SeaORM.
7. Tambahkan entry data access Diesel.
8. Tambahkan entry data access None.
9. Definisikan kombinasi yang benar-benar didukung.
10. Tolak kombinasi unsupported dengan pesan jelas.

### Task W7.2 — Framework Actix Web

1. Tambah dependency Actix Web ke template.
2. Buat bootstrap server Actix.
3. Buat router/route registration.
4. Buat middleware dasar.
5. Buat error handling dasar.
6. Update template metadata.
7. Tambah tests.

### Task W7.3 — Framework Rocket

1. Tambah dependency Rocket ke template.
2. Buat bootstrap server Rocket.
3. Buat route registration.
4. Buat fairing/middleware dasar.
5. Buat error handling dasar.
6. Update template metadata.
7. Tambah tests.

### Task W7.4 — Database MySQL

1. Tambah dependency MySQL ke SQLx.
2. Buat connection setup.
3. Buat migration workflow MySQL.
4. Buat env var handling.
5. Update template metadata.
6. Tambah tests.

### Task W7.5 — Database SQLite

1. Tambah dependency SQLite ke SQLx.
2. Buat connection setup.
3. Buat migration workflow SQLite.
4. Buat env var handling.
5. Update template metadata.
6. Tambah tests.

### Task W7.6 — Database None

1. Definisikan project structure tanpa database.
2. Nonaktifkan generator database-dependent.
3. Update manifest validation.
4. Update template metadata.
5. Tambah tests.

### Task W7.7 — Data access SeaORM

1. Tambah dependency SeaORM.
2. Buat entity/model mapping.
3. Buat repository implementation.
4. Update generator repository.
5. Update template metadata.
6. Tambah tests.

### Task W7.8 — Data access Diesel

1. Tambah dependency Diesel.
2. Buat schema/model mapping.
3. Buat repository implementation.
4. Update generator repository.
5. Update template metadata.
6. Tambah tests.

### Task W7.9 — Data access None

1. Definisikan mode tanpa data access.
2. Nonaktifkan generator repository/migration.
3. Update manifest validation.
4. Update template metadata.
5. Tambah tests.

### Task W7.10 — Generator adaptation

1. Adaptasi `module` generator per framework.
2. Adaptasi `resource` generator per framework.
3. Adaptasi `controller/handler` generator.
4. Adaptasi `service` generator.
5. Adaptasi `repository` generator per data access.
6. Adaptasi `entity/dto` generator.
7. Adaptasi `middleware` generator.
8. Adaptasi `test` generator.
9. Adaptasi `migration` generator per database.
10. Pastikan generated code buildable.

### Task W7.11 — Template wiring

1. Update template metadata.
2. Update generated source templates.
3. Pastikan marker registration tidak duplikat.
4. Pastikan route registration konsisten.
5. Pastikan capability condition tetap berfungsi.

### Task W7.12 — Manifest dan config

1. Pastikan manifest tetap source of truth.
2. Validasi compatibility.
3. Tambah config jika diperlukan.
4. Tambah tests.

### Task W7.13 — Inspection dan routes

1. Update route parser untuk Actix.
2. Update route parser untuk Rocket.
3. Tambah tests inspection.

### Task W7.14 — Testing

1. Unit tests compatibility.
2. Generator tests.
3. Integration tests.
4. Generated project `cargo check`.
5. Generated project `cargo test`.
6. Failure tests.

### Task W7.15 — Dokumentasi

1. Update README.
2. Update support matrix.
3. Update troubleshooting.
4. Dokumentasikan kombinasi yang didukung.
5. Dokumentasikan batasan.

---

## 5. File yang Akan Terpengaruh

### Dibuat

- Template framework files.
- Template database files.
- Template data access files.
- Tests baru.

### Diubah

- `src/compatibility/matrix.rs`
- `src/compatibility/resolver.rs`
- `src/template/metadata.rs`
- `src/template/context.rs`
- `src/generator/module.rs`
- `src/generator/resource.rs`
- `src/generator/repository.rs`
- `src/generator/migration.rs`
- `src/inspection/routes.rs`
- `src/application/new_project.rs`
- `README.md`
- `docs/support-matrix.md`
- `docs/troubleshooting.md`

### Tidak diubah tanpa alasan

- Core manifest schema, kecuali compatibility.
- Quality commands.
- Docker/CI asset templates.

---

## 6. Dependency / Configuration Impact

- Dependency baru pada generated project, bukan CLI.
- `config.toml` tetap schema v1.
- Feature flags untuk framework/database/data access.
- Env var baru untuk database connection.
- Tidak ada secret di manifest.

---

## 7. Risiko dan Potensi Konflik

1. Scope sangat besar.
2. Dependency risk: versi library harus kompatibel dengan MSRV.
3. Generated code complexity: banyak kombinasi.
4. Marker registration: route/module registration bisa duplikat.
5. Fake success risk: kombinasi hanya boleh didukung jika buildable.
6. Konflik dengan AGENTS.md: tidak boleh menambah dependency spekulatif.
7. Konflik dengan ARCHITECTURE.md: compatibility decision tetap terpusat.
8. Konflik dengan PRD.md: support matrix harus sesuai dan tidak overclaim.

---

## 8. Verification Plan

Setelah implementasi Wave 7:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs new actix-demo --framework actix-web --database postgres --orm sqlx --defaults`
- `rustrs new rocket-demo --framework rocket --database postgres --orm sqlx --defaults`
- `rustrs new mysql-demo --database mysql --defaults`
- `rustrs new sqlite-demo --database sqlite --defaults`
- `rustrs new none-demo --database none --orm none --defaults`
- `cargo check` pada generated project
- `cargo test` pada generated project jika feasible

Failure tests:

- unsupported combination
- missing database
- invalid data access
- generator pada template yang tidak mendukung

---

## 9. Acceptance Criteria yang Akan Terpenuhi

- P10-W7-AC-01: Framework Actix Web didukung.
- P10-W7-AC-02: Framework Rocket didukung.
- P10-W7-AC-03: Database MySQL didukung.
- P10-W7-AC-04: Database SQLite didukung.
- P10-W7-AC-05: Database None didukung.
- P10-W7-AC-06: Data access SeaORM didukung.
- P10-W7-AC-07: Data access Diesel didukung.
- P10-W7-AC-08: Data access None didukung.
- P10-W7-AC-09: Generated project buildable.
- P10-W7-AC-10: Tests smoke + unit tersedia.
- P10-W7-AC-11: Dokumentasi tidak overclaim.
- P10-W7-AC-12: Unsupported combination ditolak jelas.

---

## 10. Ambiguity yang Masih Perlu Diputuskan

1. Versi exact dependency untuk Actix, Rocket, MySQL, SQLite, SeaORM, Diesel.
2. Apakah `none` database akan mendukung migration generator.
3. Apakah SeaORM/Diesel akan full ORM atau minimal.
4. Apakah generated project untuk semua kombinasi baru akan diuji build.
5. Apakah unsupported combination akan disembunyikan dari CLI.

## 11. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 7 harus dimulai hanya setelah keputusan dependency dan scope framework/database/data access dikonfirmasi.

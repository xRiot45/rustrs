# Phase 10 — Wave 4 Implementation Plan: OpenAPI Runtime dan Redis

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 4 — OpenAPI Runtime dan Redis  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Keputusan yang Sudah Dikonfirmasi

| Keputusan | Pilihan |
|---|---|
| OpenAPI runtime | Diimplementasikan pada Wave 4 |
| Library OpenAPI | `utoipa` |
| UI documentation | Scalar |
| Scope Redis | Client + cache + rate limiting |
| `add openapi` / `add redis` | Generate code langsung |

Catatan: versi exact dependency harus diaudit terhadap MSRV 1.85.0 sebelum implementasi.

---

## 1. Tujuan Wave 4

Mengimplementasikan capability `openapi` dan `redis` secara nyata:

- OpenAPI runtime scaffolding dengan Utoipa.
- Scalar UI.
- Redis client, cache helper, dan rate limiting.
- `rustrs add openapi` dan `rustrs add redis` dengan efek file nyata.
- Generated project buildable.
- Dokumentasi yang jujur.

---

## 2. Scope

### Dalam scope

- Capability `openapi`.
- Capability `redis`.
- Utoipa runtime code.
- Scalar UI.
- Redis client setup.
- Cache helper.
- Rate limiting middleware/helper.
- `rustrs add openapi`.
- `rustrs add redis`.
- Template wiring untuk `enterprise` dan `standard`.
- Generator `dto`/`controller`/`handler` annotation jika diperlukan.
- Unit dan smoke tests.
- Dokumentasi.

### Di luar scope

- OAuth/OIDC.
- Distributed cache invalidation.
- Redis cluster/sentinel.
- Full API documentation generation untuk semua endpoint lama.
- Frontend.
- Deployment.

---

## 3. Kondisi Codebase Saat Ini

- `Capability::OpenApi` dan `Capability::Redis` sudah ada.
- `Features::openapi` dan `Features::redis` sudah ada.
- `validate_capabilities` belum memiliki dependency khusus untuk openapi/redis.
- `rustrs docs` hanya validasi config.
- Redis hanya muncul di Compose asset.
- OpenAPI runtime code belum dihasilkan.
- Template condition untuk capability sudah tersedia.
- `rustrs add` baru mendukung `docker`, `cicd`, `auth`, dan `rbac`.
- `auth` dan `rbac` sudah diimplementasikan pada Wave 3.

---

## 4. Task Wave 4 yang Akan Dikerjakan

### Task W4.0 — Preflight dan audit

1. Audit `docs` helper.
2. Audit Compose Redis service.
3. Audit template `standard` dan `enterprise`.
4. Audit dependency candidate:
   - `utoipa`
   - `utoipa-scalar` atau `utoipa-swagger-ui`
   - `redis`
   - `deadpool-redis` atau client lain
5. Verifikasi MSRV dan license.
6. Catat file yang akan berubah.

### Task W4.1 — Capability dan manifest

1. Pastikan `openapi` dan `redis` tetap capability valid.
2. Tambah dependency capability jika diperlukan.
3. Update capability metadata `is_implemented`.
4. Pastikan `rustrs info` menampilkan status openapi/redis.

### Task W4.2 — OpenAPI runtime scaffolding

1. Tambah dependency Utoipa ke template yang mendukung.
2. Generate OpenAPI config struct.
3. Generate OpenAPI router/UI route.
4. Generate `/openapi.json` atau `/api-docs/openapi.json`.
5. Generate Scalar UI route.
6. Validasi path dan route registration.
7. Update helper `rustrs docs` agar membedakan validation vs runtime.

### Task W4.3 — OpenAPI annotations

1. Update `dto` generator untuk schema derive.
2. Update `controller/handler` generator untuk operation attributes.
3. Update `resource` generator untuk wiring.
4. Pastikan generated code buildable jika OpenAPI disabled.
5. Tambah tests generator.

### Task W4.4 — Redis runtime scaffolding

1. Tambah dependency Redis ke template yang mendukung.
2. Generate Redis client setup.
3. Generate config/env untuk Redis URL.
4. Generate cache helper.
5. Generate rate limiting helper/middleware.
6. Update Compose asset agar konsisten.

### Task W4.5 — Template wiring

1. Update template metadata untuk OpenAPI dan Redis.
2. Update generated source templates.
3. Pastikan marker registration tidak duplikat.
4. Pastikan route registration konsisten.

### Task W4.6 — Integrasi `rustrs add openapi` dan `rustrs add redis`

1. Ubah `openapi` dari partial menjadi implemented.
2. Ubah `redis` dari partial menjadi implemented.
3. Generate code langsung.
4. Update capability metadata.
5. Update `add` command behavior.

### Task W4.7 — Testing

1. Unit tests:
   - OpenAPI config validation
   - Redis config parsing
   - Cache key generation
2. Generator tests:
   - DTO dengan OpenAPI annotations
   - Redis client generation
3. Integration tests:
   - `rustrs add openapi`
   - `rustrs add redis`
4. Generated project tests:
   - `cargo check`
   - `cargo test`

### Task W4.8 — Dokumentasi

1. Update README.
2. Update support matrix.
3. Update troubleshooting.
4. Dokumentasikan env var:
   - `REDIS_URL`
   - OpenAPI provider/path
5. Dokumentasikan batasan OpenAPI runtime.

---

## 5. File yang Akan Terpengaruh

### Dibuat

- Template OpenAPI files untuk `enterprise` dan `standard`.
- Template Redis files untuk `enterprise` dan `standard`.
- Tests baru untuk OpenAPI/Redis.

### Diubah

- `src/manifest/model.rs`
- `src/compatibility/capabilities.rs`
- `src/application/add.rs`
- `src/application/assets.rs`
- `src/template/metadata.rs`
- `src/template/context.rs`
- `src/generator/dto.rs`
- `src/generator/handler.rs` atau `controller.rs`
- `src/generator/resource.rs`
- `src/application/docs.rs`
- `README.md`
- `docs/support-matrix.md`
- `docs/troubleshooting.md`

### Tidak diubah tanpa alasan

- Core manifest schema, kecuali capability default.
- Database workflow.
- Quality commands.
- Docker/CI asset templates, kecuali Redis service.

---

## 6. Dependency / Configuration Impact

- Dependency baru pada generated project, bukan CLI.
- `config.toml` tetap schema v1.
- `openapi` dan `redis` tetap field yang ada.
- Env var baru:
  - `REDIS_URL`
  - OpenAPI provider/path jika diperlukan
- Tidak ada secret di manifest.

---

## 7. Risiko dan Potensi Konflik

1. Dependency risk: versi library harus kompatibel dengan MSRV.
2. Generated code complexity: OpenAPI annotations dapat gagal compile.
3. Redis integration dapat menambah dependency dan complexity.
4. Marker registration: route/module registration bisa duplikat.
5. Fake success risk: `add openapi`/`add redis` hanya boleh sukses jika code benar-benar dihasilkan.
6. Konflik dengan AGENTS.md: tidak boleh menambah dependency spekulatif.
7. Konflik dengan ARCHITECTURE.md: capability decision tetap terpusat.
8. Konflik dengan PRD.md: OpenAPI/Redis harus sesuai support matrix dan tidak overclaim.

---

## 8. Verification Plan

Setelah implementasi Wave 4:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs new openapi-demo --template enterprise --defaults`
- `rustrs new openapi-demo --template standard --defaults`
- `rustrs add openapi`
- `rustrs add redis`
- `cargo check` pada generated project
- `cargo test` pada generated project jika feasible

Failure tests:

- `rustrs add openapi` pada template yang tidak mendukung
- `rustrs add redis` pada template yang tidak mendukung
- invalid Redis URL
- OpenAPI disabled

---

## 9. Acceptance Criteria yang Akan Terpenuhi

- P10-W4-AC-01: OpenAPI runtime scaffolding dihasilkan jika capability aktif.
- P10-W4-AC-02: Scalar UI tersedia.
- P10-W4-AC-03: Redis client setup dihasilkan jika capability aktif.
- P10-W4-AC-04: Cache helper dihasilkan.
- P10-W4-AC-05: Rate limiting helper/middleware dihasilkan.
- P10-W4-AC-06: `rustrs add openapi` dan `rustrs add redis` berfungsi.
- P10-W4-AC-07: Generated project buildable.
- P10-W4-AC-08: Tests smoke + unit tersedia.
- P10-W4-AC-09: Dokumentasi tidak overclaim.
- P10-W4-AC-10: Compose Redis tetap konsisten.

---

## 10. Ambiguity yang Masih Perlu Diputuskan

1. Versi exact library Utoipa dan Redis.
2. Apakah UI docs menggunakan Scalar atau Swagger UI.
3. Apakah OpenAPI annotations ditambahkan ke semua generator atau hanya resource.
4. Apakah Redis rate limiting menggunakan middleware atau helper function.
5. Apakah `add openapi` dan `add redis` generate code langsung atau toggle manifest saja.

## 11. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 4 harus dimulai hanya setelah keputusan dependency dan scope OpenAPI/Redis dikonfirmasi.

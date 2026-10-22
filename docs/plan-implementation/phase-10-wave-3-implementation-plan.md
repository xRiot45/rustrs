# Phase 10 — Wave 3 Implementation Plan: Auth, RBAC, and Guard

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 3 — Auth, RBAC, and Guard  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Keputusan yang Sudah Dikonfirmasi

| Keputusan | Pilihan |
|---|---|
| JWT library | `jsonwebtoken` |
| Password hashing | `argon2` |
| Refresh token | Disimpan hashed di database |
| RBAC | Permission-based dengan tabel permission |
| Auth default | Aktif pada `enterprise` dan `standard` |
| Testing | Smoke + unit tests |

Catatan: versi exact dependency harus diaudit terhadap MSRV 1.85.0 sebelum implementasi.

---

## 1. Tujuan Wave 3

Mengimplementasikan capability `auth` dan `rbac` secara nyata, termasuk generated code yang buildable, middleware/guard, database schema, JWT, Argon2, permission-based RBAC, dan generator `guard` yang tidak lagi stub.

Wave 3 harus tetap menjaga prinsip:

- Tidak mengklaim capability tersedia jika hanya field manifest.
- Tidak menambah dependency spekulatif tanpa evaluasi.
- Tidak mengubah template minimal/simple menjadi kompleks tanpa alasan.
- Semua generated project yang didukung harus lolos `cargo check`/`cargo test`.
- Secret tidak disimpan di source, fixture, atau manifest.

---

## 2. Scope

### Dalam scope

- Capability `auth`.
- Capability `rbac`.
- Generator `guard`.
- Auth default pada template `enterprise` dan `standard`.
- JWT access token dan refresh token.
- Argon2 password hashing.
- Permission-based RBAC.
- Refresh token hashed di database.
- Middleware/guard Axum.
- Migration auth/RBAC.
- `rustrs add auth` dan `rustrs add rbac`.
- Unit dan smoke tests.
- Dokumentasi.

### Di luar scope

- OAuth/OIDC.
- Multi-tenant.
- Session cookie.
- Password reset email flow.
- MFA.
- Audit log integration penuh.
- Rate limiting integration penuh.
- Frontend auth.
- Deployment.

---

## 3. Kondisi Codebase Saat Ini

- `Capability::Auth`, `Capability::Rbac`, dan field `auth`/`rbac` sudah ada.
- `validate_capabilities` sudah memvalidasi `rbac -> auth`.
- `Capability::Auth` dan `Capability::Rbac` masih ditolak oleh `rustrs add` karena belum diimplementasikan.
- Generator `guard` masih stub.
- Middleware generator sudah ada dan reusable.
- Capability condition pada template asset sudah tersedia.
- Manifest discovery, validation, compatibility, dan safe writer sudah tersedia.
- OpenAPI helper baru validasi config, belum runtime scaffolding.

---

## 4. Task Wave 3 yang Akan Dikerjakan

### Task W3.0 — Preflight dan audit

1. Audit template `minimal`, `simple`, `standard`, `enterprise`.
2. Audit marker registration dan struktur module.
3. Audit `guard` generator stub.
4. Audit dependency candidate:
   - `jsonwebtoken`
   - `argon2`
   - `uuid`
   - `chrono` atau `time`
   - `sqlx`
5. Verifikasi MSRV 1.85.0 dan license.
6. Catat file yang akan berubah.

### Task W3.1 — Capability dan manifest

1. Pastikan `auth` dan `rbac` tetap capability valid.
2. Validasi `rbac -> auth`.
3. Set default capability:
   - `enterprise`: `auth = true`, `rbac = true`
   - `standard`: `auth = true`, `rbac = true`
   - `simple`: `auth = false`, `rbac = false`
   - `minimal`: `auth = false`, `rbac = false`
4. Update `Features::default()` jika diperlukan.
5. Pastikan `rustrs info` menampilkan status auth/rbac.
6. Update capability metadata `is_implemented`.

### Task W3.2 — Dependency generated project

1. Tambah dependency auth ke template yang mendukung auth.
2. Tambah feature SQLx yang diperlukan.
3. Pastikan generated `Cargo.toml` buildable.
4. Dokumentasikan dependency yang ditambahkan.

### Task W3.3 — Database schema dan migration

1. Buat migration untuk:
   - `users`
   - `roles`
   - `permissions`
   - `role_permissions`
   - `user_roles`
   - `refresh_tokens`
2. Tambah constraint, index, dan foreign key.
3. Simpan refresh token dalam bentuk hashed.
4. Pastikan migration name unik dan aman.
5. Dokumentasikan rollback limitation.

### Task W3.4 — Generated auth domain layer

1. Generate user entity/model.
2. Generate role dan permission model.
3. Generate auth errors.
4. Generate repository trait.
5. Generate domain validation rules.

### Task W3.5 — Generated auth application layer

1. Register use case.
2. Login use case.
3. Refresh token use case.
4. Logout/revoke refresh token use case.
5. Password hashing service.
6. JWT service.
7. Permission checker.

### Task W3.6 — Generated auth infrastructure layer

1. PostgreSQL user repository.
2. PostgreSQL role/permission repository.
3. Refresh token repository.
4. JWT signing/verification.
5. Argon2 password hashing.
6. Konfigurasi dari environment.

### Task W3.7 — Generated auth interface layer

1. Login request/response DTO.
2. Register request/response DTO.
3. Refresh request/response DTO.
4. HTTP handlers/controllers.
5. Route registration.
6. Error mapping ke HTTP response.
7. OpenAPI annotations jika `openapi` aktif.

### Task W3.8 — Middleware dan guard

1. JWT authentication middleware.
2. Permission guard.
3. Integrasi dengan Axum router.
4. Dokumentasikan cara melindungi route.

### Task W3.9 — Update generator `guard`

1. Hapus stub error.
2. Generate guard yang buildable.
3. Daftarkan module jika diperlukan.
4. Tambah generator tests.

### Task W3.10 — Integrasi `rustrs add auth` dan `rustrs add rbac`

1. Ubah `auth` dari rejected menjadi implemented.
2. Ubah `rbac` dari rejected menjadi implemented setelah auth selesai.
3. Validasi dependency `rbac -> auth`.
4. Update capability metadata.
5. Update `add` command behavior.

### Task W3.11 — Template wiring

1. Update template metadata untuk auth default.
2. Update generated source templates.
3. Pastikan marker registration tidak duplikat.
4. Pastikan route registration konsisten.

### Task W3.12 — Testing

1. Unit tests:
   - password hash/verify
   - JWT sign/verify
   - claims validation
   - permission check
2. Generator tests:
   - guard
   - auth-related generated files
3. Integration tests:
   - `rustrs add auth`
   - `rustrs add rbac`
4. Generated project tests:
   - `cargo check`
   - `cargo test`

### Task W3.13 — Dokumentasi

1. Update README.
2. Update support matrix.
3. Update troubleshooting.
4. Dokumentasikan env var:
   - `JWT_SECRET`
   - `JWT_ACCESS_TTL`
   - `JWT_REFRESH_TTL`
   - `DATABASE_URL`
5. Dokumentasikan batasan auth/RBAC.

---

## 5. File yang Akan Terpengaruh

### Dibuat

- Template auth files untuk `enterprise` dan `standard`.
- Migration files.
- Tests baru untuk auth/guard.

### Diubah

- `src/manifest/model.rs`
- `src/compatibility/capabilities.rs`
- `src/application/add.rs`
- `src/template/metadata.rs`
- `src/generator/guard.rs`
- `README.md`
- `docs/support-matrix.md`
- `docs/troubleshooting.md`

### Tidak diubah tanpa alasan

- Core manifest schema, kecuali default capability.
- Database workflow.
- Quality commands.
- Docker/CI asset templates, kecuali dependency pada auth.

---

## 6. Dependency / Configuration Impact

- Dependency baru pada generated project, bukan CLI.
- `config.toml` tetap schema v1.
- `auth` dan `rbac` tetap field yang ada.
- Env var baru:
  - `JWT_SECRET`
  - `JWT_ACCESS_TTL`
  - `JWT_REFRESH_TTL`
  - `DATABASE_URL`
- Tidak ada secret di manifest.

---

## 7. Risiko dan Potensi Konflik

1. Security risk: JWT secret dan password handling harus aman.
2. Dependency risk: versi library harus kompatibel dengan MSRV.
3. Generated code complexity: auth code besar dan berisiko tidak build.
4. Marker registration: route/module registration bisa duplikat.
5. Migration risk: schema auth kompleks.
6. Fake success risk: `add auth` hanya boleh sukses jika code benar-benar dihasilkan.
7. Konflik dengan AGENTS.md: tidak boleh menambah dependency spekulatif atau mengubah scope.
8. Konflik dengan ARCHITECTURE.md: capability decision tetap terpusat.
9. Konflik dengan PRD.md: auth/RBAC harus sesuai support matrix dan tidak overclaim.

---

## 8. Verification Plan

Setelah implementasi Wave 3:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs new auth-demo --template enterprise --defaults`
- `rustrs new auth-demo --template standard --defaults`
- `rustrs generate guard products` pada project valid
- `rustrs add auth`
- `rustrs add rbac`
- `cargo check` pada generated project
- `cargo test` pada generated project jika feasible

Failure tests:

- `rustrs add rbac` tanpa auth
- invalid JWT secret
- invalid permission
- missing database
- guard generator pada template yang tidak mendukung

---

## 9. Acceptance Criteria yang Akan Terpenuhi

- P10-W3-AC-01: `auth` capability memiliki integrasi nyata.
- P10-W3-AC-02: `rbac` capability memiliki permission-based RBAC.
- P10-W3-AC-03: JWT access dan refresh token bekerja.
- P10-W3-AC-04: Refresh token disimpan hashed di database.
- P10-W3-AC-05: Argon2 password hashing digunakan.
- P10-W3-AC-06: Generator `guard` tidak lagi stub.
- P10-W3-AC-07: `rustrs add auth` dan `rustrs add rbac` berfungsi.
- P10-W3-AC-08: Generated project buildable.
- P10-W3-AC-09: Tests smoke + unit tersedia.
- P10-W3-AC-10: Dokumentasi tidak overclaim.

---

## 10. Ambiguity yang Masih Perlu Diputuskan

1. Versi exact library JWT/Argon2.
2. Apakah refresh token disimpan hashed atau plain.
3. Apakah permission disimpan per route atau per action.
4. Apakah auth default pada `standard` mengaktifkan RBAC atau hanya auth.
5. Apakah generated tests mencakup login flow penuh atau smoke test saja.

## 11. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 3 harus dimulai hanya setelah keputusan dependency dan scope auth dikonfirmasi.

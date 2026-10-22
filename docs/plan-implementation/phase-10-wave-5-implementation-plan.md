# Phase 10 — Wave 5 Implementation Plan: Capability Lain + Generator Stub

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 5 — Capability Lain + Generator Stub  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Keputusan yang Sudah Dikonfirmasi

| Keputusan | Pilihan |
|---|---|
| Scope | Implementasikan semua capability dan generator stub |
| Email | Implementasikan banyak opsi sekaligus |
| Storage | Implementasikan banyak opsi sekaligus |
| Event dispatcher dan job queue | Implementasikan banyak opsi sekaligus |
| Validator | Gunakan `validator` crate |
| Capability default | Aktif pada `enterprise` dan `standard` |
| Dependency versions | Audit dependency dulu |
| Provider features | Gunakan feature flags |
| Provider scope | Implementasikan semua opsi sekaligus |
| Generated tests | Smoke tests saja |

Catatan: versi exact dependency harus diaudit terhadap MSRV 1.85.0 sebelum implementasi.

---

## 1. Tujuan Wave 5

Mengimplementasikan capability yang masih deklaratif:

- `email`
- `storage`
- `audit_log`
- `rate_limit`
- `idempotency`
- `observability`

Dan menghilangkan generator stub:

- `event`
- `event-handler`
- `job`
- `validator`

Dengan efek file nyata, generated project buildable, tests smoke, dan dokumentasi yang jujur.

---

## 2. Scope

### Dalam scope

- Capability `email`.
- Capability `storage`.
- Capability `audit_log`.
- Capability `rate_limit`.
- Capability `idempotency`.
- Capability `observability`.
- Generator `event`.
- Generator `event-handler`.
- Generator `job`.
- Generator `validator`.
- Template wiring untuk `enterprise` dan `standard`.
- `rustrs add` untuk capability yang diimplementasikan.
- Unit dan smoke tests.
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

- `Capability::Email`, `Storage`, `AuditLog`, `RateLimit`, `Idempotency`, `Observability` sudah ada.
- `Features` sudah mendukung semua capability.
- `validate_capabilities` baru memvalidasi `rbac -> auth`.
- Generator `event`, `event-handler`, `job`, `validator` masih stub.
- Middleware generator sudah ada dan reusable.
- Template condition untuk capability sudah tersedia.
- `rustrs add` mendukung `docker`, `cicd`, `auth`, `rbac`, `openapi`, `redis`.
- Capability `email`, `storage`, `audit_log`, `rate_limit`, `idempotency`, `observability` masih ditolak.
- Compose Redis service sudah ada.
- Rate limiting Redis helper sudah ada.

---

## 4. Task Wave 5 yang Akan Dikerjakan

### Task W5.0 — Preflight dan audit

1. Audit capability `email`, `storage`, `audit_log`, `rate_limit`, `idempotency`, `observability`.
2. Audit generator stub `event`, `event-handler`, `job`, `validator`.
3. Audit template `standard` dan `enterprise`.
4. Audit dependency candidate:
   - email providers
   - storage providers
   - tracing/metrics
   - validation crate
5. Verifikasi MSRV dan license.
6. Catat file yang akan berubah.

### Task W5.1 — Capability dan manifest

1. Pastikan semua capability tetap valid.
2. Tambah dependency capability jika diperlukan.
3. Update capability metadata `is_implemented`.
4. Set default capability aktif pada `enterprise` dan `standard`.
5. Pastikan `rustrs info` menampilkan semua capability.
6. Pastikan `config set features.*` tetap bekerja.

### Task W5.2 — Email integration

1. Tambah dependency email ke template yang mendukung.
2. Implementasikan provider SMTP.
3. Implementasikan provider AWS SES.
4. Implementasikan provider lain yang dipilih.
5. Generate email service trait.
6. Generate email sender implementation.
7. Generate DTO dan config.
8. Generate env var handling.
9. Tambah feature flags untuk provider.
10. Tambah tests.

### Task W5.3 — Storage integration

1. Tambah dependency storage ke template yang mendukung.
2. Implementasikan local filesystem provider.
3. Implementasikan S3-compatible provider.
4. Implementasikan provider lain yang dipilih.
5. Generate storage service trait.
6. Generate upload/download/delete.
7. Generate config/env.
8. Tambah feature flags untuk provider.
9. Tambah tests.

### Task W5.4 — Audit log

1. Tambah audit log schema.
2. Generate audit log service.
3. Generate audit log repository.
4. Generate middleware/hook untuk audit.
5. Tambah migration.
6. Tambah tests.

### Task W5.5 — Rate limiting

1. Tambah rate limit middleware.
2. Gunakan Redis jika `redis` aktif.
3. Sediakan fallback in-memory jika Redis tidak aktif.
4. Generate config.
5. Tambah tests.

### Task W5.6 — Idempotency

1. Tambah idempotency middleware.
2. Simpan idempotency key.
3. Reuse response jika key sama.
4. Tambah migration jika diperlukan.
5. Tambah tests.

### Task W5.7 — Observability

1. Tambah tracing setup.
2. Tambah health/metrics endpoint jika sesuai.
3. Tambah logging config.
4. Tambah correlation/request id jika sesuai.
5. Tambah tests.

### Task W5.8 — Event dispatcher dan generator

1. Implementasikan in-memory event dispatcher.
2. Implementasikan opsi external broker.
3. Generate event struct.
4. Generate event handler.
5. Generate registration.
6. Hapus stub error.
7. Tambah feature flags untuk broker.
8. Tambah tests.

### Task W5.9 — Job queue dan generator

1. Implementasikan in-memory job queue.
2. Implementasikan opsi external queue.
3. Generate job struct.
4. Generate worker registration.
5. Hapus stub error.
6. Tambah feature flags untuk queue.
7. Tambah tests.

### Task W5.10 — Validator generator

1. Tambah dependency `validator`.
2. Generate validator struct.
3. Generate validation rules.
4. Hapus stub error.
5. Tambah tests.

### Task W5.11 — Template wiring

1. Update template metadata.
2. Update generated source templates.
3. Pastikan marker registration tidak duplikat.
4. Pastikan route/module registration konsisten.

### Task W5.12 — Integrasi `rustrs add`

1. Ubah capability yang diimplementasikan dari rejected menjadi implemented.
2. Generate code langsung.
3. Update capability metadata.
4. Update `add` command behavior.

### Task W5.13 — Testing

1. Unit tests.
2. Generator tests.
3. Integration tests.
4. Generated project tests.
5. Failure tests.

### Task W5.14 — Dokumentasi

1. Update README.
2. Update support matrix.
3. Update troubleshooting.
4. Dokumentasikan env var.
5. Dokumentasikan batasan.

---

## 5. File yang Akan Terpengaruh

### Dibuat

- Template capability files untuk `enterprise` dan `standard`.
- Template generator files.
- Migration files.
- Tests baru.

### Diubah

- `src/manifest/model.rs`
- `src/compatibility/capabilities.rs`
- `src/application/add.rs`
- `src/application/assets.rs`
- `src/template/metadata.rs`
- `src/template/context.rs`
- `src/generator/event.rs`
- `src/generator/job.rs`
- `src/generator/validator.rs`
- `README.md`
- `docs/support-matrix.md`
- `docs/troubleshooting.md`

### Tidak diubah tanpa alasan

- Core manifest schema, kecuali capability default.
- Database workflow.
- Quality commands.
- Docker/CI asset templates, kecuali capability terkait.

---

## 6. Dependency / Configuration Impact

- Dependency baru pada generated project, bukan CLI.
- `config.toml` tetap schema v1.
- Capability baru tetap field yang ada.
- Feature flags untuk provider.
- Env var baru:
  - email provider config
  - storage provider config
  - tracing/metrics config
- Tidak ada secret di manifest.

---

## 7. Risiko dan Potensi Konflik

1. Scope sangat besar.
2. Dependency risk: versi library harus kompatibel dengan MSRV.
3. Generated code complexity: banyak file dan integrasi.
4. Marker registration: route/module registration bisa duplikat.
5. Fake success risk: capability hanya boleh aktif jika code benar-benar dihasilkan.
6. Konflik dengan AGENTS.md: tidak boleh menambah dependency spekulatif.
7. Konflik dengan ARCHITECTURE.md: capability decision tetap terpusat.
8. Konflik dengan PRD.md: capability harus sesuai support matrix dan tidak overclaim.

---

## 8. Verification Plan

Setelah implementasi Wave 5:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs new wave5-demo --template enterprise --defaults`
- `rustrs new wave5-demo --template standard --defaults`
- `rustrs add email`
- `rustrs add storage`
- `rustrs add audit_log`
- `rustrs add rate_limit`
- `rustrs add idempotency`
- `rustrs add observability`
- `rustrs generate event ...`
- `rustrs generate job ...`
- `rustrs generate validator ...`
- `cargo check` pada generated project

Failure tests:

- capability pada template yang tidak mendukung
- invalid provider config
- missing env var
- generator pada template yang tidak mendukung

---

## 9. Acceptance Criteria yang Akan Terpenuhi

- P10-W5-AC-01: Capability `email`, `storage`, `audit_log`, `rate_limit`, `idempotency`, `observability` memiliki integrasi nyata.
- P10-W5-AC-02: Generator `event`, `event-handler`, `job`, `validator` tidak lagi stub.
- P10-W5-AC-03: `rustrs add` berfungsi untuk capability yang diimplementasikan.
- P10-W5-AC-04: Generated project buildable.
- P10-W5-AC-05: Tests smoke + unit tersedia.
- P10-W5-AC-06: Dokumentasi tidak overclaim.
- P10-W5-AC-07: Feature flags untuk provider tersedia.
- P10-W5-AC-08: Capability default aktif pada `enterprise` dan `standard`.

---

## 10. Ambiguity yang Masih Perlu Diputuskan

1. Versi exact library untuk email, storage, tracing, dan validation.
2. Provider eksternal mana yang akan diimplementasikan pertama.
3. Apakah semua opsi provider langsung diimplementasikan atau bertahap.
4. Apakah capability default aktif pada `standard` dan `enterprise` akan menambah test burden.
5. Apakah generated tests mencakup provider eksternal atau smoke test saja.

## 11. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 5 harus dimulai hanya setelah keputusan dependency dan scope capability/provider dikonfirmasi.

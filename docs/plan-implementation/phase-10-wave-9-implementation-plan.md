# Phase 10 — Wave 9 Implementation Plan: Final Hardening, Release Preparation, dan Dokumentasi

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 9 — Final Hardening, Release Preparation, dan Dokumentasi  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Keputusan yang Sudah Dikonfirmasi

| Keputusan | Pilihan |
|---|---|
| Scope utama | Full release preparation |
| `rustrs update check` | Implementasikan |
| SeaORM dan Diesel | Implementasikan |
| Distribusi | Source/Cargo install only |
| Cross-platform | Verifikasi hanya jika environment tersedia |
| Generated project tests | `cargo check` + `cargo test` |
| Dead code warnings | Bersihkan |
| CHANGELOG | Buat/diperbarui |

Catatan: versi exact dependency harus diaudit terhadap MSRV 1.85.0 sebelum implementasi.

---

## 1. Tujuan Wave 9

Menyelesaikan Phase 10 dengan final hardening, release preparation, dan dokumentasi:

- Final audit CLI dan generated project.
- Implementasikan `rustrs update check`.
- Implementasikan SeaORM dan Diesel.
- Bersihkan dead code warnings.
- Siapkan release workflow dengan aman.
- Buat/diperbarui CHANGELOG.
- Verifikasi generated project.
- Dokumentasi final yang jujur.

---

## 2. Scope

### Dalam scope

- Final audit CLI.
- Final audit generated project.
- Dead code cleanup.
- `rustrs update check`.
- SeaORM integration.
- Diesel integration.
- Release metadata.
- Release workflow.
- Cross-platform verification.
- Dokumentasi final.
- Testing final.
- Quality gates.

### Di luar scope

- Publish release.
- Tag otomatis.
- Deploy.
- Prebuilt binary distribution.
- Auto-install completion.
- Cloud deployment.

---

## 3. Kondisi Codebase Saat Ini

- Wave 1–8 sudah diimplementasikan.
- Capability utama sudah memiliki integrasi minimal.
- Generator stub sudah dihilangkan.
- Database workflow sudah mendukung paired migration, rollback, dan seed.
- Framework/database expansion sudah parsial.
- SeaORM dan Diesel belum diimplementasikan.
- `rustrs update check` belum ada.
- Dead code warnings masih ada pada beberapa generated modules.
- Release workflow sudah ada dan preparation only.
- Platform yang diverifikasi: Linux.
- README, support matrix, dan troubleshooting sudah diperbarui.

---

## 4. Task Wave 9 yang Akan Dikerjakan

### Task W9.0 — Preflight dan audit

1. Audit repository state.
2. Audit semua command dan flags.
3. Audit support matrix.
4. Audit generated project build.
5. Audit release workflow.
6. Verifikasi environment.
7. Catat file yang akan berubah.

### Task W9.1 — Final audit CLI

1. Audit argument parsing.
2. Audit exit codes.
3. Audit stdout/stderr.
4. Audit help/version.
5. Audit error messages.
6. Audit non-interactive behavior.
7. Catat inconsistency.

### Task W9.2 — Final audit generated project

1. Generate semua template.
2. Jalankan `cargo check`.
3. Jalankan `cargo test`.
4. Audit marker registration.
5. Audit route registration.
6. Audit dependency consistency.
7. Catat failure.

### Task W9.3 — Dead code cleanup

1. Hapus unused imports.
2. Hapus unused functions.
3. Hapus dead code.
4. Tambah allow hanya jika justified.
5. Jalankan clippy.

### Task W9.4 — `rustrs update check`

1. Tambah command.
2. Tambah network behavior yang aman.
3. Tambah consent/non-interactive handling.
4. Tambah tests.
5. Dokumentasikan.

### Task W9.5 — SeaORM integration

1. Tambah dependency SeaORM.
2. Buat entity/model mapping.
3. Buat repository implementation.
4. Update generator repository.
5. Update template metadata.
6. Tambah tests.

### Task W9.6 — Diesel integration

1. Tambah dependency Diesel.
2. Buat schema/model mapping.
3. Buat repository implementation.
4. Update generator repository.
5. Update template metadata.
6. Tambah tests.

### Task W9.7 — Release metadata

1. Audit `Cargo.toml`.
2. Audit `--version`.
3. Audit license.
4. Audit repository URL.
5. Audit description.
6. Update metadata jika diperlukan.

### Task W9.8 — Release workflow

1. Audit triggers.
2. Audit quality gates.
3. Audit artifact naming.
4. Audit checksum.
5. Audit permissions.
6. Dokumentasikan secret/settings.

### Task W9.9 — Cross-platform verification

1. Verifikasi build pada Linux.
2. Verifikasi build pada macOS jika tersedia.
3. Verifikasi build pada Windows jika tersedia.
4. Audit path behavior.
5. Update platform claim.

### Task W9.10 — Dokumentasi final

1. Update README.
2. Update support matrix.
3. Update troubleshooting.
4. Buat/diperbarui CHANGELOG.
5. Dokumentasikan known limitations.
6. Dokumentasikan deferred features.

### Task W9.11 — Testing final

1. Unit tests.
2. Integration tests.
3. Generated project tests.
4. Structural tests.
5. Failure tests.

### Task W9.12 — Quality gates

1. `cargo fmt --check`
2. `cargo check`
3. `cargo test`
4. `cargo clippy --all-targets --all-features -- -D warnings`
5. `git diff --check`
6. Inspect git diff.

---

## 5. File yang Akan Terpengaruh

### Dibuat

- `CHANGELOG.md` jika belum ada.
- Tests baru.

### Diubah

- `src/compatibility/matrix.rs`
- `src/compatibility/resolver.rs`
- `src/template/metadata.rs`
- `src/generator/repository.rs`
- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/main.rs`
- `Cargo.toml`
- `README.md`
- `docs/support-matrix.md`
- `docs/troubleshooting.md`
- `.github/workflows/release.yml`

### Tidak diubah tanpa alasan

- Core manifest schema, kecuali compatibility.
- Quality commands.
- Docker/CI asset templates.

---

## 6. Dependency / Configuration Impact

- Dependency baru pada generated project untuk SeaORM/Diesel.
- Dependency baru pada CLI untuk `update check` jika diperlukan.
- `config.toml` tetap schema v1.
- Env var baru jika update check membutuhkan.
- Tidak ada secret di manifest.

---

## 7. Risiko dan Potensi Konflik

1. Scope sangat besar.
2. Dependency risk: versi library harus kompatibel dengan MSRV.
3. Generated code complexity: SeaORM/Diesel full ORM.
4. Network behavior untuk update check.
5. Dead code cleanup bisa merusak fitur.
6. Konflik dengan AGENTS.md: tidak boleh publish/tag otomatis.
7. Konflik dengan ARCHITECTURE.md: release decision tetap terpusat.
8. Konflik dengan PRD.md: dokumentasi harus jujur.

---

## 8. Verification Plan

Setelah implementasi Wave 9:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs --version`
- `rustrs --help`
- `rustrs update check`
- `rustrs new seaorm-demo --data-access sea-orm --defaults`
- `rustrs new diesel-demo --data-access diesel --defaults`
- `cargo check` pada generated project
- `cargo test` pada generated project jika feasible

Failure tests:

- unsupported combination
- missing network
- invalid dependency
- invalid release metadata

---

## 9. Acceptance Criteria yang Akan Terpenuhi

- P10-W9-AC-01: Final audit CLI selesai.
- P10-W9-AC-02: Final audit generated project selesai.
- P10-W9-AC-03: Dead code warnings dibersihkan.
- P10-W9-AC-04: `rustrs update check` berfungsi.
- P10-W9-AC-05: SeaORM didukung.
- P10-W9-AC-06: Diesel didukung.
- P10-W9-AC-07: Release metadata valid.
- P10-W9-AC-08: Release workflow disiapkan dengan aman.
- P10-W9-AC-09: Cross-platform verification tercatat.
- P10-W9-AC-10: Dokumentasi final tidak overclaim.
- P10-W9-AC-11: CHANGELOG tersedia.
- P10-W9-AC-12: Quality gates lulus.

---

## 10. Ambiguity yang Masih Perlu Diputuskan

1. Versi exact dependency untuk SeaORM, Diesel, dan update check.
2. Apakah SeaORM/Diesel full ORM atau minimal.
3. Apakah generated project tests akan dijalankan penuh.
4. Apakah dead code warnings akan dibersihkan.
5. Apakah CHANGELOG akan dibuat/diperbarui.

## 11. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 9 harus dimulai hanya setelah keputusan dependency dan scope dikonfirmasi.

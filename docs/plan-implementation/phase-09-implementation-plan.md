# Phase 9 — Hardening & MVP Release: Implementation Plan

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 9 of 10  
> **Status:** PLANNED — belum ada kode yang diubah; menunggu persetujuan implementasi  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-09-hardening-and-mvp-release.md`  
> **Dokumen ini:** `docs/plan-implementation/phase-09-implementation-plan.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

- Branch aktif: `main`; working tree bersih kecuali `docs/roadmap/phase-09-hardening-and-mvp-release.md` untracked.
- Versi paket: `0.1.0`; MSRV: `1.85.0`; edition: `2024`.
- Phase 0–8 sudah terimplementasi dan quality gate dasar lulus pada fase sebelumnya.
- CLI mendukung: `new`, `generate`/`g`, `info`, `modules`, `routes`, `deps`, `doctor`, `db`, `dev`, `check`, `lint`, `test`, `format`, `audit`, `docs`, `completion`.
- Support matrix nyata masih hanya `Axum + Postgres + SQLx`.
- `cargo-audit` tidak tersedia; `cargo clippy` tersedia melalui Cargo.
- Repository belum memiliki `CHANGELOG.md` dan belum memiliki release workflow repository-level.
- README masih stale: menyatakan Phase 4 selesai dan code generators belum tersedia.
- Audit preflight awal menemukan generated project `cargo fmt --check` gagal karena style template belum rustfmt-compliant.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan | Pilihan |
| --- | --- |
| Versi MVP | **Tetap `0.1.0`** |
| Channel distribusi | **Install dari source/Cargo** |
| Security audit | **Install `cargo-audit` sekarang** |
| Klaim platform | **Linux verified, macOS/Windows unverified** |

---

## 1. Kondisi Codebase Saat Ini

- Branch aktif: `main`; working tree bersih kecuali `docs/roadmap/phase-09-hardening-and-mvp-release.md` untracked.
- Versi paket: `0.1.0`; MSRV: `1.85.0`; edition: `2024`.
- Phase 0–8 sudah terimplementasi dan quality gate dasar lulus pada fase sebelumnya.
- CLI mendukung: `new`, `generate`/`g`, `info`, `modules`, `routes`, `deps`, `doctor`, `db`, `dev`, `check`, `lint`, `test`, `format`, `audit`, `docs`, `completion`.
- Support matrix nyata masih hanya `Axum + Postgres + SQLx`.
- `cargo-audit` tidak tersedia; `cargo clippy` tersedia melalui Cargo.
- Repository belum memiliki `CHANGELOG.md` dan belum memiliki release workflow repository-level.
- README masih stale: menyatakan Phase 4 selesai dan code generators belum tersedia.
- Audit preflight awal menemukan generated project `cargo fmt --check` gagal karena style template belum rustfmt-compliant.

---

## 2. Task Phase 09 yang Akan Dikerjakan

1. Preflight audit dan verifikasi Phase 0–8.
2. Freeze MVP scope dan support matrix.
3. Core CLI hardening.
4. Filesystem safety dan destructive-operation review.
5. Generated project reliability.
6. Generator dan compatibility regression.
7. Security dan dependency audit.
8. Cross-platform dan environment compatibility.
9. Performance dan resource usage.
10. Error recovery dan process lifecycle.
11. Release metadata dan versioning.
12. Distribution dan installation verification.
13. Release pipeline dan supply-chain verification.
14. Release candidate dan MVP acceptance.
15. Documentation, changelog, dan known limitations.
16. Final quality gate dan phase closure.

---

## 3. Implementation Plan

### Langkah A — Preflight dan audit
- Catat branch, commit, dan `git status`.
- Inventaris command, flags, template, framework, database, ORM, capability.
- Jalankan baseline:
  - `cargo fmt --check`
  - `cargo check`
  - `cargo test`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo metadata --no-deps --format-version 1`
  - `git diff --check`
- Buat audit report singkat di `docs/` atau plan implementation.

### Langkah B — MVP scope freeze
- Buat `docs/support-matrix.md` yang menyatakan hanya `Axum + Postgres + SQLx` didukung.
- Tandai `auth`, `redis`, `storage`, `email`, `openapi`, `cicd`, `docker` sesuai implementasi nyata.
- Pastikan help text tidak mengklaim fitur yang belum bekerja.
- Catat post-MVP backlog, termasuk `add <capability>` jika belum diimplementasikan.

### Langkah C — Hardening CLI
- Audit argumen wajib/opsional, default, invalid value, conflicting flags.
- Verifikasi `--help`, `--version`, `--path`, `--json`, quiet/verbose/no-color.
- Verifikasi exit code 0/1/2/3/4/5.
- Tambah regression tests untuk inkonsistensi yang ditemukan.

### Langkah D — Filesystem safety
- Audit path traversal, symlink, conflict, dry-run, temporary file, partial failure.
- Uji nama invalid dan input path-like.
- Uji eksekusi generator berulang.
- Tambah regression tests.

### Langkah E — Generated project reliability
- Fix template constants di `src/template/metadata.rs` agar output Rust lulus `cargo fmt --check`.
- Generate keempat template ke temporary directory.
- Jalankan `cargo metadata`, `cargo check`, `cargo test` pada setiap project.
- Verifikasi manifest, dependency, env placeholder.
- Tambah test/script yang dapat dijalankan manual atau CI.

### Langkah F — Generator dan compatibility regression
- Test setiap generator pada template yang didukung.
- Uji nama valid, invalid, reserved, path-like.
- Uji conflict, dry-run, determinisme.
- Verifikasi import dan registration tidak rusak.

### Langkah G — Security dan dependency audit
- Install `cargo-audit` sesuai persetujuan pengguna.
- Jalankan `cargo audit`.
- Jalankan `cargo tree --locked`.
- Review subprocess, path, template, config, logging, generated Docker/CI.
- Remediasi hanya finding release-blocking dengan perubahan minimal dan tests.

### Langkah H — Cross-platform dan environment
- Verifikasi MSRV `1.85.0` dan stable Rust.
- Dokumentasikan platform yang benar-benar diverifikasi: Linux (verified), macOS/Windows (unverified).
- Verifikasi missing tool diagnostics.
- Kualifikasi klaim platform di README.

### Langkah I — Performance
- Ukur startup command umum.
- Ukur scaffolding untuk template representatif.
- Inspect repeated filesystem scans atau manifest parsing.
- Optimasi hanya jika ada bukti bottleneck.

### Langkah J — Error recovery dan process lifecycle
- Test missing tool, nonzero exit, invalid root, permission error.
- Verifikasi stdout/stderr forwarding.
- Verifikasi cleanup `dev up/down/logs`.
- Tambah regression tests.

### Langkah K — Release metadata dan versioning
- Tetap gunakan versi `0.1.0`.
- Audit `Cargo.toml`, `--version`, metadata.
- Buat `CHANGELOG.md` berdasarkan perubahan aktual.
- Dokumentasikan behavior changes dan known limitations.

### Langkah L — Distribution dan installation
- Verifikasi install dari source/Cargo di environment bersih/isolated.
- Gunakan `cargo install --path . --root /tmp/...` agar tidak mengubah global toolchain.
- Jalankan `rustrs --help`, `rustrs --version`, dan smoke scaffold.
- Dokumentasikan prerequisite dan troubleshooting.

### Langkah M — Release pipeline dan supply-chain
- Tambah `.github/workflows/release.yml` (prepare only, tanpa publish/tag).
- Trigger tag `v*.*.*` dan `workflow_dispatch`.
- Quality gate sebelum build.
- Build matrix, checksum, artifact upload.
- Validasi workflow secara struktural dan safe.
- Dokumentasikan secret/settings yang perlu dikonfigurasi maintainer.

### Langkah N — Release candidate dan MVP acceptance
- Jalankan full quality gate.
- Generate semua template.
- Jalankan generated project checks.
- Jalankan primary user journey.
- Validasi docs, version, packaging.
- Buat MVP readiness report.

### Langkah O — Documentation
- Perbarui README total agar sesuai implementasi.
- Dokumentasikan support matrix, env, errors, prerequisites.
- Buat changelog dan known limitations.
- Update ARCHITECTURE.md jika keputusan arsitektur berubah.
- Perbaiki stale claims.

### Langkah P — Final quality gate
- Jalankan semua command quality gate.
- Inspect `git diff --stat`, `git diff`, `git status`.
- Pastikan tidak ada secret/temporary artifact.
- Buat final report.

---

## 4. File yang Akan Terpengaruh

### Dibuat
- `docs/support-matrix.md`
- `CHANGELOG.md`
- `.github/workflows/release.yml`
- `docs/troubleshooting.md` (jika belum ada)
- Script/test fixture untuk generated project verification (opsional)

### Diubah
- `README.md`
- `ARCHITECTURE.md`
- `src/template/metadata.rs` (format generated Rust)
- `Cargo.toml` (hanya jika metadata perlu diperbaiki)
- `src/application/quality.rs`, `assets.rs`, `docs.rs`, `completion.rs` (hanya jika defect ditemukan)
- Tests terkait defect

### Tidak diubah tanpa alasan
- Core phase 0–8 modules yang sudah lulus, kecuali defect verified.

---

## 5. Dependency / Configuration Impact

- Tidak ada dependency baru direncanakan.
- `cargo-audit` akan diinstall sesuai persetujuan pengguna.
- Workflow release hanya preparation, tidak publish.
- Versi tetap `0.1.0`.

---

## 6. Risiko dan Potensi Konflik

- `cargo-audit` memerlukan network dan dapat menemukan advisory; remediasi bisa memerlukan dependency update yang berpotensi breaking.
- Generated project `cargo check`/`cargo test` memerlukan network/cache dan waktu cukup lama.
- Platform claim harus dibatasi ke Linux karena hanya Linux yang diverifikasi.
- Release workflow tidak boleh melakukan publish/tag otomatis.
- README saat ini sangat stale dan perlu rewrite besar.
- Generated Rust template belum rustfmt-compliant.
- Rollback DB tetap limited karena migration up-only.
- OpenAPI helper status-only, bukan scaffolding.

---

## 7. Verification Plan

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo metadata --no-deps --format-version 1`
- `cargo tree --locked`
- `cargo audit` (setelah install)
- `git diff --check`
- `git status --short`
- Generate semua template dan jalankan `cargo metadata`, `cargo check`, `cargo test`
- Smoke install dari source ke temporary root
- Structural validation YAML/workflow
- Docker/Compose validation jika Docker tersedia

---

## 8. Acceptance Criteria yang Akan Terpenuhi

- P9-AC-001 sampai P9-AC-015: preflight dan scope freeze.
- P9-AC-016 sampai P9-AC-021: CLI hardening.
- P9-AC-022 sampai P9-AC-027: filesystem safety.
- P9-AC-028 sampai P9-AC-033: generated project reliability.
- P9-AC-034 sampai P9-AC-038: generator regression.
- P9-AC-039 sampai P9-AC-044: security/dependency.
- P9-AC-045 sampai P9-AC-049: platform/toolchain.
- P9-AC-050 sampai P9-AC-053: performance.
- P9-AC-054 sampai P9-AC-058: error recovery/process lifecycle.
- P9-AC-059 sampai P9-AC-062: version metadata.
- P9-AC-063 sampai P9-AC-067: distribution/installation.
- P9-AC-068 sampai P9-AC-072: release pipeline.
- P9-AC-073 sampai P9-AC-078: release candidate acceptance.
- P9-AC-079 sampai P9-AC-083: documentation.
- P9-AC-084 sampai P9-AC-089: final quality gate.

---

## 9. Pertanyaan / Ambiguity

Keputusan utama sudah dikonfirmasi:
- Versi MVP tetap `0.1.0`.
- Distribution diverifikasi dari source/Cargo.
- `cargo-audit` akan diinstall untuk audit.
- Klaim platform dibatasi ke Linux verified; macOS/Windows unverified.

Sisa detail seperti exact release workflow matrix dan format changelog akan ditetapkan saat implementasi tanpa mengubah scope Phase 9.

---

## 10. Gunakan tool question untuk pertanyaan yang akan anda berikan kepada saya

Tidak ada pertanyaan tambahan; plan siap untuk persetujuan implementasi.
EOF
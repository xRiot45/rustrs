# Phase 10 — Wave 8 Implementation Plan: Distribusi dan Platform

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Wave:** 8 — Distribusi dan Platform  
> **Bahasa:** Indonesia  
> **Status:** PLANNED — belum diimplementasikan  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-10-unimplemented-features.md` → `docs/plan-implementation/phase-10-implementation-plan.md`

---

## 0. Keputusan yang Sudah Dikonfirmasi

| Keputusan | Pilihan |
|---|---|
| Target platform | Linux, macOS, dan Windows |
| Distribusi | Source/Cargo install only |
| Archive format | tar.gz |
| Checksum | SHA256 |
| Cross-platform | Verifikasi hanya jika environment tersedia |
| Shell completion | Tidak otomatis |
| Release workflow | Hanya disiapkan |
| Docker/Compose | Validasi struktural saja |

Catatan: prebuilt binary tidak akan didistribusikan pada Wave 8; source/Cargo install tetap route MVP.

---

## 1. Tujuan Wave 8

Meningkatkan kesiapan distribusi dan platform tanpa overclaim:

- Platform matrix yang jujur.
- Cross-platform audit.
- Build verification pada platform yang tersedia.
- Release workflow preparation.
- Archive/checksum documentation.
- Shell completion verification.
- Docker/Compose structural validation.
- Dokumentasi yang akurat.

---

## 2. Scope

### Dalam scope

- Platform matrix.
- Cross-platform audit.
- Build verification.
- Release workflow preparation.
- Archive/checksum documentation.
- Shell completion verification.
- Docker/Compose structural validation.
- Dokumentasi.

### Di luar scope

- Publish release.
- Tag otomatis.
- Deploy.
- Prebuilt binary distribution.
- Auto-install completion.
- Docker build runtime.
- Cloud deployment.

---

## 3. Kondisi Codebase Saat Ini

- Release workflow sudah ada di `.github/workflows/release.yml`.
- Shell completion sudah tersedia.
- Docker/Compose/CI assets sudah tersedia.
- Platform yang diverifikasi: Linux.
- macOS dan Windows belum diverifikasi.
- Prebuilt binary distribution belum tersedia.
- README sudah mendokumentasikan source/Cargo install.

---

## 4. Task Wave 8 yang Akan Dikerjakan

### Task W8.0 — Preflight dan audit

1. Audit release workflow.
2. Audit platform claim di README dan support matrix.
3. Audit shell completion.
4. Audit Docker/Compose assets.
5. Audit Cargo metadata dan version.
6. Verifikasi environment yang tersedia.
7. Catat file yang akan berubah.

### Task W8.1 — Platform matrix

1. Definisikan platform yang diverifikasi.
2. Definisikan platform yang belum diverifikasi.
3. Update support matrix.
4. Update README.
5. Tambah catatan blocker.

### Task W8.2 — Cross-platform audit

1. Audit path separator.
2. Audit executable lookup.
3. Audit permission handling.
4. Audit process execution.
5. Audit shell completion.
6. Catat platform-specific issue.

### Task W8.3 — Build verification

1. Verifikasi build pada Linux.
2. Verifikasi build pada macOS jika tersedia.
3. Verifikasi build pada Windows jika tersedia.
4. Catat hasil build.
5. Update platform claim.

### Task W8.4 — Release workflow preparation

1. Audit `.github/workflows/release.yml`.
2. Verifikasi triggers.
3. Verifikasi quality gates.
4. Verifikasi artifact naming.
5. Verifikasi checksum metadata.
6. Verifikasi permission least-privilege.
7. Dokumentasikan secret/settings manual.

### Task W8.5 — Archive dan checksum

1. Tentukan archive layout.
2. Tentukan checksum SHA256.
3. Dokumentasikan cara verifikasi.
4. Tambah tests struktural jika diperlukan.
5. Tambah docs.

### Task W8.6 — Shell completion

1. Verifikasi `bash`, `zsh`, `fish`, `powershell`, `elvish`.
2. Dokumentasikan instalasi manual.
3. Tambah tests help/completion.
4. Tambah troubleshooting.

### Task W8.7 — Docker/Compose validation

1. Validasi struktural Dockerfile.
2. Validasi struktural Compose.
3. Validasi CI workflow YAML.
4. Tambah tests struktural.
5. Dokumentasikan batasan.

### Task W8.8 — Dokumentasi

1. Update README.
2. Update support matrix.
3. Update troubleshooting.
4. Dokumentasikan instalasi dari source.
5. Dokumentasikan platform yang diverifikasi.
6. Dokumentasikan common failure.

### Task W8.9 — Testing

1. Unit tests.
2. Integration tests.
3. Structural tests.
4. Failure tests.

### Task W8.10 — Final quality gates

1. `cargo fmt --check`
2. `cargo check`
3. `cargo test`
4. `cargo clippy --all-targets --all-features -- -D warnings`
5. `git diff --check`

---

## 5. File yang Akan Terpengaruh

### Dibuat

- Tests struktural jika diperlukan.

### Diubah

- `.github/workflows/release.yml`
- `README.md`
- `docs/support-matrix.md`
- `docs/troubleshooting.md`
- Tests terkait release/completion/Docker.

### Tidak diubah tanpa alasan

- Core CLI code, kecuali defect terverifikasi.
- Template asset engine.
- Database workflow.

---

## 6. Dependency / Configuration Impact

- Tidak ada dependency baru pada CLI.
- `config.toml` tetap schema v1.
- Release workflow tetap preparation only.
- Tidak ada secret di source atau docs.

---

## 7. Risiko dan Potensi Konflik

1. Platform claim bisa overclaim jika tidak diverifikasi.
2. Release workflow bisa gagal jika tidak divalidasi.
3. Archive/checksum bisa salah jika tidak diuji.
4. Shell completion bisa tidak sesuai command tree.
5. Docker/Compose assets bisa invalid.
6. Konflik dengan AGENTS.md: tidak boleh publish/tag otomatis.
7. Konflik dengan ARCHITECTURE.md: release decision tetap terpusat.
8. Konflik dengan PRD.md: distribution harus sesuai support matrix.

---

## 8. Verification Plan

Setelah implementasi Wave 8:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

Smoke tests:

- `rustrs --version`
- `rustrs --help`
- `rustrs completion bash`
- `rustrs completion zsh`
- `rustrs completion fish`
- `rustrs generate docker`
- `rustrs generate compose`
- `rustrs generate ci-github`
- `rustrs generate ci-gitlab`

Failure tests:

- missing tool
- invalid platform
- invalid workflow
- invalid completion shell

---

## 9. Acceptance Criteria yang Akan Terpenuhi

- P10-W8-AC-01: Platform matrix jujur.
- P10-W8-AC-02: Cross-platform audit tercatat.
- P10-W8-AC-03: Build verification pada platform tersedia.
- P10-W8-AC-04: Release workflow disiapkan dengan aman.
- P10-W8-AC-05: Archive/checksum didokumentasikan.
- P10-W8-AC-06: Shell completion diverifikasi.
- P10-W8-AC-07: Docker/Compose divalidasi struktural.
- P10-W8-AC-08: Dokumentasi tidak overclaim.
- P10-W8-AC-09: Tests smoke tersedia.
- P10-W8-AC-10: Quality gates lulus.

---

## 10. Ambiguity yang Masih Perlu Diputuskan

1. Versi exact dependency untuk release tooling.
2. Apakah macOS/Windows akan diverifikasi pada Wave 8.
3. Apakah archive/checksum akan diimplementasikan atau hanya didokumentasikan.
4. Apakah Docker build akan dijalankan.
5. Apakah release workflow akan dijalankan atau hanya disiapkan.

## 11. Catatan Eksekusi

Plan ini adalah perencanaan, bukan implementasi. Tidak ada file code yang diubah oleh plan ini. Wave 8 harus dimulai hanya setelah keputusan distribusi dan platform dikonfirmasi.

# Phase 0 — Repository Bootstrap: Implementation Plan

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 0 of 10  
> **Status:** IMPLEMENTED — Phase 0 selesai (verification lulus)  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-0-repository-bootstrap.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal

Repository `/home/riot/projects/RUST/rustrs` pada awalnya **bukan Git repository** dan hanya berisi dokumentasi:

- File yang ada: `AGENTS.md`, `ARCHITECTURE.md`, `PRD.md`, `docs/roadmap/phase-0-repository-bootstrap.md`.
- Direktori `docs/plan-implementation/` kosong.
- Tidak ada `Cargo.toml`, `Cargo.lock`, `src/`, `README.md`, `.gitignore`, CI, license, atau kode Rust.
- Toolchain terpasang: `rustc 1.94.0`, `cargo 1.94.0`, stable `x86_64-unknown-linux-gnu` (edition 2024 didukung; MSRV minimum untuk edition 2024 adalah 1.85.0).

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan                                       | Pilihan                            |
| ----------------------------------------------- | ---------------------------------- |
| Inisialisasi Git repository                     | Ya (`git init`), tanpa commit awal |
| CI minimal pada Phase 0                         | Ya (GitHub Actions)                |
| Lisensi                                         | MIT                                |
| MSRV                                            | 1.85.0                             |
| Metadata `authors`/`repository`/`homepage`      | Omit (belum diketahui)             |
| Pemegang hak cipta LICENSE                      | `rustrs contributors`              |
| `CONTRIBUTING.md` / `CHANGELOG.md` pada Phase 0 | Tunda ke fase dokumentasi          |

---

## 1. Kondisi Codebase Saat Ini

- Bukan Git repo (tidak ada `.git`); `git status`/`git diff` tidak dapat dijalankan.
- Hanya berisi dokumen spesifikasi dan roadmap Phase 0.
- Tidak ada artefak build, source code, atau konfigurasi Rust.
- Toolchain Rust stabil tersedia dan mendukung edition 2024.

Implikasi: seluruh artefak bootstrap (paket Cargo, entry point, hygiene, README, CI) harus dibuat dari nol, tanpa menyentuh fitur apa pun.

---

## 2. Task Phase 0 yang Akan Dikerjakan

Berdasarkan `docs/roadmap/phase-0-repository-bootstrap.md` (Task 0.1–0.7):

1. **Task 0.1 — Establish baseline**: inspeksi toolchain, tree, status; catat keadaan awal.
2. **Task 0.2 — Normalize Rust package**: buat `Cargo.toml` valid, package `rustrs`, edition 2024, MSRV 1.85.0, license MIT, dependency minimal (kosong).
3. **Task 0.3 — Minimal source entry point**: buat `src/main.rs` minimal, truthful, tanpa command parsing, tanpa I/O filesystem/network, tanpa panic pada invokasi normal.
4. **Task 0.4 — Repository hygiene**: `git init`, `.gitignore` standar Rust, track `Cargo.lock`, tambahkan `LICENSE` (MIT), cek secret tidak ter-commit.
5. **Task 0.5 — Project instructions/docs**: buat `README.md` akurat; review/rekonsiliasi `AGENTS.md` (sudah memadai); pertahankan `ARCHITECTURE.md` dan phase doc; tambahkan CI minimal.
6. **Task 0.6 — Quality baseline**: jalankan `cargo metadata`, `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`; perbaiki isu dari perubahan Phase 0; re-run.
7. **Task 0.7 — Review & report**: review diff/status, pastikan tidak ada scope creep, dokumentasikan keputusan/blokir.

### Out of scope (TIDAK dikerjakan pada Phase 0)

- `rustrs new`, template, generator, manifest `.rustrs/config.toml`.
- Compatibility matrix, template engine, filesystem engine.
- Database/dev commands, shell completion, release automation.
- Modul spekulatif untuk fase berikutnya.

---

## 3. Implementation Plan (Berurutan)

### Langkah A — Inisialisasi Git (Task 0.1 / 0.4)

- Jalankan `git init` di root repository.
- Set default branch `main` jika belum (non-destruktif).
- Catat bahwa repository sebelumnya non-Git; tidak ada user changes yang hilang.
- **Tidak** membuat commit awal (sesuai keputusan pengguna dan aturan AGENTS.md).

### Langkah B — Buat `Cargo.toml` (Task 0.2)

Metadata yang akan ditetapkan:

```toml
[package]
name = "rustrs"
version = "0.1.0"
edition = "2024"
rust-version = "1.85.0"
description = "Rust API scaffolding CLI"
license = "MIT"

[dependencies]
```

Catatan:

- Tidak menambahkan `[[bin]]` eksplisit (default sudah menghasilkan binary `rustrs` dari `src/main.rs`).
- Tidak menambahkan `authors`, `repository`, `homepage`, `keywords`, `categories` (belum diketahui; hindari tebakan).
- Section `[dependencies]` kosong (tanpa dependency Phase 0).

### Langkah C — Buat `src/main.rs` (Task 0.3)

Entry point minimal:

```rust
fn main() {
    println!("rustrs - Rust API scaffolding CLI");
    println!("CLI foundation is under development.");
}
```

Aturan:

- Tanpa `clap`, tanpa argument parsing.
- Tanpa filesystem/network I/O.
- Tanpa `unwrap()`/`expect()` pada path user.
- Tidak mengklaim fitur yang belum ada.

### Langkah D — Repository hygiene (Task 0.4)

- Buat `.gitignore`:

```
/target/
.env
.env.*
!.env.example
.DS_Store
```

- Buat `LICENSE` dengan teks MIT standar; baris copyright: `Copyright (c) 2026 rustrs contributors`.
- Pastikan `Cargo.lock` akan di-track (ter-generate saat `cargo check`).
- Inspeksi tracked files untuk secret (jika feasible).

### Langkah E — Dokumentasi (Task 0.5)

- Buat `README.md` singkat dan akurat:
  - Nama + tujuan satu kalimat.
  - Status: "under development; project generation not yet available."
  - Prasyarat: Rust stable ≥ 1.85.0 (MSRV).
  - Clone + setup.
  - Perintah verifikasi: `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`.
  - Roadmap ringkas / link ke `docs/roadmap/`.
  - Future commands diberi label "planned", bukan "available".
- Review `AGENTS.md`: sudah mencakup scope, verifikasi, safety; **tidak perlu perubahan** kecuali ditemukan gap. Rekonsiliasi minor boleh dilakukan jika diperlukan, tetapi hindari rewrite.
- `ARCHITECTURE.md` dan phase doc: pertahankan; tidak diubah pada Phase 0.

### Langkah F — CI minimal (Task 0.5 / 0.6)

- Buat `.github/workflows/ci.yml`:
  - Trigger: `push`, `pull_request`.
  - Job pada `ubuntu-latest`, toolchain stable.
  - Steps: checkout, install toolchain (components: rustfmt, clippy), `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`.
  - Tidak ada secret, tidak ada publish/deploy.
- Catatan: verifikasi versi action terbaru saat implementasi.

### Langkah G — Verifikasi baseline (Task 0.6)

Jalankan dari root repository:

1. `cargo metadata --no-deps --format-version 1`
2. `cargo check`
3. `cargo test`
4. `cargo fmt --check`
5. `cargo clippy --all-targets --all-features -- -D warnings`
6. Smoke test manual: jalankan binary, pastikan exit code 0, output truthful, tidak ada file yang dibuat.
7. Jika `cargo fmt --check` gagal, jalankan `cargo fmt` lalu re-check.

### Langkah H — Review & report (Task 0.7)

- Review `git status --short` dan `git diff`.
- Pastikan tidak ada modul spekulatif, dependency, atau command palsu.
- Dokumentasikan keputusan (MSRV, license, CI, git init, metadata omit) dan blokir (jika ada).

---

## 4. File yang Akan Terpengaruh

### Dibuat

- `Cargo.toml` — metadata paket dan dependency baseline (kosong).
- `src/main.rs` — entry point minimal.
- `.gitignore` — hygiene build/env.
- `README.md` — dokumentasi akurat status proyek.
- `LICENSE` — lisensi MIT (copyright "rustrs contributors").
- `.github/workflows/ci.yml` — CI minimal.
- `Cargo.lock` — dihasilkan oleh Cargo saat build (track).

### Diubah (kemungkinan minor)

- `AGENTS.md` — hanya jika ditemukan gap; target tanpa perubahan kecuali rekonsiliasi kecil.
- `ARCHITECTURE.md` — tidak diubah pada Phase 0 (kecuali pencatatan keputusan bila dianggap perlu).

### Dihapus

- Tidak ada.

### Tidak disentuh (out of scope)

- `PRD.md`, `docs/roadmap/phase-0-repository-bootstrap.md`, `docs/plan-implementation/`.

---

## 5. Dependency / Configuration Impact

- **Dependency Rust:** tidak ada dependency eksternal pada Phase 0 (hanya `std`). `Cargo.lock` tetap di-track untuk aplikasi binary.
- **Toolchain:** edition 2024, MSRV 1.85.0 (`rust-version`). CI menggunakan stable terbaru.
- **Konfigurasi baru:** `.gitignore`, `.github/workflows/ci.yml`. Tidak ada `rust-toolchain.toml` (hindari pin spekulatif).
- **Konfigurasi yang tidak ditambahkan:** `rustfmt.toml`, `clippy.toml`, `.cargo/config.toml` — gunakan default; tambahkan hanya jika ada alasan proyek-wide.
- **Metadata yang sengaja di-omit:** `authors`, `repository`, `homepage`, `keywords`, `categories` (belum diketahui; hindari tebakan).

---

## 6. Risiko dan Potensi Konflik

| Risiko                                     | Mitigasi                                                    |
| ------------------------------------------ | ----------------------------------------------------------- |
| Repository awalnya non-Git                 | `git init` saja, tanpa commit; tidak ada data yang hilang   |
| Lisensi MIT tanpa pemegang hak cipta pasti | Placeholder "rustrs contributors"; konfirmasi sebelum rilis |
| Clippy `-D warnings` pada kode minimal     | Risiko rendah; perbaiki warning (bukan `allow` broad)       |
| CI action versions                         | Verifikasi versi saat implementasi                          |
| Scope creep (modul/dependency)             | Checklist out-of-scope dan review diff                      |
| Dokumentasi mengklaim fitur                | Label "planned" dan status "under development"              |
| Konflik dengan AGENTS.md                   | Sudah memadai; perubahan minimal hanya jika gap             |
| Konflik dengan ARCHITECTURE.md/PRD.md      | Tidak ada; Phase 0 tidak implementasi arsitektur            |

---

## 7. Verification Plan

Setelah implementasi, jalankan dari root repository:

1. `cargo metadata --no-deps --format-version 1` — validasi metadata.
2. `cargo check` — kompilasi.
3. `cargo test` — test (diharapkan 0 test, lulus).
4. `cargo fmt --check` — format.
5. `cargo clippy --all-targets --all-features -- -D warnings` — lint.
6. Smoke test: jalankan binary, pastikan exit code 0, output truthful, tidak ada file yang dibuat.
7. Review: `git status --short`, `git diff` (setelah `git init`).
8. Verifikasi `.gitignore` tidak meng-ignore `Cargo.lock`.

Jika suatu perintah tidak dapat dijalankan, laporkan `BLOCKED` dengan alasan, bukan `PASS`.

---

## 8. Acceptance Criteria yang Akan Terpenuhi

| ID       | Kriteria                                                | Verifikasi           |
| -------- | ------------------------------------------------------- | -------------------- |
| P0-AC-01 | Repository state diinspeksi sebelum perubahan           | Report baseline      |
| P0-AC-02 | Metadata Cargo valid/akurat                             | `cargo metadata`     |
| P0-AC-03 | Project compile                                         | `cargo check`        |
| P0-AC-04 | `cargo test` lulus                                      | Ya (0 test)          |
| P0-AC-05 | Format bersih                                           | `cargo fmt --check`  |
| P0-AC-06 | Clippy warnings as errors                               | Lulus                |
| P0-AC-07 | README sesuai proyek aktual                             | Review manual        |
| P0-AC-08 | Agent instructions menetapkan scope & verifikasi        | `AGENTS.md`          |
| P0-AC-09 | Tidak ada fitur belum implementasi yang diklaim bekerja | Review source/README |
| P0-AC-10 | Tidak ada user work yang dibuang                        | Git diff/status      |
| P0-AC-11 | Tidak ada secret/metadata spekulatif                    | Review changed files |
| P0-AC-12 | Tidak ada generator/template engine                     | Scope review         |

---

## 9. Pertanyaan / Ambiguity (Sisa)

1. **Pemegang hak cipta LICENSE MIT** — placeholder "rustrs contributors"; perlu nama resmi sebelum rilis (tidak blokir Phase 0).
2. **CI action versions** — verifikasi versi action terbaru saat implementasi.
3. **Strategi commit** — `git init` saja tanpa commit (dikonfirmasi).
4. **Badge/CI status di README** — disarankan tidak pada Phase 0.
5. **`CONTRIBUTING.md` / `CHANGELOG.md`** — ditunda ke fase dokumentasi (dikonfirmasi).

---

## 10. Status dan Langkah Berikutnya

- Plan Phase 0 telah difinalisasi dengan keputusan yang tercatat di atas.
- **Belum ada kode yang diubah**; implementasi menunggu persetujuan pengguna.
- Setelah persetujuan, eksekusi Langkah A–H secara berurutan, lalu jalankan verification plan dan buat laporan Phase 0 sesuai format yang diminta `AGENTS.md`.
- Fase berikutnya (jika disetujui setelah Phase 0 selesai): **Phase 1 — CLI Foundation**.

# Phase 1 — CLI Foundation: Implementation Plan

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 1 of 10  
> **Status:** IMPLEMENTED — Phase 1 selesai (verification lulus)  
> **Bahasa:** Indonesia  
> **Sumber kebenaran:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → `docs/roadmap/phase-01-cli-foundation.md`

---

## 0. Konteks dan Keputusan yang Sudah Dikonfirmasi

### Kondisi codebase awal (sebelum Phase 1)

- Phase 0 selesai dan sudah dipush ke `origin/main` (commit `4826378`).
- `Cargo.toml` belum memiliki dependency; `src/main.rs` masih output statis.
- `docs/roadmap/phase-01-cli-foundation.md` ada tetapi currently untracked.
- Git status bersih kecuali file phase 01 yang untracked.
- Toolchain: Rust stable 1.94.0, MSRV 1.85.0, edition 2024.
- `AGENTS.md`, `ARCHITECTURE.md`, `PRD.md` tersedia dan menjadi sumber kebenaran.
- Belum ada tests, modul CLI, error module, atau project-root discovery.

### Keputusan yang telah dikonfirmasi pengguna

| Keputusan                              | Pilihan                                                       |
| -------------------------------------- | ------------------------------------------------------------- |
| Surface command Phase 1                | Tanpa subcommand (hanya root help/version dan global options) |
| `--color <when>`                       | Tunda ke fase berikutnya                                      |
| Strategi error type                    | `thiserror`                                                   |
| Exit code project root missing/invalid | 2                                                             |
| Canonicalization root discovery        | Canonicalize start path                                       |
| Update `ARCHITECTURE.md`               | Ya, update minor                                              |
| Path pada pesan error root discovery   | Path input pengguna                                           |

---

## 1. Kondisi Codebase Saat Ini

- Phase 0 selesai dan sudah dipush.
- `Cargo.toml` belum memiliki dependency.
- `src/main.rs` masih output statis.
- `docs/roadmap/phase-01-cli-foundation.md` ada tetapi untracked.
- Git status bersih kecuali file phase 01 yang untracked.
- Toolchain: Rust stable 1.94.0, MSRV 1.85.0, edition 2024.
- Belum ada tests, modul CLI, error module, atau project-root discovery.
- Dependency yang akan dipakai (MSRV kompatibel): `clap 4.6.7` (rust-version 1.85), `thiserror 2.0.21` (1.77), dev-deps `assert_cmd 2.2.2`, `predicates 3.1.4`, `tempfile 3.27.0`.

---

## 2. Task Phase 01 yang Akan Dikerjakan

Berdasarkan `docs/roadmap/phase-01-cli-foundation.md`:

1. **Task 1.1 — Integrasi Clap dan root command**: derive API, help/version, invalid args.
2. **Task 1.2 — Struktur command dan registrasi**: pola deklarasi/dispatch tanpa dummy handler; Phase 1 tidak mendaftarkan subcommand masa depan.
3. **Task 1.3 — Global options**: `--verbose/-v`, `--quiet/-q`, `--no-color`; `--verbose` dan `--quiet` saling konflik; `--color` ditunda.
4. **Task 1.4 — Error handling dan exit codes**: typed error kecil dengan `thiserror`, mapping ke exit code, stderr.
5. **Task 1.5 — Output terminal terpusat**: kebijakan stdout/stderr, quiet/verbose/no-color.
6. **Task 1.6 — Project-root discovery**: candidate Cargo root detection dari start path ke parent.
7. **Task 1.7 — Struktur source code awal**: modul minimal tanpa modul spekulatif.
8. **Task 1.8 — Automated tests**: unit + integration.
9. **Task 1.9 — Dokumentasi**: README dan ARCHITECTURE sesuai implementasi.

### Out of scope

- `new`, templates, generators, manifest, compatibility matrix.
- Database/dev commands, shell completion, async runtime, plugin system, global config.
- `--dry-run`, `--force`, route inspection, doctor, quality-tool orchestration.

---

## 3. Implementation Plan (Berurutan)

### Langkah A — Preflight dan baseline

- Inspeksi `git status --short`, branch, log, tree, toolchain.
- Jalankan baseline checks: `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`.
- Catat failure baseline terpisah dari regression.

### Langkah B — Tambah dependency

`Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
thiserror = "2"

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

- Jalankan `cargo check` untuk memastikan resolve dan MSRV.

### Langkah C — Struktur modul minimal

Buat hanya modul yang dibutuhkan Phase 1:

```text
src/
├── main.rs
├── cli/
│   ├── mod.rs
│   └── output.rs
├── error.rs
└── project/
    ├── mod.rs
    └── root.rs
tests/
└── cli.rs
```

Catatan:

- Tidak membuat `commands.rs` karena tidak ada subcommand pada Phase 1.
- Tidak membuat modul `application`, `domain`, `config`, `templates`, `filesystem`, `diagnostics`.
- Tidak membuat trait spekulatif.

### Langkah D — Implementasi CLI root

`src/cli/mod.rs`:

- `#[derive(Parser, Debug)] struct Cli` dengan `name = "rustrs"`, `version`, `about`.
- Global options:
  - `verbose: bool` (`-v`, `--verbose`, `global = true`, conflicts_with = "quiet")
  - `quiet: bool` (`-q`, `--quiet`, `global = true`, conflicts_with = "verbose")
  - `no_color: bool` (`--no-color`, `global = true`)
- Tidak ada field subcommand.
- `Cli::parse()` menangani help/version/invalid args dengan exit code Clap standar (2).

### Langkah E — Error dan exit code

`src/error.rs`:

- `enum CliError` dengan variant yang benar-benar dipakai:
  - `InvalidInput(String)` → exit code 2
  - `ProjectRoot(#[from] ProjectRootError)` → exit code 2
- `impl CliError { fn exit_code(&self) -> i32 }`.
- `Display` via `thiserror`.
- Jangan tambahkan variant masa depan yang tidak dipakai.

### Langkah F — Output policy

`src/cli/output.rs`:

- `struct Output { quiet, verbose, no_color }`.
- Constructor dari `&Cli`.
- Method minimal: `info`, `warn`, `error`.
- `info` ditekan jika `quiet`; `error` selalu ke stderr.
- Tidak ada styling ANSI pada Phase 1; `no_color` disimpan dan dapat diuji.
- Tidak ada progress bar/spinner/logging framework.

### Langkah G — Project-root discovery

`src/project/root.rs`:

- `fn discover_project_root(start: &Path) -> Result<PathBuf, ProjectRootError>`.
- Validasi start path ada dan direktori; jika tidak, error jelas dengan path input pengguna.
- Canonicalize start path yang valid.
- Traversal hanya ke parent, bukan descendant.
- Marker: file `Cargo.toml` pada direktori.
- Berhenti di filesystem root; jika tidak ditemukan, error `NotFound`.
- Tidak mengubah CWD, tidak membaca `.rustrs/config.toml`, tidak mutate filesystem.

`ProjectRootError` dengan variant minimal:

- `NotFound { start }`
- `InvalidPath { path }`
- `Io { path, source }`

### Langkah H — Thin `main.rs`

- Parse `Cli`.
- Bangun `Output`.
- Panggil `run(cli, &output)` (fungsi kecil di `main.rs` atau modul CLI).
- Pada error, cetak ke stderr dan `process::exit(err.exit_code())`.
- Tidak ada business logic.

### Langkah I — Tests

Unit tests:

- Root discovery: root valid, nested, not found, nonexistent path, file path, filesystem root boundary.
- Error exit-code mapping.
- Output policy: quiet menekan info, error tetap.

Integration tests (`tests/cli.rs`, `assert_cmd`):

- `--help` exit 0 dan memuat nama/usage.
- `--version` exit 0 dan sesuai package version.
- Unknown argument exit non-zero.
- Help/version dari temporary directory di luar repo.
- Global flags dapat diparse (mis. `--no-color --help`).
- Invocation dasar tidak membuat file/direktori.

### Langkah J — Dokumentasi

- `README.md`: tambahkan `--help`, `--version`, global flags; nyatakan scaffolding/generator belum tersedia.
- `ARCHITECTURE.md`: catat modul CLI, error/exit policy, root discovery candidate.
- `docs/roadmap/phase-01-cli-foundation.md`: update status setelah implementasi.

### Langkah K — Verification

- `cargo metadata --no-deps --format-version 1`
- `cargo check`
- `cargo test`
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo build`
- `cargo run -- --help`
- `cargo run -- --version`
- Smoke test dari temporary directory.
- Review `git status --short` dan `git diff`.

---

## 4. File yang Akan Terpengaruh

### Dibuat

- `src/cli/mod.rs` — definisi Clap dan global options.
- `src/cli/output.rs` — output policy terpusat.
- `src/error.rs` — typed error dan exit-code mapping.
- `src/project/mod.rs` — modul project.
- `src/project/root.rs` — candidate root discovery + tests.
- `tests/cli.rs` — integration tests binary.

### Diubah

- `Cargo.toml` — tambah `clap`, `thiserror`, dev-deps.
- `Cargo.lock` — update otomatis.
- `src/main.rs` — thin entry point, parse + dispatch + exit code.
- `README.md` — documented available features.
- `ARCHITECTURE.md` — catatan keputusan Phase 1.
- `docs/roadmap/phase-01-cli-foundation.md` — status update setelah implementasi.

### Dihapus

- Tidak ada.

### Tidak disentuh (out of scope)

- `PRD.md`, template, generator, manifest, compatibility matrix, CI (kecuali bila diperlukan).

---

## 5. Dependency / Configuration Impact

- Dependency production: `clap` (derive), `thiserror`.
- Dependency development: `assert_cmd`, `predicates`, `tempfile`.
- MSRV tetap 1.85.0; semua dependency yang dipilih kompatibel.
- Tidak menambah Tokio, Serde/TOML, Tera, dialoguer, walkdir, progress bar.
- Tidak menambah `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml` (default digunakan).
- CI existing tetap valid; test baru akan berjalan otomatis.

---

## 6. Risiko dan Potensi Konflik

- Clap parse errors keluar sebelum `main` error mapping; tests harus membedakan parse failure vs execution failure.
- `--no-color` tidak dapat mempengaruhi output Clap sendiri karena flag diparse oleh Clap; dokumentasikan sebagai limitation Phase 1.
- Canonicalization dapat mengubah path relative menjadi absolute; tests harus membandingkan path yang sudah di-canonicalize.
- Root discovery hanya mendeteksi `Cargo.toml`, bukan validasi project `rustrs`; dokumentasikan sebagai candidate detection.
- Risiko scope creep: jangan menambah subcommand atau modul masa depan.
- `thiserror` menambah dependency; namun MSRV dan lisensi kompatibel.
- `ARCHITECTURE.md` sudah menyebut struktur target; pastikan tidak menyalin roadmap berlebihan.

---

## 7. Verification Plan

1. `cargo metadata --no-deps --format-version 1`
2. `cargo check`
3. `cargo test`
4. `cargo fmt --check`
5. `cargo clippy --all-targets --all-features -- -D warnings`
6. `cargo build`
7. `cargo run -- --help`
8. `cargo run -- --version`
9. Smoke test dari temporary directory.
10. Review `git status --short` dan `git diff`.

Jika command gagal, laporkan `BLOCKED`/`FAIL`, jangan `PASS`.

---

## 8. Acceptance Criteria yang Akan Terpenuhi

- Clap derive terintegrasi; help dan version berfungsi.
- Invalid args gagal konsisten dengan exit code non-zero.
- Tidak ada command yang berpura-pura sukses.
- Global options `--verbose`, `--quiet`, `--no-color` didefinisikan; `--verbose`/`--quiet` konflik; quiet tidak menekan error.
- Error ke stderr; execution failure non-zero.
- Error/exit-code policy terdokumentasi.
- Candidate Cargo root discovery berfungsi untuk root, nested, dan gagal pada kasus invalid.
- Root discovery tidak mutate filesystem.
- Help/version bekerja tanpa project root.
- Integration tests mencakup help/version/invalid args.
- README sesuai implementasi.
- Semua quality gate lulus.

---

## 9. Pertanyaan / Ambiguity

Semua pertanyaan klarifikasi telah dijawab:

- Command surface: tanpa subcommand.
- `--color`: ditunda.
- Error type: `thiserror`.
- Project root missing/invalid exit code: 2.
- Canonicalization: canonicalize start path.
- `ARCHITECTURE.md`: update minor.
- Path pada pesan error root discovery: path input pengguna.

Sisa minor (tidak blokir implementasi):

- Apakah root discovery error harus menampilkan path absolut (hasil canonicalize) pada pesan — diputuskan menampilkan path input pengguna.

---

## 10. Status dan Langkah Berikutnya

- Plan Phase 1 telah difinalisasi dengan keputusan yang tercatat di atas.
- **Belum ada kode yang diubah**; implementasi menunggu persetujuan pengguna.
- Setelah persetujuan, eksekusi Langkah A–K secara berurutan, lalu jalankan verification plan dan buat laporan Phase 1 sesuai format yang diminta `AGENTS.md`.
- Fase berikutnya (jika disetujui setelah Phase 1 selesai): **Phase 2 — Manifest and Compatibility**.

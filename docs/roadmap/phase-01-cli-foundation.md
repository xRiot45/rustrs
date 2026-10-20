# Phase 1 — CLI Foundation

> **Project:** `rustrs` — Rust API scaffolding CLI  
> **Phase:** 1 of 10  
> **Status:** Implemented  
> **Audience:** AI coding agents and maintainers  
> **Primary outcome:** Establish a reliable, testable CLI interface that future `rustrs` features can extend without rewriting command parsing, output handling, error handling, or process entry points.  
> **Language:** Dokumentasi berbahasa Indonesia; source code dan identifier menggunakan bahasa Inggris yang idiomatis.  
> **Source of truth:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → dokumen fase ini.

---

## 1. Tujuan

Phase 1 mengganti output statis dari `src/main.rs` dengan CLI sungguhan berbasis `clap` derive API. Fase ini menetapkan struktur command, help/version, error handling, exit codes, kebijakan output terminal, project-root discovery, serta automated tests.

Pada akhir fase ini, CLI harus menjadi fondasi stabil bagi fase-fase berikutnya. Command harus diparsing secara benar, error disampaikan dengan konsisten, dan utilitas project-root discovery dapat dipakai oleh command yang membutuhkan konteks project.

**Phase 1 tidak mengimplementasikan project scaffolding, template engine, code generator, manifest, atau database workflow.** Jangan berpura-pura menjalankan fitur yang belum tersedia.

### 1.1 Hasil yang diharapkan

- `rustrs --help` menampilkan help yang dihasilkan dari definisi CLI aktual.
- `rustrs --version` menampilkan versi dari metadata Cargo.
- Argumen invalid ditolak dengan pesan yang jelas dan exit code non-zero.
- Error saat eksekusi ditampilkan ke stderr.
- Kebijakan `--verbose`, `--quiet`, dan `--no-color` konsisten.
- Project-root discovery dapat menemukan root kandidat Cargo project dari direktori saat ini atau subdirektori.
- Help dan version bekerja di luar project directory.
- Integration tests menguji binary nyata, bukan hanya helper functions.
- Dokumentasi sesuai dengan fitur yang benar-benar tersedia.
- Tidak ada fitur Phase 2 atau fase setelahnya yang diimplementasikan secara prematur.

---

## 2. Preflight dan pemeriksaan repository

AI agent wajib memeriksa repository aktual sebelum mengubah file. Jangan mengasumsikan kondisi repository sama dengan laporan sebelumnya.

### 2.1 Dokumen yang wajib dibaca

1. `PRD.md`
2. `ARCHITECTURE.md`
3. `AGENTS.md`
4. `docs/roadmap/phase-0-repository-bootstrap.md`
5. `docs/roadmap/phase-01-cli-foundation.md`
6. Source code, tests, README, dan CI workflow yang relevan.

Jika ada dokumen yang tidak ditemukan, laporkan. Jangan mengklaim sudah membacanya.

### 2.2 Inspeksi awal

Jalankan perintah berikut atau padanannya yang tersedia di sistem:

```bash
git status --short
git branch --show-current
git log -5 --oneline
find . -maxdepth 3 -type f | sort
rustc --version
cargo --version
```

Jangan membuang perubahan pengguna dan jangan menjalankan perintah Git destruktif.

### 2.3 Baseline checks

Sebelum mengubah kode, jalankan:

```bash
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Catat kegagalan baseline secara terpisah dari regression yang muncul akibat perubahan Phase 1.

---

## 3. Batasan scope

### 3.1 Termasuk Phase 1

1. Integrasi `clap` derive API.
2. Root CLI command dan global options.
3. Help dan version output.
4. Konvensi deklarasi command dan subcommand.
5. Error handling di boundary CLI/application.
6. Kebijakan exit code.
7. Output conventions: stdout, stderr, verbosity, quiet mode, dan color.
8. Project-root discovery untuk command yang membutuhkan konteks project.
9. Unit tests dan integration tests untuk CLI.
10. Update README dan architecture docs jika diperlukan.
11. Semua baseline quality checks.

### 3.2 Tidak termasuk Phase 1

Jangan implementasikan:

- `rustrs new` dan interactive project wizard.
- Isi template Enterprise, Standard, Simple, dan Minimal.
- Template resolver, renderer, atau template assets.
- Eksekusi `generate` / `g` untuk resource, module, controller, service, repository, DTO, model, migration, dan generator lainnya.
- Schema, load/save, atau validasi `.rustrs/config.toml`.
- Framework/database/ORM compatibility matrix.
- Capability resolver atau capability installer.
- Generation planner, safe writer, `--dry-run`, dan overwrite engine.
- Database migration, Docker orchestration, atau development process management.
- `doctor`, environment checks, quality-tool orchestration, dependency inspection.
- Shell completion, update check, binary distribution, publishing, atau auto-update.
- User-level global configuration.
- Async runtime tanpa kebutuhan Phase 1 yang terbukti.
- Plugin system atau abstraction spekulatif.

Jangan membuat file/modul kosong untuk fitur masa depan. Buat modul ketika fase pemilik fitur tersebut dimulai.

---

## 4. Prinsip arsitektur

Ikuti `ARCHITECTURE.md` dan `AGENTS.md`.

### 4.1 Pemisahan tanggung jawab

- **`main.rs`:** entry point, parsing CLI, memanggil command execution, dan menentukan process exit status. Jaga tetap tipis.
- **CLI definition:** mendeklarasikan command, alias, argumen, options, dan help text.
- **Command execution/application boundary:** menjalankan perilaku command yang benar-benar tersedia.
- **Error module:** mendefinisikan error execution dan memetakannya menjadi diagnostic serta exit code.
- **Output module:** memusatkan kebijakan output.
- **Project module:** menemukan dan merepresentasikan root kandidat project tanpa memuat manifest `rustrs`.

Ini adalah batas logis, bukan kewajiban membuat trait atau file untuk setiap fungsi.

### 4.2 Tidak boleh ada false success

Command masa depan tidak boleh mengembalikan sukses hanya karena namanya dikenal.

Pilihan untuk command yang belum tersedia:

1. Jangan daftarkan command sampai implementasinya tersedia; atau
2. Daftarkan dengan status unavailable hanya jika ada alasan produk yang jelas. Ketika dipanggil, command wajib memberikan pesan jujur dan exit code non-zero.

**Default: jangan daftarkan command placeholder.** Jangan membuat generic success handler atau dummy handlers hanya agar help menampilkan semua fitur dalam PRD.

### 4.3 Public CLI contract

Nama command, alias, flags, help text, output stream, dan exit behavior adalah bagian dari kontrak publik. Perubahannya harus disengaja, diuji, dan didokumentasikan.

---

## 5. Kebijakan dependency

### 5.1 Dependency wajib

Tambahkan `clap` dengan feature `derive` dan versi stabil yang kompatibel dengan MSRV proyek.

Contoh bentuk konfigurasi (ganti placeholder dengan versi yang benar-benar dipilih):

```toml
[dependencies]
clap = { version = "<compatible-stable-version>", features = ["derive"] }
```

Jangan commit literal placeholder tersebut.

### 5.2 Dependency opsional

| Dependency            | Tujuan                                        | Kebijakan                                         |
| --------------------- | --------------------------------------------- | ------------------------------------------------- |
| `thiserror`           | Typed error definitions                       | Opsional; gunakan jika meningkatkan kejelasan     |
| `anyhow`              | Context-rich errors pada application boundary | Opsional; hindari duplikasi error model sederhana |
| `assert_cmd`          | Menjalankan binary dalam integration tests    | Direkomendasikan                                  |
| `predicates`          | Assertions terhadap output proses             | Jika diperlukan bersama pendekatan test           |
| `tempfile`            | Temporary directories                         | Direkomendasikan untuk root discovery tests       |
| `console`             | Terminal styling/capability helpers           | Opsional; coba standard library dahulu            |
| `anstyle` atau setara | Kebijakan warna terminal                      | Opsional; hindari dependency styling ganda        |

Aturan:

- Verifikasi MSRV dan dukungan platform.
- Jangan menambah `dialoguer`, `inquire`, Tera, Serde/TOML, `walkdir`, Tokio, atau progress-bar library hanya karena mungkin dibutuhkan pada fase berikutnya.
- Perbarui `Cargo.lock`.
- Jangan menjalankan perintah melalui shell command string jika process API dengan argumen terpisah memadai.

---

## 6. Task 1.1 — Integrasi Clap dan root command

### Tujuan

Mengganti output statis `main.rs` dengan CLI parser berbasis Clap derive API.

### Requirements

- [ ] Tambahkan Clap dengan feature `derive`.
- [ ] Definisikan root CLI dengan nama binary `rustrs`.
- [ ] Gunakan metadata Cargo untuk package version dan description jika sesuai.
- [ ] Pastikan parser menghasilkan help dan version.
- [ ] Definisikan global options dalam struktur typed.
- [ ] Pisahkan deklarasi command dari logic eksekusi.
- [ ] Jangan menggunakan parsing manual `std::env::args()` untuk penggunaan normal.
- [ ] Argumen invalid ditolak sebelum command execution.
- [ ] Pastikan kompatibel dengan MSRV.
- [ ] Tambahkan tests untuk parser root.

### Expected behavior

```bash
cargo run -- --help
cargo run -- --version
cargo run -- --unknown-option
```

- Help: exit code `0` dan output yang terbaca.
- Version: exit code `0` dan versi dari package metadata.
- Unknown option: exit code non-zero dengan diagnostic yang jelas.

Gunakan formatting help bawaan Clap kecuali ada kebutuhan kuat untuk mengubahnya. Jangan membangun custom help renderer.

### Acceptance criteria

- [ ] `cargo check` sukses.
- [ ] Binary `rustrs --help` berfungsi.
- [ ] Binary `rustrs --version` berfungsi.
- [ ] Version tidak dipelihara manual dalam source constant.
- [ ] Invalid option tidak diteruskan ke command handler.
- [ ] `main.rs` tetap tipis dan tidak berisi generator logic.

---

## 7. Task 1.2 — Struktur command dan kebijakan registrasi

### Tujuan

Menyiapkan struktur command yang dapat diperluas tanpa mengimplementasikan fitur masa depan secara prematur.

### Requirements

- [ ] Gunakan Clap subcommands untuk functionality yang benar-benar tersedia.
- [ ] Tetapkan konvensi command names, aliases, positional arguments, dan flags.
- [ ] Gunakan `generate` sebagai nama kanonis dan `g` sebagai alias ketika command tersebut benar-benar diimplementasikan.
- [ ] Pisahkan command declaration dari business logic.
- [ ] Dokumentasikan kebijakan command yang belum tersedia.
- [ ] Jangan daftarkan semua command PRD hanya agar tampak lengkap.
- [ ] Jika command unavailable sengaja didaftarkan, tampilkan status unavailable secara eksplisit dan exit non-zero saat dipanggil.
- [ ] Jangan membuat plugin/dynamic command registry.

### Keputusan scope

Command surface Phase 1 boleh hanya terdiri dari root help/version dan fitur diagnostik minimal yang disetujui secara eksplisit. Tidak ada kewajiban untuk menampilkan semua command masa depan.

Jika placeholder command memang diperlukan, pesan yang dapat diterima misalnya:

```text
This command is not available in this version of rustrs.
```

Command tersebut wajib exit non-zero. Default-nya tetap tidak mendaftarkan placeholder tanpa alasan user-facing.

### Acceptance criteria

- [ ] Command definitions typed dan mudah ditemukan.
- [ ] Tidak ada command yang berpura-pura berhasil.
- [ ] Help mencerminkan perilaku yang benar-benar ada.
- [ ] Alias diuji ketika diperkenalkan.
- [ ] Struktur dapat diperluas pada Phase 2 tanpa refactor besar.

---

## 8. Task 1.3 — Global options

### Required options

- [ ] `--help`: help untuk root atau subcommand.
- [ ] `--version`: versi paket.
- [ ] `--verbose` / `-v`: diagnostic tambahan.
- [ ] `--quiet` / `-q`: mengurangi output informasional non-esensial.
- [ ] `--no-color`: menonaktifkan warna.

### Optional option

- [ ] `--color <when>` dengan nilai `auto`, `always`, dan `never`, hanya jika dapat hidup berdampingan secara jelas dengan `--no-color`.

### Rules

- `--verbose` tidak boleh mencetak secret, environment dump, atau informasi internal yang tidak relevan.
- Tentukan prioritas `--quiet` dan `--verbose`. Rekomendasi: tolak kombinasi ini dengan pesan argumen yang jelas, kecuali ada prioritas lain yang terdokumentasi.
- `--no-color` harus mengalahkan deteksi warna otomatis.
- Hindari flags yang redundan atau saling bertentangan.
- Uji posisi global options sebelum dan sesudah subcommand sesuai kontrak yang dipilih.
- Jangan menambah `--dry-run`, `--force`, atau flags fitur masa depan sebelum semantiknya ditetapkan oleh fase pemilik.

### Acceptance criteria

- [ ] Perilaku options terdokumentasi.
- [ ] Kombinasi invalid ditolak atau memiliki perilaku deterministik.
- [ ] No-color tidak menghasilkan ANSI styling dari `rustrs`.
- [ ] Quiet mode tidak menekan error.
- [ ] Verbose mode menambah konteks yang bermanfaat tanpa membocorkan secret.

---

## 9. Task 1.4 — Error handling dan exit codes

### Tujuan

Menyediakan cara konsisten untuk mengubah execution failure menjadi diagnostic dan process exit status.

### Error categories

Gunakan hanya kategori yang benar-benar memiliki caller di Phase 1. Kategori yang direncanakan untuk pertumbuhan CLI:

- Input atau argumen tidak valid.
- Project root yang dibutuhkan tidak ditemukan.
- Project context tidak valid.
- Konflik project/filesystem.
- Executable eksternal yang dibutuhkan tidak tersedia.
- Operasi eksternal gagal.
- Error internal/tidak terduga.

Jangan membuat semua future error variants sebagai placeholder yang tidak digunakan.

### Exit-code policy

| Code | Arti                                      |
| ---: | ----------------------------------------- |
|  `0` | Operasi selesai dengan sukses             |
|  `1` | Kegagalan umum/tidak terduga              |
|  `2` | Input atau konfigurasi invalid            |
|  `3` | Konflik project/filesystem                |
|  `4` | Executable eksternal wajib tidak tersedia |
|  `5` | Operasi eksternal gagal                   |

Clap boleh mempertahankan perilaku exit standar untuk parsing error, help, dan version. Jangan menimpa perilaku bawaan Clap tanpa kebutuhan.

### Requirements

- [ ] Definisikan error type atau strategi error kecil sesuai kebutuhan sekarang.
- [ ] Pisahkan parser errors dari execution errors.
- [ ] Pusatkan mapping application error ke exit code.
- [ ] Cetak error ke stderr.
- [ ] Tempatkan output hasil normal di stdout.
- [ ] Berikan pesan actionable tanpa membocorkan detail internal atau secret.
- [ ] Hindari panic untuk kegagalan yang diharapkan.
- [ ] Pertahankan underlying error context yang berguna untuk verbose mode jika tersedia.
- [ ] Jangan mencetak backtrace secara default.
- [ ] Unavailable command harus exit non-zero.

### Acceptance criteria

- [ ] Unit tests menguji setiap mapping yang diimplementasikan.
- [ ] Integration tests memverifikasi process status non-zero pada failure.
- [ ] Error dikirim ke stderr.
- [ ] Tidak ada failed command yang menghasilkan output seolah sukses.
- [ ] Exit-code policy terdokumentasi.

---

## 10. Task 1.5 — Output terminal terpusat

### Tujuan

Membuat output konsisten dan testable tanpa mengikat styling ke logic command.

### Requirements

- [ ] Buat modul output kecil untuk kebijakan bersama.
- [ ] Tentukan penggunaan stdout dan stderr.
- [ ] Tetapkan konvensi pesan informasi, warning, dan error.
- [ ] Hormati `--quiet`, `--verbose`, dan `--no-color`.
- [ ] Pastikan output tetap jelas tanpa warna.
- [ ] Hindari progress bar atau spinner yang tidak dibutuhkan pada fase ini.
- [ ] Hindari dekorasi yang membuat tests rapuh.
- [ ] Jangan membangun logging framework kecuali ada requirement nyata.

### Perilaku yang disarankan

- **stdout:** hasil command normal serta help/version sesuai perilaku Clap.
- **stderr:** error dan diagnostic warning.
- **quiet:** menekan informasi opsional, tetapi tidak pernah menekan error.
- **verbose:** menambahkan konteks diagnosis yang berguna.
- **no-color:** menonaktifkan ANSI styling dari aplikasi.

### Acceptance criteria

- [ ] Output policy cukup terpusat untuk menghindari implementasi ad hoc.
- [ ] Quiet mode tetap menampilkan error.
- [ ] No-color dapat diuji.
- [ ] Output tidak berisi secret atau environment dump.
- [ ] Tests fokus pada isi penting, bukan whitespace dekoratif kecuali bagian kontrak.

---

## 11. Task 1.6 — Project-root discovery

### Tujuan

Membuat utilitas reusable untuk menemukan root kandidat Cargo project bagi command yang memerlukan konteks project.

### Definisi Phase 1

Root kandidat adalah direktori yang berisi `Cargo.toml`. Utilitas ini belum membuktikan bahwa project dikelola oleh `rustrs`. Validasi `.rustrs/config.toml` menjadi bagian Phase 2.

### Requirements

- [ ] Buat modul terpisah, misalnya `src/project/root.rs`.
- [ ] Fungsi inti menerima starting directory eksplisit agar mudah diuji.
- [ ] Telusuri starting directory ke parent sampai marker ditemukan atau filesystem root tercapai.
- [ ] Kembalikan root sebagai `PathBuf` atau tipe owned path yang sesuai.
- [ ] Tetapkan kebijakan relative path dan canonicalization.
- [ ] Path yang tidak ada atau merupakan file harus menghasilkan error yang jelas.
- [ ] Jangan mengubah current working directory proses.
- [ ] Jangan membaca/menulis `.rustrs/config.toml`.
- [ ] Jangan menganggap setiap direktori sebagai project root.
- [ ] Tetap synchronous; jangan menambah Tokio.
- [ ] Jangan melakukan traversal ke semua descendant; cukup periksa starting directory dan parent-nya.

### Panduan API

Contoh bentuk API:

```rust
fn discover_project_root(start: &Path) -> Result<PathBuf, ProjectRootError>
```

Nama dan tipe tepatnya boleh disesuaikan dengan konvensi repository.

### Required tests

- [ ] Starting directory berisi `Cargo.toml`.
- [ ] Nested directory menemukan ancestor `Cargo.toml`.
- [ ] Tidak ada `Cargo.toml` pada ancestor chain.
- [ ] Starting path tidak ada.
- [ ] Starting path adalah file.
- [ ] Pencarian berhenti pada filesystem root.
- [ ] Relative path sesuai kebijakan yang didokumentasikan.
- [ ] Temporary files/directories terisolasi dan dibersihkan.

### Acceptance criteria

- [ ] Root yang ditemukan benar untuk kasus valid.
- [ ] Kasus invalid menghasilkan error, bukan panic.
- [ ] Help/version tidak memanggil root discovery.
- [ ] Discovery tidak mengubah filesystem.
- [ ] Dokumentasi menyebutnya candidate Cargo root detection, bukan validasi manifest `rustrs`.

---

## 12. Task 1.7 — Struktur source code awal

### Struktur yang direkomendasikan

```text
src/
├── main.rs
├── cli/
│   ├── mod.rs
│   ├── commands.rs
│   └── output.rs
├── error.rs
└── project/
    ├── mod.rs
    └── root.rs
tests/
└── cli.rs
```

Nama file dapat disesuaikan dengan repository yang sudah ada, selama tanggung jawab tetap jelas.

### Requirements

- [ ] Jaga `main.rs` tetap tipis.
- [ ] Letakkan deklarasi Clap dalam modul CLI.
- [ ] Pisahkan parser declaration dari execution/error mapping jika membantu kejelasan.
- [ ] Pisahkan output policy dari command-specific logic.
- [ ] Root discovery independen dari Clap.
- [ ] Jangan membuat modul kosong `application`, `domain`, `config`, `templates`, `filesystem`, atau `diagnostics`.
- [ ] Jangan membuat trait hanya untuk mensimulasikan dependency injection tanpa kebutuhan nyata.
- [ ] Jangan membuat workspace atau multi-crate architecture.

### Acceptance criteria

- [ ] Tanggung jawab modul mudah dipahami.
- [ ] Struktur mudah diperluas pada Phase 2.
- [ ] Tidak ada modul spekulatif yang tidak digunakan.
- [ ] `ARCHITECTURE.md` diperbarui jika ada keputusan jangka panjang yang berubah.

---

## 13. Task 1.8 — Automated tests

### Integration tests

Minimal tes berikut harus tersedia:

- [ ] Root help mengembalikan exit code `0`.
- [ ] Help memuat nama aplikasi dan usage yang relevan.
- [ ] Version mengembalikan exit code `0`.
- [ ] Version sesuai package version.
- [ ] Unknown argument menghasilkan non-zero.
- [ ] Missing required argument untuk command yang benar-benar tersedia menghasilkan error dan help yang berguna.
- [ ] Help/version bekerja dari temporary directory di luar Cargo project.
- [ ] Jika ada unavailable command, invocation-nya non-zero dan menyatakan status unavailable.
- [ ] Error execution ditulis ke stderr.
- [ ] Invocation dasar tidak membuat file atau direktori.

### Unit tests

- [ ] Mapping application error ke exit code.
- [ ] Root discovery pada project root.
- [ ] Root discovery pada nested directory.
- [ ] Root discovery gagal jika marker tidak ada.
- [ ] Invalid/nonexistent start paths.
- [ ] Logic khusus untuk verbosity/color jika ada di luar parsing Clap.

### Panduan testing

- Gunakan `assert_cmd` dan `predicates` jika memudahkan process-level tests.
- Gunakan `tempfile` untuk fixture direktori sementara.
- Jangan membutuhkan internet, Docker, database, atau external application.
- Jangan mengunci setiap whitespace help Clap kecuali formatting persis adalah kontrak.
- Tes success dan failure path.
- Jangan menggunakan project pengguna yang sebenarnya.
- Jangan menghapus atau melemahkan test hanya agar lulus.
- Jangan menambahkan test semata-mata untuk menaikkan jumlah test.

---

## 14. Task 1.9 — Dokumentasi

### README

Perbarui `README.md` agar menjelaskan:

- Cara build dan menjalankan CLI lokal.
- Cara menggunakan `--help` dan `--version`.
- Fitur apa yang saat ini benar-benar tersedia.
- Project scaffolding dan code generation belum tersedia hingga fase terkait selesai.
- Cara menjalankan quality checks.

Jangan mendokumentasikan command yang direncanakan seolah sudah tersedia.

### Architecture

Perbarui `ARCHITECTURE.md` jika Phase 1 menetapkan atau mengubah keputusan permanen terkait:

- CLI module layout.
- Semantik global options.
- Error dan exit-code policy.
- Root discovery.
- Output conventions.

Jangan menyalin seluruh roadmap ke architecture document.

### Phase status

Setelah implementasi, update status dokumen atau tambahkan completion report singkat sesuai konvensi repository. Bedakan requirements yang direncanakan dari bukti implementasi.

---

## 15. Quality gates dan verifikasi

Jalankan semua baseline checks dari root repository:

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Lalu build dan smoke-test binary:

```bash
cargo build
cargo run -- --help
cargo run -- --version
```

Jika memungkinkan, uji binary dari temporary directory yang berada di luar repository.

### Aturan verifikasi

- Jalankan command; jangan menyimpulkan hasil berdasarkan perkiraan.
- Jika command gagal, investigasi dan perbaiki, lalu jalankan ulang checks terkait.
- Jika tidak dapat dijalankan karena environment, tandai `BLOCKED` beserta alasannya.
- Pisahkan baseline failure dari regression.
- Jangan menandai Phase 1 `COMPLETE` jika quality gate wajib gagal tanpa pengecualian eksplisit dari pemilik proyek.
- Review `git diff` dan `git status --short`.
- Jangan commit, push, publish, atau deploy kecuali diminta.

---

## 16. Acceptance checklist

Phase 1 dinyatakan selesai hanya jika semua kriteria yang berlaku telah diverifikasi.

### CLI foundation

- [ ] Clap derive API terintegrasi.
- [ ] Root help berfungsi.
- [ ] Version berfungsi dan menggunakan package metadata.
- [ ] Invalid options/arguments gagal secara konsisten.
- [ ] Command declarations terpisah dari feature execution logic.
- [ ] Tidak ada command terdaftar yang berpura-pura sukses.

### Global behavior

- [ ] Perilaku `--verbose` didefinisikan dan diuji jika diimplementasikan.
- [ ] Perilaku `--quiet` didefinisikan dan diuji jika diimplementasikan.
- [ ] Perilaku `--no-color` didefinisikan dan diuji.
- [ ] Error dikirim ke stderr.
- [ ] Execution failure menghasilkan non-zero exit code.
- [ ] Error/exit-code policy terdokumentasi.

### Project-root discovery

- [ ] Candidate Cargo root detection berfungsi.
- [ ] Nested directory discovery berfungsi.
- [ ] Missing root dan invalid paths menghasilkan error yang jelas.
- [ ] Root discovery tidak mengubah filesystem.
- [ ] Help/version bekerja tanpa project root.

### Tests and docs

- [ ] CLI integration tests mencakup help, version, invalid arguments.
- [ ] Root discovery tests mencakup success dan failure.
- [ ] README sesuai implementasi.
- [ ] Architecture docs diperbarui bila diperlukan.
- [ ] Tidak ada fitur Phase 2+ yang tidak sengaja masuk.
- [ ] `cargo check` lulus.
- [ ] `cargo test` lulus.
- [ ] `cargo fmt --check` lulus.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` lulus.
- [ ] Final diff dan repository status telah direview.

---

## 17. Format laporan akhir

AI agent wajib memberikan laporan dalam bahasa Indonesia dengan format berikut:

```text
Phase: 1 — CLI Foundation
Status: COMPLETE | PARTIAL | BLOCKED

Summary:
- ...

Files changed:
- <path>: <purpose>

Implemented:
- ...

Tests:
- ...

Verification:
- cargo metadata --no-deps --format-version 1: PASS | FAIL | BLOCKED
- cargo check: PASS | FAIL | BLOCKED
- cargo test: PASS | FAIL | BLOCKED
- cargo fmt --check: PASS | FAIL | BLOCKED
- cargo clippy --all-targets --all-features -- -D warnings: PASS | FAIL | BLOCKED
- CLI smoke tests: PASS | FAIL | BLOCKED

Deferred / out of scope:
- ...

Known issues / decisions needed:
- ...

Next phase:
- Phase 2 — Manifest and Compatibility
```

Gunakan `COMPLETE` hanya jika semua acceptance criteria yang berlaku lulus. Gunakan `PARTIAL` jika sebagian pekerjaan selesai tetapi masih ada kriteria yang belum terpenuhi. Gunakan `BLOCKED` jika hambatan mencegah penyelesaian. Jangan mulai Phase 2 otomatis.

---

## 18. Urutan implementasi yang direkomendasikan

Kerjakan dalam urutan berikut untuk mengurangi rework:

1. **Preflight dan baseline:** inspeksi repository, baca dokumen, jalankan checks awal.
2. **Clap root command:** integrasi derive API, help, version, invalid argument handling.
3. **Command structure:** tetapkan pola deklarasi/dispatch tanpa dummy handler.
4. **Errors dan exit behavior:** definisikan error execution, diagnostics, mapping exit code.
5. **Global output policy:** implementasikan verbosity, quiet, dan no-color dengan tests.
6. **Project-root discovery:** implementasikan utilitas dan temporary-directory tests.
7. **Integration tests:** uji binary nyata dan kasus invalid.
8. **Dokumentasi:** update README dan architecture decisions.
9. **Quality gates dan review:** jalankan semua checks, periksa diff/status, laporkan bukti.

Implementasikan secara incremental, compile/test setelah perubahan bermakna, dan tetap dalam scope Phase 1.

---

## 19. Prinsip akhir

**Bangun fondasi CLI yang kecil, nyata, dan teruji. Jangan mengimplementasikan fitur masa depan, membuat abstraksi spekulatif, atau melaporkan sukses untuk perilaku yang belum ada.**

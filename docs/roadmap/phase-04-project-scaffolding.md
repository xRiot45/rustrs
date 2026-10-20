# Phase 4 — Project Scaffolding

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 4 of 10  
> **Status:** Planned  
> **Audience:** AI coding agents and maintainers  
> **Primary outcome:** Implement a real, safe, testable `rustrs new` workflow that creates a Rust REST API starter project from built-in templates and writes a valid `.rustrs/config.toml`.  
> **Documentation:** Bahasa Indonesia; use idiomatic English for Rust identifiers and source-code comments.  
> **Scope rule:** Implement project creation only. Do not implement post-scaffolding code generators, inspector commands, database workflows, or unrelated product features.

---

## 1. Tujuan dan hasil akhir

Phase 4 menghubungkan fondasi Phase 1–3 menjadi alur produk pertama yang dapat digunakan: membuat project Rust REST API baru melalui `rustrs new`.

Gunakan kembali:

- CLI parsing, output policy, error handling, dan exit codes dari Phase 1.
- Manifest model, validation, compatibility resolver, dan capability rules dari Phase 2.
- Template resolution/rendering, generation plan, path validation, conflict detection, filesystem writer, dry-run, dan operation report dari Phase 3.

**Jangan membangun ulang fondasi tersebut.** Jika API yang tersedia belum memadai, lakukan perubahan sekecil mungkin, jelaskan alasannya, dan tambahkan tests.

Pada akhir fase:

- `rustrs new <project-name>` benar-benar menjalankan scaffolding.
- Input dapat berasal dari flags, defaults terdokumentasi, atau wizard interaktif.
- Konfigurasi divalidasi sebelum filesystem target dimodifikasi.
- Kombinasi teknologi unsupported ditolak sebelum write.
- Starter template yang dinyatakan tersedia memiliki asset dan struktur konsisten.
- Project dibuat melalui generation plan dan filesystem engine Phase 3.
- Project memiliki `Cargo.toml`, source starter, README, file pendukung yang relevan, dan `.rustrs/config.toml`.
- Manifest dapat dibaca dan divalidasi ulang.
- `--dry-run` tidak memodifikasi filesystem target.
- File existing tidak ditimpa diam-diam.
- Report dan exit code mencerminkan hasil aktual.
- Automated tests memverifikasi sukses, input invalid, konflik, dry-run, dan failure behavior.
- Dokumentasi sesuai implementasi aktual.

Keberhasilan diukur dari alur `rustrs new` yang dapat dipercaya dan output project yang valid, bukan dari banyaknya flags atau template placeholder.

## 2. Sumber kebenaran dan dokumen wajib

Sebelum mengubah source code, baca:

1. `PRD.md`
2. `ARCHITECTURE.md`
3. `AGENTS.md`
4. `README.md`
5. `docs/roadmap/phase-01-cli-foundation.md`
6. `docs/roadmap/phase-02-manifest-and-compatibility.md`
7. `docs/roadmap/phase-03-template-and-filesystem-engine.md`
8. Source code aktual pada `src/cli`, `src/error.rs`, `src/project`, `src/manifest`, `src/compatibility`, `src/template`, `src/generation`, dan `src/filesystem`
9. `Cargo.toml`, `Cargo.lock`, tests, fixtures, dan CI workflows yang relevan.

Jangan mengklaim membaca dokumen yang tidak tersedia. Jika ada konflik spesifikasi yang tidak dapat diselesaikan dengan aman, laporkan dan minta klarifikasi. Periksa implementasi aktual; jangan mengasumsikan laporan fase sebelumnya selalu mutakhir.

## 3. Task 4.0 — Preflight dan audit Phase 3

### Tujuan

Memastikan fondasi Phase 3 dapat dipakai dan mengidentifikasi blocker sebelum scaffolding dibuat.

### Langkah kerja

- [ ] Jalankan `git status --short`, `git branch --show-current`, `git log -5 --oneline`.
- [ ] Jalankan `rustc --version` dan `cargo --version`.
- [ ] Inspeksi struktur repository dan perubahan lokal.
- [ ] Baca `Cargo.toml`/`Cargo.lock`; identifikasi dependency yang sudah tersedia.
- [ ] Review CLI, error/exit-code mapping, project/path utilities, manifest, compatibility, template engine, planner, path validator, conflict policy, writer, dry-run, dan report.
- [ ] Jalankan baseline:

  ```bash
  cargo metadata --no-deps --format-version 1
  cargo check
  cargo test
  cargo fmt --check
  cargo clippy --all-targets --all-features -- -D warnings
  git diff --check
  ```

- [ ] Catat baseline failures terpisah dari regression baru.
- [ ] Identifikasi API Phase 2/3 yang dapat digunakan; jangan membuat duplikat.
- [ ] Buat checklist perubahan sebelum mengedit kode.

### Acceptance criteria

- [ ] Git state dan perubahan lokal diperiksa.
- [ ] Dokumen dan modul terkait ditinjau.
- [ ] Baseline dicatat.
- [ ] Gap yang menghalangi scaffolding diketahui.
- [ ] Tidak ada perubahan pengguna yang hilang.

## 4. Batas scope

### Termasuk Phase 4

- Command `rustrs new` dan execution path nyata.
- Flags, defaults, wizard interaktif, dan non-interactive mode.
- Validasi nama project dan target directory.
- Resolusi konfigurasi final.
- Compatibility/capability preflight melalui Phase 2.
- Built-in starter template assets yang diperlukan.
- Integrasi template rendering, planning, dry-run, preview, confirmation, dan apply melalui Phase 3.
- Pembuatan file starter dan `.rustrs/config.toml`.
- Optional Git initialization hanya jika secara eksplisit diputuskan dan diuji.
- Report, diagnostics, next steps, smoke verification, tests, dan dokumentasi.

### Tidak termasuk Phase 4

- Generator `module`, `resource`, `controller`, `service`, `repository`, `entity`, `dto`, `middleware`, `guard`, `validator`, `job`, `event`, `handler`, `test`, atau `migration` untuk project yang sudah dibuat (Phase 5).
- Inspector `info`, `routes`, `modules`, `deps` (Phase 6).
- Database commands, migration execution, seeding, rollback, atau database lifecycle (Phase 7).
- Workflow development process management dan Docker orchestration di luar starter files yang dipilih template.
- Implementasi penuh capability auth, RBAC, Redis, email, storage, audit logging, rate limiting, idempotency, observability, Docker, atau CI/CD yang belum tersedia.
- Remote template registry, marketplace, plugin system, remote fetching, auto-update, publishing, release, atau deployment.
- Refactor besar yang tidak diperlukan.
- Command placeholder yang mengembalikan sukses.
- Commit, push, publish, atau release tanpa instruksi eksplisit.

**Starter template bukan code generator.** Phase 4 membuat baseline project yang dapat dibangun. Penambahan resource/controller/service/repository setelah project dibuat tetap Phase 5.

## 5. Prinsip arsitektur

Alur konseptual:

```text
CLI input
  -> input resolution
  -> name/path validation
  -> validated configuration
  -> compatibility preflight
  -> template resolution and asset selection
  -> context + rendering
  -> generation plan
  -> path/conflict validation
  -> dry-run preview or confirmation
  -> filesystem writer
  -> manifest read-back + smoke verification
  -> final report
```

Batas tanggung jawab:

- **CLI:** flags, arguments, help, parsing.
- **Scaffolding service:** orkestrasi workflow `new`.
- **Input resolver:** menggabungkan flags, defaults, dan wizard.
- **Manifest/compatibility:** validasi konfigurasi dan kombinasi.
- **Template layer:** asset selection, context, rendering.
- **Generation layer:** menyusun plan sebelum apply.
- **Filesystem layer:** validasi path/konflik dan apply.
- **Output/report:** diagnostic dan ringkasan.

Jangan membuat compatibility matrix, manifest model, renderer, atau writer kedua. Jangan menulis file langsung dari wizard, renderer, atau CLI handler. `src/main.rs` tetap tipis. Nama modul mengikuti codebase aktual; jangan membuat modul kosong atau abstraction spekulatif.

## 6. Task 4.1 — CLI contract `rustrs new`

### Tujuan

Menetapkan public CLI contract yang eksplisit dan bisa diuji.

Command utama:

```bash
rustrs new <project-name>
rustrs new ecommerce-api --template standard
rustrs new ecommerce-api --framework axum --database postgres --orm sqlx
rustrs new ecommerce-api --non-interactive --defaults
rustrs new ecommerce-api --dry-run
```

Contoh tersebut adalah target UX; pilihan hanya boleh ditawarkan sebagai supported jika implementasi dan compatibility resolver menyatakannya supported.

### Persyaratan

- [ ] Tambahkan `new` ke Clap mengikuti pola yang ada.
- [ ] Gunakan typed domain enums untuk template/framework/database/data-access jika memungkinkan.
- [ ] Definisikan flags minimum: template, framework, database, ORM, non-interactive, defaults, dry-run, dan target path jika diperlukan.
- [ ] Tentukan perilaku flags yang bertentangan.
- [ ] Tentukan aturan target path dan confirmation tanpa ambiguitas.
- [ ] Pertahankan global options Phase 1 (`--verbose`, `--quiet`, `--no-color`).
- [ ] Help hanya menampilkan opsi yang benar-benar tersedia.
- [ ] Jangan menambahkan command fase berikutnya.
- [ ] Jangan menambahkan `--force` kecuali semantik overwrite didefinisikan dan diuji.
- [ ] Jangan menganggap `--defaults` sebagai izin overwrite.
- [ ] Jangan mengembalikan sukses jika workflow tidak dijalankan.

### Acceptance criteria

- [ ] `rustrs new --help` akurat.
- [ ] Input valid diparsing ke tipe yang benar.
- [ ] Input invalid menghasilkan diagnostic dan exit code non-zero.
- [ ] Command menuju application service nyata.
- [ ] CLI integration tests tersedia.

## 7. Task 4.2 — Input resolution dan default configuration

### Tujuan

Menghasilkan satu konfigurasi final deterministik sebelum template resolution.

### Persyaratan

- [ ] Pisahkan raw CLI input dari konfigurasi final tervalidasi.
- [ ] Gabungkan flags, wizard answers, dan documented defaults.
- [ ] Nilai eksplisit tidak boleh ditimpa default atau wizard.
- [ ] Bedakan field yang belum dipilih dengan pilihan eksplisit seperti database `none`.
- [ ] Terapkan default hanya pada field yang telah memiliki keputusan produk.
- [ ] Jangan menganggap `Features::default()` otomatis merupakan konfigurasi produk yang benar.
- [ ] Default tidak boleh mengaktifkan capability yang belum diimplementasikan.
- [ ] `--defaults` harus deterministik dan tidak memerlukan prompt.
- [ ] Input wajib yang hilang pada non-interactive mode menghasilkan error actionable.
- [ ] Resolver dapat diuji tanpa terminal dan tanpa filesystem writes.

### Acceptance criteria

- [ ] Prioritas input terdokumentasi dan diuji.
- [ ] Input sama menghasilkan konfigurasi sama.
- [ ] Explicit `none` tidak tertukar dengan missing value.
- [ ] Missing required input gagal sebelum rendering/apply.
- [ ] Tidak ada silent fallback.

## 8. Task 4.3 — Interactive wizard dan non-interactive mode

### Wizard

- [ ] Periksa dependency yang ada; tambahkan paling banyak satu prompt library jika benar-benar diperlukan.
- [ ] Wizard memakai input resolver yang sama dengan non-interactive mode.
- [ ] Hanya tawarkan pilihan yang benar-benar didukung.
- [ ] Lewati pertanyaan yang tidak relevan, misalnya ORM jika database `none`.
- [ ] Tampilkan ringkasan final sebelum apply.
- [ ] Pembatalan, EOF, dan input invalid ditangani dengan jelas.
- [ ] Cancel sebelum apply tidak membuat target directory atau file.
- [ ] Jangan memulai prompt jika stdin bukan terminal interaktif.
- [ ] Jangan menaruh generation logic di callback wizard.

### Non-interactive

- [ ] Tidak menunggu input.
- [ ] Semua input wajib dapat diberikan lewat flags/defaults terdokumentasi.
- [ ] Incomplete config menghasilkan error dengan contoh perintah.
- [ ] Tidak menggunakan ANSI/terminal UI sebagai prasyarat.
- [ ] Dapat digunakan pada CI.
- [ ] Dry-run tidak memerlukan confirmation interaktif.
- [ ] Defaults tidak melewati conflict policy.

### Acceptance criteria

- [ ] Input sama menghasilkan konfigurasi sama di kedua mode.
- [ ] Tests tidak hang.
- [ ] Cancel tidak memodifikasi filesystem.
- [ ] Tidak ada prompt yang tidak relevan.

## 9. Task 4.4 — Validasi nama project dan target directory

### Persyaratan nama

- [ ] Validasi sesuai aturan Cargo package name dan kebijakan yang terdokumentasi.
- [ ] Tetapkan aturan huruf besar, underscore/hyphen, angka, reserved names, dan karakter invalid.
- [ ] Jika nama dinormalisasi, tampilkan nama final sebelum apply.
- [ ] Jangan mengubah identitas nama diam-diam.
- [ ] Dokumentasikan hubungan nama directory dan package.

### Persyaratan path

- [ ] Tentukan target root secara eksplisit.
- [ ] Validasi parent dan target directory.
- [ ] Tangani target belum ada, direktori kosong, direktori berisi file, file biasa, dan permission errors.
- [ ] Gunakan root-containment policy Phase 3.
- [ ] Tolak path traversal dan symlink escape.
- [ ] Jangan menghapus target yang ada.
- [ ] Default tidak menimpa file existing.
- [ ] Writer harus memeriksa ulang kondisi kritis sebelum write.

### Acceptance criteria

- [ ] Invalid name/path gagal sebelum apply.
- [ ] Conflict sesuai policy dan tidak merusak file pengguna.
- [ ] Tests memakai temporary directories.

## 10. Task 4.5 — Compatibility dan capability preflight

### Persyaratan

- [ ] Gunakan model dan validator manifest Phase 2.
- [ ] Gunakan compatibility resolver sebagai satu-satunya sumber kebenaran.
- [ ] Validasi template, framework, database, data-access, architecture, dan capability relevan.
- [ ] Bedakan capability yang dikenal schema dari capability yang benar-benar diimplementasikan.
- [ ] Unsupported combinations ditolak sebelum write.
- [ ] Jangan mengganti pilihan pengguna secara otomatis.
- [ ] Berikan alasan dan alternatif yang benar-benar valid jika tersedia.
- [ ] Jangan menganggap enum yang ada otomatis berarti kombinasi didukung.
- [ ] Jangan memperluas support matrix diam-diam.
- [ ] Jangan menghasilkan project setengah valid jika template memerlukan kombinasi unsupported.

Pada snapshot repository sebelum Phase 4, matrix diketahui mendukung Axum + PostgreSQL + SQLx. Agent wajib memeriksa implementasi aktual saat mulai bekerja. Fokuslah pada kombinasi yang benar-benar supported; perluasan matrix hanya dilakukan jika sesuai scope dan disertai scaffold serta tests nyata.

### Acceptance criteria

- [ ] Kombinasi supported diterima.
- [ ] Unsupported ditolak sebelum write.
- [ ] Tidak ada silent fallback atau capability palsu.
- [ ] Tests mencakup kombinasi relevan.

## 11. Task 4.6 — Built-in starter template assets

### Tujuan

Menyediakan asset starter yang benar-benar dapat digunakan oleh `rustrs new`, menggunakan engine Phase 3.

### Persyaratan

- [ ] Inspeksi metadata/asset conventions Phase 3; jangan membuat loader paralel.
- [ ] Template bawaan tersedia offline.
- [ ] Metadata dan asset konsisten.
- [ ] Sediakan baseline starter untuk setiap template yang dinyatakan available.
- [ ] Struktur sesuai tujuan:
  - **Enterprise:** struktur modular lebih terpisah; jangan mengklaim capability enterprise yang belum dibuat.
  - **Standard:** struktur modular seimbang.
  - **Simple:** abstraksi lebih sedikit.
  - **Minimal:** starter sekecil mungkin yang tetap valid dan buildable.
- [ ] Jangan mengklaim template tersedia jika asset belum lengkap.
- [ ] Jangan memaksa Simple/Minimal menggunakan struktur Enterprise.
- [ ] Context placeholders eksplisit dan tidak tertinggal di output.
- [ ] Conditional assets hanya untuk fitur yang benar-benar diimplementasikan.
- [ ] Semua target paths relatif dan aman.
- [ ] Tidak ada secret atau credential nyata.
- [ ] Output deterministik.

File starter dapat mencakup `Cargo.toml`, `src/main.rs`, source minimum framework, `.gitignore`, `README.md`, `.env.example` tanpa secret, dan `.rustrs/config.toml`. Daftar final mengikuti kebutuhan template; jangan menambah file yang tidak diperlukan.

### Buildability

- [ ] Cargo manifest valid dan package name valid.
- [ ] Source entry point/module declarations konsisten.
- [ ] Dependency sesuai kombinasi teknologi.
- [ ] Feature-disabled tidak meninggalkan reference yang rusak.
- [ ] Kombinasi yang dinyatakan supported dapat lolos `cargo check` ketika environment mengizinkan dependency tersedia.
- [ ] Starter tidak bergantung pada generator Phase 5.

### Acceptance criteria

- [ ] Template available mempunyai asset lengkap.
- [ ] Output structure/content dites.
- [ ] Tidak ada unresolved placeholders.
- [ ] Tidak ada klaim capability palsu.
- [ ] Buildability dibuktikan untuk kombinasi supported.

## 12. Task 4.7 — Integrasi rendering dan generation plan

### Persyaratan

- [ ] Scaffolding service menerima konfigurasi final yang tervalidasi.
- [ ] Template resolver memilih asset yang tepat.
- [ ] Context berasal dari konfigurasi final.
- [ ] Renderer menghasilkan konten di memori.
- [ ] Planner menyusun seluruh file/dir operations sebelum apply.
- [ ] Plan mencakup starter files dan manifest.
- [ ] Semua path dan konflik divalidasi sebelum writer dipanggil.
- [ ] Rendering/planning failure tidak menyebabkan target writes.
- [ ] Plan yang sama digunakan untuk preview dan apply.
- [ ] Tidak ada direct file writes dari CLI, wizard, atau renderer.

### Acceptance criteria

- [ ] Konfigurasi valid menghasilkan plan konsisten.
- [ ] Invalid input gagal sebelum apply.
- [ ] Planning tidak memodifikasi filesystem.
- [ ] Plan berisi semua file wajib dan path valid.

## 13. Task 4.8 — Dry-run, preview, confirmation, dan apply

### Dry-run

- [ ] Menggunakan pipeline dan plan yang sama dengan apply.
- [ ] Tidak membuat target directory, menulis/menghapus file, atau membuat temporary file di target.
- [ ] Menampilkan relative path, operation type, dan known conflicts.
- [ ] Exit status menunjukkan invalid plan atau fatal conflict.

### Confirmation

- [ ] Interactive mode menampilkan ringkasan sebelum apply.
- [ ] Confirmation dilakukan sebelum file pertama ditulis.
- [ ] Cancel tidak meninggalkan project parsial.
- [ ] Non-interactive mode tidak meminta prompt.
- [ ] Defaults tidak dianggap sebagai izin overwrite.

### Apply

- [ ] Hanya plan tervalidasi yang boleh diterapkan.
- [ ] Gunakan writer Phase 3.
- [ ] Writer memeriksa ulang kondisi target yang kritis.
- [ ] Partial failures dilaporkan dengan operation report aktual.
- [ ] Jangan mengklaim multi-file atomic transaction jika tidak didukung.
- [ ] Jangan menghapus target sebagai recovery.
- [ ] Berikan diagnostic dan recovery guidance aman jika apply gagal.

### Acceptance criteria

- [ ] Dry-run terbukti tidak mengubah filesystem.
- [ ] Preview dan apply menggunakan plan yang sama.
- [ ] Conflict dan partial failure ditangani dengan jujur.

## 14. Task 4.9 — Manifest `.rustrs/config.toml`

### Persyaratan

- [ ] Gunakan `ProjectManifest` dan schema version Phase 2.
- [ ] Manifest berasal dari konfigurasi final, bukan raw CLI input.
- [ ] Template/framework/architecture/database/data-access/features konsisten dengan project output.
- [ ] Pilih satu sumber kebenaran untuk manifest: serializer atau template asset; jangan punya dua implementasi yang dapat menyimpang.
- [ ] Serialisasi deterministik bila manifest dibuat oleh serializer.
- [ ] Manifest masuk ke generation plan dan ditulis melalui filesystem engine.
- [ ] Tidak menyimpan secrets runtime atau placeholder secret yang tampak valid.
- [ ] Baca ulang manifest setelah apply menggunakan parser/validator Phase 2.
- [ ] Jika read-back gagal, laporkan verification failure.
- [ ] Jangan menambah manifest migration yang tidak diperlukan.

### Acceptance criteria

- [ ] Manifest valid dan bisa dibaca ulang.
- [ ] Manifest cocok dengan konfigurasi final.
- [ ] Tidak ada secret.
- [ ] Tests mencakup round-trip yang relevan.

## 15. Task 4.10 — Optional Git initialization

Task ini boleh ditiadakan. Jika tidak diimplementasikan, jangan tampilkan opsi Git pada CLI.

### Jika diimplementasikan

- [ ] Tentukan default secara eksplisit; jangan menjalankan Git diam-diam tanpa keputusan produk.
- [ ] Gunakan flag yang konsisten dan tidak ambigu.
- [ ] Periksa executable Git sebelum digunakan.
- [ ] Gunakan `std::process::Command` dengan argumen terpisah, bukan shell string.
- [ ] Working directory harus target project yang tervalidasi.
- [ ] Jangan membuat remote, commit, atau push otomatis.
- [ ] Tangani Git failure terpisah dari file generation dan dokumentasikan dampaknya pada exit status.
- [ ] Tests menggunakan temporary directory/fake executable jika memungkinkan.
- [ ] Jangan menyentuh parent directory atau repository pengguna.

### Acceptance criteria

- [ ] Perilaku Git eksplisit dan teruji.
- [ ] Git missing/failure tidak menghasilkan false success.
- [ ] Tidak ada operasi di luar target.

## 16. Task 4.11 — Report, diagnostics, dan next steps

### Persyaratan

- [ ] Tampilkan project name, output path, template, dan teknologi final.
- [ ] Report sesuai operation report filesystem.
- [ ] Bedakan success, partial success, dan failure.
- [ ] Jangan tampilkan success sebelum operasi terverifikasi sesuai kontrak.
- [ ] Hormati stdout/stderr, `--quiet`, `--verbose`, `--no-color`, dan exit codes Phase 1.
- [ ] Berikan diagnostic actionable untuk invalid input, unsupported combination, missing assets, render/plan/path/conflict/write/manifest errors.
- [ ] Jangan mencetak secrets atau environment lengkap.
- [ ] Next steps sesuai output aktual, misalnya `cd <project-name>` dan `cargo check`.
- [ ] Jangan menyarankan command yang belum tersedia.
- [ ] Bedakan error generation dari dependency/network environment failures.
- [ ] Quiet mode tidak menyembunyikan error penting.

### Acceptance criteria

- [ ] Report cocok dengan hasil operasi.
- [ ] Exit code konsisten.
- [ ] Diagnostic actionable dan aman.
- [ ] Tidak ada false success atau secret leakage.

## 17. Task 4.12 — Smoke verification generated project

### Persyaratan

- [ ] Verifikasi file wajib sesuai template.
- [ ] Verifikasi `Cargo.toml` dan package name.
- [ ] Verifikasi entry point dan module declarations.
- [ ] Verifikasi `.rustrs/config.toml` dapat dibaca ulang.
- [ ] Verifikasi conditional assets cocok dengan capability final.
- [ ] Pastikan tidak ada placeholder tersisa.
- [ ] Untuk Tier 1 yang supported, jalankan `cargo check` pada generated fixture/project di temporary directory bila environment memungkinkan.
- [ ] Isolasi `CARGO_TARGET_DIR` bila diperlukan.
- [ ] Jangan membutuhkan database, Docker, secret, atau layanan eksternal untuk test dasar.
- [ ] Bedakan build failure dari network/registry failure.
- [ ] Jangan melakukan dependency download sebagai efek samping tersembunyi tanpa kebijakan jelas.
- [ ] Hapus hanya file yang dibuat test sendiri.

### Acceptance criteria

- [ ] Structural verification lulus.
- [ ] Cargo manifest dan Rustrs manifest valid.
- [ ] Build smoke test lulus atau environment blocker dilaporkan.
- [ ] Tests terisolasi dari working tree dan home directory.

## 18. Task 4.13 — Automated tests

### Unit tests

- [ ] Input resolver: flags, defaults, explicit/missing values.
- [ ] Nama project dan target path validation.
- [ ] Compatibility/capability preflight.
- [ ] Template selection dan context.
- [ ] Generation plan.
- [ ] Manifest creation/read-back.
- [ ] Next-step report dan error mapping.

### CLI integration tests

- [ ] `rustrs --help` tetap bekerja.
- [ ] `rustrs new --help` mencerminkan contract.
- [ ] Invalid name dan incomplete non-interactive config gagal.
- [ ] Unsupported combinations ditolak.
- [ ] `--defaults` tidak meminta input.
- [ ] Global options dan exit codes benar.

### Filesystem integration tests

- [ ] Happy path membuat project.
- [ ] File structure dan manifest benar.
- [ ] Dry-run tidak membuat/mengubah file atau directory.
- [ ] Existing target tidak ditimpa diam-diam.
- [ ] Render/validation failure terjadi sebelum write.
- [ ] Apply failure menghasilkan report parsial yang benar.
- [ ] Traversal/symlink escape tidak dapat menulis di luar target.
- [ ] Re-run mengikuti conflict policy.
- [ ] Optional Git tidak menyentuh parent directory.

### Template/build tests

- [ ] Setiap template available memiliki expected file list/structure.
- [ ] Nama project tidak meninggalkan placeholder.
- [ ] Disabled capability tidak menghasilkan references rusak.
- [ ] Tidak ada secret atau remote fetch.
- [ ] Generated Tier 1 project lolos `cargo check` jika environment mendukung.

Gunakan temporary directories. Tests harus memeriksa hasil/error, bukan hanya memastikan fungsi tidak panic. Jangan menonaktifkan tests atau melemahkan assertions demi membuat CI hijau.

## 19. Task 4.14 — Dokumentasi, quality gates, dan final review

### Dokumentasi

- [ ] README menjelaskan `rustrs new`, flags, defaults, supported templates, dry-run, conflicts, dan next steps yang benar-benar tersedia.
- [ ] ARCHITECTURE mendokumentasikan scaffolding orchestration serta keputusan baru yang berdampak.
- [ ] Roadmap Phase 4 checklist/status mencerminkan bukti.
- [ ] Jangan mengklaim generator Phase 5 atau fitur lain telah tersedia.

### Quality gates

Jalankan dari root repository:

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
git status --short
```

Jalankan juga generated-project smoke tests. Jika command gagal, laporkan hasil sebenarnya; jangan menyatakan `PASS`.

### Final audit

- [ ] Review seluruh diff dan perubahan lokal.
- [ ] Pastikan dependency/lockfile changes disengaja.
- [ ] Tidak ada secrets di source, fixtures, manifest, atau docs.
- [ ] Tidak ada perubahan path di luar scope.
- [ ] Tidak ada false-success commands.
- [ ] Tidak ada Phase 5 generators.
- [ ] Tidak ada commit/push/publish/release tanpa izin.

## 20. Dependency policy

- Periksa `Cargo.toml`/`Cargo.lock` sebelum menambah dependency.
- Gunakan library yang sudah tersedia jika sesuai.
- Tambahkan prompt library hanya jika wizard memerlukannya.
- Jangan menambah async runtime jika workflow synchronous memadai.
- Gunakan `std::process::Command` untuk executable eksternal bila cukup.
- Jangan menambahkan remote template-fetching dependency.
- Verifikasi MSRV, platform, license, dan maintenance.
- Perbarui lockfile melalui Cargo, jangan edit manual.
- Dokumentasikan dependency baru jika berdampak pada arsitektur.

## 21. Error handling dan exit codes

Gunakan error model Phase 1. Tambahkan kategori hanya bila caller perlu membedakannya:

- invalid arguments/configuration;
- invalid project name/target path;
- unsupported combination/capability;
- missing template asset;
- rendering/planning failure;
- filesystem conflict/I/O failure;
- manifest write/read-back failure;
- smoke verification failure;
- optional Git failure.

Persyaratan:

- [ ] Domain errors tidak bergantung pada terminal formatting.
- [ ] CLI mapping konsisten.
- [ ] Diagnostic memberi konteks relevan tanpa secret.
- [ ] Error sebelum apply tidak menyebabkan filesystem mutations.
- [ ] Apply errors melaporkan hasil parsial bila tersedia.
- [ ] Hindari `unwrap()`/`expect()` untuk input pengguna dan I/O yang dapat gagal.
- [ ] Jangan menelan error dan mengembalikan sukses.
- [ ] Jangan mengklaim check lulus jika tidak dijalankan.

## 22. Acceptance criteria Phase 4

| ID       | Acceptance criteria                                     | Bukti minimum             |
| -------- | ------------------------------------------------------- | ------------------------- |
| P4-AC-01 | Preflight Phase 3 dan baseline selesai                  | Laporan baseline          |
| P4-AC-02 | `rustrs new` memiliki CLI contract nyata                | CLI tests                 |
| P4-AC-03 | Input/defaults deterministik                            | Unit tests                |
| P4-AC-04 | Interactive/non-interactive sesuai kontrak              | Tests/manual verification |
| P4-AC-05 | Nama/path divalidasi                                    | Unit/integration tests    |
| P4-AC-06 | Resolver Phase 2 digunakan                              | Code review/tests         |
| P4-AC-07 | Unsupported combinations ditolak sebelum write          | Integration tests         |
| P4-AC-08 | Template available memiliki assets lengkap              | Template tests            |
| P4-AC-09 | Output sesuai konfigurasi final                         | Template/context tests    |
| P4-AC-10 | Generation plan dibuat sebelum apply                    | Integration tests         |
| P4-AC-11 | Dry-run tidak mengubah filesystem                       | Before/after tests        |
| P4-AC-12 | Writer Phase 3 dan conflict policy digunakan            | Integration tests         |
| P4-AC-13 | Manifest valid dan dapat dibaca ulang                   | Round-trip tests          |
| P4-AC-14 | Report sesuai operasi aktual                            | CLI tests                 |
| P4-AC-15 | Generated project lolos structural verification         | Generated-project tests   |
| P4-AC-16 | Tier 1 smoke build lulus atau blocker terbukti          | `cargo check` fixture     |
| P4-AC-17 | Tests mencakup sukses dan failure paths                 | Test suite                |
| P4-AC-18 | Docs sesuai implementasi                                | Documentation review      |
| P4-AC-19 | Quality gates dijalankan dan dilaporkan                 | Hasil command             |
| P4-AC-20 | Tidak ada generator Phase 5/out-of-scope                | Scope review              |
| P4-AC-21 | Tidak ada perubahan pengguna hilang/destructive writes  | Git diff/status           |
| P4-AC-22 | Tidak ada false success atau support claims tanpa bukti | CLI review/tests          |

Jika kriteria wajib gagal/terblokir, status harus `PARTIAL` atau `BLOCKED`, bukan `COMPLETE`.

## 23. Definition of Done

- [ ] `rustrs new` berjalan dari parsing sampai apply.
- [ ] Input divalidasi sebelum filesystem berubah.
- [ ] Compatibility rules memiliki satu sumber kebenaran.
- [ ] Engine Phase 3 digunakan ulang.
- [ ] Template yang diiklankan tersedia, konsisten, dan teruji.
- [ ] Project starter untuk kombinasi supported memiliki Cargo/source/config valid.
- [ ] Manifest mencerminkan konfigurasi final dan dapat dibaca ulang.
- [ ] Dry-run tidak memodifikasi target.
- [ ] Konflik dan partial failure dilaporkan akurat.
- [ ] Tidak ada overwrite diam-diam.
- [ ] Tests mencakup sukses, invalid input, unsupported combinations, konflik, dan failure behavior.
- [ ] Smoke verification dijalankan sesuai policy.
- [ ] Docs dan quality gates selesai.
- [ ] Tidak ada Phase 5 generators atau perubahan tidak terkait.
- [ ] Tidak ada commit/push/publish/release tanpa izin.
- [ ] Agent berhenti setelah laporan Phase 4.

## 24. Urutan implementasi yang direkomendasikan

1. Preflight dan audit API.
2. CLI contract `new`.
3. Input model/resolver.
4. Nama/path validation.
5. Compatibility preflight.
6. Starter assets untuk template yang akan didukung.
7. Template/context integration.
8. Generation planning.
9. Dry-run/preview.
10. Filesystem apply.
11. Manifest generation dan read-back.
12. Wizard interaktif.
13. Optional Git initialization jika disetujui.
14. Report dan next steps.
15. Smoke verification.
16. Unit/CLI/filesystem/template tests.
17. Documentation.
18. Quality gates, diff review, final report.

Utamakan jalur non-interactive untuk kombinasi Tier 1 yang benar-benar didukung, kemudian tambahkan wizard dan template lain berdasarkan bukti. Compile/test secara incremental.

## 25. Format laporan akhir agent

```text
Phase: 4 — Project Scaffolding
Status: COMPLETE | PARTIAL | BLOCKED

Summary:
- <kemampuan rustrs new yang benar-benar tersedia>

Baseline:
- Git status/branch:
- Baseline failures:
- Phase 2/3 APIs reused:
- Blockers:

CLI:
- Commands/options implemented:
- Defaults:
- Interactive/non-interactive:
- Dry-run:
- Git initialization:

Scaffolding:
- Templates with complete assets:
- Supported combinations:
- Name/path validation:
- Compatibility preflight:
- Generation plan and apply:
- Manifest read-back:
- Smoke verification:
- Report/next steps:

Files changed:
- <path>: <alasan>

Tests:
- <tests dan skenario>

Verification:
- cargo metadata --no-deps --format-version 1: PASS | FAIL | BLOCKED — <hasil>
- cargo check: PASS | FAIL | BLOCKED — <hasil>
- cargo test: PASS | FAIL | BLOCKED — <hasil>
- cargo fmt --check: PASS | FAIL | BLOCKED — <hasil>
- cargo clippy --all-targets --all-features -- -D warnings: PASS | FAIL | BLOCKED — <hasil>
- git diff --check: PASS | FAIL | BLOCKED — <hasil>
- Generated-project smoke check: PASS | FAIL | BLOCKED — <hasil>

Safety:
- Dry-run:
- Conflict/overwrite policy:
- Path/symlink policy:
- Partial failure:
- Secret handling:

Acceptance criteria:
- P4-AC-01 through P4-AC-22: PASS | FAIL | BLOCKED, each with evidence.

Deferred/out of scope:
- <item dan alasan>

Known issues/decisions:
- <isu dan tindak lanjut>

Recommended next phase:
- Phase 5 — Core Code Generators
```

Jangan menandai command `PASS` jika tidak benar-benar dijalankan. Jika environment tidak mendukung pemeriksaan, gunakan `BLOCKED` dan jelaskan penyebabnya.

## 26. Prinsip akhir

**Phase 4 membangun jalur project creation yang nyata dan dapat dipercaya, bukan seluruh generator Rustrs.**

CLI menerima input; application service mengorkestrasi; manifest/compatibility adalah sumber kebenaran konfigurasi; renderer menghasilkan konten di memori; planner menyusun operasi; writer menerapkan plan tervalidasi; dry-run tidak memodifikasi filesystem; dan report menggambarkan hasil aktual.

Setelah acceptance criteria dievaluasi, quality gates dijalankan, dan laporan akhir diberikan, **berhenti pada Phase 4**. Jangan mulai Phase 5 sampai pemilik repository memberi persetujuan terpisah.

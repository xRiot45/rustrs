# Final Audit, Testing & Bug Fixing — rustrs CLI

Repository: <https://github.com/xRiot45/rustrs>

## Objective

Lakukan final audit dan testing menyeluruh pada codebase **rustrs**, sebuah proyek CLI berbasis Rust. Tujuannya adalah memastikan implementasi sesuai dengan arsitektur dan roadmap proyek, seluruh fitur bekerja sebagaimana mestinya, kualitas kode terjaga, serta tidak ada bug atau regresi yang belum ditangani.

Pekerjaan harus dilakukan secara sistematis, terukur, dan berbasis bukti dari codebase aktual. Jangan langsung mengubah kode sebelum implementation plan selesai dipresentasikan dan disetujui.

## Phase 1 — Repository Audit & Implementation Plan

Mulai dengan mengaudit repository tanpa mengubah file, menjalankan formatter yang dapat menulis perubahan, atau melakukan operasi Git yang memodifikasi repository.

### 1. Repository reconnaissance

Periksa dan pahami:

- Struktur direktori dan organisasi modul Rust.
- `Cargo.toml`, `Cargo.lock`, konfigurasi workspace jika tersedia, serta dependency yang digunakan.
- `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, `ARCHITECTURE.md`, dokumen desain, dan dokumentasi relevan lainnya jika tersedia.
- Roadmap proyek, khususnya dokumen roadmap Phase 0 — Repository Bootstrap dan Phase 1 — CLI Foundation.
- Implementasi command, argument parsing, entry point, konfigurasi, error handling, logging, serta utilitas internal.
- Unit tests, integration tests, fixtures, test helpers, dan contoh penggunaan CLI.
- GitHub Actions workflows, CI checks, release configuration, dan dokumentasi pengembangan jika tersedia.
- Status Git saat ini: branch aktif, commit terakhir, working tree, perubahan yang belum di-commit, serta file untracked.

**Jangan mengasumsikan nama modul, dependency, command, atau struktur direktori sebelum memeriksa repository aktual.**

Gunakan roadmap dan kontrak implementasi yang terdokumentasi sebagai acuan utama. Jika terdapat perbedaan antara dokumentasi dan implementasi, identifikasi perbedaannya terlebih dahulu dan tentukan apakah diperlukan klarifikasi.

### 2. Audit arsitektur dan implementasi CLI

Periksa area berikut sesuai dengan fitur yang benar-benar tersedia dalam proyek.

**Command-line interface**

- Parsing argumen, subcommands, flags, options, positional arguments, dan default values.
- Perilaku `--help`, `--version`, serta pesan penggunaan CLI.
- Validasi argumen wajib, argumen opsional, nilai tidak valid, dan kombinasi argumen yang bertentangan.
- Exit codes untuk kondisi sukses, input tidak valid, dan kegagalan eksekusi.
- Konsistensi output ke `stdout` dan `stderr`.
- Penanganan invocation tanpa argumen, argumen tidak dikenal, dan subcommand tidak valid.

**Business logic dan internal modules**

- Kesesuaian implementasi dengan tanggung jawab setiap modul.
- Konsistensi tipe data, ownership, borrowing, lifetime, dan penggunaan error types.
- Penanganan edge cases, input kosong, path tidak valid, file tidak ditemukan, serta kegagalan filesystem jika relevan.
- Deteksi logic duplication, dead code, panic yang tidak perlu, dan kesalahan penanganan error.
- Konsistensi API internal antarmodul dan pemisahan concern.

**Configuration dan filesystem**

- Prioritas konfigurasi CLI, environment variables, dan file konfigurasi jika mekanisme tersebut tersedia.
- Validasi dan parsing konfigurasi.
- Penanganan file atau direktori yang tidak tersedia, tidak memiliki izin akses, atau berisi data tidak valid.
- Perilaku terhadap path relatif, path absolut, dan working directory.
- Keamanan operasi filesystem serta penanganan path yang tidak tepercaya bila relevan.

**Dependencies dan Rust quality**

- Dependency yang tidak digunakan, versi yang tidak konsisten, atau dependency berlebihan.
- Penggunaan fitur Rust yang sesuai dengan MSRV atau minimum supported Rust version yang dinyatakan proyek.
- Compiler warnings, Clippy findings, dan indikasi kode yang rawan bug.
- Kepatuhan terhadap coding conventions dan arsitektur yang sudah ditetapkan.

**Security dan robustness**

- Risiko path traversal jika relevan.
- Penanganan input berbahaya atau berukuran tidak wajar.
- Informasi sensitif yang mungkin tertulis ke log atau output error.
- Penggunaan subprocess, temporary files, atau operasi sistem yang berisiko jika fitur tersebut tersedia.
- Panic, unwrap, dan expect pada jalur eksekusi yang menerima input pengguna.

Jangan memaksakan kategori audit yang tidak relevan. Catat kategori yang tidak berlaku beserta alasannya jika diperlukan.

### 3. Implementation plan

Setelah audit awal, presentasikan implementation plan yang sekurang-kurangnya mencakup:

1. Ringkasan kondisi repository dan arsitektur aktual.
2. Temuan awal beserta lokasi file, fungsi/modul terkait, tingkat keparahan, dan bukti.
3. Potensi bug serta ketidaksesuaian antara implementasi, dokumentasi, dan roadmap.
4. Daftar perbaikan yang diusulkan, disertai alasan teknis dan dampaknya.
5. Strategi pengujian sebelum dan sesudah perbaikan.
6. Perkiraan urutan pengerjaan dan dependensi antartugas.
7. Risiko, area yang belum jelas, dan keputusan yang membutuhkan persetujuan.
8. Kriteria penerimaan yang objektif.
9. Rencana Git, termasuk branch target, branch kerja, commit, push, dan Pull Request.

Bedakan secara jelas antara **temuan yang sudah terverifikasi**, **dugaan yang masih harus diuji**, dan **rekomendasi perbaikan**.

**Berhenti setelah implementation plan selesai dipresentasikan. Jangan melakukan perubahan kode, membuat commit, push, atau membuat Pull Request sebelum saya menyetujui plan tersebut.**

Jika keputusan penting belum jelas, jelaskan pilihan yang tersedia, konsekuensi masing-masing, dan rekomendasi terbaik. Jangan membuat asumsi sendiri.

## Phase 2 — Execution After Approval

Mulai fase ini hanya setelah saya memberikan persetujuan eksplisit terhadap implementation plan.

### 1. Establish baseline

Sebelum memperbaiki kode:

- Catat branch dan commit awal.
- Periksa working tree kembali.
- Identifikasi perubahan pengguna yang sudah ada dan lindungi seluruh perubahan tersebut.
- Jalankan pemeriksaan awal yang relevan dan dapat dilakukan dengan aman.
- Simpan hasil baseline, termasuk test yang gagal sebelum perubahan.
- Bedakan kegagalan yang sudah ada dari kegagalan baru akibat perbaikan.

Jangan menghapus, menimpa, memformat ulang, atau memasukkan perubahan pengguna yang tidak terkait ke dalam pekerjaan ini.

Jika working tree memiliki perubahan yang dapat memengaruhi audit atau keselamatan pekerjaan, berhenti pada area terkait dan minta arahan sebelum mengambil tindakan yang berpotensi merusaknya.

### 2. Fix bugs and implementation inconsistencies

Perbaiki bug yang telah terverifikasi sesuai plan yang disetujui.

Prioritaskan:

1. Bug yang menyebabkan crash, panic, atau kegagalan command utama.
2. Ketidaksesuaian perilaku CLI dan hasil parsing argumen.
3. Kesalahan validasi, konfigurasi, filesystem, atau error handling.
4. Ketidaksesuaian kontrak internal antarmodul.
5. Masalah kualitas kode yang memiliki risiko nyata terhadap correctness atau maintainability.

Aturan perbaikan:

- Utamakan perubahan minimal yang menyelesaikan akar masalah.
- Pertahankan kompatibilitas CLI dan kontrak yang sudah didokumentasikan, kecuali perubahan tersebut secara eksplisit disetujui.
- Jangan menambahkan dependency baru jika solusi yang lebih sederhana menggunakan kemampuan yang sudah tersedia.
- Jangan mengubah arsitektur atau ruang lingkup fitur tanpa persetujuan.
- Tambahkan regression tests untuk setiap bug yang diperbaiki jika memungkinkan.
- Jangan menonaktifkan test, menghapus assertion, memperlemah validasi, atau menyembunyikan error demi membuat pemeriksaan lulus.
- Jangan melakukan refactor besar yang tidak diperlukan oleh temuan audit.
- Jalankan kembali test yang relevan setelah setiap kelompok perbaikan.

### 3. Required verification and testing

Gunakan toolchain dan command yang sesuai dengan repository aktual. Periksa `Cargo.toml`, konfigurasi CI, dan dokumentasi proyek sebelum menentukan command final.

Jalankan pemeriksaan berikut jika relevan dan tersedia.

**Formatting**

- `cargo fmt --all -- --check`

**Compilation**

- `cargo check --workspace --all-targets` untuk workspace, atau command setara yang sesuai jika repository bukan workspace.
- `cargo build --workspace` atau build command yang sesuai.
- `cargo build --release` jika konfigurasi release relevan.

**Tests**

- `cargo test --workspace --all-targets` untuk workspace, atau test suite yang sesuai dengan struktur proyek.
- Unit tests untuk setiap modul terkait.
- Integration tests untuk perilaku CLI yang dapat diamati pengguna.
- Regression tests untuk bug yang ditemukan dan diperbaiki.
- Pengujian error paths, input invalid, edge cases, exit codes, serta `stdout`/`stderr` jika relevan.
- Doctests dan examples jika proyek menggunakannya.

**Linting and static analysis**

- `cargo clippy --workspace --all-targets --all-features -- -D warnings` jika konfigurasi feature dan workspace mendukung command tersebut.
- Evaluasi compiler warnings dan lint findings yang relevan.
- Jangan menerapkan `--all-features` atau opsi workspace secara membabi buta jika tidak sesuai dengan konfigurasi aktual.

**CLI behavior and smoke tests**

- Periksa `--help`, `--version`, dan invocation utama yang benar-benar didukung.
- Uji argumen valid dan tidak valid.
- Periksa exit code dan konsistensi output.
- Periksa jalur sukses dan gagal, termasuk error message yang dapat dipahami pengguna.
- Pastikan smoke tests tidak menggunakan resource eksternal yang tidak tersedia tanpa mencatat keterbatasannya.

**Compatibility checks**

- Periksa target platform dan MSRV yang benar-benar dinyatakan proyek.
- Jalankan pemeriksaan lintas platform atau target compilation jika tooling dan environment mendukungnya.
- Jika tes pada Windows, Linux, atau macOS tidak dapat dilakukan di environment saat ini, laporkan keterbatasannya secara eksplisit.

**CI verification**

- Periksa workflow GitHub Actions dan bandingkan pemeriksaan lokal dengan pemeriksaan CI.
- Periksa status CI terkini jika tersedia.
- Jangan menganggap status CI berhasil hanya karena konfigurasi workflow tersedia.
- Jangan mengklaim test lintas platform berhasil jika hanya dijalankan pada satu platform.

**E2E testing**

Karena rustrs merupakan aplikasi CLI Rust, jangan secara otomatis menerapkan Playwright E2E testing yang biasanya digunakan untuk browser.

Gunakan pengujian end-to-end berbasis proses CLI, integration tests, atau black-box tests untuk memvalidasi eksekusi binary nyata dari awal sampai akhir. Pastikan pengujian mencakup input, argumen, output, exit status, dan efek filesystem jika relevan.

Jika repository memiliki komponen web atau browser yang memang memerlukan Playwright, evaluasi dan jalankan Playwright hanya untuk komponen tersebut sesuai konfigurasi yang tersedia.

Jangan menambahkan Playwright sebagai dependency hanya demi memenuhi pola pengujian aplikasi web.

### 4. Re-test and regression review

Setelah perbaikan:

- Jalankan kembali regression tests dan test suite yang terdampak.
- Ulangi format check, compilation, tests, lint, dan build yang relevan.
- Bandingkan hasil dengan baseline awal.
- Periksa kemungkinan regression pada command atau fitur yang sebelumnya berfungsi.
- Pastikan dokumentasi dan contoh penggunaan tidak bertentangan dengan perilaku implementasi.
- Periksa ulang error handling, exit codes, dan hasil output pada jalur penting.
- Evaluasi seluruh perubahan melalui `git diff` dan `git diff --check`.
- Periksa `git status` serta daftar file yang berubah.

Jika ada test gagal, kategorikan sebagai:

- Kegagalan yang sudah ada sebelum perubahan.
- Kegagalan baru akibat perubahan.
- Kegagalan lingkungan atau dependency eksternal.
- Test yang belum dapat dijalankan.

Jangan menyamakan test yang tidak dijalankan dengan test yang lulus. Jangan mengklaim seluruh test berhasil jika masih terdapat kegagalan yang belum diselesaikan.

Jika ditemukan bug baru di luar scope yang disetujui, dokumentasikan dan rekomendasikan tindak lanjut. Jangan memperluas scope secara diam-diam.

## Phase 3 — Final Review and Acceptance Gate

Sebelum operasi Git apa pun yang akan membagikan atau menyimpan perubahan secara permanen, lakukan final review.

### Acceptance criteria

Pekerjaan dinyatakan siap diajukan ke Pull Request apabila:

- Implementasi konsisten dengan arsitektur, dokumentasi, dan roadmap yang berlaku.
- Bug yang termasuk scope dan telah disetujui sudah diperbaiki.
- Test yang relevan sudah dijalankan dan hasil sebenarnya telah dicatat.
- Formatting, compilation, linting, tests, dan build memenuhi kriteria proyek.
- Kegagalan yang tersisa telah dijelaskan dan tidak disembunyikan.
- Regression tests telah ditambahkan untuk bug yang relevan.
- Tidak terdapat perubahan tidak terkait atau file pengguna yang tidak sengaja terikut.
- Tidak terdapat masalah kritis yang diketahui tetapi belum diungkapkan.
- Final diff telah ditinjau dan cakupan perubahan dapat dijelaskan.

Jika kriteria belum terpenuhi, jangan menyatakan pekerjaan selesai. Sajikan status, kegagalan, dampak, dan langkah penyelesaian yang direkomendasikan.

## Phase 4 — GitHub Delivery

Tahap ini memerlukan **dua persetujuan yang terpisah**:

1. Persetujuan implementation plan sebelum perubahan dimulai.
2. Persetujuan final delivery sebelum commit, push, dan Pull Request dibuat.

Setelah pemeriksaan yang relevan selesai dan hasilnya memenuhi kriteria penerimaan, tampilkan laporan final beserta ringkasan diff dan hasil testing. **Jangan melakukan commit, push, atau membuat Pull Request sebelum saya menyetujui final delivery.**

Setelah mendapatkan persetujuan final delivery:

1. Pastikan branch target dan branch kerja sudah benar.
2. Pastikan hanya perubahan yang disetujui dan relevan yang akan disertakan.
3. Buat commit dengan pesan yang deskriptif dan mengikuti conventional commits jika proyek menggunakannya.
4. Push branch kerja ke repository yang benar.
5. Buat Pull Request menuju branch target yang disetujui.
6. Sertakan ringkasan perubahan, alasan perbaikan, daftar pengujian beserta hasil aktual, kegagalan yang tersisa, dan keterbatasan environment.
7. Periksa kembali detail PR dan status CI jika tersedia.
8. Laporkan URL PR, branch, commit SHA, dan status pemeriksaan yang benar-benar terverifikasi.

Jangan merge Pull Request kecuali saya memberikan instruksi terpisah untuk melakukan merge.

## Reporting format

### A. Initial Audit & Plan — sebelum persetujuan

Sajikan:

- Repository overview.
- Kondisi branch dan working tree.
- Komponen yang telah diperiksa.
- Temuan awal berdasarkan bukti.
- Risiko dan ketidakjelasan.
- Proposed implementation plan.
- Daftar command testing yang direncanakan.
- Acceptance criteria.
- Keputusan yang memerlukan persetujuan.

Akhiri laporan dengan meminta persetujuan saya sebelum melanjutkan ke perubahan kode.

### B. Execution Report — setelah persetujuan plan

Sajikan:

- Perubahan yang dilakukan dan file yang terdampak.
- Bug dan akar masalah yang ditemukan.
- Perbaikan yang diterapkan.
- Regression tests yang ditambahkan.
- Hasil setiap command testing yang benar-benar dijalankan.
- Perbandingan hasil sebelum dan sesudah perbaikan.
- Kegagalan serta keterbatasan yang masih tersisa.
- Risiko regression dan hasil review diff.

### C. Final Delivery Approval — sebelum commit/push/PR

Sajikan:

- Ringkasan akhir perubahan.
- Hasil verifikasi berdasarkan bukti.
- Acceptance criteria yang terpenuhi dan belum terpenuhi.
- Daftar risiko residual.
- Daftar file yang akan disertakan.
- Branch target, nama branch kerja, dan usulan pesan commit.
- Usulan judul serta deskripsi Pull Request.

Akhiri dengan meminta persetujuan eksplisit saya sebelum menjalankan operasi GitHub tersebut.

### D. GitHub Delivery Report — setelah persetujuan final

Sajikan:

- Commit SHA dan pesan commit.
- Branch yang di-push.
- URL Pull Request.
- Ringkasan perubahan dalam PR.
- Hasil testing dan status CI terbaru yang telah diverifikasi.
- Kegagalan atau keterbatasan yang masih ada.

## Mandatory rules

- Audit terlebih dahulu; jangan langsung mengubah kode.
- Presentasikan implementation plan sebelum memulai pekerjaan implementasi.
- Tunggu persetujuan eksplisit saya pada setiap approval gate yang telah ditentukan.
- Jangan membuat asumsi pada keputusan teknis yang berdampak besar.
- Jangan mengubah atau menghapus perubahan pengguna yang sudah ada.
- Jangan melakukan commit, push, atau membuat Pull Request sebelum persetujuan final delivery.
- Jangan mengklaim perintah atau test telah berhasil jika belum benar-benar dijalankan dan hasilnya belum diverifikasi.
- Laporkan hasil, kegagalan, warning penting, dan keterbatasan secara transparan.
- Prioritaskan correctness, keamanan, maintainability, kompatibilitas, serta perubahan minimal.
- Gunakan repository dan dokumentasi aktual sebagai sumber kebenaran, bukan asumsi.
- Jika ada kegagalan atau risiko kritis, hentikan delivery dan laporkan terlebih dahulu.
- **Tugas pertama saat ini hanya melakukan audit awal repository dan menyusun implementation plan. Jangan melanjutkan ke Phase 2 sebelum saya menyetujuinya.**

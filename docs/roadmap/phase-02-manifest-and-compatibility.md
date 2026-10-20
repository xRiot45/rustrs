# Phase 2 — Manifest and Compatibility

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 2 of 10  
> **Status:** Implemented  
> **Audience:** AI coding agents and maintainers  
> **Primary outcome:** Establish a typed, versioned project manifest and a single, testable compatibility resolver for templates, frameworks, databases, data-access layers, and capabilities.  
> **Documentation language:** Indonesian; source code, identifiers, and user-facing CLI terms should follow existing repository conventions.  
> **Source of truth:** `PRD.md` → `ARCHITECTURE.md` → `AGENTS.md` → this phase document → existing implementation, in accordance with the source-of-truth priority in `AGENTS.md`.

---

## 1. Tujuan Phase

Phase 2 membangun fondasi konfigurasi project yang akan digunakan oleh scaffolding, generator, inspector, dan command lain pada fase berikutnya.

Pada akhir fase ini, `rustrs` harus mampu:

1. Merepresentasikan konfigurasi project dalam model Rust bertipe.
2. Mem-parse manifest TOML dari teks menjadi model yang dapat divalidasi.
3. Memvalidasi schema version dan nilai konfigurasi.
4. Menentukan apakah kombinasi template, framework, database, data-access layer, dan capability didukung.
5. Menemukan manifest project dari konteks direktori tanpa mengubah filesystem.
6. Menghasilkan error dan diagnostic yang konsisten dengan kontrak CLI Phase 1.
7. Menguji semua aturan penting tanpa mengandalkan project pengguna yang sebenarnya.

**Phase 2 bukan fase scaffolding.** Fase ini tidak boleh membuat project baru, merender template, menghasilkan kode aplikasi, atau menulis/mengubah manifest melalui command.

### 1.1 Hasil yang diharapkan

- Ada model manifest bertipe dengan `schema_version` eksplisit.
- Parsing TOML terpisah dari validasi domain.
- Kebijakan field tidak dikenal terdokumentasi dan diterapkan secara konsisten.
- Compatibility matrix menjadi satu-satunya sumber kebenaran untuk keputusan dukungan.
- Kombinasi yang belum didukung ditolak secara eksplisit, bukan diam-diam diubah.
- Capability memiliki model dan aturan validasi yang dapat diuji.
- Manifest discovery dapat membedakan Cargo project biasa dari project yang memiliki manifest `rustrs`.
- Error configuration/compatibility terintegrasi dengan error handling Phase 1.
- Dokumentasi dan automated tests menggambarkan perilaku aktual.
- Tidak ada fitur masa depan yang dipresentasikan sebagai tersedia.

---

## 2. Dokumen dan Repository yang Wajib Dibaca

Sebelum melakukan perubahan, agent wajib membaca:

1. `PRD.md`
2. `ARCHITECTURE.md`
3. `AGENTS.md`
4. `docs/roadmap/phase-0-repository-bootstrap.md`, jika tersedia
5. `docs/roadmap/phase-01-cli-foundation.md`
6. Dokumen Phase 2 ini
7. `Cargo.toml`, `Cargo.lock`, seluruh modul CLI/error/project yang relevan, tests, README, dan CI workflow

Jika dokumen tidak tersedia, laporkan secara eksplisit. Jangan mengklaim telah membacanya.

### 2.1 Prioritas sumber kebenaran

Ikuti urutan yang ditetapkan dalam `AGENTS.md`:

1. Instruksi eksplisit pemilik project.
2. `PRD.md`.
3. `ARCHITECTURE.md`.
4. Dokumen fase aktif.
5. `AGENTS.md`.
6. Implementasi saat ini, selama tidak bertentangan dengan sumber di atas.

Jika ada konflik yang memengaruhi keputusan penting dan tidak dapat diselesaikan dari dokumen, catat konflik dan minta klarifikasi. Jangan membuat keputusan produk diam-diam.

---

## 3. Preflight dan Baseline

### Task 2.0 — Preflight & baseline audit

#### Tujuan

Memastikan Phase 1 benar-benar menjadi fondasi yang sehat dan memahami kondisi repository terkini sebelum mengubah kode.

#### Langkah kerja

1. Jalankan `git status --short`, `git branch --show-current`, dan `git log -5 --oneline`.
2. Inspeksi struktur file dan source code yang terkait.
3. Periksa `rustc --version` dan `cargo --version`.
4. Tinjau `Cargo.toml` dan `Cargo.lock`; jangan menambah dependency yang sudah tersedia.
5. Jalankan quality gates sebelum perubahan:
   - `cargo metadata --no-deps --format-version 1`
   - `cargo check`
   - `cargo test`
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
6. Catat kegagalan baseline terpisah dari regresi yang muncul akibat Phase 2.
7. Periksa implementasi Phase 1, khususnya:
   - error types dan exit-code mapping,
   - output policy,
   - project-root discovery,
   - pola unit/integration tests.
8. Buat checklist implementasi singkat sebelum mengubah file.

#### Acceptance criteria

- [ ] Kondisi Git dan repository diperiksa.
- [ ] Dokumen wajib ditinjau atau kekurangannya dilaporkan.
- [ ] Baseline quality gates dicatat dengan status PASS/FAIL/BLOCKED.
- [ ] Perubahan pengguna yang belum di-commit tetap dipertahankan.
- [ ] Tidak ada perubahan source code sebelum baseline dan scope dipahami.

---

## 4. Batas Scope

### 4.1 Termasuk Phase 2

- Model manifest bertipe.
- Parsing/deserialisasi TOML.
- Schema versioning dasar.
- Validasi konfigurasi.
- Kebijakan field tidak dikenal.
- Model domain template/framework/database/data-access.
- Compatibility matrix dan resolver.
- Capability model serta dependency/conflict validation.
- Manifest discovery/read-only project context.
- Error mapping ke kontrak CLI Phase 1.
- Unit tests, integration tests, dan dokumentasi yang relevan.

### 4.2 Tidak termasuk Phase 2

Jangan implementasikan:

- `rustrs new` atau wizard project creation.
- Template rendering atau template assets.
- Filesystem generation engine atau safe writer.
- Pembuatan, penulisan, atau pembaruan manifest oleh command.
- Implementasi nyata `rustrs config show/get/set`.
- Generator module/resource/controller/service/repository/dto/migration.
- Capability installation ke project.
- Migrasi otomatis manifest lama dengan perubahan filesystem.
- Database commands, Docker orchestration, atau process management.
- External tool checks yang menjadi bagian dari `doctor` atau workflow development.
- Global user configuration.
- Template registry, plugin system, atau network fetching.

Jangan membuat modul kosong atau abstraction spekulatif untuk fitur-fitur tersebut.

---

## 5. Prinsip Arsitektur dan Aturan Implementasi

1. **Typed configuration:** konfigurasi penting harus direpresentasikan dengan struct/enum Rust, bukan string bebas yang tersebar.
2. **Parse vs validate:** parsing TOML hanya membentuk model; validasi domain merupakan tahap terpisah.
3. **Single source of truth:** semua keputusan compatibility harus berasal dari satu compatibility layer.
4. **No silent fallback:** pilihan yang tidak didukung tidak boleh diam-diam diganti dengan pilihan lain.
5. **No false support:** mengenali sebuah identifier tidak berarti kombinasi itu sudah bisa dihasilkan oleh generator.
6. **No secrets in manifest:** credential dan secret runtime harus disimpan melalui environment atau mekanisme runtime yang sesuai.
7. **No filesystem side effects:** parsing, validation, resolver, dan discovery Phase 2 bersifat read-only.
8. **Explicit schema version:** schema yang tidak didukung harus ditolak dengan pesan yang dapat ditindaklanjuti.
9. **Small modules:** gunakan struktur yang sesuai dengan codebase Phase 1; jangan membuat file/trait untuk setiap konsep tanpa kebutuhan.
10. **Preserve user work:** jangan menghapus atau menimpa perubahan yang tidak dibuat oleh agent.
11. **MSRV compatibility:** dependency dan source harus kompatibel dengan `rust-version` di `Cargo.toml`.
12. **Honest CLI:** command yang belum diimplementasikan tidak boleh ditambahkan hanya untuk menampilkan nama di help.

---

## 6. Keputusan Domain yang Harus Ditetapkan

Sebelum mengimplementasikan resolver, dokumentasikan keputusan domain yang dibutuhkan agar perilaku tidak ambigu.

### 6.1 Identifier teknologi

PRD menyebut pilihan berikut sebagai cakupan produk yang direncanakan:

- Template: `enterprise`, `standard`, `simple`, `minimal`.
- Framework: `axum`, `actix-web`, `rocket`.
- Database: `postgres`, `mysql`, `sqlite`, `none`.
- Data access: `sqlx`, `sea-orm`, `diesel`, `none`.

Identifier harus konsisten di model, TOML, dokumentasi, dan tests. Hindari alias yang tidak diperlukan. Jika alias diperlukan, definisikan dan uji secara eksplisit.

### 6.2 Supported vs recognized

Bedakan dua konsep berikut:

- **Recognized option:** identifier dikenal dan dapat diparse ke enum/model.
- **Supported combination:** kombinasi sudah diimplementasikan dan diuji oleh generator yang relevan sehingga dapat ditawarkan kepada pengguna.

Jangan menandai seluruh kombinasi sebagai supported hanya karena pilihan teknologinya tercantum di PRD.

### 6.3 Baseline support policy

- Tier 1 yang menjadi prioritas PRD: Axum + PostgreSQL + SQLx.
- Kombinasi lain hanya berstatus supported jika ada bukti implementasi dan pengujian yang sesuai.
- Kombinasi yang belum didukung harus ditolak atau tidak ditawarkan oleh command yang kelak menyajikan pilihan.
- Phase 2 boleh mendefinisikan metadata untuk kombinasi yang belum tersedia, tetapi statusnya harus jelas dan tidak boleh mengklaim scaffold sudah buildable.

### 6.4 Database dan data access

Tentukan dan dokumentasikan kebijakan untuk kasus seperti:

- `database = "none"` dengan `data_access = "none"`.
- Database aktif tetapi `data_access = "none"`.
- `data_access = "sqlx"` dengan database tertentu.
- Driver/ORM yang belum didukung untuk template atau database yang dipilih.

Jangan mengasumsikan aturan hanya dari nama teknologi. Aturan akhir harus konsisten dengan PRD dan compatibility matrix yang benar-benar diterapkan.

### 6.5 Template dan architecture profile

- Validasi template yang dikenal.
- Validasi architecture profile yang dikenal.
- Jika architecture profile merupakan pilihan terbatas berdasarkan template, nyatakan batasannya dalam matrix.
- Jangan membuat daftar architecture profile spekulatif. Gunakan nilai yang diperlukan PRD dan implementasi.

### 6.6 Capability support

Capability dapat meliputi `auth`, `rbac`, `openapi`, `redis`, `email`, `storage`, `audit_log`, `rate_limit`, `idempotency`, `observability`, `docker`, dan `cicd`.

Daftar final harus dibandingkan dengan PRD dan kebutuhan MVP. Jangan menambahkan capability hanya karena secara teori umum dipakai API.

---

## 7. Task 2.1 — Definisi Model Manifest

### Tujuan

Membuat model konfigurasi bertipe yang menjadi kontrak utama untuk project yang dikelola `rustrs`.

### Persyaratan

- [ ] Buat model utama, misalnya `ProjectManifest`, sesuai konvensi nama repository.
- [ ] Definisikan `schema_version` dengan tipe yang sesuai dan validasi nilai yang didukung.
- [ ] Definisikan field project seperti `project_name`, template, framework, architecture, database, dan data-access.
- [ ] Pertimbangkan metadata generator/template version yang dibutuhkan PRD dan architecture.
- [ ] Definisikan nested structures untuk fitur, dokumentasi, dan CI bila field tersebut diputuskan sebagai bagian schema Phase 2.
- [ ] Gunakan enum untuk pilihan terbatas.
- [ ] Tentukan field wajib, optional, default, dan nilai yang tidak valid.
- [ ] Hindari menyimpan secret runtime.
- [ ] Jangan menambahkan field spekulatif yang belum memiliki pemilik/penggunaan.
- [ ] Tambahkan unit tests untuk enum dan default yang ditetapkan.

### Panduan desain

- Gunakan tipe eksplisit untuk identifier terbatas.
- Jangan menggunakan `String` untuk semua field jika domain memiliki himpunan nilai yang jelas.
- Jangan menyatukan parsing, validasi, dan compatibility dalam satu fungsi besar.
- Hindari terlalu banyak `Option<T>` tanpa alasan domain yang jelas.
- Default harus eksplisit dan terdokumentasi; jangan bergantung pada default implisit yang tidak terlihat oleh pengguna.

### Output

Model manifest yang dapat di-deserialize dan kemudian divalidasi tanpa melakukan I/O.

---

## 8. Task 2.2 — Parsing TOML

### Tujuan

Mem-parse teks TOML ke model manifest dengan error yang dapat dipahami.

### Persyaratan

- [ ] Tambahkan `serde` dengan fitur yang dibutuhkan dan `toml` jika belum tersedia.
- [ ] Verifikasi versi dependency kompatibel dengan MSRV.
- [ ] Buat fungsi parser yang menerima teks/str dan mengembalikan model mentah atau error bertipe.
- [ ] Parsing tidak membaca filesystem secara langsung.
- [ ] Parsing tidak melakukan compatibility resolution.
- [ ] Bedakan error sintaks TOML dari validasi domain setelah parsing.
- [ ] Sertakan informasi konteks seperti path manifest pada boundary pembacaan file.
- [ ] Manfaatkan informasi lokasi/baris/kolom dari parser jika library menyediakan informasi tersebut.
- [ ] Jangan mencetak seluruh isi manifest dalam error.
- [ ] Definisikan kebijakan unknown fields secara eksplisit.
- [ ] Tambahkan tests untuk TOML valid, TOML malformed, tipe field salah, field wajib hilang, dan field unknown.

### Kebijakan unknown fields

Pilih dan dokumentasikan kebijakan yang konsisten dengan PRD. Default yang disarankan untuk schema v1 adalah **menolak field yang tidak dikenal**, kecuali ada alasan kompatibilitas yang kuat untuk kebijakan lain.

Jika memilih untuk mengizinkan unknown fields:

- jelaskan alasan dan risiko,
- pastikan field tidak diam-diam mengubah perilaku,
- buat tests yang membuktikan perilakunya.

Jangan mengklaim unknown field “ditangani secara eksplisit” jika implementasi sebenarnya hanya mengabaikannya.

### Output

Parser murni yang menghasilkan model manifest atau diagnostic parsing.

---

## 9. Task 2.3 — Schema Versioning

### Tujuan

Memastikan format manifest dapat dikenali dan ditolak dengan aman jika versinya tidak didukung.

### Persyaratan

- [ ] Tentukan schema version pertama yang didukung, sesuai PRD; gunakan versi awal yang terdokumentasi.
- [ ] Validasi `schema_version` setelah parsing.
- [ ] Tolak versi yang lebih tinggi atau tidak dikenal dengan pesan yang jelas.
- [ ] Jangan mengasumsikan versi yang hilang sebagai versi terbaru tanpa keputusan schema yang eksplisit.
- [ ] Dokumentasikan perbedaan `schema_version`, versi binary `rustrs`, dan versi template.
- [ ] Jangan implementasikan migrasi file otomatis pada Phase 2.
- [ ] Buat tests untuk versi didukung, versi lebih tinggi, versi lebih rendah/tidak dikenal, dan field versi yang hilang.

### Output

Aturan schema versioning yang jelas dan dapat diuji tanpa mengubah manifest di disk.

---

## 10. Task 2.4 — Validasi Manifest

### Tujuan

Memastikan nilai yang berhasil diparse memenuhi aturan konfigurasi project.

### Persyaratan

- [ ] Validasi nama project sesuai aturan yang sudah ada atau ditetapkan secara eksplisit.
- [ ] Validasi field wajib dan field yang tidak boleh kosong.
- [ ] Validasi schema version.
- [ ] Validasi template, framework, architecture, database, dan data-access.
- [ ] Validasi nilai default dan hubungan antar-field.
- [ ] Validasi bahwa nilai konfigurasi tidak mengandung asumsi yang tidak didukung.
- [ ] Bedakan validation errors dari parse errors.
- [ ] Kumpulkan beberapa error jika pendekatan yang dipilih memungkinkan tanpa mengaburkan penyebab.
- [ ] Jangan menyimpan atau menampilkan secret.
- [ ] Tambahkan unit tests untuk kasus valid dan invalid.

### Model validasi yang disarankan

Gunakan pipeline konseptual:

1. Baca teks.
2. Parse TOML menjadi model manifest.
3. Validasi schema version dan nilai field.
4. Jalankan compatibility resolution.
5. Hasilkan konfigurasi tervalidasi yang dapat digunakan oleh consumer.

Tidak semua langkah harus menjadi modul/file terpisah. Pemisahan logis dan testability lebih penting daripada jumlah file.

### Output

Validator yang mengembalikan hasil sukses bertipe atau error validasi yang dapat ditindaklanjuti.

---

## 11. Task 2.5 — Compatibility Matrix dan Resolver

### Tujuan

Menjadi sumber kebenaran tunggal untuk menentukan kombinasi teknologi yang didukung.

### Persyaratan

- [ ] Definisikan model hasil compatibility, misalnya supported/unsupported beserta alasan.
- [ ] Definisikan matrix atau policy yang eksplisit untuk template, framework, database, dan data-access.
- [ ] Tandai baseline Tier 1 dari PRD secara eksplisit.
- [ ] Jangan menandai kombinasi sebagai supported tanpa bukti scaffold/test pada fase yang sesuai.
- [ ] Resolver menerima konfigurasi pilihan secara eksplisit dan tidak melakukan I/O.
- [ ] Resolver menolak kombinasi yang tidak didukung sebelum file generation pada fase berikutnya.
- [ ] Error menyebutkan kombinasi yang ditolak dan alasan.
- [ ] Jika aman dan tersedia dari matrix, berikan alternatif yang benar-benar didukung.
- [ ] Jangan mengubah pilihan pengguna diam-diam.
- [ ] Jangan menggandakan aturan matrix dalam CLI command atau template.
- [ ] Buat unit tests untuk semua kombinasi yang didukung dan kombinasi invalid penting.

### Format keputusan

Pilih representasi yang paling sederhana untuk codebase. Contoh konseptual:

- status: supported / unsupported / not implemented,
- alasan keputusan,
- alternatif yang valid (opsional).

Jangan membangun framework policy generik atau plugin system. Enum, tabel statis, atau fungsi domain eksplisit cukup jika memenuhi kebutuhan MVP.

### Output

Compatibility resolver deterministik yang bisa diuji tanpa filesystem, network, atau subprocess.

---

## 12. Task 2.6 — Capability Model dan Validation

### Tujuan

Mendefinisikan bagaimana fitur opsional berhubungan dengan konfigurasi project.

### Persyaratan

- [ ] Definisikan capability yang benar-benar diperlukan oleh PRD/MVP.
- [ ] Tentukan apakah capability direpresentasikan sebagai struct berisi boolean, enum/set, atau model lain yang paling sesuai.
- [ ] Definisikan dukungan capability berdasarkan template dan kombinasi teknologi jika diperlukan.
- [ ] Definisikan dependency dan conflict antar-capability hanya jika ada aturan produk yang jelas.
- [ ] Validasi capability yang tidak tersedia pada kombinasi terpilih.
- [ ] Bedakan “capability dikenal dalam schema” dengan “capability sudah diimplementasikan oleh generator”.
- [ ] Jangan otomatis menonaktifkan capability yang diminta pengguna.
- [ ] Jangan menganggap capability yang belum diimplementasikan didukung hanya karena field-nya ada di manifest.
- [ ] Tambahkan tests untuk capability valid, unsupported, dependency, dan conflict.

### Catatan penting

Phase 2 membangun model dan aturan. Ia belum menginstal capability, menambahkan dependency ke `Cargo.toml` project target, menghasilkan file, atau memodifikasi source code.

### Output

Capability validator/resolver yang menyediakan keputusan eksplisit untuk fase generator.

---

## 13. Task 2.7 — Manifest Discovery dan Project Context

### Tujuan

Menemukan manifest project dengan aman berdasarkan direktori kerja dan aturan pencarian yang terdokumentasi.

### Persyaratan

- [ ] Evaluasi kembali project-root discovery Phase 1 sebelum menulis ulang fungsi yang sudah ada.
- [ ] Integrasikan discovery manifest dengan root discovery tanpa mengasumsikan setiap Cargo project dikelola `rustrs`.
- [ ] Tetapkan aturan pencarian: mulai dari current directory, lalu parent directories sesuai kontrak yang dipilih.
- [ ] Tetapkan perilaku bila beberapa kandidat ditemukan.
- [ ] Bedakan manifest tidak ditemukan, file tidak bisa dibaca, dan manifest invalid.
- [ ] Berikan path manifest yang ditemukan pada project context.
- [ ] Jangan membuat folder atau manifest secara otomatis.
- [ ] Jangan menulis atau mengubah filesystem.
- [ ] Hindari canonicalization atau symlink assumptions yang tidak portabel; tangani error path dengan baik.
- [ ] Gunakan temporary directories untuk integration tests.

### Acceptance criteria

- [ ] Menemukan manifest di direktori saat ini.
- [ ] Menemukan manifest dari subdirektori jika aturan parent traversal mengizinkan.
- [ ] Menghasilkan hasil not-found jika manifest tidak tersedia.
- [ ] Tidak menganggap `Cargo.toml` saja sebagai bukti project `rustrs`.
- [ ] Menangani file yang tidak dapat dibaca dengan error yang sesuai.
- [ ] Tidak membuat file atau direktori selama discovery.

### Output

Fungsi discovery read-only yang menghasilkan project context atau error terstruktur.

---

## 14. Task 2.8 — Error Model dan Integrasi dengan CLI Phase 1

### Tujuan

Menghubungkan error manifest dan compatibility dengan error handling, diagnostic, dan exit codes yang sudah ditetapkan.

### Persyaratan

- [ ] Review error model Phase 1 sebelum menambah type baru.
- [ ] Tambahkan error untuk parsing, schema version tidak didukung, validation, manifest tidak ditemukan, I/O, dan compatibility.
- [ ] Gunakan `thiserror` bila sesuai dengan pola repository yang ada.
- [ ] Map error ke kategori CLI/exit code yang telah ditetapkan di Phase 1.
- [ ] Jangan membuat mapping berbeda di setiap command.
- [ ] Tampilkan error input/configuration di stderr sesuai kontrak CLI.
- [ ] Pertahankan pesan yang actionable.
- [ ] Jangan mencetak secret atau seluruh manifest.
- [ ] Jangan mencetak backtrace untuk error input biasa.
- [ ] Uji error category, output stream, dan exit code pada boundary CLI yang relevan.

### Prinsip

Error domain harus menjelaskan penyebab; CLI layer bertanggung jawab mengubahnya menjadi diagnostic dan exit status. Hindari memasukkan format terminal ke dalam validator atau resolver domain.

### Output

Integrasi error yang konsisten dan tetap mempertahankan pemisahan domain, application, dan CLI.

---

## 15. Task 2.9 — Automated Tests

### Tujuan

Memastikan model manifest, parser, validator, resolver, capability rules, dan discovery bekerja sesuai kontrak.

### 15.1 Unit tests

Minimal mencakup:

- [ ] Enum identifier valid dan invalid.
- [ ] Manifest minimal yang valid.
- [ ] Manifest lengkap yang valid.
- [ ] TOML malformed.
- [ ] Tipe field salah.
- [ ] Field wajib hilang.
- [ ] Unknown fields sesuai kebijakan yang dipilih.
- [ ] Schema version supported dan unsupported.
- [ ] Nilai default yang ditetapkan.
- [ ] Nama project invalid.
- [ ] Kombinasi template/framework/database/data-access supported.
- [ ] Kombinasi unsupported dan alasan error.
- [ ] Capability dependencies/conflicts.
- [ ] Error mapping yang relevan.

### 15.2 Integration tests

Minimal mencakup:

- [ ] Discovery menemukan manifest dari temporary directory.
- [ ] Discovery dari nested directory mengikuti kontrak pencarian.
- [ ] Cargo project tanpa manifest `rustrs` tidak dianggap managed.
- [ ] Manifest tidak ditemukan menghasilkan error yang benar.
- [ ] Manifest tidak dapat dibaca ditangani dengan benar jika skenario dapat direproduksi di platform test.
- [ ] Error CLI yang relevan menghasilkan exit code non-zero.
- [ ] Tidak ada operasi tulis saat parsing, validation, resolution, atau discovery.

### 15.3 Kualitas tests

- Gunakan fixture TOML yang kecil dan jelas.
- Hindari test yang tergantung pada repository lokal pengguna.
- Gunakan temporary directory untuk filesystem tests.
- Jangan menandai test lulus hanya karena fungsi tidak panic; assert hasil, error type, atau diagnostic yang relevan.
- Jangan menambahkan snapshot framework hanya untuk satu assertion jika assertion biasa memadai.
- Perbaiki akar masalah daripada menonaktifkan test atau menambahkan `allow` secara luas.

### Output

Suite tests yang membuktikan perilaku sukses dan gagal, termasuk regression tests untuk bug yang ditemukan.

---

## 16. Task 2.10 — Dokumentasi dan Quality Gates

### Dokumentasi yang harus ditinjau

- [ ] `ARCHITECTURE.md`: catat keputusan schema, parsing, compatibility, dan capability yang benar-benar diterapkan.
- [ ] `README.md`: perbarui hanya jika status kemampuan yang ditampilkan berubah; jangan mengiklankan command yang belum tersedia.
- [ ] `docs/roadmap/phase-02-manifest-and-compatibility.md`: checklist dan status akhir akurat.
- [ ] Dokumentasikan contoh schema manifest v1.
- [ ] Dokumentasikan unknown-field policy.
- [ ] Dokumentasikan schema-version behavior.
- [ ] Dokumentasikan compatibility matrix dan status kombinasi yang benar-benar supported.
- [ ] Dokumentasikan bahwa Phase 2 tidak membuat atau menulis manifest melalui command.

### Quality gates

Jalankan dari root repository:

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Jika ada test/command tambahan yang sudah ditetapkan oleh CI atau repository, jalankan juga.

Jika `cargo fmt --check` gagal karena perubahan Phase 2, jalankan `cargo fmt`, lalu ulangi check. Jangan menyembunyikan lint dengan `#[allow(...)]` yang luas.

### Final review

- [ ] Review `git diff --check`.
- [ ] Review `git status --short` dan seluruh diff.
- [ ] Pastikan tidak ada perubahan tidak terkait.
- [ ] Pastikan `Cargo.lock` diperbarui jika dependency berubah.
- [ ] Pastikan tidak ada secret atau data sensitif di fixture/dokumentasi.
- [ ] Pastikan tidak ada file yang dibuat di luar scope.
- [ ] Pastikan tidak ada command placeholder atau false success.

---

## 17. Rekomendasi Struktur Modul

Struktur berikut adalah panduan, bukan kewajiban mutlak. Pertahankan pola yang sudah digunakan di Phase 1 dan hindari memindahkan file tanpa kebutuhan nyata.

```text
src/
├── main.rs
├── cli/
│   └── ...
├── error.rs
├── project/
│   ├── mod.rs
│   └── root.rs
├── manifest/
│   ├── mod.rs
│   ├── model.rs
│   ├── parser.rs
│   ├── validation.rs
│   └── discovery.rs
└── compatibility/
    ├── mod.rs
    ├── types.rs
    ├── matrix.rs
    ├── resolver.rs
    └── capabilities.rs

tests/
├── cli_manifest.rs
└── manifest_discovery.rs
```

Tanggung jawab yang disarankan:

- `manifest/model.rs`: struct, enum, dan representasi konfigurasi.
- `manifest/parser.rs`: parsing teks TOML ke model.
- `manifest/validation.rs`: validasi schema dan field.
- `manifest/discovery.rs`: penemuan dan pembacaan manifest sesuai kebutuhan.
- `compatibility/types.rs`: jenis pilihan dan hasil compatibility jika tidak lebih tepat diletakkan di domain lain.
- `compatibility/matrix.rs`: aturan dukungan sebagai sumber kebenaran.
- `compatibility/resolver.rs`: validasi kombinasi teknologi.
- `compatibility/capabilities.rs`: aturan capability.
- `error.rs`: error yang digunakan lintas modul; jangan jadikan satu enum raksasa tanpa alasan.

Tidak semua file di atas harus dibuat. Jika satu modul kecil cukup, gunakan satu modul. Hindari duplikasi enum dan aturan antara manifest dan compatibility.

---

## 18. Kebijakan Dependency

### Kandidat dependency

| Dependency            | Tujuan                       | Kebijakan                                                 |
| --------------------- | ---------------------------- | --------------------------------------------------------- |
| `serde` dengan derive | Deserialisasi model manifest | Tambahkan jika belum tersedia                             |
| `toml`                | Parsing TOML                 | Tambahkan jika belum tersedia                             |
| `thiserror`           | Error domain bertipe         | Sudah ada pada baseline Phase 1; evaluasi pemakaian ulang |
| `tempfile`            | Temporary directory tests    | Gunakan dependency yang sudah tersedia                    |
| `assert_cmd`          | CLI integration tests        | Gunakan bila sesuai pola tests saat ini                   |
| `predicates`          | Assertion output CLI         | Gunakan bila diperlukan                                   |

### Aturan dependency

1. Periksa `Cargo.toml` dan `Cargo.lock` terlebih dahulu.
2. Untuk setiap dependency baru, jelaskan kebutuhan Phase 2 yang dilayani.
3. Verifikasi MSRV dan dukungan platform.
4. Gunakan fitur minimum yang dibutuhkan.
5. Jangan menambahkan Tera, Tokio, prompt library, `walkdir`, atau generator dependencies pada fase ini.
6. Jangan mengubah dependency hanya demi mengikuti versi terbaru jika versi yang ada sudah sesuai dan kompatibel.
7. Perbarui lockfile melalui Cargo; jangan mengedit lockfile secara manual.

---

## 19. Acceptance Criteria Phase 2

Phase 2 hanya boleh dilaporkan `COMPLETE` jika seluruh acceptance criteria yang berlaku telah lulus.

| ID       | Acceptance criteria                                                     | Bukti/verifikasi                |
| -------- | ----------------------------------------------------------------------- | ------------------------------- |
| P2-AC-01 | Repository dan baseline diperiksa sebelum perubahan                     | Laporan agent dan hasil command |
| P2-AC-02 | Manifest direpresentasikan dengan model Rust bertipe                    | Review model dan unit tests     |
| P2-AC-03 | Manifest dapat diparse dari TOML valid                                  | Unit tests                      |
| P2-AC-04 | TOML malformed dan tipe field invalid menghasilkan error yang benar     | Unit tests                      |
| P2-AC-05 | Unknown-field policy diterapkan dan terdokumentasi                      | Tests dan dokumentasi           |
| P2-AC-06 | Schema version eksplisit dan versi unsupported ditolak                  | Unit tests                      |
| P2-AC-07 | Validasi manifest terpisah dari parsing                                 | Review struktur dan tests       |
| P2-AC-08 | Compatibility matrix memiliki satu sumber kebenaran                     | Review domain code              |
| P2-AC-09 | Kombinasi unsupported ditolak tanpa silent fallback                     | Unit tests                      |
| P2-AC-10 | Baseline support Tier 1 dinyatakan secara eksplisit                     | Matrix dan dokumentasi          |
| P2-AC-11 | Capability rules divalidasi dan diuji                                   | Unit tests                      |
| P2-AC-12 | Discovery membedakan Cargo project biasa dari project bermanifest       | Integration tests               |
| P2-AC-13 | Discovery dan resolver tidak menulis filesystem                         | Tests/review                    |
| P2-AC-14 | Error terintegrasi dengan kontrak CLI Phase 1                           | Integration tests               |
| P2-AC-15 | README/architecture tidak mengklaim fitur yang belum ada                | Documentation review            |
| P2-AC-16 | Seluruh quality gates lulus atau blocker dilaporkan dengan jujur        | Hasil command aktual            |
| P2-AC-17 | Diff hanya berisi perubahan yang disengaja                              | `git diff` dan `git status`     |
| P2-AC-18 | Tidak ada generator/template engine atau command placeholder yang masuk | Scope review                    |

Jika kriteria wajib gagal atau terblokir, jangan laporkan `COMPLETE`. Gunakan `PARTIAL` atau `BLOCKED` sesuai keadaan dan jelaskan tindak lanjutnya.

---

## 20. Definition of Done

Phase 2 selesai jika:

- [ ] Semua task in-scope selesai atau secara eksplisit dicatat sebagai blocked dengan alasan.
- [ ] Seluruh acceptance criteria yang berlaku lulus.
- [ ] Manifest model, parser, validator, compatibility resolver, capability rules, dan discovery teruji.
- [ ] Error handling konsisten dengan Phase 1.
- [ ] Semua quality gates dijalankan dan hasil aktual dilaporkan.
- [ ] Dokumentasi mencerminkan implementasi aktual.
- [ ] Tidak ada fitur di luar scope yang diimplementasikan.
- [ ] Tidak ada perubahan pengguna yang hilang.
- [ ] Tidak ada commit, push, publish, atau release yang dilakukan tanpa instruksi eksplisit.
- [ ] Agent berhenti setelah Phase 2 dan menunggu persetujuan untuk memulai Phase 3.

---

## 21. Urutan Implementasi yang Direkomendasikan

Kerjakan secara incremental dengan compile/test setelah perubahan bermakna:

1. **Preflight:** baca dokumen, inspect repository, dan catat baseline.
2. **Domain identifiers:** tetapkan enum teknologi dan keputusan support policy.
3. **Manifest model:** definisikan struct/enum dan default eksplisit.
4. **TOML parser:** parse teks menjadi model dengan error yang jelas.
5. **Schema versioning:** validasi schema version dan unknown-field policy.
6. **Manifest validation:** validasi field dan invariant dasar.
7. **Compatibility matrix:** deklarasikan dukungan kombinasi.
8. **Compatibility resolver:** validasi pilihan dan hasilkan error/alternatif.
9. **Capability rules:** validasi capability, dependency, dan conflict.
10. **Manifest discovery:** integrasikan dengan root discovery Phase 1.
11. **CLI error integration:** map error domain ke diagnostic dan exit code.
12. **Tests:** lengkapi unit dan integration tests.
13. **Documentation:** perbarui architecture/README jika diperlukan.
14. **Quality gates:** jalankan seluruh checks dan review diff.
15. **Final report:** laporkan status, bukti, blocker, dan rekomendasi fase selanjutnya.

Jangan memulai semua modul sekaligus tanpa compile/test. Hindari refactor besar yang tidak diperlukan oleh acceptance criteria.

---

## 22. Format Laporan Akhir Agent

Agent wajib menyampaikan laporan dalam format berikut:

```text
Phase: 2 — Manifest and Compatibility
Status: COMPLETE | PARTIAL | BLOCKED

Summary:
- <ringkasan singkat hasil>

Files changed:
- <path>: <tujuan perubahan>

Implemented:
- Manifest model:
- TOML parser:
- Schema versioning:
- Manifest validation:
- Compatibility matrix/resolver:
- Capability validation:
- Manifest discovery:
- CLI error integration:

Tests:
- <test yang ditambahkan dan skenario yang dibuktikan>

Verification:
- cargo metadata --no-deps --format-version 1: PASS | FAIL | BLOCKED — <detail>
- cargo check: PASS | FAIL | BLOCKED — <detail>
- cargo test: PASS | FAIL | BLOCKED — <detail>
- cargo fmt --check: PASS | FAIL | BLOCKED — <detail>
- cargo clippy --all-targets --all-features -- -D warnings: PASS | FAIL | BLOCKED — <detail>
- git diff --check: PASS | FAIL | BLOCKED — <detail>

Compatibility policy:
- <kombinasi yang supported>
- <kombinasi yang unsupported/not implemented>

Deferred / out of scope:
- <item dan alasan>

Known issues / decisions needed:
- <isu, dampak, dan tindak lanjut>

Recommended next phase:
- Phase 3 — Template and Filesystem Engine
```

Jangan melaporkan command sebagai PASS jika tidak benar-benar dijalankan. Jika environment tidak menyediakan tool yang diperlukan, tandai `BLOCKED` dan jelaskan alasannya.

---

## 23. Prinsip Akhir

**Phase 2 harus menghasilkan konfigurasi yang bertipe, kompatibilitas yang eksplisit, dan keputusan domain yang dapat diuji.** Jangan membangun generator lebih awal, jangan menganggap opsi yang dikenal otomatis didukung, dan jangan diam-diam memperbaiki konfigurasi pengguna dengan mengganti pilihan mereka.

Setelah seluruh acceptance criteria lulus dan laporan diberikan, berhenti. Phase 3 — Template and Filesystem Engine memerlukan persetujuan terpisah sebelum dimulai.

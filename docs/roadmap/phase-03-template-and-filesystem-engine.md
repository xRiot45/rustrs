# Phase 3 — Template and Filesystem Engine

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 3 of 10  
> **Status:** Implemented  
> **Audience:** AI coding agents and maintainers  
> **Primary outcome:** Build a deterministic, testable template engine and a safe filesystem planning/application layer that later phases can use for project scaffolding and code generation.  
> **Documentation language:** Indonesian. Use idiomatic English for Rust identifiers and source-code comments according to repository conventions.  
> **Scope rule:** Implement only the engine and its tests. Do not implement the end-user project scaffolding workflow or production-ready API templates in this phase.

---

## 1. Tujuan Phase

Phase 3 membangun dua fondasi yang saling terhubung tetapi harus tetap terpisah:

1. **Template engine:** menemukan template yang tersedia, membaca metadata, membangun context, dan merender asset menjadi konten di memori.
2. **Filesystem engine:** menyusun generation plan, memvalidasi path dan konflik, menerapkan operasi file secara aman, serta melaporkan hasil yang benar-benar terjadi.

Engine ini akan digunakan oleh Phase 4 untuk project scaffolding dan oleh fase berikutnya untuk code generation. Phase 3 harus menghasilkan API internal yang jelas dan teruji, bukan langsung menyelesaikan alur produk `rustrs new`.

### 1.1 Hasil akhir yang diharapkan

Pada akhir Phase 3:

- Template memiliki metadata dan konvensi asset yang terdokumentasi.
- Template resolver dapat menemukan dan memvalidasi template yang tersedia.
- Renderer menerima input eksplisit dan menghasilkan output deterministik di memori.
- Template rendering tidak melakukan filesystem writes.
- Generation plan merepresentasikan operasi file sebelum perubahan diterapkan.
- Path target divalidasi agar operasi tidak keluar dari project root.
- Konflik file ditangani berdasarkan kebijakan yang eksplisit.
- Filesystem writer menerapkan plan yang telah divalidasi dan melaporkan hasil aktual.
- Dry-run dapat menampilkan plan tanpa mengubah filesystem target.
- Unit, integration, dan security tests membuktikan perilaku sukses dan gagal.
- Dokumentasi menggambarkan implementasi aktual.
- Tidak ada project scaffolding end-to-end atau template API production-ready yang diimplementasikan secara prematur.

### 1.2 Prinsip keberhasilan

**Keberhasilan Phase 3 diukur dari ketepatan, determinisme, keamanan, dan testability engine—bukan dari jumlah template atau command produk yang sudah tersedia.**

---

## 2. Dokumen dan Source Code yang Wajib Dibaca

Sebelum mengubah kode, baca:

1. `PRD.md`
2. `ARCHITECTURE.md`
3. `AGENTS.md`
4. `docs/roadmap/phase-0-repository-bootstrap.md`, jika tersedia
5. `docs/roadmap/phase-01-cli-foundation.md`
6. `docs/roadmap/phase-02-manifest-and-compatibility.md`
7. Dokumen Phase 3 ini
8. Source code manifest, compatibility, CLI, error handling, project root, tests, `Cargo.toml`, `Cargo.lock`, README, dan CI workflow yang relevan

Jangan mengklaim membaca dokumen yang tidak tersedia. Laporkan bila dokumen yang hilang menghalangi implementasi yang aman.

### 2.1 Prioritas sumber kebenaran

Ikuti urutan prioritas dalam `AGENTS.md`:

1. Instruksi eksplisit pemilik project.
2. `PRD.md`.
3. `ARCHITECTURE.md`.
4. Dokumen fase aktif.
5. `AGENTS.md`.
6. Implementasi aktual jika tidak bertentangan dengan sumber di atas.

Jika terdapat konflik keputusan yang penting dan tidak dapat diselesaikan dari dokumen, jelaskan konflik tersebut dan minta klarifikasi. Jangan membuat keputusan produk diam-diam.

---

## 3. Preflight dan Baseline

### Task 3.0 — Preflight dan audit Phase 2

#### Tujuan

Memastikan implementasi Phase 2 yang ada benar-benar dapat dipakai sebagai fondasi Phase 3.

#### Langkah kerja

1. Jalankan:

   ```bash
   git status --short
   git branch --show-current
   git log -5 --oneline
   rustc --version
   cargo --version
   ```

2. Inspeksi struktur repository aktual. Jangan mengandalkan asumsi dari laporan sebelumnya.
3. Baca `Cargo.toml` dan `Cargo.lock`; identifikasi dependency yang sudah tersedia.
4. Review model manifest, parser, validator, compatibility matrix/resolver, capability rules, project-root/manifest discovery, error handling, dan test yang sudah ada.
5. Jalankan baseline:

   ```bash
   cargo metadata --no-deps --format-version 1
   cargo check
   cargo test
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings
   ```

6. Catat kegagalan baseline secara terpisah dari regression baru.
7. Tentukan bagaimana engine menerima manifest/konfigurasi tervalidasi dari Phase 2. Jangan menyalin ulang model manifest atau compatibility rules.
8. Buat checklist perubahan sebelum mengedit source code.

#### Acceptance criteria

- [ ] Git status dan perubahan yang belum di-commit telah diperiksa.
- [ ] Dokumen dan modul yang relevan telah ditinjau.
- [ ] Baseline quality gates dicatat.
- [ ] Dependency yang tersedia sudah diperiksa.
- [ ] Perubahan pengguna dipertahankan.
- [ ] Tidak ada source change sebelum scope dan baseline dipahami.

---

## 4. Batas Scope Phase 3

### 4.1 Termasuk Phase 3

- Konvensi template assets dan metadata.
- Built-in template lookup/resolution.
- Template context.
- Template renderer.
- Pemilihan asset opsional berdasarkan metadata/configuration.
- Generation plan model.
- Planning pipeline.
- Path validation dan project-root containment.
- File conflict detection dan write policy.
- Filesystem writer.
- Dry-run/preview dan operation report.
- Unit, integration, dan security tests untuk engine.
- Dokumentasi serta quality gates.

### 4.2 Tidak termasuk Phase 3

Jangan implementasikan:

- `rustrs new` sebagai alur scaffolding end-to-end.
- Interactive wizard atau prompt scaffolding.
- Non-interactive scaffolding CLI flags.
- Isi lengkap/production-ready template `enterprise`, `standard`, `simple`, atau `minimal`.
- Generator `resource`, `module`, `controller`, `service`, `repository`, DTO, migration, atau generator lainnya.
- Command `config show/get/set` yang membaca atau mengubah konfigurasi.
- Penulisan `.rustrs/config.toml` sebagai bagian project creation.
- Registrasi route/module dalam project target.
- Database migration, Docker orchestration, process management, atau external tooling.
- Remote template registry/download.
- Plugin system.
- Auto-upgrade project yang sudah dibuat.
- Merge engine untuk perubahan source code yang kompleks.
- Rollback transaction generik untuk semua operasi filesystem.
- Perubahan CLI public command tree kecuali diperlukan untuk menjaga build/test yang sudah ada.

### 4.3 Penggunaan fixtures

Phase 3 boleh menggunakan template fixture kecil untuk membuktikan resolver, context, rendering, dan file planning. Fixture tersebut adalah **test assets**, bukan klaim bahwa template REST API lengkap sudah tersedia.

---

## 5. Prinsip Arsitektur

1. **Separation of concerns:** resolver, context builder, renderer, planner, path validator, conflict detector, writer, dan output/report memiliki tanggung jawab berbeda.
2. **Explicit inputs:** konfigurasi, context, template metadata, dan target root harus diberikan secara eksplisit.
3. **Single source of truth:** compatibility/capability decisions tetap berasal dari Phase 2.
4. **Render in memory:** renderer tidak melakukan file I/O.
5. **Plan before apply:** seluruh plan harus divalidasi sebelum filesystem dimodifikasi sejauh memungkinkan.
6. **Safe by default:** jangan menimpa file yang sudah ada secara diam-diam.
7. **No path escape:** template tidak boleh menulis di luar target root.
8. **No arbitrary execution:** template adalah data yang dirender, bukan kode yang dieksekusi.
9. **Determinism:** input yang sama, template version yang sama, dan engine version yang sama menghasilkan plan/output yang sama.
10. **No speculative abstraction:** jangan membuat plugin API, registry generik, atau trait yang belum diperlukan.
11. **Preserve user work:** jangan menghapus atau menimpa perubahan pengguna.
12. **Honest reporting:** bedakan planned operations dengan operasi yang benar-benar berhasil.
13. **MSRV compatibility:** seluruh dependency dan implementasi harus sesuai dengan `rust-version` di `Cargo.toml`.
14. **Phase discipline:** berhenti setelah Phase 3 dan tunggu persetujuan sebelum Phase 4.

---

## 6. Task 3.1 — Template Assets dan Metadata Model

### Tujuan

Menetapkan format yang konsisten untuk mendeskripsikan template dan file-file yang dimilikinya.

### Persyaratan

- [ ] Tentukan lokasi built-in template assets sesuai kebutuhan binary dan struktur repository.
- [ ] Tentukan konvensi template ID yang konsisten dengan enum manifest Phase 2: `enterprise`, `standard`, `simple`, `minimal`.
- [ ] Definisikan metadata bertipe untuk identitas template dan versi.
- [ ] Metadata dapat menyatakan deskripsi, asset list, format asset, kebutuhan context, dan kondisi inclusion jika memang dibutuhkan.
- [ ] Catat dukungan framework/database/data-access/capability dengan mengacu ke compatibility layer yang sudah ada; jangan membuat matrix kedua.
- [ ] Bedakan file statis yang disalin dari file teks yang dirender.
- [ ] Tentukan bagaimana asset opsional ditentukan secara eksplisit.
- [ ] Validasi metadata sebelum template dipakai.
- [ ] Definisikan perilaku ketika metadata atau asset wajib hilang.
- [ ] Buat fixture template minimal untuk unit/integration tests.
- [ ] Jangan menambahkan seluruh isi API template pada fase ini.

### Keputusan teknis

Pilih representasi metadata yang paling sederhana. Metadata dapat didefinisikan sebagai Rust structs, file metadata, atau gabungan yang sesuai kebutuhan. Jangan membuat metadata system yang lebih kompleks dari jumlah built-in templates yang direncanakan.

Jika metadata ditulis dalam file, parsing dan validation harus memiliki error yang jelas. Jika metadata ditulis sebagai Rust data, pastikan test membuktikan konsistensinya dengan asset yang benar-benar ada.

### Acceptance criteria

- [ ] Template ID dikenal dan konsisten dengan manifest.
- [ ] Metadata valid dapat dibaca.
- [ ] Metadata invalid atau asset wajib hilang menghasilkan error.
- [ ] File statis dan file yang perlu dirender dapat dibedakan.
- [ ] Fixture tidak dipresentasikan sebagai template API production-ready.

---

## 7. Task 3.2 — Template Resolver

### Tujuan

Memilih template yang tersedia berdasarkan pilihan template dan konfigurasi tervalidasi.

### Persyaratan

- [ ] Resolver menerima template ID dan konfigurasi/hasil validasi secara eksplisit.
- [ ] Resolver hanya mengembalikan template yang tersedia pada build saat ini.
- [ ] Resolver memvalidasi keberadaan metadata dan asset yang wajib.
- [ ] Resolver mengacu pada compatibility/capability layer Phase 2 untuk keputusan dukungan.
- [ ] Resolver menolak template ID yang tidak dikenal atau belum tersedia.
- [ ] Resolver menolak kombinasi yang tidak didukung sebelum rendering.
- [ ] Error menjelaskan template atau pilihan yang bermasalah.
- [ ] Resolver tidak menulis filesystem.
- [ ] Resolver tidak mengubah pilihan pengguna secara diam-diam.
- [ ] Resolver tidak mengambil template dari jaringan.
- [ ] Unit tests mencakup template valid, template tidak dikenal, metadata invalid, asset hilang, dan konfigurasi unsupported.

### Hasil yang diharapkan

Sebuah resolved-template value yang membawa informasi cukup untuk rendering tanpa membuat renderer bertanggung jawab atas pemilihan template.

---

## 8. Task 3.3 — Template Context

### Tujuan

Menyediakan data yang eksplisit dan tervalidasi kepada renderer.

### Persyaratan

- [ ] Definisikan context model untuk nilai yang benar-benar diperlukan oleh fixtures dan engine.
- [ ] Context dibangun dari manifest/configuration tervalidasi serta metadata template.
- [ ] Context tidak membaca global state atau environment secara tersembunyi.
- [ ] Context serializable ke format yang diterima template engine.
- [ ] Validasi variable wajib sebelum rendering jika metadata mendeklarasikannya.
- [ ] Tetapkan bagaimana nilai opsional direpresentasikan.
- [ ] Pastikan secret tidak disalin ke context secara otomatis.
- [ ] Uji field wajib hilang, nilai opsional, Unicode yang didukung, punctuation, dan value yang memerlukan escaping.
- [ ] Jangan menambahkan business-domain fields yang baru diperlukan oleh Phase 4, kecuali dibutuhkan untuk membuktikan API engine.

### Hasil yang diharapkan

Context builder yang deterministik dan dapat diuji tanpa filesystem.

---

## 9. Task 3.4 — Template Renderer

### Tujuan

Merender template teks menjadi konten di memori tanpa efek samping filesystem.

### Pemilihan engine

`ARCHITECTURE.md` menyebut Tera sebagai kandidat utama. Sebelum menetapkannya:

1. Periksa versi crate dan kompatibilitas MSRV.
2. Uji parsing, context, include/inheritance jika benar-benar dibutuhkan.
3. Verifikasi escaping untuk Rust source, TOML, YAML, shell scripts, dan plain text.
4. Dokumentasikan keputusan penggunaan Tera atau alternatifnya di `ARCHITECTURE.md`.

Jangan memilih engine lain hanya berdasarkan preferensi pribadi tanpa evaluasi yang relevan.

### Persyaratan

- [ ] Renderer menerima template content dan context secara eksplisit.
- [ ] Renderer mengembalikan konten yang sudah dirender atau error bertipe.
- [ ] Renderer tidak membuat, membaca, atau menulis target file.
- [ ] Renderer tidak mengeksekusi template sebagai program.
- [ ] Syntax error, missing variable, invalid context, dan render error dilaporkan secara jelas.
- [ ] Kebijakan escaping tidak merusak sintaks kode output.
- [ ] Jangan menonaktifkan escaping global tanpa menilai risiko dan kebutuhan.
- [ ] Output deterministik untuk input yang sama.
- [ ] Unit tests membuktikan output yang diharapkan.
- [ ] Unit tests membuktikan error pada template invalid/context tidak lengkap.
- [ ] Uji punctuation, quotes, Unicode yang didukung, dan identifier contoh yang valid.

### Batas scope

Tidak perlu membuat template lengkap untuk keempat API profiles. Gunakan fixtures kecil seperti README atau file konfigurasi sederhana.

---

## 10. Task 3.5 — Template Asset Selection dan Conditional Assets

### Tujuan

Memilih asset mana yang akan diproses berdasarkan metadata template dan configuration yang tervalidasi.

### Persyaratan

- [ ] Dukung daftar asset yang wajib dan opsional jika diperlukan oleh model metadata.
- [ ] Tentukan cara mendeklarasikan asset yang disalin apa adanya versus asset yang dirender.
- [ ] Evaluasi kondisi inclusion secara eksplisit dan terbatas.
- [ ] Jangan menulis expression evaluator generik atau menjalankan kode dari metadata.
- [ ] Capability decisions harus menggunakan aturan Phase 2.
- [ ] Jika capability belum didukung generator, jangan menyertakan asset yang seolah mengimplementasikannya.
- [ ] Jika kondisi asset tidak valid atau merujuk field yang tidak tersedia, hasilkan error.
- [ ] Uji kondisi enabled/disabled dan asset wajib yang hilang.

### Hasil yang diharapkan

Daftar asset terpilih yang stabil dan dapat digunakan oleh planning pipeline.

---

## 11. Task 3.6 — Generation Plan Model

### Tujuan

Merepresentasikan perubahan file yang direncanakan sebagai data yang dapat divalidasi sebelum operasi filesystem.

### Persyaratan

- [ ] Definisikan plan dan file-operation types.
- [ ] Setiap operation memiliki target path relatif.
- [ ] Setiap operation memiliki jenis operasi yang eksplisit.
- [ ] Operation dapat menyimpan konten hasil render atau representasi yang sesuai.
- [ ] Operation memuat alasan/deskripsi yang dapat digunakan pada preview.
- [ ] Conflict policy eksplisit dan tidak tersirat dari terminal output.
- [ ] Pisahkan planned operation dari actual operation result.
- [ ] Plan tidak bergantung pada format output CLI.
- [ ] Jangan menambahkan konsep merge/patch yang belum memiliki semantik.
- [ ] Unit tests membuktikan plan yang dibuat dari input yang sama stabil.

### Model konseptual

Model berikut hanya ilustrasi tanggung jawab, bukan kewajiban menyalin nama type secara persis:

```rust
struct GenerationPlan {
    target_root: PathBuf,
    operations: Vec<FileOperation>,
}

struct FileOperation {
    relative_path: PathBuf,
    kind: OperationKind,
    description: String,
    content: Option<Vec<u8>>,
}

enum OperationKind {
    Create,
    Replace,
    Skip,
}
```

Sesuaikan dengan desain aktual. Jangan gunakan `Replace` sebagai izin overwrite otomatis; setiap jenis operasi harus memiliki kebijakan konflik yang terdefinisi.

---

## 12. Task 3.7 — Planning Pipeline

### Tujuan

Menghubungkan resolver, context builder, renderer, dan generation plan tanpa melakukan mutasi filesystem.

### Urutan pipeline

1. Terima konfigurasi yang sudah divalidasi.
2. Resolve template.
3. Pilih asset yang berlaku.
4. Bangun context.
5. Render asset teks di memori.
6. Bentuk operasi untuk asset statis dan hasil render.
7. Validasi seluruh target path.
8. Deteksi konflik berdasarkan target root saat ini.
9. Hasilkan plan lengkap atau error sebelum apply.

### Persyaratan

- [ ] Planning tidak menulis file atau membuat direktori target.
- [ ] Semua asset wajib harus dapat diproses sebelum plan dinyatakan valid.
- [ ] Kegagalan render tidak menghasilkan partial writes.
- [ ] Target path diperiksa sebelum dimasukkan ke plan final.
- [ ] Konflik dicatat sebelum apply sejauh memungkinkan.
- [ ] Plan membedakan operation yang siap diterapkan dari operation yang ditolak/skip.
- [ ] Unit/integration tests membuktikan tidak ada mutasi filesystem selama planning.

### Hasil yang diharapkan

Pipeline yang mengubah konfigurasi dan template menjadi plan siap apply, tanpa menulis file.

---

## 13. Task 3.8 — Path Validation dan Root Containment

### Tujuan

Mencegah operasi file di luar target root, termasuk path traversal dan symlink escape.

### Persyaratan

- [ ] Semua path dari metadata template diperlakukan sebagai untrusted input.
- [ ] Tolak path absolut.
- [ ] Tolak parent-directory traversal seperti `../`.
- [ ] Validasi komponen path sesuai platform.
- [ ] Pastikan target tetap berada di bawah root yang diizinkan.
- [ ] Pertimbangkan symlink pada target root, parent directories, dan target file.
- [ ] Jangan menganggap lexical normalization saja cukup untuk mencegah symlink escape.
- [ ] Definisikan kebijakan untuk symlink yang sudah ada: reject atau dukungan terbatas yang aman.
- [ ] Tangani error canonicalization/metadata secara eksplisit.
- [ ] Pertimbangkan race condition antara validasi dan write.
- [ ] Jangan menjalankan operasi shell untuk menyelesaikan path.
- [ ] Buat tests path traversal, absolute path, komponen invalid, dan symlink escape bila platform memungkinkan.

### Kebijakan keamanan

- Tolak target yang keluar dari root.
- Jangan mengikuti symlink yang membawa operasi keluar dari root.
- Jika operasi aman tidak dapat dipastikan pada platform tertentu, gagalkan dengan diagnostic yang jelas daripada melanjutkan secara spekulatif.
- Jangan menghapus file atau direktori sebagai bagian dari validasi.

### Hasil yang diharapkan

Path validator yang dipakai baik oleh planning maupun writer, dengan satu kebijakan root-containment yang konsisten.

---

## 14. Task 3.9 — Conflict Detection dan Write Policy

### Tujuan

Mendefinisikan perilaku ketika target file sudah ada atau berubah di antara planning dan apply.

### Persyaratan

- [ ] Deteksi file target yang sudah ada.
- [ ] Bedakan target absent, target file, target directory, dan kondisi filesystem yang invalid.
- [ ] Default: jangan overwrite file yang sudah ada secara diam-diam.
- [ ] Tetapkan perilaku conflict per operation secara eksplisit.
- [ ] Jangan mengimplementasikan automatic merge source code.
- [ ] Jangan mengartikan force sebagai delete-and-recreate.
- [ ] Re-check kondisi target sebelum write untuk mengurangi risiko perubahan setelah planning.
- [ ] Tangani konflik yang muncul setelah planning.
- [ ] Laporkan target dan alasan konflik.
- [ ] Uji existing file, existing directory, conflict, dan target yang berubah jika dapat disimulasikan.

### Batas scope

Phase 3 membuat policy dan mekanisme conflict yang diperlukan engine. Ia tidak perlu menambahkan CLI `--force` ke command scaffolding yang belum diimplementasikan.

---

## 15. Task 3.10 — Filesystem Writer dan Safe Writes

### Tujuan

Menerapkan generation plan yang sudah lolos validasi dengan efek samping yang terkendali.

### Persyaratan

- [ ] Writer menerima plan yang sudah divalidasi.
- [ ] Writer tidak memilih template atau membuat keputusan capability.
- [ ] Writer memvalidasi ulang kondisi kritis yang dapat berubah antara plan dan apply.
- [ ] Parent directories dibuat hanya ketika diperlukan oleh operasi yang diizinkan.
- [ ] Jangan menulis ke target yang melanggar root containment.
- [ ] Gunakan temporary-file/atomic replacement bila benar-benar dibutuhkan dan dapat dilakukan dengan aman.
- [ ] Jangan membangun atomicity semu lintas beberapa file.
- [ ] Tangani error create, write, flush, close, rename, dan cleanup sesuai API yang digunakan.
- [ ] Jangan menyembunyikan error cleanup yang penting.
- [ ] Jangan menghapus arbitrary files.
- [ ] Jangan menggunakan shell untuk menulis file.
- [ ] Catat hasil tiap operation secara terstruktur.
- [ ] Jika beberapa operation sudah berhasil sebelum failure, laporkan hasil parsial dengan jujur.
- [ ] Uji permission/error scenarios yang dapat direproduksi pada CI.

### Transaction limitations

Sekumpulan operasi filesystem bukan otomatis transaksi atomik. MVP harus memvalidasi plan terlebih dahulu, meminimalkan partial output, dan melaporkan keadaan aktual jika terjadi kegagalan. Jangan membangun rollback generik kecuali ada requirement yang jelas.

### Hasil yang diharapkan

Writer yang hanya menerapkan operasi yang diizinkan dan menghasilkan operation report yang akurat.

---

## 16. Task 3.11 — Dry-run, Preview, dan Operation Report

### Tujuan

Menyediakan preview yang dapat dipercaya dan laporan hasil operasi yang tidak menyesatkan.

### Persyaratan

- [ ] Dry-run memakai generation plan yang sama dengan eksekusi nyata.
- [ ] Dry-run tidak membuat atau mengubah target directories/files.
- [ ] Dry-run tidak menghapus file.
- [ ] Dry-run tidak membuat temporary files di target root.
- [ ] Preview menyebutkan path relatif, jenis operasi, dan konflik yang diketahui.
- [ ] Preview membedakan operation yang akan dibuat, ditolak, atau di-skip.
- [ ] Operation report membedakan planned, succeeded, skipped, failed, dan not attempted bila relevan.
- [ ] Jangan melaporkan operasi sebagai sukses sebelum hasilnya terverifikasi sesuai kontrak.
- [ ] Output model tidak bergantung pada ANSI colors atau format terminal.
- [ ] Tambahkan tests yang membandingkan filesystem sebelum dan sesudah dry-run.

### Hasil yang diharapkan

Model preview/report yang dapat digunakan oleh CLI pada fase berikutnya tanpa mencampur logika terminal dengan filesystem engine.

---

## 17. Task 3.12 — Testing

### Tujuan

Membuktikan engine bekerja pada kondisi normal, kondisi invalid, dan skenario keamanan penting.

### 17.1 Unit tests

- [ ] Metadata valid dan invalid.
- [ ] Template ID dikenal/tidak dikenal.
- [ ] Asset wajib tersedia/hilang.
- [ ] Template resolution dan unsupported configuration.
- [ ] Context valid, missing values, dan optional values.
- [ ] Rendering valid, invalid syntax, dan missing variables.
- [ ] Output deterministik.
- [ ] Asset selection untuk capability enabled/disabled.
- [ ] Generation plan berisi operasi yang diharapkan.
- [ ] Path validation untuk relative/absolute/traversal paths.
- [ ] Conflict policy.
- [ ] Operation report model.

### 17.2 Integration tests

- [ ] Render fixture menjadi expected content.
- [ ] Planning tidak mengubah filesystem.
- [ ] Writer membuat file yang direncanakan di temporary directory.
- [ ] Existing files tidak ditimpa secara diam-diam.
- [ ] File/directory conflict menghasilkan hasil yang sesuai.
- [ ] Dry-run tidak mengubah file atau struktur directory.
- [ ] Failure di tengah operasi menghasilkan report parsial yang benar.
- [ ] Invalid template/context gagal sebelum writer dipanggil.

### 17.3 Security tests

- [ ] Path traversal ditolak.
- [ ] Absolute paths ditolak.
- [ ] Symlink escape ditolak atau ditangani dengan kebijakan aman.
- [ ] Template content tidak dieksekusi sebagai program.
- [ ] Asset metadata tidak dapat mengarahkan write ke luar root.
- [ ] Kegagalan validasi tidak menghasilkan filesystem mutations.
- [ ] Target yang berubah antara plan dan apply ditangani sesuai kebijakan.

### 17.4 Test hygiene

- Gunakan `tempfile` jika sudah tersedia.
- Jangan menguji di home directory atau repository kerja pengguna.
- Hindari test yang mengandalkan permission behavior yang tidak portabel tanpa conditional/platform notes.
- Jangan menonaktifkan test untuk membuat CI hijau.
- Tambahkan regression tests untuk bug yang ditemukan.

---

## 18. Struktur Modul yang Disarankan

Struktur berikut adalah kandidat, bukan instruksi untuk melakukan refactor besar. Pertahankan pola codebase aktual dan buat hanya modul yang diperlukan.

```text
src/
├── template/
│   ├── mod.rs
│   ├── metadata.rs
│   ├── resolver.rs
│   ├── context.rs
│   ├── asset.rs
│   └── renderer.rs
├── generation/
│   ├── mod.rs
│   ├── plan.rs
│   └── planner.rs
└── filesystem/
    ├── mod.rs
    ├── paths.rs
    ├── conflicts.rs
    ├── writer.rs
    └── report.rs

templates/
└── fixtures/
    └── ...
```

Tanggung jawab:

- `template/metadata.rs`: template metadata model dan validation.
- `template/resolver.rs`: resolve template yang tersedia.
- `template/context.rs`: membangun context dari konfigurasi eksplisit.
- `template/asset.rs`: mendeskripsikan asset dan cara memprosesnya jika diperlukan.
- `template/renderer.rs`: merender teks ke konten di memori.
- `generation/plan.rs`: model operation plan.
- `generation/planner.rs`: menghubungkan resolver, asset selection, context, dan renderer.
- `filesystem/paths.rs`: validasi target path dan root containment.
- `filesystem/conflicts.rs`: pemeriksaan conflict.
- `filesystem/writer.rs`: penerapan operasi filesystem.
- `filesystem/report.rs`: hasil aktual operasi.

Tidak semua file harus dibuat. Jika satu atau dua modul cukup, pilih struktur sederhana. Jangan menggandakan manifest types atau compatibility matrix Phase 2.

---

## 19. Kebijakan Dependency

### Kandidat

| Dependency                  | Tujuan                                                 | Kebijakan                                                                      |
| --------------------------- | ------------------------------------------------------ | ------------------------------------------------------------------------------ |
| `tera`                      | Template rendering                                     | Kandidat utama sesuai architecture; evaluasi escaping dan MSRV terlebih dahulu |
| `serde`                     | Serialize/deserialize context/metadata bila diperlukan | Sudah tersedia; gunakan ulang                                                  |
| `toml`                      | Metadata TOML bila dipilih                             | Sudah tersedia; gunakan ulang                                                  |
| `tempfile`                  | Temporary directories/files untuk tests                | Sudah tersedia sebagai dev dependency                                          |
| Crate temporary/atomic file | Safe temporary writes jika diperlukan                  | Tambahkan hanya bila standard library tidak cukup                              |

### Aturan

1. Periksa `Cargo.toml` dan `Cargo.lock` sebelum menambah dependency.
2. Setiap dependency baru harus memenuhi kebutuhan Phase 3 yang jelas.
3. Verifikasi MSRV, platform support, maintenance, dan license.
4. Jangan menambahkan prompt library, async runtime, remote fetching library, atau plugin dependencies.
5. Jangan menambahkan `walkdir` jika traversal recursive tidak diperlukan.
6. Jangan mengubah dependency tanpa alasan teknis yang terdokumentasi.
7. Perbarui `Cargo.lock` melalui Cargo, bukan edit manual.

---

## 20. Error Model dan Diagnostic

### Persyaratan

- [ ] Review error model yang sudah ada sebelum menambahkan type baru.
- [ ] Definisikan error yang relevan: template tidak ditemukan, metadata invalid, asset hilang, render error, context invalid, path invalid, root escape, conflict, serta filesystem I/O.
- [ ] Gunakan typed errors jika caller perlu membedakan kategori error.
- [ ] Berikan context seperti template ID atau target path jika aman dan membantu.
- [ ] Jangan memasukkan seluruh context atau secret ke dalam error.
- [ ] Pisahkan error domain dari formatting terminal.
- [ ] Jangan menambahkan CLI commands baru untuk mengekspos engine pada Phase 3.
- [ ] Pastikan error yang terjadi sebelum apply tidak menghasilkan write.
- [ ] Pastikan error saat apply melaporkan operasi yang berhasil dan gagal.

---

## 21. Dokumentasi

Update dokumentasi hanya untuk keputusan dan kemampuan yang benar-benar diimplementasikan.

### `ARCHITECTURE.md`

- [ ] Catat template engine yang dipilih dan alasan teknisnya.
- [ ] Dokumentasikan escaping policy untuk file non-HTML.
- [ ] Dokumentasikan template metadata/asset conventions.
- [ ] Dokumentasikan generation plan dan pemisahan rendering/write.
- [ ] Dokumentasikan path safety, symlink policy, conflict behavior, dan batas transaction.
- [ ] Dokumentasikan dry-run semantics.

### `README.md`

- [ ] Jangan menyatakan scaffolding atau templates sudah tersedia jika baru ada engine dan fixtures.
- [ ] Update status hanya jika status project yang ditampilkan perlu disesuaikan.
- [ ] Jangan menambahkan usage commands yang belum benar-benar berfungsi.

### Roadmap Phase 3

- [ ] Checklist setiap task akurat.
- [ ] Acceptance criteria tidak ditandai selesai tanpa bukti.
- [ ] Blocker dan keputusan yang belum dibuat tercatat.

---

## 22. Quality Gates dan Final Review

Jalankan dari root repository:

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
```

Jika CI mendefinisikan checks tambahan, jalankan checks tersebut juga.

Sebelum laporan akhir:

- [ ] Review seluruh `git diff`.
- [ ] Review `git status --short`.
- [ ] Pastikan perubahan pengguna yang sudah ada tidak hilang.
- [ ] Pastikan dependency/lockfile changes disengaja.
- [ ] Pastikan tidak ada credential atau secret di fixtures.
- [ ] Pastikan tidak ada file yang ditulis ke luar temporary test directories selama tests.
- [ ] Pastikan tidak ada perubahan Phase 4 yang menyusup.
- [ ] Jangan commit, push, publish, atau release tanpa instruksi eksplisit.

---

## 23. Acceptance Criteria Phase 3

Phase 3 hanya boleh berstatus `COMPLETE` jika semua kriteria yang berlaku lulus.

| ID       | Acceptance criteria                                                    | Bukti                |
| -------- | ---------------------------------------------------------------------- | -------------------- |
| P3-AC-01 | Baseline Phase 2 dan kondisi Git diperiksa                             | Laporan preflight    |
| P3-AC-02 | Template metadata memiliki kontrak yang jelas                          | Model dan tests      |
| P3-AC-03 | Resolver hanya memilih template/asset yang tersedia                    | Unit tests           |
| P3-AC-04 | Context dibangun secara eksplisit dari konfigurasi tervalidasi         | Unit tests           |
| P3-AC-05 | Renderer menghasilkan output deterministik di memori                   | Unit tests           |
| P3-AC-06 | Escaping dan template errors ditangani dengan benar                    | Unit tests           |
| P3-AC-07 | Asset selection tidak menduplikasi compatibility/capability rules      | Review arsitektur    |
| P3-AC-08 | Generation plan merepresentasikan operasi sebelum apply                | Unit tests           |
| P3-AC-09 | Planning tidak mengubah filesystem                                     | Integration tests    |
| P3-AC-10 | Path traversal dan absolute target paths ditolak                       | Security tests       |
| P3-AC-11 | Root containment dan symlink policy ditangani                          | Tests/review         |
| P3-AC-12 | Konflik file ditangani sesuai write policy                             | Integration tests    |
| P3-AC-13 | Writer melaporkan hasil aktual secara akurat                           | Integration tests    |
| P3-AC-14 | Dry-run tidak mengubah filesystem target                               | Before/after tests   |
| P3-AC-15 | Error sebelum apply tidak menyebabkan partial writes                   | Integration tests    |
| P3-AC-16 | Dokumentasi sesuai implementasi aktual                                 | Review docs          |
| P3-AC-17 | Semua quality gates lulus atau blocker dilaporkan jujur                | Hasil command aktual |
| P3-AC-18 | Tidak ada scaffolding end-to-end atau template API lengkap yang dibuat | Scope review         |

Jika kriteria wajib gagal atau terblokir, status harus `PARTIAL` atau `BLOCKED`, bukan `COMPLETE`.

---

## 24. Definition of Done

Phase 3 selesai jika:

- [ ] Seluruh task in-scope telah selesai atau blocker dijelaskan secara eksplisit.
- [ ] Template resolver, context builder, renderer, planner, path validator, conflict detector, dan writer memiliki tests yang relevan.
- [ ] Rendering dan planning tidak menulis filesystem.
- [ ] File writes hanya dilakukan setelah plan dan path divalidasi.
- [ ] Default policy tidak menimpa file yang sudah ada secara diam-diam.
- [ ] Dry-run tidak memodifikasi target.
- [ ] Error dan partial results dilaporkan dengan jujur.
- [ ] Quality gates dijalankan dan hasil aktual disertakan.
- [ ] Dokumentasi sesuai implementasi.
- [ ] Tidak ada scope creep ke Phase 4.
- [ ] Tidak ada perubahan pengguna yang hilang.
- [ ] Tidak ada commit/push/release tanpa izin.
- [ ] Agent berhenti setelah Phase 3 dan menunggu persetujuan untuk Phase 4.

---

## 25. Urutan Implementasi yang Direkomendasikan

Implementasikan secara incremental dan jalankan test setelah perubahan bermakna:

1. Preflight dan audit Phase 2.
2. Tentukan metadata model serta fixture assets.
3. Implementasikan template resolver.
4. Implementasikan context builder.
5. Evaluasi dan implementasikan renderer.
6. Implementasikan asset selection.
7. Implementasikan generation plan.
8. Implementasikan planning pipeline tanpa writes.
9. Implementasikan path validation dan root containment.
10. Implementasikan conflict detection/write policy.
11. Implementasikan filesystem writer.
12. Implementasikan dry-run model dan operation report.
13. Lengkapi unit/integration/security tests.
14. Update architecture/roadmap docs.
15. Jalankan quality gates dan review diff.
16. Tulis laporan akhir dan berhenti.

Jangan membuat semua modul sekaligus tanpa compile/test. Pilih abstraksi sekecil mungkin yang memenuhi acceptance criteria.

---

## 26. Format Laporan Akhir Agent

Agent wajib melaporkan hasil dalam bahasa Indonesia dengan format berikut:

```text
Phase: 3 — Template and Filesystem Engine
Status: COMPLETE | PARTIAL | BLOCKED

Summary:
- <ringkasan hasil>

Files changed:
- <path>: <tujuan perubahan>

Implemented:
- Template metadata:
- Template resolver:
- Context builder:
- Renderer:
- Asset selection:
- Generation plan:
- Planning pipeline:
- Path validation/root containment:
- Conflict detection:
- Filesystem writer:
- Dry-run/operation report:

Tests:
- <test yang ditambahkan dan skenario yang dibuktikan>

Verification:
- cargo metadata --no-deps --format-version 1: PASS | FAIL | BLOCKED — <hasil aktual>
- cargo check: PASS | FAIL | BLOCKED — <hasil aktual>
- cargo test: PASS | FAIL | BLOCKED — <hasil aktual>
- cargo fmt --check: PASS | FAIL | BLOCKED — <hasil aktual>
- cargo clippy --all-targets --all-features -- -D warnings: PASS | FAIL | BLOCKED — <hasil aktual>
- git diff --check: PASS | FAIL | BLOCKED — <hasil aktual>

Security and filesystem policy:
- Path traversal/root containment:
- Symlink policy:
- Conflict policy:
- Dry-run verification:
- Partial failure behavior:

Deferred / out of scope:
- <item dan alasan>

Known issues / decisions needed:
- <isu, dampak, tindak lanjut>

Recommended next phase:
- Phase 4 — Project Scaffolding
```

Jangan melaporkan command sebagai PASS jika tidak benar-benar dijalankan. Jika environment tidak dapat menjalankan suatu check, tandai `BLOCKED` dan jelaskan alasannya.

---

## 27. Prinsip Akhir

**Bangun engine yang dapat dipercaya sebelum membangun workflow yang menggunakannya.** Template resolver tidak boleh mengambil alih compatibility rules; renderer tidak boleh menulis file; planner tidak boleh mengubah disk; writer tidak boleh membuat keputusan template; dan dry-run tidak boleh menimbulkan efek samping pada target.

Phase 3 hanya membangun serta menguji fondasi tersebut. Setelah acceptance criteria terpenuhi dan laporan diberikan, berhenti. Phase 4 — Project Scaffolding harus dimulai hanya setelah ada persetujuan terpisah.

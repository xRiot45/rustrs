# Product Requirements Document (PRD) - Rustrs --- Rust REST API Project Generator CLI

**Dokumen:** `PRD.md`\
**Produk:** Rustrs CLI\
**Versi PRD:** 1.0.0\
**Status:** Baseline spesifikasi MVP\
**Bahasa dokumentasi:** Indonesia\
**Target implementasi:** AI coding agent, dari bootstrap repository
sampai MVP selesai

------------------------------------------------------------------------

## 1. Ringkasan Produk

Rustrs adalah command-line interface (CLI) yang dibangun menggunakan
Rust untuk membuat project Rust REST API, menghasilkan boilerplate code
di dalam project, serta membantu workflow development sehari-hari.
Pengalaman pengguna terinspirasi dari NestJS CLI: developer dapat
membuat project baru, menghasilkan resource/module/controller/service,
dan menjalankan perintah tooling dari root project.

Rustrs bukan web framework dan tidak menggantikan Cargo. Rustrs adalah
generator dan developer toolkit yang menghasilkan project Rust
berdasarkan template dan konfigurasi yang eksplisit.

### 1.1 Tujuan utama

1. Membuat project REST API Rust baru melalui wizard interaktif atau
    flags.
2. Menyediakan empat template: `enterprise`, `standard`, `simple`, dan
    `minimal`.
3. Menyediakan generator code yang memahami template, framework,
    database, serta capability project.
4. Mengelola konfigurasi project melalui `.rustrs/config.toml`.
5. Membantu workflow development melalui perintah `dev`, `db`, `check`,
    `test`, `lint`, `format`, `doctor`, dan `info`.
6. Menghasilkan project yang konsisten, bisa diuji, terdokumentasi, dan
    mudah dikembangkan.
7. Menyediakan fondasi arsitektur CLI yang memungkinkan template dan
    generator dikembangkan tanpa menulis ulang command layer.

### 1.2 Prinsip produk

- **Template-aware:** struktur code mengikuti template yang dipilih.
- **Configuration-driven:** project yang sudah dibuat menyimpan
    konfigurasi agar CLI tidak terus menanyakan pilihan yang sama.
- **Safe by default:** generator tidak menimpa file pengguna secara
    diam-diam.
- **Predictable:** command yang sama dengan konfigurasi dan versi
    generator yang sama menghasilkan output yang konsisten.
- **Composable:** fitur diaktifkan melalui capability, bukan dengan
    menyalin empat implementasi CLI yang berbeda.
- **Explicit compatibility:** pilihan framework/database hanya
    ditawarkan untuk kombinasi yang benar-benar didukung generator.
- **No hidden magic:** perubahan file dapat dilihat melalui
    `--dry-run`.
- **Rust-first:** CLI ditulis dalam Rust dan output project adalah
    project Rust/Cargo.
- **Incremental delivery:** fitur dibangun dalam fase yang dapat
    diverifikasi melalui test dan acceptance criteria.

------------------------------------------------------------------------

## 2. Sasaran Pengguna

### 2.1 Pengguna utama

- Backend developer Rust yang berulang kali membuat REST API.
- Developer yang ingin struktur project konsisten antar-project.
- Developer yang ingin menghasilkan boilerplate CRUD, migration,
    tests, dan dokumentasi dengan cepat.
- Tim kecil yang ingin menetapkan baseline arsitektur untuk project
    internal.

### 2.2 Kebutuhan pengguna

- Tidak ingin menyalin struktur folder dan file dari project lama.
- Ingin menghasilkan module/resource baru tanpa mendaftarkan setiap
    file secara manual.
- Ingin mengetahui command, route, module, dependency, dan konfigurasi
    project.
- Ingin menjalankan checks umum dengan satu command.
- Ingin mencegah generator merusak file yang sudah dimodifikasi.
- Ingin template ringan maupun template enterprise dengan command
    generator yang konsisten.

### 2.3 Bukan target MVP

- GUI atau dashboard web.
- Cloud-hosted template marketplace.
- Plugin registry publik.
- Remote project management.
- AI code generation yang bergantung pada layanan eksternal.
- Orkestrasi deployment cloud end-to-end.
- Migrasi otomatis antar-semua versi arsitektur project.
- Pengganti Cargo, rustfmt, Clippy, SQLx CLI, Docker, atau Git.

------------------------------------------------------------------------

## 3. Definisi dan Terminologi

- **CLI:** executable `rustrs`.
- **Scaffolding:** membuat project awal dari template.
- **Template:** preset struktur project dan dependency.
- **Capability:** fitur yang bisa diaktifkan atau dimatikan, misalnya
    Redis, OpenAPI, storage, atau audit logging.
- **Generator:** fungsi untuk menghasilkan atau memodifikasi file di
    project yang sudah ada.
- **Resource:** satu feature vertikal, misalnya `products`, yang dapat
    mencakup route, handler/controller, service, repository, DTO, test,
    dan migration.
- **Project root:** direktori yang berisi `Cargo.toml` dan/atau
    `.rustrs/config.toml`.
- **Manifest:** `.rustrs/config.toml`, sumber konfigurasi generator
    untuk project.
- **Dry run:** menampilkan rencana perubahan tanpa mengubah
    filesystem.
- **Safe write:** menulis file baru atau mengubah file yang memang
    dikelola generator tanpa kehilangan perubahan pengguna.
- **Capability-aware generation:** generator hanya menghasilkan bagian
    yang sesuai dengan capability aktif.
- **Support matrix:** daftar kombinasi template, framework, database,
    dan ORM yang sudah diimplementasikan serta diuji.

------------------------------------------------------------------------

## 4. Scope MVP

### 4.1 Termasuk MVP

1. CLI Rust dengan command parsing, help, version, dan error handling.
2. Interactive wizard serta non-interactive flags untuk membuat
    project.
3. Template `enterprise`, `standard`, `simple`, `minimal`.
4. Template rendering dan filesystem engine.
5. Generator module, resource, controller, service, repository,
    entity/model, DTO, middleware, guard, validator, job, event,
    handler, test, dan migration.
6. Project manifest `.rustrs/config.toml`.
7. Project inspector: `info`, `routes`, `modules`, `deps`.
8. Diagnostics: `doctor`, `env check`.
9. Database commands: `migrate`, `rollback`, `status`, `seed`,
    `create-migration`.
10. Development commands: `dev`, `dev --watch`, `dev up`, `dev down`,
    `dev logs`.
11. Quality commands: `check`, `lint`, `test`, `format`, `audit`.
12. OpenAPI/documentation integration sesuai template dan support
    matrix.
13. Docker/Docker Compose generation.
14. CI/CD templates GitHub Actions dan GitLab CI.
15. Shell completion untuk shell yang ditentukan pada implementasi.
16. Testing untuk CLI, template rendering, generated output, dan command
    integration.
17. Dokumentasi penggunaan dan development.
18. Build dan release binaries untuk target yang disepakati.

### 4.2 Tidak wajib untuk MVP

Fitur berikut tidak boleh menghambat rilis MVP: - Plugin system
eksternal. - Remote template registry/marketplace. - Auto-upgrade
struktur project lama. - Deploy ke cloud provider. - GUI. - AI-generated
business logic. - Distributed project management. - Automatic rollback
terhadap semua perubahan source code. - Dukungan seluruh kombinasi
framework/database/ORM yang secara teknis mungkin tanpa batasan.

Jika waktu implementasi terbatas, seluruh fitur MVP harus diprioritaskan
berdasarkan fase dan support matrix di dokumen ini; jangan mengklaim
fitur selesai jika baru berupa stub.

------------------------------------------------------------------------

## 5. Keputusan Teknologi CLI

### 5.1 Bahasa dan runtime

- Bahasa: Rust.
- Edition: Rust 2024.
- Build: Cargo.
- Minimum supported Rust version (MSRV): tetapkan secara eksplisit di
    README dan CI; gunakan versi stabil yang mendukung seluruh
    dependency terpilih.
- Executable: `rustrs`.

### 5.2 Library yang direkomendasikan

Pilih versi stabil yang kompatibel ketika implementasi dimulai, kemudian
pin/lock dependency melalui `Cargo.lock`.

- `clap` dengan derive untuk command parsing.
- `dialoguer` atau `inquire` untuk wizard interaktif; pilih satu
    library.
- `console` dan/atau `indicatif` untuk output terminal dan progress.
- `serde` + `toml` untuk konfigurasi.
- `tera` untuk template rendering, kecuali evaluasi teknis membuktikan
    alternatif yang lebih sesuai.
- `anyhow` untuk error di boundary CLI dan `thiserror` untuk error
    domain internal.
- `walkdir` untuk inspeksi filesystem jika diperlukan.
- `assert_cmd`, `predicates`, dan snapshot testing library yang
    dipilih untuk pengujian CLI/output.
- `tempfile` untuk filesystem test yang terisolasi.

Library yang dipilih harus dicatat di `ARCHITECTURE.md`. Jangan menambah
dependency hanya untuk satu fungsi kecil jika standard library sudah
memadai.

### 5.3 Prinsip dependency

- CLI tidak boleh memerlukan Node.js.
- CLI tidak boleh memerlukan akses internet untuk membuat project dari
    template bawaan.
- Git, Docker, database, SQLx CLI, dan tools lain adalah executable
    eksternal opsional; periksa ketersediaannya sebelum menjalankan
    command terkait.
- Hindari menjalankan shell command melalui string gabungan. Gunakan
    process API dengan argumen terpisah untuk mengurangi masalah quoting
    dan command injection.
- Jangan pernah mencatat secret ke log.

------------------------------------------------------------------------

## 6. Template dan Support Matrix

Empat template harus menggunakan command dan generator engine yang sama,
tetapi menghasilkan struktur dan capability yang berbeda.

### 6.1 Enterprise API

**Tujuan:** baseline paling lengkap dan reference architecture untuk
template lain.

Baseline: - Rust 2024, Tokio, Axum. - PostgreSQL + SQLx sebagai default
yang paling matang. - Clean Architecture dengan feature/module
organization. - Serde, validation, structured application errors. - JWT
access/refresh token dan Argon2 jika authentication diaktifkan. - RBAC
dan permissions. - OpenAPI dengan Utoipa dan UI dokumentasi Scalar atau
opsi yang dikonfigurasi. - `tracing`, health checks, Prometheus metrics,
OpenTelemetry bila diaktifkan. - Redis cache/rate limiting bila
diaktifkan. - Background jobs, email, file storage, audit logging,
idempotency, dan event abstractions sebagai capability opsional. -
Unit/integration/E2E test scaffolding sesuai konfigurasi. - Docker,
Compose, CI/CD, security audit config, dan documentation.

Struktur referensi:

``` text
src/
├── modules/
├── application/
├── infrastructure/
├── interfaces/
├── middleware/
├── errors/
├── shared/
└── main.rs
migrations/
tests/
docs/
```

Resource pada Enterprise harus membentuk vertical slice: - domain
entity/errors, - repository trait, - application use case/service, -
infrastructure repository implementation, - HTTP
DTO/controller/routes, - test, - migration bila memakai database, -
registration/wiring.

Fitur enterprise lanjutan seperti CQRS, Outbox, Saga, multi-tenancy, dan
Kafka tidak harus aktif default. Fitur tersebut tidak boleh ditampilkan
sebagai tersedia jika belum benar-benar diimplementasikan. Untuk MVP,
implementasikan hanya capability yang tercantum dalam konfigurasi dan
support matrix final.

### 6.2 Standard API

Tujuan: struktur modular yang seimbang.

Contoh:

``` text
src/modules/products/
├── controller.rs
├── service.rs
├── repository.rs
├── entity.rs
├── dto.rs
├── routes.rs
├── tests.rs
└── mod.rs
```

Menghasilkan: - handler/controller, - service, - repository, -
entity/model, - DTO dan validation, - routes, - tests, - migration bila
database aktif, - OpenAPI registration bila aktif.

### 6.3 Simple API

Tujuan: sedikit abstraksi dan onboarding mudah.

Contoh:

``` text
src/modules/products/
├── handlers.rs
├── models.rs
├── routes.rs
├── tests.rs
└── mod.rs
```

Business logic sederhana boleh berada di handler atau helper; tidak
wajib membuat service dan repository terpisah.

### 6.4 Minimal API

Tujuan: project paling kecil untuk API sederhana atau prototyping.

Contoh:

``` text
src/
├── handlers/
├── models/
└── routes/
```

Jangan menghasilkan abstraction enterprise jika tidak diminta.

### 6.5 Support matrix yang wajib

Jangan menyatakan setiap kombinasi framework/database/ORM didukung
secara penuh tanpa implementasi dan test.

- **Tier 1 (wajib matang):** Axum + PostgreSQL + SQLx.
- **Tier 2 (wajib sesuai opsi wizard MVP):** kombinasi lain yang
    benar-benar disediakan pada wizard harus memiliki scaffold yang
    buildable dan tes dasar.
- **Tier 3 (tidak ditawarkan pada MVP):** kombinasi yang belum teruji
    harus disembunyikan atau ditandai belum didukung.

Framework yang menjadi pilihan produk: Axum, Actix Web, Rocket.
Database: PostgreSQL, MySQL, SQLite, None. Data access: SQLx, SeaORM,
Diesel, None.

Implementasi harus memiliki compatibility resolver. Jika kombinasi tidak
didukung, tampilkan pesan yang jelas dan sarankan kombinasi valid.
Jangan menghasilkan project rusak hanya demi menampilkan opsi.

------------------------------------------------------------------------

## 7. CLI UX dan Command Contract

Nama executable: `rustrs`. Gunakan subcommand eksplisit dan alias `g`
untuk `generate`.

### 7.1 Root commands

``` text
rustrs --help
rustrs --version
rustrs new <project-name>
rustrs generate <type> <name>
rustrs g <type> <name>
rustrs add <capability>
rustrs info
rustrs routes
rustrs modules
rustrs deps
rustrs doctor
rustrs config <subcommand>
rustrs env check
rustrs db <subcommand>
rustrs dev [options]
rustrs check
rustrs lint
rustrs test
rustrs format
rustrs audit
rustrs docs
rustrs completion <shell>
rustrs update check
```

Command yang belum diimplementasikan tidak boleh muncul sebagai command
sukses/stub. Help harus mencerminkan fitur yang benar-benar tersedia
pada rilis tersebut.

### 7.2 `new`

Contoh:

``` bash
rustrs new ecommerce-api
rustrs new ecommerce-api --template enterprise
rustrs new ecommerce-api --template standard --framework axum
rustrs new ecommerce-api --database postgres --orm sqlx
rustrs new ecommerce-api --non-interactive --defaults
```

Persyaratan: - Interactive mode jika terminal mendukung input dan
pilihan belum lengkap. - Non-interactive mode wajib menerima semua
pilihan penting melalui flags atau default yang terdokumentasi. -
Validasi nama project dan direktori tujuan. - Tampilkan ringkasan
konfigurasi sebelum menulis jika mode interaktif. - Hindari overwrite
tanpa persetujuan/flag. - Buat `.rustrs/config.toml`. - Jalankan
format/build smoke check jika diminta atau jika aman dilakukan; jelaskan
bila dependency download gagal. - Inisialisasi Git hanya jika dipilih
dan Git tersedia. - Tampilkan next steps yang sesuai dengan project yang
dibuat.

### 7.3 `generate` / `g`

Tipe generator MVP: - `module` - `resource` - `controller` - `service` -
`repository` - `entity` - `dto` - `middleware` - `guard` - `validator` -
`job` - `event` - `handler` - `test` - `migration`

Contoh:

``` bash
rustrs g resource products
rustrs g resource products --dry-run
rustrs g resource products --skip-tests
rustrs g controller products
rustrs g migration add_product_status
```

Opsi umum: - `--dry-run`: hanya tampilkan perubahan. - `--force`:
izinkan overwrite sesuai aturan; tetap jelaskan file yang diganti. -
`--skip-tests`: lewati test scaffolding. - `--skip-docs`: lewati
dokumentasi/schema scaffolding. - `--no-interactive`: gagal dengan error
yang membantu jika ada input wajib yang belum ditentukan.

### 7.4 `add`

Capability yang dapat ditambahkan harus berasal dari daftar capability
yang benar-benar diimplementasikan, misalnya: - `auth` - `redis` -
`storage` - `email` - `openapi`

`add` wajib: 1. Membaca manifest. 2. Memvalidasi kompatibilitas
template/framework. 3. Menghitung dependency/config/file changes. 4.
Menampilkan rencana dan meminta persetujuan bila interaktif. 5.
Memperbarui manifest dan file terkait dengan safe write. 6. Tidak
membuat konfigurasi secret dengan nilai palsu yang tampak valid. 7.
Memberikan instruksi langkah selanjutnya.

### 7.5 Exit codes

- `0`: berhasil.
- `1`: kegagalan umum.
- `2`: argumen/konfigurasi tidak valid.
- `3`: konflik filesystem atau project.
- `4`: dependency/executable eksternal tidak tersedia.
- `5`: operasi eksternal gagal (misalnya Docker/database).
- Exit code spesifik boleh disederhanakan jika didokumentasikan dan
    konsisten.

------------------------------------------------------------------------

## 8. Project Manifest

Setiap project hasil scaffolding memiliki:

``` text
.rustrs/
└── config.toml
```

Contoh konseptual:

``` toml
schema_version = 1
project_name = "ecommerce-api"
template = "enterprise"
framework = "axum"
architecture = "clean_modular"
database = "postgres"
data_access = "sqlx"

[features]
auth = true
rbac = true
openapi = true
redis = true
email = false
storage = false
audit_log = true
rate_limit = true
idempotency = false
observability = true
docker = true
cicd = true

[documentation]
provider = "scalar"

[ci]
provider = "github_actions"
```

Ini contoh schema, bukan kewajiban untuk memakai persis semua key di
atas. Schema final harus didefinisikan dalam Rust structs dengan Serde,
diberi `schema_version`, dan didokumentasikan.

Persyaratan: - Parse error menunjukkan lokasi/key yang bermasalah jika
memungkinkan. - Field unknown ditangani secara eksplisit dan tidak
diam-diam mengubah konfigurasi. - Migration schema manifest versi lama
harus dipikirkan sejak awal, meskipun hanya schema v1 yang diperlukan
untuk MVP. - Semua generator membaca manifest yang sama. - `config set`
hanya menerima key yang valid dan value yang sesuai tipe. - Secret
runtime disimpan dalam environment, bukan manifest.

------------------------------------------------------------------------

## 9. Functional Requirements

### FR-001 --- CLI bootstrap

CLI dapat dibangun melalui Cargo dan menampilkan help/version.

**Acceptance criteria** - `cargo build --release` berhasil. -
`rustrs --help` menampilkan command yang tersedia. - `rustrs --version`
menampilkan versi paket. - Argumen invalid menghasilkan pesan yang
berguna dan non-zero exit code.

### FR-002 --- Project scaffolding

CLI membuat project dari empat template yang dipilih.

**Acceptance criteria** - Output memiliki `Cargo.toml`, `src/`, README,
konfigurasi yang diperlukan, dan `.rustrs/config.toml`. - Project root
valid dan nama project dinormalisasi secara konsisten. - Tidak menimpa
direktori yang sudah berisi file tanpa persetujuan eksplisit. - Setiap
template punya test output dan smoke build. - Kombinasi pilihan yang
tidak didukung ditolak sebelum file ditulis.

### FR-003 --- Interactive dan non-interactive wizard

**Acceptance criteria** - Wizard melewatkan pertanyaan yang tidak
relevan. - Memilih `None` untuk database tidak memunculkan pertanyaan
ORM/migration. - Memilih auth `None` tidak menghasilkan auth
routes/secrets yang tidak terpakai. - Non-interactive mode dapat
digunakan pada CI. - Ringkasan akhir mencerminkan konfigurasi yang
benar-benar akan dibuat.

### FR-004 --- Template engine

**Acceptance criteria** - Template memiliki context variables dan
conditional sections. - Rendering error menampilkan template yang gagal
dan alasan. - Template bawaan tersedia offline. - Template tidak dapat
menulis di luar target project akibat path traversal. - Template dan
file generator memiliki test.

### FR-005 --- Safe filesystem operations

**Acceptance criteria** - `--dry-run` tidak membuat, menghapus, atau
memodifikasi file. - File baru dibuat hanya pada target path yang
tervalidasi. - File yang sudah ada tidak ditimpa secara diam-diam. -
File registration tidak diduplikasi ketika generator dijalankan dua
kali. - Error di tengah proses tidak meninggalkan project dalam keadaan
setengah tertulis jika rollback operasi memungkinkan. - Semua planned
changes dapat ditampilkan dengan status `CREATE`, `UPDATE`, `SKIP`, atau
`CONFLICT`.

### FR-006 --- Code generators

**Acceptance criteria** - Setiap generator mengikuti template dan
capability manifest. - Generator menghasilkan file yang valid dan naming
convention konsisten. - Generator mendaftarkan module/routes/wiring bila
dibutuhkan. - Generator tidak mengubah file unrelated. - Test fixture
memeriksa output per template. - Generated project tetap dapat di-build
setelah generator dijalankan.

### FR-007 --- Project configuration

**Acceptance criteria** - `config show`, `config get <key>`, dan
`config set <key> <value>` tersedia. - Nilai yang tidak valid ditolak
tanpa merusak manifest. - Manifest menjadi sumber kebenaran untuk
generator dan inspector. - Secret tidak disimpan ke manifest.

### FR-008 --- Inspector

**Acceptance criteria** - `info` menampilkan template, framework,
database, architecture, dan capability. - `modules` menampilkan module
yang ditemukan. - `routes` menampilkan route dari metadata yang dikelola
generator atau sumber yang dapat dianalisis secara andal. - `deps`
menampilkan dependency project dari `Cargo.toml`. - Jika route dinamis
tidak dapat dideteksi statis, command menjelaskan keterbatasan; jangan
mengarang daftar route.

### FR-009 --- Doctor dan environment check

**Acceptance criteria** - `doctor` memeriksa project root, Rust/Cargo,
manifest, tools relevan, dan konfigurasi. - Tool opsional diberi status
`SKIP`/`NOT CONFIGURED`, bukan error palsu. - `env check` memvalidasi
keberadaan dan format environment variables yang diwajibkan. - Output
tidak pernah mencetak nilai secret. - Diagnostic membedakan `PASS`,
`WARN`, `FAIL`, dan `SKIP`.

### FR-010 --- Database commands

**Acceptance criteria** - Command memeriksa tool/database configuration
sebelum operasi. - Migration dan seed menggunakan konfigurasi project. -
`status` menunjukkan informasi yang benar dari tool/database yang
tersedia. - `create-migration` membuat nama file unik dan aman. -
`rollback` menjelaskan perilaku dan keterbatasan; jangan menghapus data
secara tak terduga. - `reset` bukan command MVP wajib; jika ditambahkan,
harus meminta konfirmasi eksplisit.

### FR-011 --- Development commands

**Acceptance criteria** - `dev` menjalankan aplikasi project. -
`dev --watch` hanya bekerja jika watcher tersedia/diimplementasikan dan
memberi pesan yang jelas. - `dev up/down/logs` mengelola Docker Compose
jika konfigurasi tersedia dan Docker terpasang. - Command eksternal
memakai process API, bukan shell string yang tidak aman. - Sinyal Ctrl-C
diteruskan dengan baik.

### FR-012 --- Quality commands

**Acceptance criteria** - `check`, `lint`, `test`, `format`, `audit`
menjalankan tool yang benar. - Ketiadaan tool eksternal menghasilkan
instruksi instalasi, bukan panic. - Exit code mencerminkan hasil
proses. - `format` memiliki mode check/diff atau dokumentasi jika file
akan dimodifikasi. - `audit` hanya menyatakan pemeriksaan yang
benar-benar dijalankan.

### FR-013 --- Docker dan CI/CD generation

**Acceptance criteria** - Dockerfile menggunakan multi-stage build jika
sesuai. - `.dockerignore` tersedia. - Compose hanya menyertakan service
untuk capability yang dipilih. - CI menjalankan format/check/lint/test
sesuai konfigurasi. - Secret tidak ditulis sebagai credential default
yang berfungsi. - Workflow menggunakan action/version yang valid pada
saat implementasi dan didokumentasikan.

### FR-014 --- Documentation

**Acceptance criteria** - README project menjelaskan setup, env,
database, migration, run, test, dan docs. - README menyesuaikan pilihan
template/capability. - CLI docs menjelaskan setiap command dan option
yang tersedia. - Troubleshooting memuat error umum. - Dokumentasi tidak
mengklaim fitur yang belum tersedia.

### FR-015 --- Distribution

**Acceptance criteria** - Release workflow membangun binary untuk target
yang didukung. - Version ditampilkan melalui `--version`. - Artifact
release diberi nama dan checksum yang konsisten jika checksum
disediakan. - Instruksi install/update terdokumentasi. - Release tidak
dianggap selesai sebelum smoke test binary berhasil.

------------------------------------------------------------------------

## 10. Generator Behavior per Template

### 10.1 Resource generator --- Enterprise

Contoh struktur:

``` text
src/modules/products/
├── domain/
│   ├── entities/product.rs
│   ├── errors.rs
│   └── repositories/product_repository.rs
├── application/
│   ├── commands/create_product.rs
│   ├── commands/update_product.rs
│   ├── commands/delete_product.rs
│   ├── queries/get_product.rs
│   ├── queries/list_products.rs
│   └── services/product_service.rs
├── infrastructure/
│   └── repositories/postgres_product_repository.rs
├── interfaces/
│   └── http/
│       ├── controller.rs
│       ├── routes.rs
│       └── dto/
│           ├── create_product.rs
│           ├── update_product.rs
│           └── product_response.rs
├── tests/
└── mod.rs
```

Migration dibuat bila database dan migration tool aktif. Test, OpenAPI
annotations, validation, route registration, dan wiring dibuat sesuai
capability.

### 10.2 Resource generator --- Standard

``` text
src/modules/products/
├── controller.rs
├── service.rs
├── repository.rs
├── entity.rs
├── dto.rs
├── routes.rs
├── tests.rs
└── mod.rs
```

### 10.3 Resource generator --- Simple

``` text
src/modules/products/
├── handlers.rs
├── models.rs
├── routes.rs
├── tests.rs
└── mod.rs
```

### 10.4 Resource generator --- Minimal

``` text
src/handlers/products.rs
src/models/product.rs
src/routes/products.rs
```

### 10.5 Generator individual

Jenis: - `module` - `resource` - `controller` - `service` -
`repository` - `entity` - `dto` - `middleware` - `guard` - `validator` -
`job` - `event` - `handler` - `test` - `migration`

Semua jenis harus memiliki mapping template per template architecture.
Jika suatu layer tidak ada pada template, generator harus: 1.
menghasilkan implementasi paling sesuai dengan template, atau 2. menolak
dengan pesan jelas dan menunjukkan generator alternatif.

Jangan memaksa struktur Enterprise ke template Minimal.

------------------------------------------------------------------------

## 11. Safe Generation dan File Modification Strategy

File generation adalah bagian berisiko tinggi. Terapkan aturan berikut.

### 11.1 Kategori file

1. **Generator-owned file:** file yang ditandai atau dicatat sebagai
    dikelola generator.
2. **User-owned file:** file source yang dibuat/diubah pengguna.
3. **Shared registration file:** file seperti `mod.rs`, route registry,
    atau manifest yang memerlukan penambahan deklarasi.

### 11.2 Strategi MVP

- Buat file baru tanpa overwrite.
- Untuk shared registration, gunakan marker atau parser/strategi
    deterministic yang diuji.
- Jangan melakukan regex replacement umum pada seluruh source code.
- Bila perubahan otomatis tidak aman, tampilkan conflict dan instruksi
    manual.
- `--force` tidak boleh menjadi izin untuk menghapus seluruh
    direktori.
- `--dry-run` menunjukkan diff jika tersedia, minimal planned
    operations.
- Simpan manifest/config hanya setelah validasi perubahan selesai.
- Gunakan staging/temp files untuk mengurangi risiko operasi parsial.

### 11.3 Rust module registration

Generator harus memperbarui `mod.rs` atau pola module declaration yang
dipilih secara idempotent. Jika proyek menggunakan struktur yang tidak
dikenali, generator harus berhenti dengan pesan yang menjelaskan konflik
daripada menebak.

### 11.4 Route registration

Untuk MVP, tentukan satu route registration convention per
framework/template. Generator mengikuti convention itu. Hindari AST
rewriting kompleks kecuali diperlukan dan sudah diuji. Jika menggunakan
parser Rust seperti `syn`, jelaskan batasan preserve-formatting dan
pastikan output diuji.

------------------------------------------------------------------------

## 12. Architecture CLI

Pisahkan command handling, application logic, template, dan filesystem.

Contoh struktur internal yang disarankan:

``` text
rustrs/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── ARCHITECTURE.md
├── PRD.md
├── templates/
│   ├── projects/
│   │   ├── enterprise/
│   │   ├── standard/
│   │   ├── simple/
│   │   └── minimal/
│   └── generators/
│       ├── module/
│       ├── resource/
│       ├── controller/
│       ├── service/
│       ├── repository/
│       ├── entity/
│       ├── dto/
│       ├── middleware/
│       ├── guard/
│       ├── validator/
│       ├── job/
│       ├── event/
│       ├── handler/
│       ├── test/
│       └── migration/
└── src/
    ├── main.rs
    ├── cli/
    ├── commands/
    ├── config/
    ├── project/
    ├── generator/
    ├── template_engine/
    ├── filesystem/
    ├── compatibility/
    ├── diagnostics/
    ├── process/
    ├── errors/
    └── utils/
```

Ini adalah baseline, bukan larangan membuat modul tambahan. Jangan
menaruh seluruh logika dalam `main.rs`.

### 12.1 Tanggung jawab layer

- `cli`: definisi command, args, help, parsing.
- `commands`: orkestrasi use case CLI.
- `config`: schema dan load/save manifest.
- `project`: deteksi root, metadata, project validation.
- `generator`: generator per tipe.
- `template_engine`: resolver, context, rendering.
- `filesystem`: safe create/update, conflict, dry run, diff.
- `compatibility`: validasi kombinasi
    framework/database/ORM/capability.
- `diagnostics`: doctor dan env check.
- `process`: menjalankan tool eksternal dan meneruskan output/exit
    code.
- `errors`: error domain yang konsisten.
- `utils`: naming conversion dan helper kecil.

### 12.2 Embedded templates

Template bawaan harus tersedia tanpa koneksi internet. Pilih strategi
embedding saat compile (misalnya include directory pada binary) atau
strategi lain yang memberi pengalaman offline dan release yang
reproducible. Dokumentasikan bagaimana template versioned dan bagaimana
kompatibilitasnya dengan versi CLI.

------------------------------------------------------------------------

## 13. Naming Conventions

Input generator perlu dinormalisasi dan divalidasi.

Contoh input: - `user` - `users` - `user-profile` - `UserProfile`

Expected mapping contoh: - File: `user_profile.rs` - Module:
`user_profile` - Type: `UserProfile` - Route: `/users` atau route yang
ditentukan resource metadata.

Aturan: - Tolak path separator dan path traversal. - Tolak nama
kosong/reserved yang tidak valid. - Hindari pluralization otomatis yang
tidak dapat diprediksi; jika pluralization digunakan, buat aturan dan
test eksplisit. - Jangan mengubah nama tabel/route dengan tebakan
tersembunyi. - Untuk singular/plural, sediakan metadata atau konvensi
deterministik.

------------------------------------------------------------------------

## 14. Security Requirements

- Tidak pernah mencetak nilai secret dalam `doctor`, `env check`, atau
    logs.
- Template rendering tidak boleh menulis di luar project root.
- Validasi seluruh path sebelum filesystem write.
- Hindari command injection saat menjalankan `cargo`, `git`, `docker`,
    atau tool lain.
- Jangan membuat default JWT secret atau credential production yang
    diketahui.
- `.env` tidak boleh ditambahkan ke Git secara tidak sengaja;
    `.env.example` boleh.
- Docker dan CI examples tidak boleh menyertakan password nyata.
- Error runtime generated project tidak boleh membocorkan stack trace
    pada production secara default.
- Generated auth harus menggunakan password hashing yang aman jika
    authentication diaktifkan.
- Token refresh harus dapat dicabut bila capability refresh-token
    aktif.
- CORS default harus aman dan configurable; hindari `allow all`
    sebagai production default.
- Rate limiting harus menjelaskan backend dan batasan (in-memory vs
    Redis).
- Dependency audit hanya menjalankan tool yang tersedia dan melaporkan
    hasil sebenarnya.

------------------------------------------------------------------------

## 15. Non-Functional Requirements

### NFR-001 --- Reliability

- Error ditangani tanpa panic untuk input user normal.
- Kegagalan tool eksternal menghasilkan konteks command dan exit code
    yang benar.
- Filesystem changes harus sesedikit mungkin dan dapat diprediksi.

### NFR-002 --- Performance

- CLI startup terasa cepat untuk command umum.
- Template rendering lokal tidak membutuhkan network.
- Jangan mengunduh dependency sebelum user meminta build/check atau
    proses yang membutuhkannya.

### NFR-003 --- Portability

- Source CLI lintas platform.
- Path handling memakai API path Rust, bukan asumsi slash Linux.
- Output terminal tetap terbaca jika warna dinonaktifkan atau output
    diarahkan ke file.
- Release target ditetapkan dan diuji.

### NFR-004 --- Maintainability

- Command layer tidak berisi rendering template.
- Generator berbagi abstraction yang konsisten.
- Template punya metadata dan test.
- Compatibility matrix menjadi satu sumber kebenaran.

### NFR-005 --- Usability

- Error message menjelaskan masalah, lokasi, dan langkah perbaikan.
- Help memiliki contoh penggunaan.
- Wizard hanya menampilkan pertanyaan relevan.
- Output membedakan sukses, warning, skipped, dan failure.

### NFR-006 --- Reproducibility

- Dependency dan template versions dikelola dengan jelas.
- Test snapshot deterministik.
- Generated output tidak menyertakan timestamp acak kecuali diminta.

------------------------------------------------------------------------

## 16. Testing Strategy

Testing wajib dilakukan pada tiga level.

### 16.1 Unit tests

- Naming conversion.
- Manifest parsing/validation.
- Compatibility resolver.
- Template context builder.
- Template rendering.
- Path safety.
- Change planner.
- Safe filesystem writer.
- Exit code mapping.

### 16.2 Integration tests

Jalankan binary/command terhadap temporary directories: - `new` untuk
setiap template. - `generate` untuk setiap tipe yang didukung. -
`--dry-run` tidak mengubah filesystem. - Conflict handling. - Manifest
update. - Module/route registration. - Missing executable behavior. -
Invalid project root. - Config get/set. - Doctor output tanpa
membocorkan secrets.

### 16.3 Generated-project tests

Untuk setiap template dan kombinasi Tier 1: 1. Generate project ke
temporary directory. 2. Pastikan file wajib tersedia. 3. Pastikan
`.rustrs/config.toml` valid. 4. Jalankan `cargo fmt --check`. 5.
Jalankan `cargo check` atau smoke build yang sesuai. 6. Jalankan test
yang bisa dilakukan di CI. 7. Validasi migration/template files. 8.
Generate satu resource. 9. Pastikan generated project masih dapat
di-build.

Database integration tests yang memerlukan Docker harus dijalankan pada
CI yang mendukungnya atau ditandai secara eksplisit sebagai integration
tier; jangan menyatakan lulus jika tidak dijalankan.

### 16.4 Snapshot tests

Gunakan snapshot untuk generated files penting. Snapshot harus
menormalisasi path/temp directory dan nilai nondeterministic agar tidak
flaky.

### 16.5 Definition of Done untuk generator

Generator dianggap selesai hanya jika: - output template tersedia, -
nama/file/path tervalidasi, - idempotency dan conflict behavior diuji, -
module/route wiring benar, - output diuji build jika berlaku, -
dokumentasi command tersedia.

------------------------------------------------------------------------

## 17. Documentation Requirements

Repository wajib memiliki:

- `README.md`: overview, installation, quick start, examples.
- `PRD.md`: dokumen ini.
- `ARCHITECTURE.md`: architecture CLI, layers, template engine,
    filesystem strategy.
- `CONTRIBUTING.md`: cara menjalankan tests, format, lint, dan
    menambah generator.
- `CHANGELOG.md`: perubahan per versi.
- `docs/installation.md`.
- `docs/commands.md`.
- `docs/templates.md`.
- `docs/configuration.md`.
- `docs/generators.md`.
- `docs/troubleshooting.md`.
- `docs/development.md`.

Dokumentasi command harus dibuat/diupdate bersamaan dengan implementasi
command; jangan menunda semua dokumentasi sampai akhir.

------------------------------------------------------------------------

## 18. Release and Distribution

MVP menyediakan: - `rustrs --version`. - Build release binary. - GitHub
Releases. - CI workflow untuk test dan build. - Target binary yang
diputuskan pada fase release planning; rekomendasi awal: Linux x86_64,
Linux aarch64, macOS x86_64, macOS aarch64, Windows x86_64. - Checksum
artifact jika workflow release mendukungnya. - Installation instructions
untuk setiap target yang tersedia. - `rustrs update check` boleh
memeriksa versi terbaru melalui release metadata jika akses internet
tersedia; kegagalan network tidak boleh mengganggu command lokal lain.

Jangan mengimplementasikan auto-update yang mengganti executable secara
diam-diam pada MVP. `update check` cukup memberi tahu versi yang
tersedia dan tautan release.

------------------------------------------------------------------------

## 19. Roadmap Implementasi AI Agent

AI agent harus mengerjakan fase berurutan. Setiap fase wajib berakhir
dengan test, review, dan update dokumentasi. Jangan mulai fase
berikutnya jika fase sekarang memiliki test/build failure yang belum
dipahami.

### Phase 0 --- Repository Bootstrap

- Buat Cargo package.
- Tetapkan nama binary `rustrs`.
- Setup lint/format/test.
- Setup `.gitignore`, README awal, ARCHITECTURE awal.
- Tambahkan CI minimal.
- Pastikan `--help` dan `--version` bekerja.

**Exit criteria:** `cargo fmt --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, dan
`cargo test` lulus.

### Phase 1 --- CLI Foundation

- Definisikan command tree di Clap.
- Definisikan error types dan exit codes.
- Tambahkan output styles dan no-color behavior.
- Tambahkan root project discovery.
- Tambahkan integration tests untuk help/invalid args.

**Exit criteria:** seluruh command dasar yang belum diimplementasikan
tampil dengan status scope yang jujur; jangan gunakan dummy success
handler.

### Phase 2 --- Manifest and Compatibility

- Definisikan `.rustrs/config.toml` schema.
- Implementasikan load/save/validate.
- Implementasikan compatibility matrix.
- Implementasikan config show/get/set.
- Tambahkan schema tests dan invalid configuration tests.

**Exit criteria:** konfigurasi round-trip valid dan kombinasi yang tidak
didukung ditolak.

### Phase 3 --- Template and Filesystem Engine

- Implementasikan template resolver dan renderer.
- Embed template bawaan.
- Implementasikan naming converter.
- Implementasikan change planner: CREATE/UPDATE/SKIP/CONFLICT.
- Implementasikan dry run dan safe write.
- Implementasikan conflict detection dan path traversal protection.

**Exit criteria:** template rendering dan filesystem tests lulus; dry
run tidak mengubah disk.

### Phase 4 --- Project Scaffolding

- Implementasikan wizard.
- Implementasikan non-interactive flags.
- Buat template Minimal.
- Buat template Simple.
- Buat template Standard.
- Buat Enterprise baseline.
- Buat `.rustrs/config.toml` dan README per project.
- Tambahkan smoke test build per template.

**Exit criteria:** setiap template menghasilkan project yang valid untuk
support matrix yang dinyatakan.

### Phase 5 --- Core Code Generators

Implementasikan urutan: 1. module, 2. entity/model, 3. DTO, 4. service,
5. repository, 6. controller/handler, 7. resource, 8. middleware, 9.
guard, 10. validator, 11. test, 12. migration, 13. job, 14. event, 15.
handler.

Resource generator harus menjadi orchestration dari generator terkait,
bukan duplikasi implementasi.

**Exit criteria:** setiap generator punya fixture/snapshot, conflict
test, registration test, dan generated-project smoke test.

### Phase 6 --- Project Inspector and Diagnostics

- `info`, `modules`, `routes`, `deps`.
- `doctor`.
- `env check`.
- Output PASS/WARN/FAIL/SKIP.
- Secret redaction.

**Exit criteria:** command bekerja pada project valid, project tidak
valid, dan environment dengan tool opsional yang tidak terpasang.

### Phase 7 --- Database and Development Workflow

- `db create-migration`, `migrate`, `rollback`, `status`, `seed`.
- `dev`, `dev --watch`, `dev up`, `dev down`, `dev logs`.
- Process execution abstraction.
- Docker Compose detection.

**Exit criteria:** command yang bergantung pada tool eksternal memeriksa
tool, meneruskan output, dan memetakan exit code dengan benar.

### Phase 8 --- Quality, Docs, Docker, CI/CD

- `check`, `lint`, `test`, `format`, `audit`.
- OpenAPI docs helper.
- Dockerfile/Compose generation.
- GitHub Actions dan GitLab CI generation.
- Shell completion.
- Release workflow.

**Exit criteria:** generated workflow dan Docker files diuji secara
struktural; quality commands lulus di repository CLI.

### Phase 9 --- Hardening and MVP Release

- Jalankan semua unit/integration/snapshot/generated-project tests.
- Review security, overwrite, path safety, dan secret redaction.
- Pastikan support matrix sesuai kenyataan.
- Lengkapi README dan docs.
- Build release binaries.
- Uji instalasi binary dari release artifacts.
- Tandai versi MVP.

**Exit criteria:** seluruh release checklist lulus dan tidak ada command
yang mengklaim fitur yang belum diimplementasikan.

------------------------------------------------------------------------

## 20. Definition of MVP Complete

MVP dinyatakan selesai jika semua kriteria berikut terpenuhi:

- [ ] CLI dibangun dengan Rust dan executable bernama `rustrs`.
- [ ] Help, version, error handling, dan exit codes konsisten.
- [ ] Empat template tersedia dan sesuai spesifikasi.
- [ ] Wizard interaktif dan non-interactive scaffolding tersedia.
- [ ] Project manifest `.rustrs/config.toml` dibuat dan divalidasi.
- [ ] Support matrix terdokumentasi dan ditegakkan.
- [ ] Generator inti dan generator tambahan yang disepakati tersedia.
- [ ] Resource generator menghasilkan struktur sesuai template.
- [ ] Module/routes/wiring registration idempotent atau konflik
    ditangani dengan aman.
- [ ] `--dry-run` tidak mengubah filesystem.
- [ ] Generator tidak overwrite source code secara diam-diam.
- [ ] Config manager berfungsi.
- [ ] `info`, `routes`, `modules`, `deps`, `doctor`, dan `env check`
    berfungsi.
- [ ] Database commands berfungsi untuk driver/migration tool yang
    didukung.
- [ ] Development commands menangani tools eksternal yang hilang
    dengan baik.
- [ ] Quality commands menjalankan tools yang benar dan mengembalikan
    exit code yang benar.
- [ ] Docker dan CI/CD templates tersedia sesuai konfigurasi.
- [ ] Dokumentasi CLI dan generated project lengkap.
- [ ] Unit, integration, snapshot, dan generated-project tests lulus.
- [ ] `cargo fmt --check` lulus.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
    lulus.
- [ ] `cargo test` lulus.
- [ ] Release build dan smoke test binary lulus.
- [ ] Tidak ada command penting yang hanya berupa stub atau
    berpura-pura berhasil.

------------------------------------------------------------------------

## 21. Panduan Kerja untuk AI Coding Agent

AI agent yang menerima PRD ini wajib mengikuti aturan kerja berikut.

1. **Audit terlebih dahulu.** Periksa repository, file yang sudah ada,
    git status, Rust toolchain, dan konfigurasi sebelum mengubah apa
    pun.
2. **Buat implementation plan.** Turunkan PRD menjadi fase, task,
    dependency antar-task, dan acceptance criteria.
3. **Jangan mengasumsikan repository kosong.** Pertahankan pekerjaan
    yang sudah ada dan hindari overwrite tanpa memahami konteks.
4. **Implementasikan end-to-end secara bertahap.** Setiap fase harus
    compile dan test sebelum lanjut.
5. **Tidak membuat stub palsu.** Jika fitur belum diimplementasikan,
    tandai belum tersedia, jangan memberi exit code sukses seolah fitur
    berjalan.
6. **Jangan menebak API dependency.** Periksa dokumentasi lokal/versi
    dependency yang digunakan dan compile code.
7. **Jaga satu sumber kebenaran.** Manifest, compatibility matrix,
    template metadata, dan command help harus konsisten.
8. **Jangan menambah scope tanpa alasan.** Fitur di luar MVP hanya
    boleh ditambahkan bila diperlukan untuk memenuhi requirement dan
    tidak memperumit desain tanpa manfaat.
9. **Jalankan test yang relevan setelah setiap perubahan.** Perbaiki
    regression sebelum melanjutkan.
10. **Verifikasi output yang dihasilkan.** Jangan hanya mengetes
    generator engine; build project hasil scaffolding dan code
    generation.
11. **Update dokumentasi.** Command, flags, konfigurasi, dan template
    yang berubah harus terdokumentasi.
12. **Laporkan status berdasarkan bukti.** Bedakan implementasi selesai,
    sebagian, tidak diuji, dan gagal.
13. **Jangan menyimpan secret.** Gunakan placeholder yang jelas dan
    environment variables.
14. **Hindari perubahan destruktif.** Jangan menghapus file, mereset
    Git, atau menimpa project user tanpa persetujuan eksplisit.
15. **Jangan mengklaim MVP selesai** sampai Definition of MVP Complete
    diverifikasi.

### Format laporan akhir setiap fase

AI agent harus melaporkan: - Fitur yang diimplementasikan. - File
penting yang dibuat/diubah. - Command yang berhasil diuji. -
Test/build/lint yang dijalankan dan hasilnya. - Known limitations. -
Acceptance criteria yang belum terpenuhi. - Fase berikutnya.

------------------------------------------------------------------------

## 22. Keputusan yang Harus Dibuat Sebelum Implementasi

AI agent harus memeriksa dan menetapkan keputusan ini pada Phase 0/1,
mencatatnya di `ARCHITECTURE.md`, dan tidak mengubahnya tanpa alasan
teknis:

1. Versi minimum Rust yang didukung.
2. Library prompt interaktif (`dialoguer` atau `inquire`).
3. Template engine (`tera` atau alternatif yang dibenarkan).
4. Strategi embedding template ke binary.
5. Strategi safe update untuk shared registration files.
6. Support matrix final untuk framework/database/ORM.
7. Strategi route inspection: metadata generator atau static analysis.
8. Strategi hot reload dan apakah memerlukan tool eksternal.
9. Target release binaries dan metode distribusi.
10. Daftar capability yang benar-benar masuk MVP versus yang ditunda.

Keputusan harus dipilih berdasarkan kesederhanaan, testability,
portabilitas, dan keandalan. Jangan meminta konfirmasi user untuk
keputusan implementasi kecil yang dapat diambil dengan aman;
dokumentasikan keputusan dan lanjutkan.

------------------------------------------------------------------------

## 23. Contoh End-to-End User Journey

### 23.1 Membuat project

``` bash
rustrs new ecommerce-api
```

User memilih: - Template: Enterprise API. - Framework: Axum. - Database:
PostgreSQL. - Data access: SQLx. - Authentication: JWT. - OpenAPI:
Utoipa + Scalar. - Docker: Yes. - CI/CD: GitHub Actions.

CLI menghasilkan project dan `.rustrs/config.toml`.

### 23.2 Memulai development

``` bash
cd ecommerce-api
rustrs doctor
rustrs env check
rustrs dev up
rustrs db migrate
rustrs dev
```

### 23.3 Membuat resource

``` bash
rustrs g resource products --dry-run
rustrs g resource products
```

CLI menampilkan planned file changes, meminta konfirmasi bila perlu,
menghasilkan resource, memperbarui registrasi, menambahkan
migration/test/docs sesuai konfigurasi, dan memastikan tidak ada file
yang tertimpa secara diam-diam.

### 23.4 Memeriksa hasil

``` bash
rustrs modules
rustrs routes
rustrs check
rustrs test
rustrs format
```

### 23.5 Menyiapkan CI

``` bash
rustrs info
rustrs audit
```

Project sudah memiliki Docker dan CI templates sesuai pilihan saat
scaffolding.

------------------------------------------------------------------------

## 24. Ringkasan

Rustrs MVP adalah CLI Rust untuk project scaffolding, template-aware
code generation, project configuration, diagnostics,
database/development workflow, quality checks, dan template Docker/CI.

Prioritas arsitektur adalah satu generator engine dengan empat template
yang berbagi mekanisme yang sama. Enterprise menjadi reference
architecture dan superset capability; Standard, Simple, dan Minimal
memakai generator yang sama dengan struktur serta capability yang lebih
ringan.

**Ukuran keberhasilan bukan jumlah command yang tersedia, tetapi project
yang dihasilkan benar-benar buildable, generator aman digunakan berulang
kali, konfigurasi konsisten, serta workflow MVP dapat diuji dari awal
sampai akhir.**

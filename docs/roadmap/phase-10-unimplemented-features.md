# Phase 10 — Fitur yang Belum Dikerjakan dan Rencana Implementasinya

> **Project:** `rustrs` — Rust REST API scaffolding CLI  
> **Phase:** 10 — Unimplemented, Partial, and Deferred Features  
> **Recommended repository path:** `docs/roadmap/phase-10-unimplemented-features.md`  
> **Status:** Planned  
> **Primary outcome:** Menyediakan roadmap eksplisit untuk fitur yang saat ini belum dikerjakan, hanya dideklarasikan, atau ditunda, tanpa mengklaim bahwa fitur tersebut sudah tersedia.

---

## 1. Tujuan dan batasan

Dokumen ini mencatat fitur-fitur yang belum dikerjakan di codebase `rustrs` saat ini. Tujuannya adalah memisahkan dengan jelas antara:

1. Fitur yang sudah bekerja dan diverifikasi.
2. Fitur yang baru dideklarasikan di manifest/config tetapi belum menghasilkan integrasi nyata.
3. Generator atau command yang eksplisit ditolak karena infrastruktur pendukung belum ada.
4. Fitur yang memang ditunda untuk post-MVP.
5. Fitur yang baru berupa placeholder/stub.

Dokumen ini **bukan** perintah untuk mengimplementasikan semua fitur sekaligus. Fitur-fitur besar harus dipecah menjadi phase atau task terpisah, dengan verifikasi dan tests yang proporsional.

### Dalam scope

- Inventaris fitur yang belum dikerjakan.
- Task map untuk mengimplementasikannya secara bertahap.
- Kriteria penerimaan untuk setiap task.
- Batasan support matrix dan keamanan.
- Rencana verifikasi dan dokumentasi.

### Di luar scope

- Mengimplementasikan seluruh fitur dalam satu perubahan.
- Menambah framework/database/ORM yang belum teruji.
- Mengklaim capability tersedia jika hanya ada deklarasi manifest.
- Menjalankan deploy, publish, tag, atau release.
- Mengubah core Phase 0–9 tanpa alasan defect yang terverifikasi.
- Menghapus atau melemahkan tests yang sudah ada.

---

## 2. Sumber kebenaran

Sebelum mengimplementasikan item apa pun dari dokumen ini, baca:

1. `PRD.md`
2. `ARCHITECTURE.md`
3. `AGENTS.md`
4. `docs/support-matrix.md`
5. Roadmap phase 0–9 yang relevan
6. Source code, tests, templates, dan dokumentasi yang terkait
7. `README.md`

Ikuti urutan source-of-truth di `AGENTS.md`. Jika ada konflik, laporkan, jangan diam-diam memilih.

---

## 3. Status implementasi saat ini

### 3.1 Sudah bekerja pada MVP

- CLI parsing, help, version, global flags.
- `rustrs new` untuk template `minimal`, `simple`, `standard`, `enterprise`.
- Kombinasi teknologi yang benar-benar didukung: `Axum + PostgreSQL + SQLx`.
- Template rendering dan filesystem engine.
- Generator: `module`, `entity`, `dto`, `service`, `repository`, `controller`/`handler`, `middleware`, `test`, `migration`, `resource`.
- Generator asset: `docker`, `compose`, `ci-github`, `ci-gitlab`, `release`.
- Manifest `.rustrs/config.toml`.
- Inspector: `info`, `modules`, `routes`, `deps`.
- Diagnostics: `doctor`.
- Database: `create-migration`, `migrate`, `rollback`, `status`.
- Development: `dev`, `dev --watch`, `dev up`, `dev down`, `dev logs`.
- Quality: `check`, `lint`, `test`, `format`, `audit`.
- OpenAPI helper: `docs`, `docs --json`.
- Shell completion: `bash`, `zsh`, `fish`, `powershell`, `elvish`.
- Dokumentasi support matrix dan troubleshooting.

### 3.2 Belum dikerjakan, parsial, atau ditunda

Tabel berikut adalah inventaris fitur yang belum lengkap.

| ID   | Fitur                                        | Status saat ini                  | Catatan                                                               |
| ---- | -------------------------------------------- | -------------------------------- | --------------------------------------------------------------------- |
| U-01 | `rustrs add <capability>`                    | Belum ada                        | Capability hanya dideklarasikan di manifest.                          |
| U-02 | `rustrs config <subcommand>`                 | Belum ada                        | Tidak ada command konfigurasi khusus.                                 |
| U-03 | `rustrs env check`                           | Belum ada sebagai command khusus | Sebagian cakupan ada di `doctor`.                                     |
| U-04 | `rustrs update check`                        | Belum ada                        | Tidak ada mekanisme update check.                                     |
| U-05 | `rustrs generate ... --no-interactive`       | Belum ada                        | Flag tidak tersedia pada `GenerateArgs`.                              |
| U-06 | Inisialisasi Git otomatis saat `new`         | Belum ada                        | PRD menyebutkan opsi Git init, tetapi tidak diimplementasikan.        |
| U-07 | Instalasi shell completion otomatis          | Sengaja tidak otomatis           | Hanya generator output; user memasang manual.                         |
| U-08 | Actix Web                                    | Belum didukung                   | CLI dapat menerima nilai, tetapi compatibility resolver menolak.      |
| U-09 | Rocket                                       | Belum didukung                   | Sama seperti Actix Web.                                               |
| U-10 | MySQL                                        | Belum didukung                   | Hanya PostgreSQL.                                                     |
| U-11 | SQLite                                       | Belum didukung                   | Hanya PostgreSQL.                                                     |
| U-12 | Database `none`                              | Belum didukung                   | Tidak ada workflow tanpa database.                                    |
| U-13 | SeaORM                                       | Belum didukung                   | Hanya SQLx.                                                           |
| U-14 | Diesel                                       | Belum didukung                   | Hanya SQLx.                                                           |
| U-15 | Data access `none`                           | Belum didukung                   | Tidak ada mode tanpa data access.                                     |
| U-16 | Capability `auth`                            | Parsial/deklaratif               | Field manifest ada, integrasi generated code belum ada.               |
| U-17 | Capability `rbac`                            | Parsial/deklaratif               | Membutuhkan `auth`, integrasi belum ada.                              |
| U-18 | Capability `openapi` runtime                 | Parsial                          | Helper validasi config saja; runtime code tidak dihasilkan.           |
| U-19 | Capability `redis`                           | Parsial                          | Hanya service di Compose, belum integrasi aplikasi.                   |
| U-20 | Capability `email`                           | Deklaratif                       | Belum ada integrasi.                                                  |
| U-21 | Capability `storage`                         | Deklaratif                       | Belum ada integrasi.                                                  |
| U-22 | Capability `audit_log`                       | Deklaratif                       | Belum ada integrasi.                                                  |
| U-23 | Capability `rate_limit`                      | Deklaratif                       | Belum ada integrasi.                                                  |
| U-24 | Capability `idempotency`                     | Deklaratif                       | Belum ada integrasi.                                                  |
| U-25 | Capability `observability`                   | Deklaratif                       | Belum ada integrasi.                                                  |
| U-26 | Generator `guard`                            | Stub/tidak tersedia              | Eksplisit error karena infrastruktur authorization belum ada.         |
| U-27 | Generator `event`                            | Stub/tidak tersedia              | Eksplisit error karena event dispatcher belum ada.                    |
| U-28 | Generator `event-handler`                    | Stub/tidak tersedia              | Eksplisit error karena event dispatcher belum ada.                    |
| U-29 | Generator `job`                              | Stub/tidak tersedia              | Eksplisit error karena job queue/worker belum ada.                    |
| U-30 | Generator `validator`                        | Stub/tidak tersedia              | Eksplisit error karena validation crate/infra belum ada.              |
| U-31 | `rustrs db seed`                             | Unsupported                      | Tidak ada seeder mechanism.                                           |
| U-32 | Rollback untuk migration up-only             | Parsial                          | Membutuhkan down migration file; generated migration default up-only. |
| U-33 | `none` database workflow                     | Belum didukung                   | Seluruh database workflow hanya Postgres + SQLx.                      |
| U-34 | OpenAPI runtime scaffolding                  | Belum ada                        | Helper hanya validasi.                                                |
| U-35 | Docker build otomatis                        | Belum ada                        | Asset generation tidak menjalankan Docker build.                      |
| U-36 | Binary distribution prebuilt                 | Belum ada                        | Route MVP hanya source/Cargo install.                                 |
| U-37 | Verifikasi macOS                             | Belum diverifikasi               | Hanya Linux.                                                          |
| U-38 | Verifikasi Windows                           | Belum diverifikasi               | Hanya Linux.                                                          |
| U-39 | Exit code spesifik yang terdokumentasi penuh | Belum diverifikasi               | Perlu audit konsistensi.                                              |
| U-40 | Git init otomatis                            | Belum ada                        | Opsional menurut PRD.                                                 |
| U-41 | Capability toggling pasca-scaffold           | Belum ada                        | `add` belum diimplementasikan.                                        |

---

## 4. Preflight audit yang wajib sebelum implementasi

Sebelum mengerjakan item mana pun:

1. Jalankan `git status --short`.
2. Verifikasi command yang diklaim dari `--help` dan source code.
3. Verifikasi support matrix dari `docs/support-matrix.md`.
4. Buat project contoh untuk template yang relevan di direktori temporary.
5. Identifikasi stub/placeholder dengan pencarian source.
6. Catat blocker, missing tool, dan platform limitation.
7. Jangan mengubah core Phase 0–9 kecuali defect terverifikasi mengharuskannya.

---

## 5. Task map

| Task  | Area                                                              | Prioritas     | Ketergantungan               |
| ----- | ----------------------------------------------------------------- | ------------- | ---------------------------- |
| 10.0  | Preflight audit                                                   | Tinggi        | Repository saat ini          |
| 10.1  | Command CLI yang hilang                                           | Tinggi        | 10.0                         |
| 10.2  | Dukungan framework/database/data access tambahan                  | Sedang/Tinggi | 10.0, compatibility resolver |
| 10.3  | Capability system dan `rustrs add`                                | Tinggi        | 10.0, manifest               |
| 10.4  | Auth dan RBAC                                                     | Tinggi        | 10.3                         |
| 10.5  | OpenAPI runtime integration                                       | Sedang        | 10.3, template engine        |
| 10.6  | Redis integration                                                 | Sedang        | 10.3, 10.4                   |
| 10.7  | Email, storage, audit log, rate limit, idempotency, observability | Sedang        | 10.3                         |
| 10.8  | Generator yang masih stub                                         | Sedang        | 10.4, 10.7                   |
| 10.9  | Database seed dan down migration                                  | Sedang        | Phase 7                      |
| 10.10 | Git init dan metadata project                                     | Rendah/Sedang | `rustrs new`                 |
| 10.11 | Distribusi binary                                                 | Sedang        | Phase 9                      |
| 10.12 | Verifikasi cross-platform                                         | Sedang        | CI/environment               |
| 10.13 | Dokumentasi dan support matrix                                    | Tinggi        | Semua task terkait           |
| 10.14 | Tests dan regression coverage                                     | Tinggi        | Semua task terkait           |

---

## 6. Task 10.0 — Preflight audit

### Work items

- [ ] 10.0.1 Inventarisasi command, flags, templates, framework, database, ORM, dan capability.
- [ ] 10.0.2 Bandingkan inventaris dengan PRD dan support matrix.
- [ ] 10.0.3 Verifikasi setiap kombinasi yang diklaim terhadap compatibility resolver dan output generated.
- [ ] 10.0.4 Tandai fitur sebagai `supported`, `partial`, `unsupported`, atau `blocked`.
- [ ] 10.0.5 Identifikasi stub, no-op, placeholder, dan implementasi tidak lengkap.
- [ ] 10.0.6 Catat tool eksternal dan service yang dibutuhkan.
- [ ] 10.0.7 Catat platform yang benar-benar diverifikasi.

### Acceptance criteria

- [ ] **P10-AC-01:** Preflight report membedakan code defect, documentation drift, missing prerequisite, dan unsupported feature.
- [ ] **P10-AC-02:** Tidak ada fitur yang diklaim tersedia tanpa verifikasi.
- [ ] **P10-AC-03:** Support matrix diperbarui hanya jika implementasi dan tests mendukungnya.

---

## 7. Task 10.1 — Command CLI yang hilang

### 10.1.1 `rustrs add <capability>`

**Tujuan:** menambah capability ke project yang sudah ada secara aman.

#### Work items

- [ ] 10.1.1.1 Tambah command dan help text.
- [ ] 10.1.1.2 Baca dan validasi manifest.
- [ ] 10.1.1.3 Validasi kompatibilitas template/framework/capability.
- [ ] 10.1.1.4 Hitung perubahan dependency, config, dan file.
- [ ] 10.1.1.5 Tampilkan plan dan minta konfirmasi pada mode interaktif.
- [ ] 10.1.1.6 Update manifest dan file terkait dengan safe write.
- [ ] 10.1.1.7 Jangan membuat secret dengan nilai palsu yang tampak valid.
- [ ] 10.1.1.8 Berikan next steps setelah perubahan.

#### Acceptance criteria

- [ ] **P10-AC-04:** `rustrs add` hanya menawarkan capability yang benar-benar diimplementasikan.
- [ ] **P10-AC-05:** Unsupported combination ditolak sebelum filesystem mutation.
- [ ] **P10-AC-06:** Manifest dan file terkait diperbarui secara konsisten.
- [ ] **P10-AC-07:** Dry-run tidak mengubah filesystem.

### 10.1.2 `rustrs config <subcommand>`

**Tujuan:** menyediakan command untuk inspeksi/update konfigurasi project.

#### Work items

- [ ] 10.1.2.1 Tentukan subcommand minimal yang diperlukan, misalnya `show`, `validate`.
- [ ] 10.1.2.2 Reuse manifest discovery dan validation.
- [ ] 10.1.2.3 Hindari menampilkan secret.
- [ ] 10.1.2.4 Dokumentasikan command yang benar-benar tersedia.

#### Acceptance criteria

- [ ] **P10-AC-08:** Config command tidak memperkenalkan schema baru tanpa ADR/dokumentasi.
- [ ] **P10-AC-09:** Invalid config ditolak dengan pesan actionable.

### 10.1.3 `rustrs env check`

**Tujuan:** memeriksa environment secara eksplisit.

#### Work items

- [ ] 10.1.3.1 Tentukan apakah command terpisah diperlukan atau `doctor` cukup.
- [ ] 10.1.3.2 Jika diimplementasikan, reuse environment detection yang ada.
- [ ] 10.1.3.3 Dokumentasikan perbedaan dengan `doctor`.

#### Acceptance criteria

- [ ] **P10-AC-10:** Tidak ada duplikasi command yang membingungkan.
- [ ] **P10-AC-11:** Output konsisten dengan `doctor`.

### 10.1.4 `rustrs update check`

**Tujuan:** memeriksa versi CLI yang lebih baru.

#### Work items

- [ ] 10.1.4.1 Tentukan apakah fitur ini masuk MVP atau post-MVP.
- [ ] 10.1.4.2 Jika diimplementasikan, hindari network access tak terduga saat command biasa.
- [ ] 10.1.4.3 Dokumentasikan perilaku offline.

#### Acceptance criteria

- [ ] **P10-AC-12:** Tidak ada network call tanpa consent yang jelas.
- [ ] **P10-AC-13:** Failure jaringan dilaporkan tanpa panic.

### 10.1.5 `rustrs generate --no-interactive`

#### Work items

- [ ] 10.1.5.1 Tambah flag ke `GenerateArgs`.
- [ ] 10.1.5.2 Pastikan generator yang memerlukan input tambahan gagal dengan pesan yang membantu.
- [ ] 10.1.5.3 Tambah tests untuk mode non-interaktif.

#### Acceptance criteria

- [ ] **P10-AC-14:** Mode non-interaktif tidak menunggu input.
- [ ] **P10-AC-15:** Error message actionable.

### 10.1.6 Inisialisasi Git otomatis

#### Work items

- [ ] 10.1.6.1 Tambah opsi `--git` atau prompt yang jelas.
- [ ] 10.1.6.2 Periksa ketersediaan `git` sebelum menjalankan.
- [ ] 10.1.6.3 Gunakan `std::process::Command` dengan argumen terpisah.
- [ ] 10.1.6.4 Jangan commit atau push otomatis.
- [ ] 10.1.6.5 Dokumentasikan bahwa Git init opsional.

#### Acceptance criteria

- [ ] **P10-AC-16:** Git init hanya berjalan jika user memilih atau flag aktif.
- [ ] **P10-AC-17:** Kegagalan Git tidak merusak project yang sudah dibuat.
- [ ] **P10-AC-18:** Tidak ada secret atau credential yang dicatat.

---

## 8. Task 10.2 — Dukungan framework, database, dan data access tambahan

### 10.2.1 Actix Web dan Rocket

#### Work items

- [ ] 10.2.1.1 Audit dependency, template, dan code generation untuk masing-masing framework.
- [ ] 10.2.1.2 Tambahkan compatibility matrix entry hanya setelah scaffold buildable.
- [ ] 10.2.1.3 Update templates, routes, server bootstrap, dan middleware integration.
- [ ] 10.2.1.4 Tambah tests generated project.
- [ ] 10.2.1.5 Dokumentasikan status support.

#### Acceptance criteria

- [ ] **P10-AC-19:** Framework baru hanya ditawarkan jika scaffold lolos `cargo check`/`cargo test`.
- [ ] **P10-AC-20:** Unsupported combination ditolak sebelum write.

### 10.2.2 MySQL dan SQLite

#### Work items

- [ ] 10.2.2.1 Audit SQLx usage, connection string, migration commands.
- [ ] 10.2.2.2 Tambahkan dependency dan feature yang diperlukan.
- [ ] 10.2.2.3 Update template dan env example.
- [ ] 10.2.2.4 Tambah tests generated project.
- [ ] 10.2.2.5 Dokumentasikan perbedaan behavior migration.

#### Acceptance criteria

- [ ] **P10-AC-21:** Database baru tidak merusak Postgres path.
- [ ] **P10-AC-22:** Migration command sesuai database yang dipilih.

### 10.2.3 SeaORM dan Diesel

#### Work items

- [ ] 10.2.3.1 Audit entity/repository/service generation.
- [ ] 10.2.3.2 Tentukan level abstraksi yang realistis.
- [ ] 10.2.3.3 Update compatibility resolver.
- [ ] 10.2.3.4 Tambah tests.
- [ ] 10.2.3.5 Dokumentasikan batasan.

#### Acceptance criteria

- [ ] **P10-AC-23:** Repository generator data-access-aware.
- [ ] **P10-AC-24:** Tidak ada klaim persistence behavior yang tidak diimplementasikan.

### 10.2.4 Database `none` dan data access `none`

#### Work items

- [ ] 10.2.4.1 Tentukan struktur project tanpa database.
- [ ] 10.2.4.2 Nonaktifkan generator database-dependent secara jelas.
- [ ] 10.2.4.3 Update manifest validation.
- [ ] 10.2.4.4 Tambah tests.

#### Acceptance criteria

- [ ] **P10-AC-25:** `repository` dan `migration` ditolak dengan pesan jelas jika database `none`.
- [ ] **P10-AC-26:** Project tanpa database tetap buildable.

---

## 9. Task 10.3 — Capability system dan `rustrs add`

### Work items

- [ ] 10.3.1 Finalisasi daftar capability yang benar-benar akan diimplementasikan.
- [ ] 10.3.2 Buat compatibility rules capability terpusat.
- [ ] 10.3.3 Definisikan dependency/config/file changes per capability.
- [ ] 10.3.4 Implementasikan preview dan approval.
- [ ] 10.3.5 Implementasikan safe write dan rollback minimal.
- [ ] 10.3.6 Update manifest secara konsisten.
- [ ] 10.3.7 Tambah tests.

### Acceptance criteria

- [ ] **P10-AC-27:** Capability tidak dinyalakan hanya di manifest.
- [ ] **P10-AC-28:** `add` menampilkan plan sebelum write.
- [ ] **P10-AC-29:** Unsupported capability ditolak dengan pesan actionable.

---

## 10. Task 10.4 — Auth dan RBAC

### Work items

- [ ] 10.4.1 Pilih library auth yang kompatibel dengan MSRV.
- [ ] 10.4.2 Implementasikan JWT access/refresh token.
- [ ] 10.4.3 Implementasikan password hashing (Argon2).
- [ ] 10.4.4 Tambah middleware/guard yang benar-benar berfungsi.
- [ ] 10.4.5 Implementasikan RBAC dan permissions.
- [ ] 10.4.6 Update generator `guard`.
- [ ] 10.4.7 Tambah tests unit/integration.
- [ ] 10.4.8 Dokumentasikan secret handling dan env vars.

### Acceptance criteria

- [ ] **P10-AC-30:** Generated auth code buildable.
- [ ] **P10-AC-31:** Tidak ada secret real di source/fixture.
- [ ] **P10-AC-32:** Guard generator tidak lagi stub.

---

## 11. Task 10.5 — OpenAPI runtime integration

### Work items

- [ ] 10.5.1 Pilih library OpenAPI yang sesuai (misalnya Utoipa) jika masuk scope.
- [ ] 10.5.2 Generate registration/schema code untuk resource yang didukung.
- [ ] 10.5.3 Update `rustrs docs` agar membedakan validation-only vs runtime helper.
- [ ] 10.5.4 Tambah tests.
- [ ] 10.5.5 Dokumentasikan batasan.

### Acceptance criteria

- [ ] **P10-AC-33:** OpenAPI runtime code hanya dihasilkan jika capability aktif.
- [ ] **P10-AC-34:** Helper tidak mengklaim scaffolding jika hanya validasi.

---

## 12. Task 10.6 — Redis integration

### Work items

- [ ] 10.6.1 Tambah dependency dan client setup.
- [ ] 10.6.2 Integrasi cache/rate limiting jika capability aktif.
- [ ] 10.6.3 Update Compose asset agar konsisten.
- [ ] 10.6.4 Tambah tests.
- [ ] 10.6.5 Dokumentasikan env vars.

### Acceptance criteria

- [ ] **P10-AC-35:** Redis code hanya dihasilkan jika capability aktif.
- [ ] **P10-AC-36:** Project tetap buildable jika Redis tidak aktif.

---

## 13. Task 10.7 — Email, storage, audit log, rate limit, idempotency, observability

### Work items

- [ ] 10.7.1 Definisikan scope minimal untuk masing-masing capability.
- [ ] 10.7.2 Implementasikan abstraksi yang realistis, bukan stub.
- [ ] 10.7.3 Tambah generator atau wiring yang diperlukan.
- [ ] 10.7.4 Tambah tests.
- [ ] 10.7.5 Dokumentasikan batasan dan env vars.

### Acceptance criteria

- [ ] **P10-AC-37:** Capability tidak dihasilkan jika tidak diimplementasikan.
- [ ] **P10-AC-38:** Tidak ada placeholder yang tampak seperti implementasi lengkap.
- [ ] **P10-AC-39:** Generated project buildable untuk kombinasi yang didukung.

---

## 14. Task 10.8 — Generator yang masih stub

### 10.8.1 `guard`

- [ ] 10.8.1.1 Implementasikan auth infrastructure terlebih dahulu.
- [ ] 10.8.1.2 Ganti error stub dengan generator yang berfungsi.
- [ ] 10.8.1.3 Tambah tests.

### 10.8.2 `event` dan `event-handler`

- [ ] 10.8.2.1 Implementasikan event dispatcher minimal.
- [ ] 10.8.2.2 Generate event dan handler yang terhubung.
- [ ] 10.8.2.3 Tambah tests.

### 10.8.3 `job`

- [ ] 10.8.3.1 Implementasikan job queue/worker minimal atau integrasi dengan library yang sesuai.
- [ ] 10.8.3.2 Generate job yang buildable.
- [ ] 10.8.3.3 Tambah tests.

### 10.8.4 `validator`

- [ ] 10.8.4.1 Pilih validation crate yang kompatibel dengan MSRV.
- [ ] 10.8.4.2 Generate validator yang buildable.
- [ ] 10.8.4.3 Tambah tests.

### Acceptance criteria

- [ ] **P10-AC-40:** Generator stub tidak lagi mengembalikan error "not available" untuk capability yang didukung.
- [ ] **P10-AC-41:** Generated code buildable.
- [ ] **P10-AC-42:** Tests mencakup success dan failure.

---

## 15. Task 10.9 — Database seed dan down migration

### 15.1 `rustrs db seed`

#### Work items

- [ ] 10.9.1.1 Tentukan mekanisme seeder yang realistis.
- [ ] 10.9.1.2 Implementasikan command atau hapus dari help jika tidak didukung.
- [ ] 10.9.1.3 Tambah tests.
- [ ] 10.9.1.4 Dokumentasikan batasan.

#### Acceptance criteria

- [ ] **P10-AC-43:** `seed` tidak lagi fake success.
- [ ] **P10-AC-44:** Jika unsupported, command jelas dan exit code sesuai.

### 15.2 Down migration dan rollback

#### Work items

- [ ] 10.9.2.1 Tambah opsi generate down migration.
- [ ] 10.9.2.2 Update migration generator.
- [ ] 10.9.2.3 Validasi rollback.
- [ ] 10.9.2.4 Tambah tests.
- [ ] 10.9.2.5 Dokumentasikan up-only default.

#### Acceptance criteria

- [ ] **P10-AC-45:** Rollback hanya dijanjikan jika down migration tersedia.
- [ ] **P10-AC-46:** Generated migration up-only tetap didokumentasikan.

---

## 16. Task 10.10 — Git init dan metadata project

### Work items

- [ ] 10.10.1 Implementasikan Git init opsional.
- [ ] 10.10.2 Validasi Git availability.
- [ ] 10.10.3 Tambah tests.
- [ ] 10.10.4 Dokumentasikan perilaku.

### Acceptance criteria

- [ ] **P10-AC-47:** Git init tidak otomatis tanpa consent.
- [ ] **P10-AC-48:** Kegagalan Git tidak merusak project.

---

## 17. Task 10.11 — Distribusi binary

### Work items

- [ ] 10.11.1 Tentukan target triple yang akan didukung.
- [ ] 10.11.2 Tambah release workflow build matrix.
- [ ] 10.11.3 Verifikasi archive, checksum, dan metadata.
- [ ] 10.11.4 Dokumentasikan instalasi binary.
- [ ] 10.11.5 Tambah smoke test.

### Acceptance criteria

- [ ] **P10-AC-49:** Binary hanya diiklankan jika dibangun dan diverifikasi.
- [ ] **P10-AC-50:** Source/Cargo install tetap didokumentasikan sebagai route MVP.

---

## 18. Task 10.12 — Verifikasi cross-platform

### Work items

- [ ] 10.12.1 Verifikasi build pada macOS dan Windows jika environment tersedia.
- [ ] 10.12.2 Audit path separator, executable lookup, permission.
- [ ] 10.12.3 Verifikasi shell completion untuk shell yang diklaim.
- [ ] 10.12.4 Update platform claim di README dan support matrix.
- [ ] 10.12.5 Catat blocker jika tidak dapat diverifikasi.

### Acceptance criteria

- [ ] **P10-AC-51:** Platform claim hanya untuk platform yang diverifikasi.
- [ ] **P10-AC-52:** Missing tool menghasilkan diagnostic actionable.

---

## 19. Task 10.13 — Dokumentasi dan support matrix

### Work items

- [ ] 10.13.1 Update README untuk setiap fitur yang benar-benar diimplementasikan.
- [ ] 10.13.2 Update `docs/support-matrix.md`.
- [ ] 10.13.3 Update troubleshooting.
- [ ] 10.13.4 Hapus atau kualifikasikan klaim stale.
- [ ] 10.13.5 Verifikasi contoh command terhadap `--help`.

### Acceptance criteria

- [ ] **P10-AC-53:** Dokumentasi tidak mengklaim fitur yang belum ada.
- [ ] **P10-AC-54:** Semua command documented dapat dijalankan.

---

## 20. Task 10.14 — Testing dan regression coverage

### Work items

- [ ] 10.14.1 Tambah unit tests untuk logic baru.
- [ ] 10.14.2 Tambah integration tests untuk command baru.
- [ ] 10.14.3 Tambah tests generated project untuk kombinasi yang didukung.
- [ ] 10.14.4 Tambah regression tests untuk bug yang ditemukan.
- [ ] 10.14.5 Gunakan temporary directory, jangan menulis ke project nyata.
- [ ] 10.14.6 Verifikasi dry-run immutability.

### Acceptance criteria

- [ ] **P10-AC-55:** Setiap behavior change memiliki verifikasi proporsional.
- [ ] **P10-AC-56:** Tidak ada test yang dilemahkan hanya agar lulus.

---

## 21. Prinsip implementasi

1. Implementasikan hanya task yang disetujui.
2. Jangan menambah dependency tanpa kebutuhan phase yang jelas.
3. Jangan expose kombinasi yang tidak bisa menghasilkan output coherent.
4. Reuse abstraksi yang ada.
5. Safe write, dry-run, dan conflict policy wajib.
6. Error harus actionable.
7. Tidak ada network access tak terduga.
8. Tidak ada secret di source, docs, logs, fixture.
9. Update docs dan tests bersamaan dengan code.
10. Laporkan status jujur: `COMPLETE`, `PARTIAL`, atau `BLOCKED`.

---

## 22. Verifikasi yang disarankan

Untuk setiap perubahan yang menyentuh code:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
git diff --check
```

Untuk fitur generated project:

```bash
cargo metadata --no-deps --format-version 1
cargo check
cargo test
```

Untuk asset delivery:

```bash
# validasi struktural workflow/YAML bila tool tersedia
```

---

## 23. Batasan dan item yang sengaja ditunda

- Plugin system eksternal.
- Remote template registry/marketplace.
- Auto-upgrade struktur project lama.
- Deploy ke cloud provider.
- GUI.
- AI-generated business logic.
- Distributed project management.
- Automatic rollback terhadap semua perubahan source code.
- Dukungan seluruh kombinasi framework/database/ORM tanpa batas.
- Prebuilt binary distribution sampai route tersebut diverifikasi.

---

## 24. Definisi selesai untuk dokumen ini

Dokumen ini dianggap selesai sebagai roadmap jika:

1. Semua fitur yang belum dikerjakan tercatat.
2. Setiap fitur memiliki task dan acceptance criteria.
3. Batasan support matrix jelas.
4. Tidak ada klaim bahwa fitur stub sudah selesai.
5. Verifikasi dan dokumentasi disertakan.

Status dokumen: `PLANNED`  
Implementasi fitur di atas harus dilakukan secara terpisah, bertahap, dan dengan verifikasi.

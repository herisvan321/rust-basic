# 🏛️ RustBasic Architecture

Dokumen ini menjelaskan arsitektur dasar dari RustBasic Framework dan bagaimana komponen-komponennya berinteraksi.

## 1. Standalone CLI & Delegation
RustBasic menggunakan pendekatan **Hybrid CLI**:
- **Global Binary (`rustbasic`)**: Terinstal di sistem melalui `cargo install`. Bertanggung jawab untuk pembuatan project (`new`), manajemen aset, dan scaffolding kode.
- **Local Delegation**: Perintah yang memerlukan runtime project (seperti `migrate` dan `seed`) didelegasikan dari CLI global ke binary project lokal.
- **Mechanism**: CLI global memanggil `cargo run -- <command>` di dalam direktori project. Project mengenali perintah ini di `src/cli.rs`.

## 2. Framework Core (`rustbasic-core`)
Seluruh logika inti framework (routing, database connection, middleware, session) dibungkus dalam crate `rustbasic-core`. Hal ini memungkinkan:
- Pembaruan framework tanpa harus mengubah kode di starter kit.
- Konsistensi antar project RustBasic.

## 3. Web & API Routing
Struktur rute dipisahkan untuk keamanan dan fleksibilitas:
- **Web Routes (`src/routes/web.rs`)**: Digunakan untuk rendering halaman HTML. Dilengkapi middleware **CSRF** untuk keamanan form.
- **API Routes (`src/routes/api.rs`)**: Digunakan untuk komunikasi data (JSON). Dilengkapi middleware **CORS** dan melewati pemeriksaan CSRF.

## 4. Global View Interceptors
RustBasic menggunakan sistem interseptor yang menyuntikkan data secara otomatis ke dalam template **MiniJinja**:
- `user_roles`: Peran pengguna yang sedang login.
- `user_permissions`: Daftar izin pengguna.
- `csrf_token`: Token keamanan untuk form.

## 5. Middleware Stack
Middleware dijalankan dalam urutan berikut:
1. Logging (Request/Response tracking)
2. Security Headers
3. CORS (Hanya untuk API) / CSRF (Hanya untuk Web)
4. Application Logic

# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) khusus untuk framework RustBasic. Alat ini dirancang untuk mempercepat alur kerja pengembangan Anda.

## 🚀 Cara Menjalankan
Gunakan perintah `cargo rustbasic` diikuti dengan sub-perintah yang diinginkan:

```bash
cargo rustbasic <perintah> [argumen]
```

---

## ⚡ Pengembangan (Shortcuts)

Untuk mempermudah alur kerja harian, gunakan alias berikut:

### `cargo serve`
Menjalankan server dalam mode pengembangan:
- **Auto-Watch**: Memantau perubahan pada `src/`, `resources/` (template), dan file `.env`.
- **Live Reload**: Otomatis merestart server dan me-refresh browser (via `tower-livereload`).
- **Opsi**: Menggunakan `-c` (clear screen) agar log tetap bersih.
- **Contoh**: `cargo serve`

---

## 📂 1. Generator Komponen (Scaffolding)

### `make:model`
Membuat file Entity Sea-ORM baru di folder `src/app/models/`.
- **Argumen**: `<NamaModel>`
- **Opsi**: `-m` (Otomatis buat file migrasi terkait)
- **Contoh**: `cargo rustbasic make:model Product -m`

### `make:migration`
Membuat file migrasi Rust (Sea-ORM) baru dengan timestamp otomatis di `database/migrations/`.
- **Argumen**: `<NamaMigration>`
- **Contoh**: `cargo rustbasic make:migration add_price_to_products`

### `make:controller`
Membuat Controller Axum baru di `src/app/http/controllers/` dan otomatis mendaftarkannya di `mod.rs`.
- **Argumen**: `<NamaController>`
- **Contoh**: `cargo rustbasic make:controller Product`

### `make:middleware`
Membuat Middleware Axum baru di `src/app/http/middleware/` dan otomatis mendaftarkannya di `mod.rs`.
- **Argumen**: `<NamaMiddleware>`
- **Contoh**: `cargo rustbasic make:middleware CheckRole`

---

## 🔐 2. Authentication Scaffolding

### `auth` / `make:auth`
- **Integration**: Menambahkan middleware `guest` untuk halaman login dan `auth` untuk dashboard.
- **Forgot Password**: Sistem reset password otomatis via email (SMTP).
- **Remember Me**: Fitur "Ingat Saya" menggunakan cookie yang aman.
- **Email Templates**: Membuat folder `resources/views/emails` dengan desain premium.
- **UI**: Menambahkan tombol Login/Register secara dinamis di halaman Welcome jika fitur terpasang.
- **Logic**: Mengintegrasikan Sea-ORM, Bcrypt, dan validasi secara otomatis.
- **Contoh**: `cargo rustbasic auth`

### `auth back` / `auth:back`
**Fitur Unggulan Penghapusan:**
- **Robust Clean-up**: Secara otomatis membersihkan import, deklarasi route, model, dan file migrasi.
- **Safety**: Memastikan project tetap bisa dikompilasi setelah penghapusan dengan merapikan file `mod.rs`.
- **Note**: Disarankan menjalankan `cargo rustbasic migrate:back` terlebih dahulu jika ingin membersihkan tabel di database.
- **Contoh**: `cargo rustbasic auth back`

---

## 🗄️ 2. Database & Cache

### `migrate`
Menjalankan seluruh file migrasi yang ada ke database (SQLite/MySQL).
- **Contoh**: `cargo rustbasic migrate`

### `migrate:back` / `migrate:rollback`
Membatalkan (rollback) satu migrasi terakhir dari database.
- **Fungsi**: Menjalankan fungsi `down` pada file migrasi terbaru.
- **Penting**: Jalankan ini sebelum menghapus file migrasi (misal sebelum `auth:back`).
- **Contoh**: `cargo rustbasic migrate:back`

### `cache:clear`
Membersihkan sistem secara menyeluruh:
1. Mengosongkan file log di `storage/logs/` (truncate).
2. Menghapus seluruh data sesi di database.
- **Contoh**: `cargo rustbasic cache:clear`
    
### `key:generate`
Membuat kunci aplikasi baru (`APP_KEY`) yang aman.
- **Fungsi**: Menghasilkan 32-byte random key, di-encode ke base64, dan otomatis memperbarui file `.env`.
- **Contoh**: `cargo rustbasic key:generate`

---

## 🔍 3. Monitoring & Security

### `route:list`
Menampilkan tabel daftar rute yang terdaftar di aplikasi (Method, Path, dan Handler).
- **Contoh**: `cargo rustbasic route:list`

### `check:security`
Melakukan audit keamanan aplikasi:
- Cek keberadaan Middleware CSRF.
- Cek library hashing password (Bcrypt).
- Proteksi SQL Injection & XSS.
- Audit kerentanan dependency via `cargo audit`.
- **Contoh**: `cargo rustbasic check:security`

### `check:update`
Menghubungi crates.io untuk mengecek apakah ada versi terbaru dari paket (dependencies) yang Anda gunakan.
- **Contoh**: `cargo rustbasic check:update`

---

## 🚀 4. Build Manager

### `build`
Menjalankan menu interaktif untuk membangun (compile) aplikasi ke berbagai sistem operasi:
- **Target OS**: Native, Windows, Linux, atau macOS.
- **Mode**: Development atau Production (Release).
- **Fitur**: Mendukung `cargo-zigbuild` untuk cross-compilation yang lebih mudah.
- **Contoh**: `cargo rustbasic build`

---

_Dokumentasi ini diekstrak langsung dari implementasi teknis di `src/config/cli.rs`._

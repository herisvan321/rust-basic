# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) khusus untuk framework RustBasic.

## 🚀 Perintah Utama (Shortcuts)
Framework ini menyediakan beberapa cara singkat untuk menjalankan perintah:

### `rustbasic new`
Membuat project RustBasic baru dari template resmi.
```bash
rustbasic new myapp
```

### `rustbasic <perintah>`
Gunakan perintah `rustbasic` di root proyek untuk tugas sehari-hari:
```bash
rustbasic make:controller BlogController
```

---

## ⚡ Pengembangan (Shortcuts)

### `rustbasic serve`
Menjalankan server dalam mode pengembangan dengan fitur:
- **Auto-Watch**: Memantau perubahan pada kode Rust, template, dan konfigurasi.
- **Live Reload**: Otomatis me-refresh browser saat Anda menyimpan perubahan.

---

## 📂 1. Generator Komponen

### `make:controller`
Membuat Controller baru di `src/app/http/controllers/`.
- Perintah: `rustbasic make:controller NamaController`

### `make:model`
Membuat Entity Sea-ORM baru di `src/app/models/`.
- Perintah: `rustbasic make:model Nama -m`

### `make:middleware`
Membuat Middleware Axum baru di `src/app/http/middleware/`.
- Perintah: `rustbasic make:middleware NamaMiddleware`

### `make:seeder`
Membuat file seeder baru di `database/seeders/`.
- Perintah: `rustbasic make:seeder NamaSeeder`

---

## 🗄️ 2. Database & Cache

### `migrate`
Menjalankan semua migrasi database yang belum dieksekusi.
- Perintah: `rustbasic migrate`

### `migrate:refresh`
Melakukan rollback pada seluruh migrasi dan menjalankannya kembali dari awal. Berguna untuk mereset struktur database.
- Perintah: `rustbasic migrate:refresh`

### `migrate:back` (atau `migrate:rollback`)
Membatalkan (rollback) satu langkah migrasi terakhir.
- Perintah: `rustbasic migrate:back`

### `db:seed`
Menjalankan seluruh database seeder yang terdaftar di `database/seeders/mod.rs`.
- Perintah: `rustbasic db:seed`

---

## 🔐 3. Authentication Scaffolding (Breeze)
Pemasangan sistem autentikasi lengkap kini terintegrasi secara otomatis. Anda hanya perlu menambahkan dependensi `rustbasic-breeze` di `Cargo.toml`. Silakan baca [Panduan Autentikasi](auth_cli.md) untuk detail selengkapnya.

---

## 🔍 4. Monitoring & Utilitas

### `route:list`
Menampilkan tabel daftar rute yang aktif di aplikasi Anda.
- Perintah: `rustbasic route:list`

### `key:generate`
Membuat application key (`APP_KEY`) baru di berkas `.env`.
- Perintah: `rustbasic key:generate`

### `cache:clear`
Membersihkan logs dan database session cache.
- Perintah: `rustbasic cache:clear`

### `storage:link`
Menghubungkan folder storage ke folder public agar berkas dapat diakses publik.
- Perintah: `rustbasic storage:link`

### `check:security`
Menjalankan audit pengaturan keamanan dependency proyek.
- Perintah: `rustbasic check:security`

### `check:update`
Memeriksa pembaruan dependency framework di crates.io.
- Perintah: `rustbasic check:update`

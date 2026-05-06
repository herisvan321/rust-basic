# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) khusus untuk framework RustBasic.

## 🚀 Perintah Utama
Gunakan perintah `cargo rustbasic` diikuti dengan sub-perintah:

```bash
cargo rustbasic <perintah> [argumen]
```

---

## ⚡ Pengembangan (Shortcuts)

### `cargo serve`
Menjalankan server dalam mode pengembangan dengan fitur:
- **Auto-Watch**: Memantau perubahan pada kode Rust (`src/`), template (`.rsx`), dan konfigurasi (`.env`).
- **Live Reload**: Otomatis me-refresh browser saat Anda menyimpan perubahan template.
- **RSX Transpilation**: Otomatis mengolah sintaks RSX menjadi Minijinja yang valid.

---

## 📂 1. Generator Komponen

### `make:controller`
Membuat Controller baru di `src/app/http/controllers/`.
- Secara otomatis mereferensikan template `.rsx`.
- Otomatis mendaftarkannya di `mod.rs`.

### `make:model`
Membuat Entity Sea-ORM baru di `src/app/models/`.
- Gunakan `-m` untuk sekaligus membuat file migrasi.

---

## 🔐 2. Authentication Scaffolding

### `auth` / `make:auth`
Mempasang sistem autentikasi lengkap:
- **Views**: Membuat halaman Login, Register, Reset Password menggunakan **sintaks RSX**.
- **Email**: Menghasilkan template email premium di `src/resources/views/emails/`.
- **Logic**: Mengintegrasikan sistem Session, Bcrypt, dan Middleware secara otomatis.

---

## 🔍 3. Monitoring

### `route:list`
Menampilkan tabel daftar rute yang aktif di aplikasi Anda (Method, Path, dan Handler).

---

## 🏗️ 4. Build Manager

### `build`
Menu interaktif untuk melakukan kompilasi aplikasi ke berbagai target OS (Windows, Linux, macOS) dengan optimasi produksi.

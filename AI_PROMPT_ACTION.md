# 🦾 AI Action Center: RustBasic RSX Edition

Gunakan referensi ini untuk memandu asisten AI dalam memodifikasi proyek.

---

## 📂 Struktur Folder (Modular & Clean)

Aplikasi telah dipisahkan menjadi modul-modul kecil untuk skalabilitas tinggi:

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, Gambar)
├── src/resources/
│   └── css/              # CSS files
│   └── js/               # JS files
│   └── views/            # Template .rsx (RSX Syntax)
│       ├── components/   # Modular UI Library (Auto-imported)
│       └── layouts/      # Layouts
├── src/
│   ├── main.rs           # Entry point (Strict Config & Mandatory .env)
│   ├── app/              # Folder Inti Aplikasi (Controllers, Middleware)
│   ├── config/           # Pusat Konfigurasi (Server, Session, View Engine)
│   └── routes/           # Pengaturan rute
├── storage/              # Penyimpanan File & Log
└── .env                  # Environment Variables (Wajib Ada)
```

---

## 🛡️ Standar Penulisan RSX (WAJIB)
AI harus selalu menggunakan standar ini saat memodifikasi tampilan:
1. **Ekstensi**: Selalu gunakan `.rsx`.
2. **Tag Komponen**: Gunakan `<Namespace.Component />`. Contoh: `<Forms.Input />`, `<Buttons.Button />`.
3. **Auto-Import**: Jangan gunakan `{% from ... import ... %}` manual.
4. **Source Protection**: Output HTML otomatis diminifikasi oleh server.

---

# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) lengkap untuk framework RustBasic.

## 🚀 Cara Menjalankan
Gunakan perintah `cargo rustbasic` diikuti dengan sub-perintah yang diinginkan:

```bash
cargo rustbasic <perintah> [argumen]
```

---

## ⚡ Pengembangan (Shortcuts)

### `cargo serve`
Menjalankan server dalam mode pengembangan:
- **RSX Transpilation**: Mengolah sintaks RSX menjadi Minijinja.
- **Auto-Watch**: Memantau perubahan pada `src/`, `src/resources/` (template), dan file `.env`.
- **Live Reload**: Otomatis merestart server dan me-refresh browser.
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
Membangun sistem autentikasi lengkap secara otomatis.
- **Fitur**: Membuat Login, Register, dan Dashboard dengan **sintaks RSX**.
- **Logic**: Mengintegrasikan Sea-ORM, Bcrypt, dan validasi secara otomatis.
- **Contoh**: `cargo rustbasic auth`

### `auth back` / `auth:back`
Menghapus seluruh sistem autentikasi dan mengembalikan project ke kondisi bersih.
- **Contoh**: `cargo rustbasic auth back`

---

## 🗄️ 3. Database & Cache

### `migrate`
Menjalankan seluruh file migrasi yang ada ke database (SQLite/MySQL).
- **Contoh**: `cargo rustbasic migrate`

### `cache:clear`
Membersihkan sistem secara menyeluruh (log dan data sesi).
- **Contoh**: `cargo rustbasic cache:clear`
    
### `key:generate`
Membuat kunci aplikasi baru (`APP_KEY`) yang aman.
- **Contoh**: `cargo rustbasic key:generate`

---

## 🔍 4. Monitoring & Security

### `route:list`
Menampilkan tabel daftar rute yang terdaftar di aplikasi (Method, Path, dan Handler).

### `check:security`
Melakukan audit keamanan aplikasi (CSRF, Bcrypt, XSS protection).

### `check:update`
Mengecek pembaruan dependencies di crates.io.

---

## 🚀 5. Build Manager

### `build`
Menu interaktif untuk kompilasi aplikasi ke berbagai sistem operasi (Cross-build).
- **Contoh**: `cargo rustbasic build`

---

_Dokumentasi ini adalah instruksi operasional untuk AI agar menjaga integritas RustBasic RSX Framework._

# 🦾 AI Action Center: RustBasic Framework (React SPA Edition)

## 📂 Struktur Folder (Modular & Clean)

Aplikasi telah dipisahkan menjadi modul-modul kecil untuk skalabilitas tinggi:

```text
rustbasic/
├── database/             # Lokasi database SQLite, seeder, & migrasi
├── public/               # File statis dan assets hasil build Vite
├── src/resources/
│   ├── js/               # Frontend React SPA (Pages, Components)
│   └── views/            # Template HTML Root (app.rb.html)
├── src/
│   ├── main.rs           # Entry point (Strict Config & Mandatory .env)
│   ├── app/              # Folder Inti Aplikasi (Controllers, Models, Middleware)
│   ├── config/           # Pusat Konfigurasi (Server, Session, Logging)
│   └── routes/           # Pengaturan rute (web.rs)
├── storage/              # Penyimpanan File & Log
├── docs/                 # Dokumentasi Lengkap
└── .env                  # Environment Variables (Wajib Ada)
```

---

## 🛡️ Standar Penulisan React SPA (WAJIB)
AI harus selalu menggunakan standar ini saat memodifikasi tampilan:
1. **Frontend (React SPA)**: WAJIB menggunakan komponen React fungsional (`.jsx`) di bawah `src/resources/js/Pages/` dan disajikan via `inertia(req, component, props)`. DILARANG menggunakan rendering template HTML murni banyak-halaman (MPA).
2. **Interaktivitas**: WAJIB menggunakan React Hooks (`useState`, `useEffect`) dan Inertia hooks (`useForm`, `usePage`). DILARANG keras menambahkan script JQuery, HTMX, atau pustaka manipulasi DOM luar secara acak.
3. **Styling**: Wajib menggunakan utility class Tailwind CSS dengan estetika modern premium (Glassmorphism, Bento card, glowing orbs, dark mode default).
4. **Hybrid Embedding**: Memahami bahwa seluruh HTML templates dan React compiled assets terkompilasi (`public/build`) disematkan (di-embed) langsung ke dalam satu file biner Rust saat rilis produksi.
5. **Rute Internal**: Navigasi antar halaman internal WAJIB menggunakan `<Link>` dari `@inertiajs/react` agar SPA tidak mengalami reload halaman secara penuh.

---

# 🛠️ RustBasic CLI Documentation

Panduan penggunaan alat baris perintah (**CLI**) lengkap untuk framework RustBasic.

## 🚀 Cara Menjalankan
Gunakan perintah `rustbasic` diikuti dengan sub-perintah yang diinginkan:

```bash
rustbasic <perintah> [argumen]
```

---

## ⚡ Pengembangan (Shortcuts)

### `rustbasic serve`
Menjalankan server backend Axum dalam mode pengembangan (Hot-Reload).

### `npm run dev`
Menjalankan Vite dev server untuk instant Hot Module Replacement (HMR) saat development lokal.

---

## 📂 1. Generator (Scaffolding)

### `make:model`
Membuat file Entity Sea-ORM baru di folder `src/app/models/`.
- **Argumen**: `<NamaModel>`
- **Opsi**: `-m` (Otomatis buat file migrasi terkait)
- **Contoh**: `rustbasic make:model Product -m`

### `make:migration`
Membuat file migrasi Rust baru dengan timestamp otomatis di `database/migrations/` menggunakan helper `Schema` & `Blueprint`.
- **Argumen**: `<NamaMigration>`
- **Contoh**: `rustbasic make:migration create_products`

### `make:controller`
Membuat Controller Axum baru di `src/app/http/controllers/`.
- **Argumen**: `<NamaController>`
- **Contoh**: `rustbasic make:controller ProductController`

### `make:middleware`
Membuat Middleware Axum baru di `src/app/http/middleware/`.
- **Argumen**: `<NamaMiddleware>`
- **Contoh**: `rustbasic make:middleware CheckRole`

---

## 🔐 2. Authentication Scaffolding

### `make:auth`
Memasang sistem autentikasi lengkap (Breeze) secara otomatis berbasis React & Inertia SPA.
- **Contoh**: `rustbasic make:auth`

### `auth:back`
Menghapus seluruh sistem autentikasi dan mengembalikan project ke kondisi bersih.
- **Contoh**: `rustbasic auth:back`

---

## 🗄️ 3. Database & Cache

### `migrate`
Menjalankan seluruh file migrasi yang ada ke database (SQLite/MySQL).
- **Contoh**: `rustbasic migrate`

### `migrate:refresh`
Melakukan rollback seluruh migrasi dan menjalankannya ulang dari awal. Sangat berguna saat fase development untuk mereset struktur tabel.
- **Contoh**: `rustbasic migrate:refresh`

### `migrate:back` / `migrate:rollback`
Pembatalan migrasi terakhir (rollback 1 step).
- **Contoh**: `rustbasic migrate:back`

### `cache:clear`
Membersihkan log dan data sesi.
- **Contoh**: `rustbasic cache:clear`
    
### `key:generate`
Membuat kunci aplikasi baru (`APP_KEY`) yang aman di file `.env`.
- **Contoh**: `rustbasic key:generate`

---

## 🔍 4. Monitoring & Security

### `route:list`
Menampilkan tabel daftar rute yang terdaftar di aplikasi (Method, Path, dan Handler).

### `check:security`
Melakukan audit keamanan aplikasi.

---

_Dokumentasi ini adalah instruksi operasional untuk AI agar menjaga integritas RustBasic Framework SPA Edition._

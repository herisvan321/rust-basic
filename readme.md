# 🚀 RustBasic (Axum SPA)

Aplikasi web modern berbasis Rust dengan arsitektur **Laravel-inspired**. Dirancang untuk performa maksimal, keamanan tinggi, dan kemudahan pengembangan.

---

## 💎 Fitur Unggulan
- **⚡ Performa Axum**: Backend super cepat dengan framework Axum 0.8 dan Tokio.
- **📊 Premium Dashboard**: Panel kendali modern dengan statistik real-time dan desain glassmorphism.
- **🐞 Smart Error Reporting**: Halaman debug detail saat pengembangan dan halaman minimalis saat produksi.
- **🗄️ Dual-Database Ready**: Dukungan otomatis untuk **SQLite** dan **MySQL** menggunakan **Sea-ORM**.
- **🔑 Session ala Laravel**: Sistem session dengan skema tabel database standar Laravel.
- **🛡️ CSRF & Security Ready**: Proteksi CSRF terintegrasi dengan HTMX dan Security Headers otomatis.
- **🎨 Modern Monolith UI**: Single Page Application (SPA) experience menggunakan HTMX dan Alpine.js.
- **📂 Modular Structure**: Organisasi kode yang bersih mengikuti pola MVC Laravel.

---

## 📂 Struktur Proyek Terbaru
```text
rustbasic/
├── database/             # SQLite DB & SQL Schema
├── public/               # Static Files (CSS, JS)
├── resources/            # views/ (Minijinja templates)
├── src/
│   ├── main.rs           # Core Initialization
│   ├── config/           # .env configuration
│   ├── database/         # DB Connections & Sessions
│   ├── routes/           # Routing Logic
│   └── app/              # Core Application Logic
│       └── http/         
│           ├── controllers/ # Auth (sub-folder), Dashboard, etc.
│           ├── requests/    # Form Validation
│           └── responses/   # Laravel-style Helpers
└── .env                  # Environment Variables
```

---

## 🚀 Cara Menjalankan

### 1. Persiapan Lingkungan
Salin file `.env.example` ke `.env` (atau buat file `.env` baru) dan sesuaikan:
```env
APP_NAME=RustBasic
APP_PORT=4000
APP_DEBUG=true
DB_CONNECTION=sqlite
DB_DATABASE=rustbasic
SESSION_DRIVER=database
```

### 2. Jalankan Aplikasi
```bash
cargo run
```
Setelah berjalan, akses di:
👉 **[http://localhost:4000](http://localhost:4000)**

---

## 📝 Troubleshooting & Tips
- **Debug Mode**: Aktifkan `APP_DEBUG=true` di `.env` untuk melihat detail error yang cantik saat terjadi masalah render.
- **Database Error?** Jika mendapat error "readonly database", pastikan folder `database/` memiliki izin tulis dan gunakan format `sqlite:database/filename.sqlite` di kode.
- **SPA Experience**: Seluruh navigasi menggunakan `hx-boost`, memberikan sensasi aplikasi instan tanpa reload halaman.

---
*Dibuat dengan ❤️ untuk komunitas Rust Indonesia. Arsitektur Bersih, Desain Mewah.*

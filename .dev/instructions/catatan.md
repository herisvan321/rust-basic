# 📘 Catatan Dokumentasi RustBasic (HTMX Edition)

Dokumentasi ini berisi panduan struktur folder, fitur, dan cara penggunaan framework **RustBasic** (Axum bergaya Monolith dengan filosofi HTMX & Pure CSS).

---

## 📂 1. Struktur Folder (Modular & Clean)

Aplikasi telah dipisahkan menjadi modul-modul kecil untuk skalabilitas tinggi:

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, Gambar)
├── src/resources/
│   └── views/            # Template HTML (Minijinja - .rb.html)
│       └── layouts/      # Layout Utama
├── src/
│   ├── main.rs           # Entry point (Strict Config & Mandatory .env)
│   ├── app/              # Folder Inti Aplikasi (Controllers, Middleware)
│   ├── config/           # Pusat Konfigurasi (Server, Session, Logging)
│   └── routes/           # Pengaturan rute
├── storage/              # Penyimpanan File & Log
└── .env                  # Environment Variables (Wajib Ada)
```

---

## ⚙️ 2. Konfigurasi & Keamanan (Hardened)

Aplikasi menerapkan standar keamanan produksi:

- **Mandatory .env**: Aplikasi akan `panic!` jika file `.env` tidak ditemukan untuk mencegah salah konfigurasi.
- **Session-IP Binding**: Setiap sesi dikunci ke alamat IP pembuatnya. Jika IP berubah secara drastis saat sesi aktif, sistem akan menolak akses untuk mencegah pembajakan sesi.
- **Dual Logging**: 
    - Terminal: Output berwarna untuk visibilitas instan.
    - File: Log bersih (tanpa kode warna ANSI) di `storage/logs/` untuk audit.
- **Cache:Clear**: Perintah `cargo rustbasic cache:clear` sekarang memotong (truncate) file log tanpa menghapusnya, menjaga kompatibilitas dengan server yang sedang berjalan.
- **Flexible Assets & Binary Embedding**: Library HTMX dan File CSS inti dapat ditanam ke dalam file eksekusi (binary) aplikasi untuk performa maksimal. Namun, framework kini juga mendukung penggunaan **CDN eksternal** secara fleksibel, memudahkan integrasi library pihak ketiga tanpa harus meng-host file tersebut secara lokal.
- **Browser Live Reload**: Menggunakan `tower-livereload` yang hanya aktif jika `APP_DEBUG=true`. Fitur ini memungkinkan browser melakukan refresh otomatis setiap kali server melakukan restart atau ada perubahan pada file template/aset (aktif saat `cargo rustbasic serve`).
- **Source Minification**: Output HTML secara default diminifikasi oleh server (spasi/komentar dihapus) untuk melindungi struktur source code.
- **Hybrid Embedding (rust-embed)**: Seluruh folder template (`src/resources/views`) kini di-embed ke dalam binary saat kompilasi rilis. Saat pengembangan (debug), aplikasi tetap membaca dari disk untuk mendukung *Live Reload*.
- **Modern Premium UI**: Framework kini mewajibkan standar desain tinggi (Split-Screen, Glassmorphism) untuk semua modul inti seperti Autentikasi dan Dashboard.

- **Hybrid API & Web Routing**: Pemisahan rute antara `web.rs` (Stateful/Session) dan `api.rs` (Stateless/CORS). Rute API secara otomatis melewati proteksi CSRF agar bisa diakses oleh client eksternal.

---

## 🎨 3. Frontend & UI (HTMX & Pure CSS Philosophy)

RustBasic meninggalkan library JavaScript berat (seperti Alpine.js) dan beralih ke **Murni HTMX + CSS**:

- **HTML Murni & Jinja**: Menggunakan ekstensi `.rb.html`. Tidak menggunakan sistem komponen reaktif yang ajaib, murni HTML dengan kelas utilitas.
- **Floating Alerts**: Notifikasi tidak lagi mendorong konten, melainkan melayang di pojok kanan atas dengan animasi halus.
- **SPA Experience**: Navigasi instan menggunakan `hx-boost` dan `hx-indicator` untuk feedback visual.
- **CORS Support**: API sekarang mendukung CORS secara bawaan (konfigurasi di `src/app/http/middleware/cors.rs`), memungkinkan integrasi dengan frontend modern (React/Next.js/Vue).

---

## 🗄️ 4. Database & Time Management

- **Multi-Database**: Mendukung SQLite dan MySQL via Sea-ORM.
- **RustBasicSessionStore**: Menyimpan IP Address untuk setiap sesi guna keamanan ekstra.
- **Timezone Aware**: Semua fungsi waktu merujuk pada `APP_TIMEZONE` di `.env`.
- **Intelligent Migrations**: Perintah `make:migration:add` secara otomatis mendeteksi pola penambahan kolom dan menghasilkan template `ALTER TABLE`.

---

## 🚀 5. Perintah Pengembangan (CLI Standalone)

RustBasic kini menggunakan binary mandiri `rustbasic`. Anda tidak perlu lagi menggunakan `cargo rustbasic`.

```bash
rustbasic new nama_app        # Membuat project baru dari template resmi
rustbasic serve               # Menjalankan server dengan Hot-Reload
rustbasic key:generate        # Membuat APP_KEY baru di file .env

## Scaffolding (Generator Kode)
rustbasic make:controller <Name>      # Membuat Controller baru
rustbasic make:model <Name> -m        # Membuat Model & Migration
rustbasic make:migration <Name>       # Membuat file migrasi (Create Table)
rustbasic make:migration:add <Col> <Tab> # Membuat migrasi tambah kolom (Alter Table)
rustbasic make:seeder <Name>          # Membuat Seeder baru
rustbasic make:middleware <Name>      # Membuat Middleware baru
rustbasic make:auth                   # Scaffold sistem Login/Register/Dashboard

## Database & Migrasi (Delegasi ke Project Lokal)
rustbasic migrate                     # Jalankan migrasi yang belum dieksekusi
rustbasic migrate:refresh             # Reset semua dan jalankan ulang
rustbasic migrate:rollback            # Rollback langkah terakhir
rustbasic db:seed                     # Jalankan seeder database
```

---

## 🏛️ 6. Arsitektur Delegasi
Perintah database (`migrate`, `seed`) sekarang dijalankan melalui project lokal itu sendiri. CLI Global hanya bertindak sebagai "jembatan" yang memanggil `cargo run -- <command>`. Hal ini memastikan versi skema database selalu sinkron dengan kode project yang sedang Anda kembangkan.

---

_Dokumentasi ini diperbarui pada Mei 2026 mencerminkan Standalone CLI Architecture, Intelligent Migrations, dan API CORS support._

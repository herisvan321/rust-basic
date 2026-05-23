# 📘 Catatan Dokumentasi RustBasic (React SPA Edition)

Dokumentasi ini berisi panduan struktur folder, fitur, dan cara penggunaan framework **RustBasic** (Axum bergaya Monolith dengan integrasi React.js & Inertia.js SPA).

---

## 📂 1. Struktur Folder (Modular & Clean)

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
- **Cache:Clear**: Perintah `rustbasic cache:clear` sekarang memotong (truncate) file log tanpa menghapusnya, menjaga kompatibilitas dengan server yang sedang berjalan.
- **Asset serving & Binary Embedding**: Seluruh file template HTML (`.rb.html`) dan React compiled assets terkompilasi (`public/build/`) disematkan langsung ke dalam satu file biner Rust saat rilis produksi.
- **Browser Live Reload**: Menggunakan `tower-livereload` yang hanya aktif jika `APP_DEBUG=true` untuk sinkronisasi otomatis.
- **Modern Premium UI**: Estetika modern (Glassmorphism, Tema Gelap/Terang, Bento Grid) dengan reaktivitas SPA dari React.
- **Hybrid API & Web Routing**: Pemisahan rute antara `web.rs` (Stateful/Session/Inertia) dan `api.rs` (Stateless/CORS).

---

## 🎨 3. Frontend & UI (React & Inertia.js SPA)

RustBasic mengintegrasikan React.js + Inertia.js untuk pengalaman SPA monolith yang mulus:

- **React Page Components**: Dibuat di `src/resources/js/Pages/` menggunakan `.jsx` dan Tailwind CSS.
- **Inertia Link**: Navigasi bebas refresh halaman penuh menggunakan `<Link href="...">` dari `@inertiajs/react`.
- **Form State Handling**: Menggunakan hook reaktif `useForm` dari Inertia untuk handling form submission, validasi error, dan state loading.
- **Floating Toasts**: Notifikasi melayang dengan transisi visual yang responsif.

---

## 🗄️ 4. Database & Migration Engine (Blueprint Schema)

- **Multi-Database**: Mendukung SQLite dan MySQL via Sea-ORM.
- **Blueprint Schema & Blueprint**: Mendukung penulisan migrasi yang super simpel dan elegan:
  ```rust
  Schema::create(manager, "users", |table| {
      table.id();
      table.string("name").not_null();
      table.string("email").not_null().unique().index();
      table.date_time("email_verified_at").nullable();
      table.string("password").not_null();
      table.timestamps();
  }).await
  ```
- **Lengkap dengan Fitur Relasi & Tipe Data**: Mendukung `uuid()`, `primary_key()`, `foreign()` (cascade, restrict, dll), `index()`, `json()`, `json_binary()`, `float()`, `double()`, dll.
- **Database Seeding**: Mengisi database secara modular dengan `rustbasic db:seed` dan mendaftarkan seeder di `src/app/seeder.rs`.

---

## 🚀 5. Perintah Pengembangan (CLI Standalone)

RustBasic menggunakan binary mandiri `rustbasic`.

```bash
rustbasic new nama_app        # Membuat project baru dari template resmi
rustbasic serve               # Menjalankan server backend Axum dengan Hot-Reload
npm run dev                   # Menjalankan Vite dev server untuk React HMR
rustbasic key:generate        # Membuat APP_KEY baru di file .env

## Scaffolding (Generator Kode)
rustbasic make:controller <Name>      # Membuat Controller baru
rustbasic make:model <Name> -m        # Membuat Model & Migration
rustbasic make:migration <Name>       # Membuat file migrasi baru
rustbasic make:seeder <Name>          # Membuat Seeder baru
rustbasic make:middleware <Name>      # Membuat Middleware baru
rustbasic make:auth                   # Scaffold sistem Login/Register/Dashboard (Breeze)

## Database & Migrasi
rustbasic migrate                     # Jalankan migrasi yang belum dieksekusi
rustbasic migrate:refresh             # Reset semua dan jalankan ulang
rustbasic migrate:rollback            # Rollback langkah terakhir
rustbasic db:seed                     # Jalankan seeder database
```

---

_Dokumentasi ini diperbarui pada Mei 2026 mencerminkan React SPA Edition, Standalone CLI, dan Blueprint Migration Schema._

```text
    ██████╗ ██╗   ██╗███████╗████████╗██████╗  █████╗ ███████╗██╗ ██████╗
    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██║██╔════╝
    ██████╔╝██║   ██║███████╗   ██║   ██████╔╝███████║███████╗██║██║     
    ██╔══██╗██║   ██║╚════██║   ██║   ██╔══██╗██╔══██║╚════██║██║██║     
    ██║  ██║╚██████╔╝███████║   ██║   ██████╔╝██║  ██║███████║██║╚██████╗
    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝ ╚═════╝
```

# 🚀 RustBasic Framework (Axum Edition 2026)

Aplikasi web modern berbasis Rust dengan arsitektur **Clean Monolith**. Dirancang untuk performa maksimal, keamanan tinggi, dan pengalaman pengembang yang luar biasa dengan sintaks standar HTML dan Minijinja.

---

## 💎 Fitur Unggulan

### 🚀 Core & Performance
- **⚡ Performa Axum**: Backend super cepat dengan framework Axum 0.8 dan Tokio.
- **🎨 Template System**: Menulis template dengan sintaks HTML murni dan tag Minijinja (`.rb.html`).
- **🛡️ Source Protection**: Otomatis melakukan **Minifikasi HTML** pada output untuk menyembunyikan struktur kode asli dari "View Source".
- **📦 Hybrid Embedding**: Menggunakan `rust-embed` untuk menanam seluruh template ke dalam binary saat rilis, menghasilkan file executable mandiri yang sangat portabel.
- **🔄 Live Reload**: Browser otomatis refresh saat mengubah file kode atau template `.rb.html` (aktif saat `cargo serve`).

### 🎨 UI/UX Architecture
- **🪄 HTMX SPA Experience**: Pengalaman Single Page Application yang ringan tanpa library JavaScript berat.
- **🚀 Premium Design System**: Estetika modern kelas atas (Split-Screen, Glassmorphism) menggunakan utility CSS khusus dan komponen HTML standar.

### 🔐 Hardened Security
- **🛡️ Security First**: Proteksi CSRF otomatis, *Strict Env Enforcement*, dan *Bcrypt Password Hashing*.
- **🌐 Session-IP Binding**: Sesi dikunci berdasarkan IP Address untuk mencegah hijacking sesi.
- **🔑 Password Recovery**: Sistem reset password lengkap dengan token aman dan template email HTML premium.

---

## 🛠️ CLI Terintegrasi

Manajemen proyek penuh melalui alat baris perintah bawaan:
- **Scaffolding**: Otomatis generate Model, Controller, dan Middleware (`make:model`, `make:controller`).
- **Database**: Jalankan atau reset migrasi (`migrate`, `migrate:refresh`, `migrate:back`).
- **Authentication**: Pasang atau copot sistem autentikasi lengkap dengan sekali perintah (`auth`, `auth:back`).
- **Monitoring**: Lihat daftar rute aktif (`route:list`) atau bersihkan cache (`cache:clear`).

---

## 📂 Struktur Proyek Terbaru

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (Gambar, dll)
├── src/resources/
│   ├── css/              # Asset CSS (Embedded)
│   ├── js/               # Asset JS (HTMX Embedded)
│   └── views/            # Template .rb.html (HTML + Minijinja)
│       ├── auth/         # Halaman Login & Register
│       ├── emails/       # Template Email
│       ├── errors/       # Template Error (404, 500, Debug)
│       └── layouts/      # Layout Utama
├── src/
│   ├── main.rs           # Entry point
│   ├── app/              # Folder Inti (Controllers, Models, Services)
│   ├── config/           # Modular Engine (View, DB, Server)
│   └── routes/           # Web Routes
├── storage/              # Storage (Logs, Uploads, etc.)
└── .env                  # Environment Variables (Mandatory)
```

---

## 📚 Pusat Dokumentasi

Akses panduan lengkap untuk setiap aspek framework RustBasic:

### 🛠️ Core & Development
- **[`catatan.md`](catatan.md)**: Ringkasan fitur, keamanan, dan perubahan arsitektur terbaru.
- **[`agents.md`](agents.md)**: Manual prosedur standar (SOP) untuk pengembang/AI Agent.
- **[`AI_PROMPT.md`](AI_PROMPT.md)**: Instruksi sistem khusus untuk asisten AI.

### 🎨 Frontend & Interaksi
- **[`htmx.md`](htmx.md)**: Panduan lengkap interaksi HTMX.

### ⚙️ Backend & Operasional
- **[`docs/cli.md`](docs/cli.md)**: Daftar perintah lengkap `cargo rustbasic`.
- **[`docs/database.md`](docs/database.md)**: Manajemen database dan migrasi Sea-ORM.
- **[`docs/deployment.md`](docs/deployment.md)**: Panduan rilis ke server produksi.

---

## 🚀 Langkah Memulai (Setup & Development)

1.  **Environment**: Salin `.env.example` menjadi `.env`.
    ```bash
    cp .env.example .env
    ```
2.  **App Key**: Generate kunci keamanan unik.
    ```bash
    cargo rustbasic key:generate
    ```
3.  **Database**: Jalankan migrasi.
    ```bash
    cargo rustbasic migrate
    ```
4.  **Auth (Opsional)**: Pasang sistem autentikasi siap pakai.
    ```bash
    cargo rustbasic auth
    ```
5.  **Jalankan Server**: Gunakan mode pengembangan dengan auto-refresh.
    ```bash
    cargo serve
    ```
    _Atau `cargo rustbasic serve` jika ingin melalui menu CLI utama._
    _Akses aplikasi di: 👉 **[http://localhost:4000](http://localhost:4000)**_

---

## 🛡️ Keamanan & Pemeliharaan
- **Session Security**: Sesi divalidasi silang dengan alamat IP pengguna.
- **Source Privacy**: Output HTML diminifikasi secara otomatis untuk menyulitkan "View Source" oleh pihak luar.
- **Log Management**: Gunakan `cargo rustbasic cache:clear` untuk membersihkan log dan sesi.

---

_Dibuat dengan ❤️ untuk ekosistem Rust. Arsitektur Bersih, Sintaks Modern, Kecepatan Cahaya._

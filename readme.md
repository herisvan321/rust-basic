# 🚀 RustBasic (Axum SPA Edition 2026)

Aplikasi web modern berbasis Rust dengan arsitektur **Clean Monolith**. Dirancang untuk performa maksimal, keamanan tinggi, dan pengalaman pengembang yang luar biasa.

---

## 💎 Fitur Unggulan

### 🚀 Core & Performance
- **⚡ Performa Axum**: Backend super cepat dengan framework Axum 0.8 dan Tokio.
- **📦 Flexible Assets & Binary Embedding**: File CSS/JS inti ditanam ke binary via `include_str!` atau via CDN eksternal secara fleksibel.
- **🔄 Live Reload**: Browser otomatis refresh saat mengubah file `.rs` atau `.html` (aktif saat `APP_DEBUG=true`).

### 🎨 UI/UX Architecture
- **🪄 HTMX SPA Experience**: Pengalaman Single Page Application yang ringan tanpa library JavaScript berat.
- **🧩 Modular Minijinja Macros**: UI dibangun dengan komponen reusable (`forms`, `buttons`, `display`, `overlays`, `feedback`).
- **🚀 Premium Design System**: Desain *Splitscreen UI* modern dan *Premium Dashboard* dengan statistik real-time.
- **🔘 Smart Overlays**: Modal konfirmasi (seperti Logout) menggunakan teknik **CSS Checkbox Hack** (Zero JS).

### 🔐 Hardened Security
- **🛡️ Security First**: Proteksi CSRF otomatis, *Strict Env Enforcement*, dan *Bcrypt Password Hashing*.
- **🌐 Session-IP Binding**: Sesi dikunci berdasarkan IP Address untuk mencegah hijacking sesi.
- **🔑 Password Recovery**: Sistem reset password lengkap dengan token aman dan template email HTML premium.

### 🛠️ Utilities & Communication
- **📧 SMTP Mail Service**: Integrasi `lettre` untuk pengiriman email asinkron (mendukung Gmail, Mailtrap, dll).
- **📅 Time Management**: Penanganan waktu kuat via `chrono` (mendukung `.diff_for_humans()` & Timezone dinamis).
- **📝 Pro-Grade Logging**: Dual-output logging (Terminal berwarna & File audit di `storage/logs/`).

---

## 🚀 Development

Untuk mempermudah pengembangan, pastikan Anda memiliki `cargo-watch` terinstal agar fitur **Auto-Reload** dan **Live Refresh** aktif secara otomatis.

```bash
cargo install cargo-watch
```

---

## 🛠️ Tech Stack & Components

### UI Component Library
Terletak di `resources/views/components/`:
- **`forms.html`**: Input field, checkbox, dll.
- **`buttons.html`**: Tombol aksi, link button, tombol kembali.
- **`display.html`**: Card, Premium Alerts (Floating), Stat Cards.
- **`overlays.html`**: Modal dinamis dan Konfirmasi Logout.
- **`feedback.html`**: Loading Spinners dan HTMX Indicators.

---

## 📂 Struktur Proyek Terbaru

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, Gambar)
├── resources/
│   ├── css/              # Asset CSS (Hidden/Embedded)
│   ├── js/               # Asset JS (Hidden/Embedded)
│   └── views/            # Template HTML (Minijinja)
│       ├── auth/         # Halaman Login & Register
│       ├── emails/       # Template Email (Reset Password, dll)
│       ├── components/   # Modular UI Components
│       ├── errors/       # Template Error (404, 500, Debug)
│       └── layouts/      # Layout Utama
├── src/
│   ├── main.rs           # Entry point (Strict Config)
│   ├── app/              # Core Application Logic
│   │   ├── controllers/  # Logika Request-Response
│   │   ├── models/       # Definisi Tabel & Entity
│   │   └── services/     # Layanan Pendukung (MailService, dll)
│   ├── config/           # Modular Configuration (DB, Session, Server, Log)
│   └── routes/           # Web Routes
├── storage/              # Storage (Logs, Uploads, etc.)
└── .env                  # Environment Variables (Mandatory)
```

---

## 📚 Pusat Dokumentasi

Akses panduan lengkap untuk setiap aspek framework RustBasic:

### 🛠️ Core & Development
- **[`catatan.md`](catatan.md)**: Ringkasan fitur, keamanan, dan perubahan arsitektur terbaru.
- **[`agents.md`](agents.md)**: Manual prosedur standar (Standard Operating Procedure) untuk pengembang.
- **[`AI_PROMPT.md`](AI_PROMPT.md)**: Instruksi sistem khusus jika Anda bekerja dengan asisten AI (seperti ChatGPT/Claude).

### 🎨 Frontend & Interaksi
- **[`htmx.md`](htmx.md)**: **"The Complete HTMX Bible"** - Panduan lengkap interaksi HTMX.
- **[`docs/views.md`](docs/views.md)**: Panduan pembuatan template MiniJinja dan penggunaan UI Macro.

### ⚙️ Backend & Operasional
- **[`docs/deployment.md`](docs/deployment.md)**: Panduan lengkap deployment ke server produksi (Systemd, Nginx, Perms).
- **[`docs/cli.md`](docs/cli.md)**: Daftar perintah baris perintah (`cargo rustbasic`) untuk scaffolding.
- **[`docs/database.md`](docs/database.md)**: Dokumentasi manajemen database dan migrasi Sea-ORM.
- **[`docs/http.md`](docs/http.md)**: Detail mengenai penanganan Request, Response, dan Middleware.

---

## 📥 Instalasi & Setup

Pilih metode instalasi yang paling sesuai dengan alur kerja Anda:

### 1. Metode Git Clone (Direkomendasikan)
Cocok jika Anda ingin tetap mendapatkan pembaruan framework terbaru.
- **Kelebihan**: Mudah melakukan `git pull` untuk update fitur dan perbaikan bug.
- **Kekurangan**: Folder `.git` tetap ada, membawa riwayat framework induk.

### 2. Metode Download ZIP
Cocok untuk memulai proyek baru yang benar-benar terpisah dari framework asli.
- **Kelebihan**: File bersih, tidak perlu Git, langsung siap pakai.
- **Kekurangan**: Sulit untuk menyinkronkan fitur baru dari framework di masa depan.

---

## 🚀 Langkah Memulai (Setup & Development)

Ikuti urutan ini agar aplikasi berjalan sempurna tanpa hambatan:

1.  **Environment**: Salin `.env.example` menjadi `.env`.
    ```bash
    cp .env.example .env
    ```
2.  **App Key**: Generate kunci keamanan unik untuk aplikasi Anda.
    ```bash
    cargo rustbasic key:generate
    ```
3.  **Database**: Jalankan migrasi untuk menyiapkan struktur tabel.
    ```bash
    cargo rustbasic migrate
    ```
4.  **Auth (Opsional)**: Pasang sistem autentikasi siap pakai jika dibutuhkan.
    ```bash
    cargo rustbasic auth
    ```
5.  **Jalankan Server**: Gunakan perintah singkat untuk mengaktifkan mode pengembangan (Auto-Reload + Live Refresh).
    ```bash
    cargo serve
    ```
    _Akses aplikasi di: 👉 **[http://localhost:4000](http://localhost:4000)**_

    *(Untuk kontrol manual, gunakan: `cargo watch -w src -w resources -x run`)*

---

## 🛡️ Keamanan & Pemeliharaan

- **Session Security**: Sesi divalidasi silang dengan alamat IP pengguna.
- **Log Management**: Gunakan `cargo rustbasic cache:clear` untuk membersihkan log dan sesi tanpa mengganggu server.
- **Key Security**: Jangan pernah membagikan `APP_KEY` Anda di repositori publik.

---

_Dibuat dengan ❤️ untuk ekosistem Rust. Arsitektur Bersih, Desain Premium, Kecepatan Cahaya._

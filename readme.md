# 🚀 RustBasic (Axum RSX Edition 2026)

Aplikasi web modern berbasis Rust dengan arsitektur **Clean Monolith**. Dirancang untuk performa maksimal, keamanan tinggi, dan pengalaman pengembang yang luar biasa dengan sintaks RSX (Rust-style XML).

---

## 💎 Fitur Unggulan

### 🚀 Core & Performance
- **⚡ Performa Axum**: Backend super cepat dengan framework Axum 0.8 dan Tokio.
- **🎨 RSX Template System**: Menulis template dengan sintaks RSX (`<Namespace.Component />`) menggunakan engine Minijinja yang dioptimalkan.
- **🛡️ Source Protection**: Otomatis melakukan **Minifikasi HTML** pada output untuk menyembunyikan struktur kode asli dari "View Source".
- **🔄 Live Reload**: Browser otomatis refresh saat mengubah file kode atau template `.rsx` (aktif saat `cargo serve`).

### 🎨 UI/UX Architecture
- **🪄 HTMX SPA Experience**: Pengalaman Single Page Application yang ringan tanpa library JavaScript berat.
- **🧩 Auto-Import Components**: Panggil komponen UI langsung tanpa perlu import manual di dalam template.
- **🚀 Premium Design System**: Desain modern dengan sistem komponen yang modular dan siap pakai.

### 🔐 Hardened Security
- **🛡️ Security First**: Proteksi CSRF otomatis, *Strict Env Enforcement*, dan *Bcrypt Password Hashing*.
- **🌐 Session-IP Binding**: Sesi dikunci berdasarkan IP Address untuk mencegah hijacking sesi.
- **🔑 Password Recovery**: Sistem reset password lengkap dengan token aman dan template email HTML premium.

---

## 🛠️ Tech Stack & Components

### UI Component Library
Terletak di `src/resources/views/components/`:
- **`Forms`**: Input field, checkbox, select, dll.
- **`Buttons`**: Tombol aksi, link button, tombol kembali.
- **`Display`**: Card, Premium Alerts (Floating), Stat Cards.
- **`Overlays`**: Modal dinamis dan Konfirmasi Logout.
- **`Feedback`**: Loading Spinners dan HTMX Indicators.

---

## 📂 Struktur Proyek Terbaru

```text
rustbasic/
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (Gambar, dll)
├── src/resources/
│   ├── css/              # Asset CSS (Embedded)
│   ├── js/               # Asset JS (HTMX Embedded)
│   └── views/            # Template .rsx (RSX Syntax)
│       ├── auth/         # Halaman Login & Register
│       ├── emails/       # Template Email
│       ├── components/   # Modular UI Components (Auto-imported)
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
- **[`docs/views.md`](docs/views.md)**: **[UTAMA]** Panduan lengkap sintaks RSX dan UI Components.
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
4.  **Auth (Opsional)**: Pasang sistem autentikasi RSX siap pakai.
    ```bash
    cargo rustbasic auth
    ```
5.  **Jalankan Server**: Gunakan mode pengembangan dengan auto-refresh.
    ```bash
    cargo serve
    ```
    _Akses aplikasi di: 👉 **[http://localhost:4000](http://localhost:4000)**_

---

## 🛡️ Keamanan & Pemeliharaan
- **Session Security**: Sesi divalidasi silang dengan alamat IP pengguna.
- **Source Privacy**: Output HTML diminifikasi secara otomatis untuk menyulitkan "View Source" oleh pihak luar.
- **Log Management**: Gunakan `cargo rustbasic cache:clear` untuk membersihkan log dan sesi.

---

_Dibuat dengan ❤️ untuk ekosistem Rust. Arsitektur Bersih, Sintaks Modern, Kecepatan Cahaya._

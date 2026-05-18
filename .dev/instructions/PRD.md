# 📄 Product Requirements Document (PRD) Prompt (React SPA Edition)

Gunakan template ini untuk mendefinisikan fitur atau aplikasi baru yang akan dibangun di atas framework **RustBasic** versi **Modern SPA (React + Inertia)**. Masukkan detail aplikasi Anda di bawah setiap section.

---

## 🚀 1. Ringkasan Proyek (Project Overview)
*Deskripsikan secara singkat aplikasi apa yang ingin dibuat.*
*   **Nama Aplikasi**: 
*   **Tujuan Utama**: 
*   **Target Pengguna**: 

---

## 🛠️ 2. Fitur Utama (Core Features)
*Daftar fungsionalitas yang harus ada.*
*   [ ] **Fitur A**: Deskripsi alur kerja komponen.
*   [ ] **Fitur B**: Deskripsi alur kerja komponen.
*   [ ] **Auth System**: Sistem login dan registrasi berbasis React component dengan *state verification* backend Axum.

---

## 📊 3. Struktur Data (Data Model)
*Definisikan entitas yang dibutuhkan untuk database (Sea-ORM).*
*   **Nama Tabel**: `users`
    *   Fields: `id`, `username`, `email`, `password`, `created_at`.
*   **Nama Tabel**: `...`
    *   Fields: `...`

---

## 🎨 4. Antarmuka Pengguna (UI/UX Requirements)
*Sesuai dengan Standar RustBasic: Modern, Premium, HSL Tailwind CSS, Bento Box, Glassmorphism.*
*   **Tema Warna**: Dark Mode default / Adaptif Light-Dark dengan HSL warna premium.
*   **Layout**: Bento grid layout, rounded card, glowing orbs, sidebar/header menu interaktif React.
*   **Interaktivitas SPA (Inertia)**:
    *   [ ] Transisi halaman instan menggunakan Inertia `<Link>` (bebas reload browser).
    *   [ ] Form submission instan tanpa reload menggunakan Inertia Form Helper (`useForm`).
    *   [ ] Indikator progress bar otomatis dari Inertia saat memuat halaman baru.
    *   [ ] Modal dinamis dan pop-up reaktif murni menggunakan React State.

---

## 🛣️ 5. Alur Pengguna & Routing (User Flow)
*Daftar endpoint rute web Inertia.*
*   `GET /`: Merender halaman `Welcome.jsx` (Landing Page).
*   `GET /dashboard`: Merender halaman `Dashboard.jsx` setelah login berhasil.
*   `POST /login`: Proses validasi login di backend yang mengembalikan redirect Inertia.

---

## ⚠️ 6. Batasan Teknis (Technical Constraints)
*Wajib diikuti oleh AI Agent:*
1.  **Frontend (React SPA)**: WAJIB menggunakan komponen React fungsional (`.jsx`) di bawah `src/resources/js/Pages/` dan disajikan via `inertia(req, component, props)`. DILARANG menggunakan rendering template HTML murni banyak-halaman (MPA).
2.  **Interaktivitas**: WAJIB menggunakan React Hooks (`useState`, `useEffect`) dan Inertia hooks (`useForm`, `usePage`). DILARANG keras menambahkan script JQuery, HTMX, atau pustaka manipulasi DOM luar secara acak.
3.  **Styling**: WAJIB menggunakan utility class **Tailwind CSS**. 
4.  **Database**: WAJIB menggunakan Sea-ORM via perintah CLI `rustbasic migrate`.
5.  **Deployment**: Seluruh aset wajib terkompilasi (`npm run build`) dan tertanam (ter-embed) ke dalam satu file biner executable Rust saat rilis produksi.

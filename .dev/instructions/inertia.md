# 📑 The Complete Inertia.js & React SPA Bible (RustBasic Monolith Edition)

Selamat datang di panduan paling komprehensif untuk **Inertia.js** dan **React SPA** di dalam ekosistem **RustBasic**. Dokumen ini dirancang sebagai panduan tertinggi bagi pengembang dan AI Agent untuk membangun aplikasi monolith bergaya modern, kencang, dan bebas reload.

---

## 🧭 Daftar Isi
1. [Filosofi & Konsep Dasar](#1-filosofi--konsep-dasar)
2. [Cara Kerja Backend (Axum + Rust)](#2-cara-kerja-backend-axum--rust)
3. [Cara Kerja Frontend (React.js SPA)](#3-cara-kerja-frontend-reactjs-spa)
4. [Inertia Form Helper (useForm)](#4-inertia-form-helper-useform)
5. [Inertia Shared Page Props (usePage)](#5-inertia-shared-page-props-usepage)
6. [Keamanan, Session & CSRF](#6-keamanan-session--csrf)
7. [Penanganan Masalah & Gotchas](#7-penanganan-masalah--gotchas)

---

## 1. Filosofi & Konsep Dasar

Inertia.js bukanlah kerangka kerja (framework) frontend biasa. Ia adalah **jembatan (connector)** yang memungkinkan Anda menulis backend monolith standar (mengatur routing, database, controller, authorization) dan menghubungkannya langsung ke frontend modern **React.js** tanpa repot menulis API REST atau GraphQL terpisah!

### Mengapa Inertia.js di RustBasic?
*   **SPA Bebas Reload**: Browser memuat seluruh kerangka CSS/JS hanya sekali di awal. Navigasi setelah itu hanya menukar data JSON, menghasilkan kecepatan perpindahan halaman secepat kedipan mata.
*   **Single Source of Truth**: Logika bisnis, validasi, dan routing tetap 100% berada di server Rust Anda yang aman dan cepat.
*   **No Complex State Manager**: Anda tidak perlu lagi menggunakan Redux, Zustand, atau state manager kompleks lainnya di frontend hanya untuk sinkronisasi data server.

---

## 2. Cara Kerja Backend (Axum + Rust)

Setiap request yang dikirimkan oleh browser dianalisis oleh jembatan Inertia di backend RustBasic:

1.  **Standard Request (Kunjungan Awal)**:
    Jika pengguna membuka URL langsung di address bar (misal: `http://localhost:4000/about`), backend akan merender dan mengembalikan halaman HTML utuh [`app.rb.html`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/views/app.rb.html). Data halaman dibungkus di atribut `data-page` dalam format JSON.
2.  **Inertia Request (Navigasi SPA)**:
    Jika pengguna berpindah halaman via tombol navigasi di React (menggunakan `<Link>`), Inertia mengirimkan request AJAX dengan header `"X-Inertia: true"`. Backend mendeteksi header ini dan **hanya mengirim data JSON mentah** berisi nama komponen React dan data (*props*) terbaru!

### Pola Penulisan Controller di Rust:
```rust
use crate::app::inertia::inertia;
use rustbasic_core::requests::Request;
use axum::response::Response;
use rustbasic_core::serde_json::json;

pub async fn show_profile(req: Request) -> Response {
    // Menyediakan nama komponen React ("Dashboard/Profile") dan payload JSON
    inertia(req, "Dashboard/Profile", json!({
        "username": "Heris Van Hendra",
        "email": "heris@example.com",
        "role": "Superadmin"
    }))
}
```

---

## 3. Cara Kerja Frontend (React.js SPA)

### 3.1 Navigasi Bebas Reload (Komponen `<Link>`)
Untuk berpindah halaman secara internal, **selalu gunakan komponen `<Link>`** dari paket `@inertiajs/react`. DILARANG menggunakan tag anchor murni `<a>` karena akan memaksa browser melakukan muat ulang halaman utuh (MPA) yang merusak state SPA.

```jsx
import { Link } from '@inertiajs/react';

export default function Sidebar() {
  return (
    <nav className="flex flex-col gap-2">
      <Link href="/dashboard" className="hover:text-indigo-400">Beranda</Link>
      <Link href="/about" className="hover:text-indigo-400">Tentang Aplikasi</Link>
    </nav>
  );
}
```

---

## 4. Inertia Form Helper (`useForm`)

Salah satu fitur terbaik Inertia adalah **`useForm`**. Hook ini mengelola seluruh state form, memantau error validasi dari server, mengelola status pemuatan (*loading state*), dan mereset form secara instan.

### Contoh Implementasi Form Registrasi Premium:
```jsx
import { useForm } from '@inertiajs/react';
import React from 'react';

export default function Register() {
  const { data, setData, post, processing, errors, reset } = useForm({
    username: '',
    email: '',
    password: '',
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    post('/register', {
      onSuccess: () => reset('password'), // Reset password jika sukses
    });
  };

  return (
    <form onSubmit={handleSubmit} className="p-6 bg-slate-900 border border-slate-800 rounded-2xl glassmorphism max-w-md mx-auto">
      <h2 className="text-xl font-bold mb-4">Buat Akun Baru</h2>
      
      {/* Input Username */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-1">Username</label>
        <input 
          type="text" 
          value={data.username} 
          onChange={e => setData('username', e.target.value)} 
          className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2"
        />
        {errors.username && <span className="text-red-500 text-xs">{errors.username}</span>}
      </div>

      {/* Input Email */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-1">Email</label>
        <input 
          type="email" 
          value={data.email} 
          onChange={e => setData('email', e.target.value)} 
          className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2"
        />
        {errors.email && <span className="text-red-500 text-xs">{errors.email}</span>}
      </div>

      {/* Submit Button */}
      <button 
        type="submit" 
        disabled={processing}
        className="w-full py-2 bg-indigo-600 rounded-lg font-semibold hover:bg-indigo-700 disabled:opacity-50"
      >
        {processing ? 'Mendaftarkan...' : 'Daftar Sekarang'}
      </button>
    </form>
  );
}
```

---

## 5. Inertia Shared Page Props (`usePage`)

Backend Axum secara otomatis menyuntikkan data-data global (seperti user login, notifikasi flash message, atau CSRF token) ke seluruh halaman React SPA Anda. Data-data global ini dapat diakses di komponen mana pun menggunakan hook **`usePage`**.

### Contoh Mengakses Data User Login di Navbar:
```jsx
import { usePage } from '@inertiajs/react';

export default function Header() {
  const { props } = usePage();
  const { auth, flash } = props; // Diambil dari shared props global

  return (
    <header className="flex justify-between items-center p-4 bg-slate-900 border-b border-slate-800">
      <span className="font-bold text-white text-lg">RustBasic SPA</span>
      
      {flash?.success && (
        <div className="text-green-400 text-sm px-3 py-1 bg-green-950/50 rounded-lg border border-green-800">
          {flash.success}
        </div>
      )}

      {auth?.user ? (
        <div className="flex items-center gap-3">
          <span className="text-sm">Halo, {auth.user.username}</span>
          <button className="text-xs px-2 py-1 bg-red-950 text-red-400 rounded">Keluar</button>
        </div>
      ) : (
        <a href="/login" className="text-sm hover:underline">Masuk</a>
      )}
    </header>
  );
}
```

---

## 6. Keamanan, Session & CSRF

Di dalam monolith RustBasic, Anda tidak perlu repot menyimpan token JWT di `localStorage` browser yang rentan terhadap serangan XSS. 
*   **Cookie-based Sessions**: Sesi login disimpan dengan aman di dalam HTTP-only Cookie terenkripsi yang dikelola oleh middleware Axum.
*   **CSRF Protection**: Proteksi CSRF diaktifkan secara default. Saat data dikirim menggunakan `useForm` (Inertia helper), Inertia secara otomatis meloloskan token CSRF di header HTTP, sehingga Anda bebas dari kekhawatiran serangan pembajakan request!

---

## 7. Penanganan Masalah & Gotchas (SANGAT PENTING!)

### 7.1 Atribut `data-page` Wajib Menggunakan Single Quote (`'`)
Di dalam berkas template root HTML [`app.rb.html`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/views/app.rb.html), atribut inisialisasi hidrasi halaman wajib diapit dengan tanda petik tunggal (`'`) agar tidak terjadi bentrokan tanda petik ganda (`"`) dari objek JSON:

```html
<!-- KODE YANG BENAR (WAJIB): -->
<div id="app" data-page='{{ page | tojson | safe }}'></div>

<!-- KODE YANG SALAH (AKAN MERUSAK HIDRASI REACT): -->
<div id="app" data-page="{{ page | tojson | safe }}"></div>
```

### 7.2 Masalah File Virtual Spasi Vite di macOS
Jika proyek Anda terletak di dalam folder macOS yang mengandung spasi (misalnya `Desktop new/`), server pengembang Vite (`npm run dev`) mungkin akan mengembalikan status `404 Not Found` pada module HMR (`/@vite/client`). 
*   **Solusi**: Ubah nilai `APP_DEBUG=false` di `.env` untuk menjalankan mode kompilasi aset produksi. Jalankan `npm run build` dan biarkan backend Axum menyajikan file secara langsung dari folder `public/build/` secara optimal.

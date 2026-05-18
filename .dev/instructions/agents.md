# 🤖 RustBasic AI Agents Handbook (SPA Edition)

Dokumen ini mendefinisikan standar kerja operasional bagi AI Agent (seperti Antigravity, Cursor, dll.) saat mengembangkan fitur atau memodifikasi kode di dalam framework **RustBasic**.

---

## 🏗️ 0. GOLDEN RULES (Prinsip Utama)
1.  **React.js + Inertia.js SPA Monolith**: SEMUA halaman web disajikan sebagai Single Page Application (SPA). Backend Axum dan frontend React berkomunikasi erat menggunakan **Inertia.js** tanpa memisahkan repository atau port!
2.  **No HTMX / No Legacy Jinja**: Kami tidak menggunakan HTMX atau rendering multi-page tradisional. File HTML `.rb.html` hanya digunakan sekali sebagai *root layout container* ([`app.rb.html`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/views/app.rb.html)). Seluruh halaman dinamis dibuat sebagai komponen React (`.jsx`) di bawah folder [`src/resources/js/Pages/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Pages).
3.  **Single-Binary Compile-Time Embedding**: 
    *   Seluruh berkas template HTML dan aset React hasil kompilasi (`public/build/`) dimasukkan langsung (di-embed) ke dalam satu file biner executable Rust saat release kompilasi menggunakan `rust-embed`.
    *   Jika `APP_DEBUG=true`, sistem akan membaca dari disk secara dinamis untuk mendukung Live Reload/HMR. Jika `APP_DEBUG=false`, sistem akan membaca 100% dari RAM biner ter-embed.
4.  **Premium Aesthetics**: Desain visual wajib terlihat ultra-premium, modern, tema gelap/terang adaptif, menggunakan grid bento box, orbs bersinar, efek glassmorphism, dan transisi navigasi mikro yang fluid.

---

## 🛠️ 1. PERINTAH EKSEKUSI (CLI)

AI Agent wajib menggunakan CLI RustBasic dan NPM untuk siklus pengembangan:

| Tugas | Perintah |
| :--- | :--- |
| **Jalankan Backend (Lokal)** | `rustbasic serve` |
| **Jalankan Frontend (HMR)** | `npm run dev` |
| **Kompilasi Frontend (Build)** | `npm run build` |
| **Buat Controller Baru** | `rustbasic make:controller <Name>` |
| **Buat Model & Migrasi** | `rustbasic make:model <Name> -m` |
| **Jalankan Migrasi Database** | `rustbasic migrate` |

---

## ⚙️ 2. PROCESS (Langkah Kerja Teknis)

### A. Routing & Controller (Rust)
1.  Daftarkan rute Anda di [`src/routes/web.rs`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/routes/web.rs).
2.  Buat controller di [`src/app/http/controllers/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/app/http/controllers/). Gunakan pola return `inertia` untuk menyuplai komponen React dan data JSON:

```rust
use crate::app::inertia::inertia;
use rustbasic_core::requests::Request;
use axum::response::Response;
use rustbasic_core::serde_json::json;

pub async fn index(req: Request) -> Response {
    inertia(req, "Welcome", json!({
        "welcomeMessage": "Halo dari Backend RustBasic!",
        "stats": { "users": 120, "active": 85 }
    }))
}
```

### B. Frontend Views (React Halaman & Komponen)
1.  Buat berkas komponen halaman React baru di dalam [`src/resources/js/Pages/<Name>.jsx`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Pages/).
2.  Menerima properti (*props*) dari backend Axum secara otomatis sebagai argumen fungsi:

```jsx
import { Link } from '@inertiajs/react';
import React, { useState } from 'react';

export default function Welcome({ welcomeMessage, stats }) {
  const [count, setCount] = useState(0);

  return (
    <div className="min-h-screen bg-slate-950 text-white flex flex-col justify-center items-center">
      <h1 className="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-indigo-500">
        {welcomeMessage}
      </h1>
      <div className="mt-8 p-6 bg-slate-900 border border-slate-800 rounded-2xl glassmorphism">
        <p>Pengguna Terdaftar: {stats.users}</p>
        <button onClick={() => setCount(count + 1)} className="mt-4 px-4 py-2 bg-indigo-600 rounded-lg">
          Tambah Angka: {count}
        </button>
      </div>
      <Link href="/about" className="mt-6 text-indigo-400 hover:underline">
        Tentang Aplikasi →
      </Link>
    </div>
  );
}
```

---

## 📂 3. FOLDER MAPPING

| Area | Path Folder |
| :--- | :--- |
| **Logika Controller** | `src/app/http/controllers/` |
| **Model Database** | `src/app/models/` |
| **Rute & URL** | `src/routes/` |
| **React Halaman SPA** | `src/resources/js/Pages/` |
| **React Komponen Modular**| `src/resources/js/Components/` |
| **Root HTML Container** | `src/resources/views/app.rb.html` |
| **Migrasi Database** | `database/migrations/` |

---

_Dokumentasi ini adalah instruksi operasional tertinggi bagi AI Agent untuk menjaga integritas arsitektur Modern SPA RustBasic._

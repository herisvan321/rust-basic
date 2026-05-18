# Dasar-Dasar RustBasic SPA

Dokumen ini menjelaskan tiga pilar utama saat membangun aplikasi web dengan **RustBasic Modern SPA**: **Routing**, **Controllers**, dan **Views (React Pages)**.

---

## 🛣️ 1. Routing (Perutean)

Seluruh rute web untuk halaman SPA didaftarkan di dalam berkas [`src/routes/web.rs`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar rust/rustbasic/src/routes/web.rs). Framework ini menggunakan **Axum** sebagai mesin perutean utama yang sangat cepat.

### Mendefinisikan Rute Web SPA
Gunakan Axum Router standar untuk mendaftarkan URL dan memetakan fungsi kontroler yang sesuai:

```rust
use axum::{routing::get, Router};
use crate::app::http::controllers::welcome_controller;
use rustbasic_core::server::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        // Mengarahkan halaman beranda ke fungsi 'index' di welcome_controller
        .route("/", get(welcome_controller::index))
}
```

---

## ⚙️ 2. Controllers (Kontroler)

Controller bertindak sebagai pengolah data bisnis, otentikasi, validasi, dan penyuplai data ke frontend. Controller disimpan di direktori [`src/app/http/controllers/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/app/http/controllers/).

### Membuat Controller Baru
Anda dapat menggunakan CLI RustBasic untuk mempercepat pembuatan berkas kontroler:
```bash
rustbasic make:controller WelcomeController
```

### Menyajikan Halaman SPA dengan `inertia`
Bukan lagi memanggil render HTML MiniJinja lama, controller SPA mengembalikan pembantu **`inertia`** yang menerima parameter rujukan request, nama komponen halaman React, dan payload JSON:

```rust
use crate::app::inertia::inertia;
use rustbasic_core::requests::Request;
use axum::response::Response;
use rustbasic_core::serde_json::json;

pub async fn index(req: Request) -> Response {
    // 1. "Welcome" merujuk pada berkas src/resources/js/Pages/Welcome.jsx
    // 2. Objek json! berisi properti (props) yang otomatis diterima oleh React
    inertia(req, "Welcome", json!({ 
        "title": "Halaman Utama",
        "description": "Selamat datang di starter kit super cepat RustBasic SPA!"
    }))
}
```

---

## 🎨 3. Views (React Pages)

Di dalam RustBasic SPA, berkas tampilan tidak lagi ditulis menggunakan HTML/Minijinja tradisional di server, melainkan ditulis sebagai **Komponen React fungsional (`.jsx`)** di sisi klien browser.

### Folder Halaman React
Seluruh file halaman utama React SPA Anda wajib diletakkan di dalam folder [`src/resources/js/Pages/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Pages/).

Setiap file di dalam folder tersebut otomatis dipetakan sebagai nama komponen yang bisa dipanggil oleh controller backend Anda (seperti `inertia(req, "Welcome", ...)`).

### Contoh Komponen React SPA (`Pages/Welcome.jsx`):
```jsx
import { Link } from '@inertiajs/react';
import React from 'react';

export default function Welcome({ title, description }) {
  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col justify-center items-center p-6">
      <div className="max-w-md text-center p-8 bg-slate-900 border border-slate-800 rounded-2xl glassmorphism">
        <h1 className="text-3xl font-extrabold text-white mb-4">{title}</h1>
        <p className="text-slate-400 text-sm mb-6">{description}</p>
        
        {/* Link SPA Bebas Reload */}
        <Link 
          href="/about" 
          className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 rounded-lg font-semibold transition"
        >
          Pelajari Lebih Lanjut
        </Link>
      </div>
    </div>
  );
}
```

---

## 📦 4. Asset Management (Vite & Manifest)

Aset statis dikelola dan dikompilasi secara optimal menggunakan **Vite**.
*   **Aset Bundling**: Ketika Anda menjalankan `npm run build`, Vite akan mengompilasi berkas React, JS, CSS, dan gambar ke dalam folder `public/build/` dengan *content hashing* untuk keamanan cache-busting.
*   **Manifest Lookup**: Backend Axum secara cerdas membaca berkas manifest hasil kompilasi tersebut untuk memuat file Javascript dan CSS dengan nama file ter-hash yang tepat ke dalam HTML root container.
*   **Zero-Dependency Embed**: Seluruh berkas di `public/build/` secara otomatis ditanamkan langsung (di-embed) ke RAM biner saat rilis produksi.

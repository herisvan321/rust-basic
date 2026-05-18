# 🦀 RustBasic Starter Kit — Modern SPA Edition

**RustBasic** adalah framework Full-stack modern berkinerja tinggi untuk bahasa pemrograman Rust. Pada edisi **Modern SPA**, framework ini mendobrak batasan web tradisional dengan menyatukan ketangguhan backend **Axum** dan reaktivitas stateful **React.js** melalui jembatan elegan **Inertia.js**, serta dibekali teknologi **Single-Binary Asset Embedding** berbasis `rust-embed`.

---

## 🚀 Fitur Unggulan SPA Edition
*   ⚡ **Axum Powered**: Backend super cepat, aman, dan efisien berbasis **Axum 0.8+**.
*   ⚛️ **Modern React + Inertia.js SPA**: Selamat tinggal pemisahan repository API dan frontend terpisah! Tulis frontend Anda dalam komponen React modern yang terhubung erat (monolith) dengan *state* controller backend secara langsung tanpa *full-page reload*.
*   📦 **Single-Binary Compilation (Zero-Dependency Deployment)**: Seluruh berkas template HTML (`.rb.html`) dan aset statis React terkompilasi (`public/`) dibundel langsung ke dalam **satu file executable biner Rust** saat *release build*!
*   🔄 **Dual-Mode Dinamis (Debug vs Production)**:
    *   **Mode Debug (`APP_DEBUG=true`)**: Membaca file secara dinamis langsung dari disk fisik komputer Anda. Fitur **Live Reload / HMR** tetap aktif untuk mempercepat iterasi coding Anda tanpa kompilasi ulang biner.
    *   **Mode Production (`APP_DEBUG=false`)**: Secara otomatis mengalihkan pembacaan aset statis dan template dari RAM memori biner ter-embed. Anda bebas menghapus folder `public/` dan `src/` di disk produksi server!
*   🗄️ **Sea-ORM**: Manajemen database *async* yang mudah, aman, dan berkinerja tinggi.
*   🛡️ **Built-in CLI**: Generator otomatis untuk mempercepat workflow pembuatan Controller, Model, dan Migrasi.

---

## ⚙️ Siklus Pembangunan & Deployment (Build Pipeline)
Agar seluruh aset React.js hasil kompilasi menyatu secara sempurna ke dalam biner Rust saat dideploy ke produksi, ikuti alur pembangunan berikut:

```bash
# 1. Compile aset React.js + Inertia ke folder public/build/
npm run build

# 2. Compile biner Rust dengan kompresi optimal
cargo build --release
```

Setelah kompilasi selesai, Anda hanya memerlukan satu berkas executable di **`target/release/rustbasic`** untuk dideploy ke server VPS mana pun. **Tidak perlu membawa folder `public/` atau folder kode sumber lainnya!**

---

## 📝 Panduan Struktur Pengembangan

Pengembangan aplikasi pada edisi SPA terpusat pada direktori berikut:

```text
├── src/
│   ├── app/
│   │   ├── http/
│   │   │   └── controllers/    # Kontroller Axum yang memanggil inertia(component, props)
│   │   └── inertia.rs          # Jembatan Inertia & Asset manifest lookup
│   ├── resources/
│   │   ├── js/
│   │   │   ├── Pages/          # Berkas Komponen Halaman React SPA (e.g. Welcome.jsx, About.jsx)
│   │   │   └── main.jsx        # Client bootstrapper React & Inertia
│   │   └── views/
│   │       └── app.rb.html     # HTML root container template
│   └── routes/
│       └── web.rs              # Registrasi rute web Inertia
```

### 1. Menampilkan Halaman SPA dari Controller
Untuk merender halaman React SPA, Anda cukup menggunakan pembantu `inertia` yang menyuplai komponen React dan data (*props*):

```rust
use rustbasic_core::serde_json::json;
use crate::app::inertia::inertia;

pub async fn welcome(req: Request) -> Response {
    inertia(req, "Welcome", json!({
        "title": "Selamat Datang di RustBasic SPA",
        "auth_installed": false
    }))
}
```

### 2. Navigasi Bebas Reload di React
Gunakan komponen `<Link>` bawaan Inertia untuk berpindah halaman secara instan tanpa memuat ulang browser (tanpa *full round-trip* server):

```jsx
import { Link } from '@inertiajs/react';

export default function Welcome({ title }) {
  return (
    <div>
      <h1>{title}</h1>
      <Link href="/about" className="btn-primary">
        Jelajahi Tentang Kami
      </Link>
    </div>
  );
}
```

---

## 🛠️ Penggunaan CLI (`rustbasic`)

RustBasic menyediakan CLI untuk mempercepat workflow pengembangan Anda:

### Manajemen Project & Development
```bash
rustbasic serve               # Menjalankan server Axum (Auto-Reload/Hot-Reload)
npm run dev                   # Menjalankan Vite dev server untuk React (jika folder tanpa spasi)
rustbasic key:generate        # Generate APP_KEY baru di .env
```

### Scaffolding (Generator Kode)
```bash
rustbasic make:controller BlogController
rustbasic make:model Post -m             # -m untuk otomatis membuat migration
```

### Database & Migrasi
```bash
rustbasic make:migration create_users    # Membuat file migrasi baru
rustbasic migrate                        # Jalankan migrasi database
rustbasic migrate:refresh                # Reset dan jalankan ulang migrasi
rustbasic db:seed                        # Jalankan seeder database
```

---

## ⚙️ Environment (.env)
Pastikan Anda telah mengonfigurasi file `.env` sebelum menjalankan aplikasi:
*   `APP_DEBUG`: Set `true` untuk pengembangan lokal dengan HMR. Set `false` di server produksi untuk mengaktifkan pemuatan memori *Embedded Assets* secepat kilat.
*   `APP_URL`: Alamat dasar aplikasi (Default: `http://localhost:4000`).
*   `DATABASE_URL`: Alamat koneksi database (SQLite/MySQL).
*   `DB_CONNECTION`: Pilihan database, gunakan `sqlite` untuk database berbasis file instan.

---

## 🤝 Kontribusi & Ecosystem
Edisi SPA ini dikembangkan secara open source. Silakan kirimkan Pull Request atau laporkan Issue di repositori GitHub kami.

**Selamat membangun aplikasi SPA berskala besar yang aman, instan, dan super kencang dengan RustBasic! 🚀**
*Ditenagai oleh [rustbasic-core](https://github.com/herisvan321/rustbasic-core).*

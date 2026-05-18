# 🎨 Panduan Views & JSX Komponen (React SPA)

Edisi **Modern SPA** pada framework RustBasic meninggalkan sistem templating server-side murni, dan berpindah secara penuh menggunakan **React.js (JSX)** sebagai mesin antarmuka pengguna yang sangat reaktif, cepat, dan terstruktur.

---

## ⚙️ 1. Struktur Halaman & Komponen React

Untuk menjaga keteraturan kode, struktur folder frontend di bawah [`src/resources/js/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/) dipisahkan menjadi dua bagian utama:

1.  **Halaman Utama SPA (`Pages/`)**: Berkas komponen halaman utama yang dirujuk langsung oleh rute controller backend (seperti `Welcome.jsx`, `About.jsx`). Berkas disimpan di [`src/resources/js/Pages/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Pages/).
2.  **Subkomponen Reusable (`Components/`)**: Bagian antarmuka modular yang dipanggil berulang kali di dalam halaman Page (seperti `Navbar.jsx`, `Footer.jsx`, `ButtonPremium.jsx`). Berkas disimpan di [`src/resources/js/Components/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Components/).

---

## ⚡ 2. Komposisi Navigasi & Link Bebas Reload

Semua tautan navigasi internal antar halaman SPA **WAJIB menggunakan komponen `<Link>`** yang disediakan oleh `@inertiajs/react`. 
DILARANG menggunakan tag anchor `<a>` biasa karena akan memaksa browser memuat ulang halaman secara keseluruhan (round-trip), yang menghilangkan pengalaman instan SPA dan merusak state React.

```jsx
import { Link } from '@inertiajs/react';
import Navbar from '../Components/Navbar';

export default function Welcome({ title }) {
  return (
    <div>
      <Navbar />
      <main className="p-8">
        <h1 className="text-3xl font-bold">{title}</h1>
        {/* Navigasi Instan SPA */}
        <Link href="/about" className="mt-4 inline-block text-indigo-400 hover:underline">
          Pelajari Tentang Kami →
        </Link>
      </main>
    </div>
  );
}
```

---

## 💎 3. Desain Sistem & Penataan Gaya (Tailwind CSS)

RustBasic SPA didesain untuk tampil ultra-premium dengan memanfaatkan utilitas **Tailwind CSS**. Dibandingkan menulis file CSS mentah manual yang panjang, gunakan utility class Tailwind untuk menerapkan estetika modern:

*   **Glassmorphism**: Kombinasi transparansi background, efek blur, dan border tipis:
    `bg-slate-900/60 backdrop-blur-md border border-slate-800/80`
*   **Warna Premium**: Gunakan palet gelap bergradasi elegan:
    `bg-gradient-to-tr from-slate-950 via-slate-900 to-indigo-950`
*   **Shadow Glowing**: Efek pendaran cahaya orbs pada tombol premium:
    `shadow-[0_0_20px_rgba(99,102,241,0.2)]`

---

## 🗂️ 4. Pengiriman Formulir Dinamis (`useForm`)

Untuk memproses input data dan mengirimkan form ke backend Axum secara aman, reaktif, dan instan, manfaatkan hook pembantu **`useForm`** dari Inertia:

```jsx
import { useForm } from '@inertiajs/react';

export default function EditProfile({ user }) {
  const { data, setData, put, processing, errors } = useForm({
    name: user.name,
    email: user.email,
  });

  const onSubmit = (e) => {
    e.preventDefault();
    put('/profile/update'); // Mengirim data via HTTP PUT ke backend Axum
  };

  return (
    <form onSubmit={onSubmit} className="p-6 bg-slate-900 rounded-xl border border-slate-800 max-w-sm">
      <input 
        type="text" 
        value={data.name} 
        onChange={e => setData('name', e.target.value)} 
        className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2 text-white mb-2"
      />
      {errors.name && <div className="text-red-500 text-xs mb-2">{errors.name}</div>}

      <button type="submit" disabled={processing} className="w-full py-2 bg-indigo-600 rounded text-white font-semibold">
        {processing ? 'Menyimpan...' : 'Simpan Perubahan'}
      </button>
    </form>
  );
}
```

---

## 📦 5. Single-Binary Asset Embedding (`rust-embed`)

RustBasic menggunakan teknologi **Dual-Mode Serving** untuk memuat aset statis React hasil kompilasi (`public/build`) dan berkas root HTML template (`app.rb.html`) secara cerdas:

1.  **Mode Debug (`APP_DEBUG=true`)**: 
    File HTML root dan berkas CSS/JS dibaca secara langsung dari direktori fisik di komputer lokal Anda. Vite HMR Dev Server (`http://localhost:5173`) menyuplai perubahan kode React Anda secara instan ke layar browser saat Anda menekan tombol simpan (Save) tanpa perlu me-reload halaman browser atau me-rebuild biner Rust.
2.  **Mode Production (`APP_DEBUG=false`)**: 
    Seluruh file HTML template (`src/resources/views/`) dan seluruh berkas hasil bundel Vite React (`public/build/`) disematkan (**di-embed**) ke dalam satu file biner executable Rust saat Anda menjalankan perintah `cargo build --release`. 
    *   Begitu dideploy ke server VPS, berkas-berkas tersebut dilayani langsung dari **RAM memori biner ter-embed**, menghasilkan kecepatan respon secepat kilat!
    *   **Anda bebas menghapus folder `public/` dan `src/` di disk produksi server!**

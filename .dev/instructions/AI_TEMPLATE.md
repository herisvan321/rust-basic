# 🎨 RustBasic AI Template Workflow (React SPA Edition)

Dokumen ini mendefinisikan alur kerja standar bagi AI Agent saat melakukan porting atau implementasi halaman/komponen UI ke dalam framework **RustBasic** menggunakan stack **React.js + Inertia.js**.

---

## 📥 1. INPUT (Analisis & Persiapan)
Sebelum melakukan modifikasi file, AI harus mengumpulkan data berikut:
*   **Nama Halaman**: Tentukan nama komponen React yang akan dibuat (contoh: `About.jsx`, `Welcome.jsx`, `Dashboard/Profile.jsx`).
*   **Analisis Desain UI**: Tinjau desain/komponen visual asli untuk disesuaikan dengan estetika premium RustBasic (glassmorphism, tema gelap, bento box, orbs).
*   **Identifikasi State & Interaktivitas**: Identifikasi bagian UI mana saja yang memerlukan perubahan dinamis (seperti counter, modal, input form) untuk diimplementasikan menggunakan React hooks.

---

## ⚙️ 2. PROSES (Teknis Pemisahan & Kompilasi React)

### A. Konversi Gaya (Tailwind CSS)
*   **Adaptasi Tailwind**: Gunakan class utility **Tailwind CSS** untuk penataan gaya yang seragam, premium, responsif, dan mudah dipelihara.
*   **Hindari Pure CSS Manual**: Di era React SPA, kurangi penulisan file CSS mentah manual. Maksimalkan utility class Tailwind untuk visual glassmorphism, gradasi warna modern, dan bayangan orbs.

### B. Pembuatan Halaman React (Pages)
*   Buat berkas halaman baru di dalam folder [`src/resources/js/Pages/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Pages/).
*   Komponen wajib diekspor secara *default* (`export default`).
*   Seluruh navigasi antar halaman internal **WAJIB** menggunakan `<Link>` dari `@inertiajs/react` agar SPA tidak mengalami reload halaman secara penuh:

```jsx
import { Link } from '@inertiajs/react';

export default function About({ author, version }) {
  return (
    <div className="p-8 bg-slate-950 min-h-screen text-slate-100">
      <h1 className="text-3xl font-bold">Tentang Aplikasi</h1>
      <p className="mt-4">Pembuat: {author}</p>
      <p>Versi: {version}</p>
      <Link href="/" className="mt-6 inline-block text-indigo-400 hover:underline">
        ← Kembali ke Beranda
      </Link>
    </div>
  );
}
```

### C. Ekstraksi Komponen Modular (Components)
*   Jika ada bagian UI yang digunakan berulang-kali (seperti Navbar, Footer, Button Premium), pisahkan ke dalam folder [`src/resources/js/Components/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Components/).
*   Panggil komponen tersebut di dalam halaman Page Anda:

```jsx
import Navbar from '../Components/Navbar';

export default function Dashboard() {
  return (
    <div>
      <Navbar user="Admin" />
      <main className="p-6">Konten Utama...</main>
    </div>
  );
}
```

### D. Routing & Controller (Axum)
*   Daftarkan rute Anda di [`src/routes/web.rs`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/routes/web.rs).
*   Buat controller Axum yang memanggil helper `inertia` untuk memicu render komponen React SPA di browser:

```rust
pub async fn about(req: Request) -> Response {
    inertia(req, "About", json!({
        "author": "Heris",
        "version": "1.0.0"
    }))
}
```

---

## 📤 3. OUTPUT (Struktur File Akhir)
```text
src/resources/js/
├── Pages/
│   ├── Welcome.jsx
│   └── About.jsx
├── Components/
│   ├── Navbar.jsx
│   └── Footer.jsx
└── main.jsx
```

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan)
*   **Ekstensi React**: Seluruh berkas halaman React menggunakan ekstensi `.jsx`.
*   **Link Bebas Reload**: DILARANG menggunakan tag anchor murni `<a>` untuk navigasi internal halaman SPA. Selalu gunakan komponen `<Link>` dari `@inertiajs/react`.
*   **Compile First**: Ingat untuk selalu menjalankan kompilasi frontend (`npm run build`) sebelum mengompilasi biner Rust akhir di server produksi agar perubahan visual Anda tertanam ke dalam memori RAM biner.

---

## 🛠️ 5. ACTION (Verifikasi Jalannya Kode)
| Perintah | Kegunaan |
| :--- | :--- |
| `npm run dev` | Menjalankan Vite dev server untuk fitur instant Hot Module Replacement (HMR) saat development lokal. |
| `rustbasic serve` | Menjalankan backend server Axum. |
| `npm run build` | Melakukan kompilasi optimal frontend React ke folder `public/build/` sebelum rilis produksi. |

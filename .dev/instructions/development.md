# 🛠️ Development Instructions (React SPA Edition)

Panduan ini ditujukan bagi pengembang yang ingin memodifikasi, menjalankan, atau merilis aplikasi berbasis framework **RustBasic Modern SPA**.

---

## 1. Menjalankan Mode Pengembangan (Local Development)

Untuk kenyamanan pengembangan lokal dengan reaktivitas penuh dan fitur **Hot Module Replacement (HMR)** pada React.js, Anda wajib menjalankan **dua server** secara berdampingan:

### Langkah A: Jalankan Backend Axum (Port 4000)
Terminal ini bertugas melayani data, validasi, rute, dan interaksi database Sea-ORM:
```bash
rustbasic serve
```

### Langkah B: Jalankan Vite Dev Server (Port 5173)
Terminal ini bertugas memproses dan menyajikan file React.js secara instan ke browser tanpa perlu me-reload halaman:
```bash
npm run dev
```

*Catatan: Pastikan nilai `APP_DEBUG=true` aktif di berkas `.env` Anda agar sistem melacak perubahan kode real-time.*

---

## 2. Struktur Kode Utama

*   [`src/main.rs`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/main.rs): Titik masuk utama backend. Menjalankan server Axum dan memuat konfigurasi global.
*   [`src/routes/web.rs`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/routes/web.rs): Tempat pendaftaran rute web yang menyajikan Inertia.
*   [`src/app/http/controllers/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/app/http/controllers/): Logika kontroler yang mengolah data bisnis dan mengembalikannya ke React.
*   [`src/resources/js/Pages/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/js/Pages/): Lokasi seluruh komponen halaman utama React SPA Anda.
*   [`src/resources/views/app.rb.html`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/resources/views/app.rb.html): Satu-satunya HTML root container tempat React SPA dihidrasi (*hydrated*).

---

## 3. Alur Kompilasi & Rilis Produksi (Build Pipeline)

Untuk menyatukan seluruh aset frontend dan template ke dalam satu file biner mandiri agar aplikasi siap di-deploy ke server produksi:

```bash
# 1. Compile aset React.js + Inertia ke folder public/build/
npm run build

# 2. Compile biner Rust dengan kompresi optimal
cargo build --release
```

### Penting untuk Produksi:
*   Ubah nilai `APP_DEBUG=false` di berkas `.env` server produksi Anda.
*   Axum akan secara otomatis mematikan pembacaan disk lokal dan melayani berkas statis dari memori RAM biner ter-embed.
*   **Anda bebas menghapus folder `public/` dan `src/` di disk produksi server VPS Anda!** Hanya satu file biner `target/release/rustbasic` yang Anda butuhkan untuk menjalankan aplikasi.

---

## 4. Penambahan Dependensi Baru
*   **Frontend**: Jika menambah library npm, jalankan `npm install <package_name>`.
*   **Backend**: Jika menambah dependensi di `Cargo.toml`, jalankan `cargo check` untuk memastikan tidak ada konflik versi dengan core framework.

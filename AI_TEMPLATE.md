# 🎨 RustBasic AI Template Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent saat melakukan porting atau implementasi template HTML ke dalam framework **RustBasic**.

---

## 📥 1. INPUT (Analisis & Persiapan)
Sebelum melakukan modifikasi file, AI harus mengumpulkan data berikut:
- **Nama Template**: Tanyakan nama template kepada user (contoh: `argon_dashboard`, `retro`). Nama ini akan menjadi nama folder utama.
- **Source Code**: Minta user memberikan file `template.html` atau potongan kode yang ingin di-porting.
- **Identifikasi Aset**: Scan file source untuk menemukan blok `<style>`, `<script>`, dan struktur HTML utama.

---

## ⚙️ 2. PROSES (Teknis Pemisahan)
Lakukan pemisahan kode secara sistematis berdasarkan folder mapping:

### A. Ekstraksi CSS
- Buat folder: `resources/css/<template_name>/`.
- Pindahkan semua CSS dari tag `<style>` ke `resources/css/<template_name>/style.css`.
- Jika ada file CSS eksternal, download/copy ke folder tersebut.

### B. Ekstraksi HTML
- Buat folder: `resources/views/<template_name>/`.
- Pindahkan struktur HTML ke `resources/views/<template_name>/index.html`.
- Integrasikan dengan layout: Tambahkan `{% extends "layouts/app.html" %}` atau buat layout khusus di folder tersebut.

### C. Ekstraksi JavaScript
- Buat folder: `resources/js/<template_name>/`.
- Pindahkan script dari tag `<script>` ke `resources/js/<template_name>/script.js`.

### D. Routing & Controller
- Tambahkan endpoint baru di `src/routes/web.rs` yang mengarah ke template tersebut.
- Buat controller di `src/app/http/controllers/` untuk memanggil view baru.
- Contoh: `view(&req, "<template_name>/index.html", context! { ... })`.

---

## 📤 3. OUTPUT (Struktur File Akhir)
Hasil akhir harus mengikuti pola folder berikut:
```text
resources/
├── css/
│   └── <template_name>/
│       └── style.css
├── js/
│   └── <template_name>/
│       └── script.js
└── views/
    └── <template_name>/
        ├── layout.html (opsional)
        └── index.html
```

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan)
- **No Inline**: Dilarang membiarkan inline CSS atau inline JS di dalam file HTML view.
- **No CDN**: Semua aset harus bersifat lokal (Offline First). Gunakan `{{ asset('path') }}` atau helper yang sesuai untuk memanggil file.
- **HTMX First**: Jika ada interaksi JS yang bisa digantikan dengan HTMX, AI wajib menyarankannya.

---

## 🛠️ 5. ACTION (Verifikasi)
| Perintah | Kegunaan |
| :--- | :--- |
| `cargo serve` | Menjalankan server untuk melihat hasil render template. |
| `cargo rustbasic cache:clear` | Jalankan jika perubahan CSS/JS tidak langsung terlihat. |

---
_Instruksi ini melengkapi `agents.md` khusus untuk bagian manajemen template UI._

# 🤖 RustBasic AI Agents Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent (seperti Antigravity, Cursor, dll) saat memodifikasi atau mengembangkan fitur di dalam framework **RustBasic**.

---

## 🏗️ 0. GOLDEN RULES (Prinsip Utama)
1. **HTMX & Pure CSS Philosophy**: DILARANG menambahkan library JS baru. Semua interaksi dinamis menggunakan **HTMX** dan UI menggunakan **Pure CSS**.
2. **Type Safety**: Manfaatkan struct dan enum Rust semaksimal mungkin. Hindari `unwrap()` tanpa pesan error yang jelas.
3. **Consistency**: Ikuti pola penamaan (snake_case untuk file, CamelCase untuk struct) dan lokasi folder yang sudah ada.
4. **Flexible Assets**: Aset utama (HTMX & CSS) bisa di-embed ke binary via `include_str!` atau menggunakan CDN eksternal jika diperlukan.

---

## 📥 1. INPUT & CONTEXT (Analisis Awal)
Sebelum menulis baris kode pertama, AI harus mengecek:
- **`src/routes/web.rs`**: Lihat daftar endpoint dan middleware yang aktif.
- **`src/config/requests.rs`**: Pahami fungsi helper di `Request` (misal: `req.input()`, `req.session()`).
- **`.env`**: Pastikan konfigurasi (DB, Port, Debug) sudah sesuai.
- **`resources/views/components/`**: Cek daftar macro yang sudah tersedia (buttons, forms, display, overlays, feedback).

---

## ⚙️ 2. PROCESS (Langkah Kerja Teknis)

### A. Routing & Controller
1. Tambahkan rute di `src/routes/web.rs`.
2. Buat controller di `src/app/http/controllers/`. Gunakan pola ini:
```rust
pub async fn name(req: Request) -> impl IntoResponse {
    view(&req, "page_name.html", context! {
        data => "value"
    })
}
```

### B. Database (Sea-ORM)
1. Buat model: `cargo rustbasic make:model Name -m`.
2. Edit file migration di `database/migrations/`.
3. Jalankan: `cargo rustbasic migrate`.

### C. Frontend (HTMX + MiniJinja)
1. Gunakan `{% extends "layouts/app.html" %}` di setiap halaman baru.
2. Manfaatkan macro: `{% from "components/forms.html" import input %}`.
3. Gunakan atribut HTMX untuk interaksi:
   - `hx-post="/path"`
   - `hx-target="#element-id"`
   - `hx-swap="innerHTML"`
   - `hx-indicator="#indicator"` (Wajib untuk feedback visual).

---

## 📂 3. FOLDER MAPPING (Lokasi File)
| Area | Path Folder | Keterangan |
| :--- | :--- | :--- |
| **Logika Bisnis** | `src/app/http/controllers/` | Pusat logika request-response. |
| **Model DB** | `src/app/models/` | Definisi tabel & relasi (Entity). |
| **Middleware** | `src/app/http/middleware/` | Filter keamanan & session. |
| **Template** | `resources/views/` | File HTML (MiniJinja). |
| **UI Macro** | `resources/views/components/` | Reusable UI Library. |
| **Konfigurasi** | `src/config/` | Inti engine (View, DB, Server). |

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan Ketat)
AI Agent **DILARANG** melakukan:
- **Allow CDN**: Diperbolehkan menggunakan link `<script src="https://cdn...">` atau CSS eksternal jika diperlukan.
- **No Inline Styles**: Masukkan CSS baru ke `resources/css/style.css` (bukan ad-hoc di tag HTML).
- **Session Protection**: Jangan pernah menonaktifkan `csrf_middleware` atau `guest_middleware` pada rute sensitif.
- **Logging**: Jangan menghapus `tracing::debug!` atau `tracing::info!` yang sudah ada.

---

## 🛠️ 5. ACTION (Perintah Eksekusi)
| Perintah | Kegunaan |
| :--- | :--- |
| `cargo serve` | **Wajib dipakai** saat dev (Auto-Reload + Live Browser Refresh). |
| `cargo rustbasic make:controller <Name>` | Membuat controller boilerplate. |
| `cargo rustbasic make:model <Name> -m` | Membuat model & file migrasi. |
| `cargo rustbasic migrate` | Sinkronisasi struktur tabel database. |
| `cargo rustbasic route:list` | Debugging endpoint yang aktif. |
| `cargo rustbasic cache:clear` | Membersihkan log & sesi jika error/penuh. |

---

_Dokumentasi ini adalah instruksi operasional untuk AI agar menjaga integritas RustBasic Framework._

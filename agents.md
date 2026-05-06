# 🤖 RustBasic AI Agents Workflow

Dokumen ini mendefinisikan standar kerja bagi AI Agent (seperti Antigravity, Cursor, dll) saat memodifikasi atau mengembangkan fitur di dalam framework **RustBasic**.

---

## 🏗️ 0. GOLDEN RULES (Prinsip Utama)
1. **RSX Syntax**: WAJIB menggunakan sintaks `<Namespace.Component />` di dalam file `.rsx`.
2. **HTMX & Pure CSS Philosophy**: DILARANG menambahkan library JS baru. Semua interaksi dinamis menggunakan **HTMX** dan UI menggunakan **Pure CSS**.
3. **Auto-Imports**: Jangan melakukan `{% from ... import ... %}` manual. Sistem secara otomatis melakukan import komponen dari folder `components/`.
4. **Source Protection**: Output HTML otomatis diminifikasi oleh server (spasi dihapus, komentar dibuang) untuk menyembunyikan struktur kode asli dari "View Source".
5. **Consistency**: Ikuti pola penamaan (snake_case untuk file, CamelCase untuk struct) dan lokasi folder yang sudah ada.

---

## 📥 1. INPUT & CONTEXT (Analisis Awal)
Sebelum menulis baris kode pertama, AI harus mengecek:
- **`src/routes/web.rs`**: Lihat daftar endpoint dan middleware yang aktif.
- **`src/config/requests.rs`**: Pahami fungsi helper di `Request` (misal: `req.input()`, `req.session()`).
- **`src/config/view.rs`**: Pahami logika transpiler RSX jika perlu menambahkan aturan tag baru.
- **`src/resources/views/components/`**: Cek daftar macro yang sudah tersedia.

---

## ⚙️ 2. PROCESS (Langkah Kerja Teknis)

### A. Routing & Controller
1. Tambahkan rute di `src/routes/web.rs`.
2. Buat controller di `src/app/http/controllers/`. Gunakan pola ini:
```rust
pub async fn name(req: Request) -> impl IntoResponse {
    view(&req, "page_name.rsx", context! {
        data => "value"
    })
}
```

### B. Database (Sea-ORM)
1. Buat model: `cargo rustbasic make:model Name -m`.
2. Edit file migration di `database/migrations/`.
3. Jalankan: `cargo rustbasic migrate`.

### C. Frontend (RSX Syntax)
1. Gunakan `{% extends "layouts/app.rsx" %}` di setiap halaman baru.
2. Panggil komponen dengan PascalCase:
   - `<Forms.Input name="email" label="Email" />`
   - `<Buttons.Button label="Login" />`
   - `<Overlays.Logout_confirm_button id="logout" />`
3. Gunakan underscore jika nama komponen mengandung lebih dari satu kata: `<Buttons.Link_button />`.
4. Gunakan atribut HTMX untuk interaksi: `hx-post`, `hx-target`, `hx-indicator`.

---

## 📂 3. FOLDER MAPPING (Lokasi File)
| Area | Path Folder | Keterangan |
| :--- | :--- | :--- |
| **Logika Bisnis** | `src/app/http/controllers/` | Pusat logika request-response. |
| **Model DB** | `src/app/models/` | Definisi tabel & relasi (Entity). |
| **Middleware** | `src/app/http/middleware/` | Filter keamanan & session. |
| **Template** | `src/resources/views/` | File `.rsx` (RSX Syntax). |
| **UI Macro** | `src/resources/views/components/` | Sumber komponen (Namespace). |
| **Konfigurasi** | `src/config/` | Inti engine (View, DB, Server). |

---

## ⚠️ 4. LIMIT & RESTRICTIONS (Batasan Ketat)
AI Agent **DILARANG** melakukan:
- **No Manual Imports**: Dilarang menggunakan `{% from ... import ... %}` di dalam file `.rsx`.
- **No Inline Styles**: Masukkan CSS baru ke `src/resources/css/style.css` (bukan ad-hoc di tag HTML).
- **Session Protection**: Jangan pernah menonaktifkan `csrf_middleware` atau `guest_middleware` pada rute sensitif.
- **Logging**: Jangan menghapus `tracing::debug!` atau `tracing::info!` yang sudah ada.

---

## 🛠️ 5. ACTION (Perintah Eksekusi)
| Perintah | Kegunaan |
| :--- | :--- |
| `cargo serve` | **Wajib dipakai** (Auto-Reload + Live Browser Refresh + RSX Transpiler). |
| `cargo rustbasic make:controller <Name>` | Menghasilkan boilerplate controller yang merujuk ke `.rsx`. |
| `cargo rustbasic auth` | Memasang sistem auth lengkap dengan sintaks RSX. |
| `cargo rustbasic migrate` | Sinkronisasi struktur tabel database. |
| `cargo rustbasic route:list` | Debugging endpoint yang aktif. |

---

_Dokumentasi ini adalah instruksi operasional untuk AI agar menjaga integritas RustBasic RSX Framework._

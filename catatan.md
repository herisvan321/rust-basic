# 📘 Catatan Dokumentasi RustBasic

Dokumentasi ini berisi panduan struktur folder, fitur, dan cara penggunaan framework **RustBasic** (Axum bergaya Laravel).

---

## 📂 1. Struktur Folder (Struktur Modular)

```text
rustbasic/
├── config/               # Modul konfigurasi (.env loader)
├── database/             # Lokasi database SQLite & SQL migrasi
├── public/               # File statis (CSS, JS, Gambar)
├── resources/
│   └── views/            # Template HTML (Minijinja)
├── src/
│   ├── main.rs           # Entry point & Inisialisasi Middleware
│   ├── app/              # Folder Inti Aplikasi
│   │   ├── http/         # Logic HTTP (Controllers, Requests, Responses, Middleware)
│   │   ├── providers/    # Service Providers (Database, View)
│   │   └── mod.rs        # Helper global (seperti fungsi render)
│   ├── database/         # Koneksi DB & Session Manager (Laravel Style)
│   ├── routes/           # Pengaturan rute (web.rs, mod.rs)
│   └── config/           # Definisi struct konfigurasi
└── .env                  # Pengaturan environment (Port, DB, App Key)
```

---

## ⚙️ 2. Konfigurasi (.env)

Gunakan file `.env` untuk mengatur perilaku aplikasi tanpa mengubah kode:
- `APP_NAME`: Nama aplikasi Anda.
- `APP_PORT`: Port server (default: 4000).
- `APP_KEY`: Kunci enkripsi (wajib 64 byte atau akan diderivasi otomatis).
- `DB_CONNECTION`: `sqlite` atau `mysql`.
- `SESSION_DRIVER`: `database` atau `file`.

---

## 🗄️ 3. Database & Session

### Koneksi Database
Aplikasi menggunakan **Sea-ORM**. Koneksi diatur di `src/database/mod.rs`. Secara default, SQLite akan disimpan di folder `database/`.

### Laravel-Style Session
Session disimpan di tabel `sessions` dengan kolom:
`id`, `user_id`, `ip_address`, `user_agent`, `payload`, `last_activity`.
Data dienkripsi secara otomatis menggunakan `APP_KEY`.

---

## 🎨 4. Frontend Stack (SPA & UI)

Aplikasi ini menggunakan pendekatan **Modern Monolith** untuk pengalaman SPA yang ringan:

- **✨ HTMX**: Mengatur navigasi antar halaman (AJAX) sehingga aplikasi terasa seperti Single Page Application (SPA) tanpa reload.
- **🎨 Vanilla CSS**: Seluruh desain menggunakan CSS murni (Glassmorphism) yang disimpan di `public/css/style.css`. Tidak ada library CSS eksternal yang memberatkan.
- **🪄 Alpine.js**: Menangani interaktivitas di sisi client (seperti modal, dropdown, dan animasi) secara deklaratif.

---

## 📥 5. Request & Response (Laravel Style)

### Menggunakan Request
Anda bisa mengambil input dari URL atau form dengan mudah:
```rust
pub async fn controller_method(req: Request) -> impl IntoResponse {
    let name = req.input("name").unwrap_or_default();
    let all = req.all(); // Mengambil semua input dalam bentuk JSON
}
```

### Menggunakan Response
```rust
// Mengembalikan JSON
Response::json(json!({ "status": "ok" }))

// Mengembalikan View (HTML)
render("nama_file.html", context! { data => "isi" })

// Redirect
Response::redirect("/login")
```

---

## 🛡️ 6. Keamanan (Security)

### CSRF Protection
Melindungi dari serangan CSRF. Wajib untuk method `POST`, `PUT`, `PATCH`, dan `DELETE`.
**Cara Penggunaan di Form (HTMX):**
```html
<form hx-post="/submit" hx-headers='{"X-CSRF-TOKEN": "{{ csrf_token }}"}'>
    ...
</form>
```

### Security Headers
Aplikasi menyertakan:
- **CSP**: Mengizinkan script/style dari sumber terpercaya.
- **X-Frame-Options**: Mencegah Clickjacking.
- **X-Content-Type-Options**: Mencegah MIME sniffing.

---

## 🚀 7. Cara Menambah Fitur Baru

1.  **Buat Template**: Tambah file `.html` di `resources/views/`.
2.  **Buat Controller**: Tambah fungsi di `src/app/http/controllers/`.
3.  **Daftarkan Rute**: Tambah baris baru di `src/routes/web.rs`.
    ```rust
    .route("/url-baru", get(nama_controller::fungsi))
    ```

---
*Dokumentasi ini dibuat untuk membantu pengembangan RustBasic agar tetap terstruktur dan aman.*

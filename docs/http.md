# HTTP Stack, CSRF & Middleware (SPA Edition)

Dokumen ini menjelaskan bagaimana framework **RustBasic Modern SPA** memproses request HTTP, menangani keamanan CSRF secara otomatis, dan mengelola alur response di sisi server Axum.

---

## 🔒 1. Keamanan & Proteksi CSRF Otomatis

Keamanan aplikasi Anda terlindungi secara penuh menggunakan proteksi **CSRF (Cross-Site Request Forgery)** di tingkat rute web.

### Bagaimana CSRF Bekerja di Edisi SPA?
Di era Multi-Page Application (MPA) lama, Anda harus menyisipkan input teks tersembunyi (`<input type="hidden" name="_token">`) di setiap formulir HTML secara manual. 

Pada edisi **Modern SPA**, semua proses tersebut terjadi **secara otomatis di balik layar**:
1.  Setiap kali halaman SPA dimuat, middleware Axum secara otomatis membangkitkan token CSRF baru yang unik dan menyimpannya di cookie sesi browser yang aman.
2.  Saat Anda mengirim data menggunakan Inertia form helper (`useForm` atau `router.post`), **Inertia secara otomatis membaca token CSRF tersebut dari cookie dan meloloskannya di header HTTP** pada setiap request AJAX yang mengubah data (POST, PUT, DELETE, PATCH).
3.  Middleware RustBasic di backend akan memotong request, memvalidasi keabsahan token header tersebut, dan menolak request (mengembalikan respon `403 Forbidden`) jika token tidak cocok.

> [!TIP]
> **Bebas Kode Manual**: Anda tidak perlu lagi menulis kode input hidden CSRF atau memanggil helper token manual di berkas React. Cukup gunakan form helper bawaan Inertia, dan keamanan CSRF Anda terjamin 100%!

---

## ⚙️ 2. Middleware Stack (Axum Layers)

Middleware adalah komponen filter yang memproses request sebelum dialirkan ke controller Anda. Middleware disimpan di [`src/app/http/middleware/`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/app/http/middleware/).

### Membuat Middleware Baru
Gunakan CLI RustBasic untuk membuat kerangka file middleware baru:
```bash
rustbasic make:middleware CheckRole
```

### Mendaftarkan Middleware pada Rute
Anda dapat menumpuk layer middleware pada rute Axum di dalam [`src/routes/web.rs`](file:///Users/herisvanhendra/Desktop/Desktop%20new/project/belajar%20rust/rustbasic/src/routes/web.rs):

```rust
Router::new()
    .route("/admin/dashboard", get(admin_controller::index))
    // Hanya pengguna yang terverifikasi sebagai Admin yang boleh masuk
    .layer(axum::middleware::from_fn(admin_auth_middleware))
```

---

## 📤 3. HTTP Responses di Server Axum

Backend Axum pada RustBasic Modern SPA melayani response yang dirancang khusus untuk memenuhi protokol komunikasi Inertia:

### A. Rendring Halaman SPA (`inertia`)
Metode utama untuk memicu render komponen React SPA Anda. Fungsi ini secara cerdas mendeteksi apakah request berupa kunjungan awal browser (mengembalikan HTML utuh) atau navigasi internal SPA (mengembalikan JSON mentah):

```rust
use crate::app::inertia::inertia;
use rustbasic_core::serde_json::json;

pub async fn show_about(req: Request) -> Response {
    inertia(req, "About", json!({
        "appVersion": "1.0.0",
        "author": "Heris Van Hendra"
    }))
}
```

### B. Redirect Dinamis (SPA Friendly)
Saat melakukan proses penyimpanan data (seperti login atau registrasi), Anda sering kali perlu mengarahkan pengguna kembali ke halaman lain. 

Gunakan redirect bawaan Axum yang secara otomatis meloloskan kode status HTTP `303 See Other` (kode status wajib menurut protokol Inertia agar browser SPA mengetahui bahwa redirect ditujukan untuk memuat data JSON baru, bukan merusak siklus navigasi React):

```rust
// Mengarahkan pengguna kembali ke halaman login setelah registrasi sukses
axum::response::Redirect::to("/login")
```

### C. JSON Mentah (API Responses)
Jika Anda membangun endpoint API murni yang tidak menyajikan halaman visual, Anda tetap dapat mengembalikan objek JSON standar Axum:

```rust
axum::Json(serde_json::json!({
    "success": true,
    "message": "Data berhasil dihapus secara permanen."
}))
```

# 🔐 RustBasic Authentication Scaffolding (Breeze)

Dokumentasi fitur *scaffolding* autentikasi otomatis pada framework RustBasic menggunakan paket **Breeze**.

---

## 🚀 Pemasangan Otomatis (Automatic Installation)

Autentikasi di RustBasic tidak lagi dipicu secara manual via perintah CLI khusus, melainkan dipasang secara otomatis saat aplikasi dijalankan ketika paket **`rustbasic-breeze`** ditambahkan ke dependensi proyek Anda.

### Langkah-langkah Pemasangan:

1. **Tambahkan Dependensi**
   Buka file `Cargo.toml` proyek Anda, lalu tambahkan `rustbasic-breeze` ke dalam bagian `[dependencies]`:
   ```toml
   [dependencies]
   rustbasic-core = "0.1"
   rustbasic-breeze = "0.0"
   ```

2. **Jalankan Proyek**
   Jalankan server aplikasi Anda menggunakan Cargo atau CLI:
   ```bash
   rustbasic serve
   # atau
   cargo run
   ```

3. **Inisialisasi Otomatis**
   Sistem startup RustBasic akan mendeteksi kehadiran paket Breeze dan secara otomatis membangun seluruh sistem autentikasi (Login, Registrasi, Forgot Password, Reset Password, Dashboard, Middleware, Model, Rute, dan Migrasi Database).

---

## 📂 Struktur File Tergenerasi

Setelah proses inisialisasi sukses, berkas-berkas berikut akan dibuat secara otomatis di proyek Anda:

| Path | Keterangan |
| :--- | :--- |
| `src/app/models/password_resets.rs` | Model database untuk token reset password menggunakan macro `model!`. |
| `src/app/http/controllers/auth/auth_controller.rs` | Kontroller backend untuk logika autentikasi (Inertia & JSON API). |
| `src/app/http/controllers/dashboard_controller.rs` | Kontroller backend untuk halaman dashboard premium. |
| `src/app/http/middleware/auth.rs` | Middleware pelindung rute autentikasi. |
| `src/routes/auth.rs` | Registrasi rute web khusus autentikasi. |
| `database/migrations/m2026xxxxxx_create_password_resets_table.rs` | Migrasi tabel token reset password database. |
| `src/resources/js/Pages/Auth/` | Folder berisi halaman React SPA (Login, Register, ForgotPassword, ResetPassword). |
| `src/resources/js/Pages/Dashboard.jsx` | Tampilan dashboard React SPA dengan gaya premium. |

---

## 🔒 Fitur Keamanan & UX Unggulan

* **split-screen UI Premium**: Menggunakan estetika modern dengan efek *Glassmorphism* dan *Mesh Gradient*.
* **React & Inertia SPA**: Transisi halaman instan tanpa *full page reload*.
* **Inertia useForm**: Validasi input instan dan penanganan error langsung dari backend.
* **Mass-assignment Safe**: Pendaftaran akun dan manajemen token menggunakan `User::create` dan `PasswordReset::create` yang divalidasi secara kuat.
* **CSRF & Session Protection**: Dilindungi sepenuhnya oleh token CSRF dan database session driver bawaan.

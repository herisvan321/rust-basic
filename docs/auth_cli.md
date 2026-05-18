# 🔐 RustBasic Authentication CLI

Dokumentasi fitur *scaffolding* autentikasi otomatis pada framework RustBasic.

---

## 🚀 Perintah Utama

### 1. Memasang Autentikasi (`auth`)
Membangun sistem login, registrasi, forgot password, reset password, dan dashboard secara otomatis dengan desain premium.

```bash
rustbasic auth
```

**Fitur Unggulan:**
- **Modern Split-Screen UI**: Desain antarmuka kelas atas dengan estetika *Glassmorphism* dan *Mesh Gradient*.
- **React & Inertia SPA**: Membangun Single Page Application menggunakan React.js dan Inertia.js (tanpa reload halaman).
- **Inertia.js useForm**: Penanganan state form reaktif, transisi halus, dan penanganan error instan.
- **Floating Toasts**: Sistem pesan sukses/error melayang dengan animasi CSS otomatis (tanpa JS tambahan).
- **Secure by Default**: Terproteksi oleh `csrf_middleware` dan validasi *server-side* yang kuat.

---

## 📂 Struktur File Tergenerasi

Setelah menjalankan `auth`, file-file berikut akan dibuat:

| Path | Keterangan |
| :--- | :--- |
| `src/resources/js/Pages/Auth/Login.jsx` | Halaman login React dengan desain split-screen modern. |
| `src/resources/js/Pages/Auth/Register.jsx` | Halaman registrasi React dengan state reaktif `useForm`. |
| `src/resources/js/Pages/Auth/Forgot.jsx` | Alur React untuk pemulihan password. |
| `src/resources/js/Pages/Auth/Reset.jsx` | Alur React untuk reset password. |
| `src/resources/js/Pages/Dashboard.jsx` | Dashboard administrator premium berbasis React SPA. |
| `src/app/http/controllers/auth/auth_controller.rs` | Logika backend autentikasi modern berbasis JSON & Inertia. |
| `src/resources/views/emails/reset.rb.html` | Template email reset password Minijinja. |

---

### 2. Menghapus Autentikasi (`auth:back`)
Jika Anda ingin membersihkan alur auth dan memulai dari awal:

```bash
rustbasic auth back
```
Atau:
```bash
rustbasic auth:back
```

**Proses Pembersihan:**
- Menghapus seluruh file rute, controller, middleware, model, dan view yang terkait dengan autentikasi.
- Mengembalikan konfigurasi `mod.rs` ke keadaan semula.
- **Pembersihan Database**: Secara otomatis menghapus catatan migrasi dari tabel `seaql_migrations` sehingga tidak terjadi konflik saat Anda memasang ulang autentikasi nanti.

---

## 🛠️ Kustomisasi
Anda dapat memodifikasi logika di `src/app/http/controllers/auth/auth_controller.rs` dan memperbarui tampilan langsung di file `.rb.html` terkait. Setiap perubahan pada template akan memicu auto-refresh di browser jika `rustbasic serve` sedang berjalan.

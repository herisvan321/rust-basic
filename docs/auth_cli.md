# 🔐 RustBasic Authentication CLI

Dokumentasi fitur *scaffolding* autentikasi otomatis pada framework RustBasic.

---

## 🚀 Perintah Utama

### 1. Memasang Autentikasi (`auth`)
Membangun sistem login, registrasi, dan dashboard secara otomatis dengan desain premium.

```bash
cargo rustbasic auth
```

**Fitur Baru di Edisi RSX:**
- **RSX Native**: Semua template yang dihasilkan menggunakan ekstensi `.rsx` dan sintaks RSX (`<Namespace.Component />`).
- **Auto-Components**: Halaman login dan register menggunakan komponen `<Forms.Input />` dan `<Buttons.Button />` secara otomatis tanpa import manual.
- **Minified Output**: Output HTML dari halaman auth secara otomatis diminifikasi untuk keamanan (menyembunyikan source code).

---

## 📂 Struktur File Tergenerasi

Setelah menjalankan `auth`, file-file berikut akan dibuat dengan sintaks RSX:

| Path | Keterangan |
| :--- | :--- |
| `src/resources/views/auth/login.rsx` | Halaman login menggunakan sintaks RSX. |
| `src/resources/views/auth/register.rsx` | Halaman registrasi menggunakan sintaks RSX. |
| `src/resources/views/dashboard.rsx` | Template dashboard premium dengan statistik. |
| `src/resources/views/emails/reset.rsx` | Template email reset password HTML premium. |

---

## 🛠️ Kustomisasi
Anda dapat memodifikasi logika di `src/app/http/controllers/auth/auth_controller.rs` dan memperbarui tampilan langsung di file `.rsx` terkait. Berkat fitur `cargo serve`, setiap perubahan pada template akan memicu auto-refresh di browser.

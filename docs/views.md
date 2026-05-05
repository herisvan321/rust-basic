# 🎨 Panduan View & Komponen (HTMX)

RustBasic menggunakan **Minijinja Macros** dan **HTMX** untuk membangun UI yang modular, cepat, dan murni tanpa library JavaScript eksternal.

## 🧩 Struktur Komponen
Komponen terletak di `resources/views/components/` dan dibagi berdasarkan fungsinya:

### 1. Forms (`forms.html`)
Gunakan untuk input data dan formulir.
- **`input(name, type, label, placeholder, value, errors, required)`**
  ```html
  {% from "components/forms.html" import input %}
  {{ input("email", type="email", label="Alamat Email", placeholder="user@example.com") }}
  ```

### 2. Buttons (`buttons.html`)
Gunakan untuk tombol aksi dan navigasi.
- **`button(label, variant, class, style, hx_post, hx_target)`**
- **`link_button(href, label, variant, class, style)`**
- **`link_back(href, label)`**
  ```html
  {% from "components/buttons.html" import button, link_button %}
  {{ button("SIMPAN", variant="primary") }}
  {{ link_button("/register", "DAFTAR", variant="outline") }}
  ```

### 3. Display (`display.html`)
Gunakan untuk elemen presentasi data.
- **`alert(message, type, dismissible)`**: Menampilkan pesan melayang (Floating) di pojok kanan atas.
- **`stat_card(label, value, color)`**: Kartu statistik untuk dashboard.
- **`card(title)`**: Kontainer card premium.
  ```html
  {% from "components/display.html" import alert, stat_card %}
  {{ alert("Data berhasil disimpan!", type="success") }}
  {{ stat_card("Total User", "1,240") }}
  ```

### 4. Overlays (`overlays.html`)
Gunakan untuk modal dan dialog konfirmasi.
- **`modal(id, title, size)`**: Kontainer modal standar.
- **`logout_confirm_button(id, label, variant)`**: Tombol logout dengan konfirmasi popup (Pure CSS/Checkbox Hack).
  ```html
  {% from "components/overlays.html" import logout_confirm_button %}
  {{ logout_confirm_button(id="confirm-out", label="KELUAR") }}
  ```

### 5. Feedback (`feedback.html`)
Gunakan untuk indikator status sistem.
- **`indicator(id, label)`**: Overlay loading full-screen (muncul otomatis saat request HTMX).
- **`spinner()`**: Animasi loading melingkar.
- **`skeleton_text(lines)`**: Efek loading placeholder.
  ```html
  {% from "components/feedback.html" import indicator %}
  {{ indicator() }}
  ```

---

## 📅 Filter Waktu & Tanggal (Carbon-like)
Anda dapat memanipulasi tampilan waktu langsung di template menggunakan filter berikut:

1. **`diff_for_humans`**: Mengubah tanggal menjadi teks relatif.
   - Contoh: `{{ user.created_at | diff_for_humans }}` -> *"2 hours ago"*
2. **`format_date`**: Memformat tanggal sesuai pola (Otomatis konversi ke `APP_TIMEZONE`).
   - Contoh: `{{ now() | format_date("%d %B %Y") }}` -> *"02 May 2026"*
3. **`now()`**: Fungsi global untuk mendapatkan waktu saat ini.
   - Contoh: `{{ now() }}`

---

## ⚡ Filosofi HTMX & Pure CSS
Semua komponen dirancang agar browser tidak perlu memuat file `.js` tambahan (selain library HTMX). Namun, framework kini mendukung penggunaan aset eksternal (CSS/JS via CDN) jika Anda ingin mengintegrasikan library pihak ketiga. Interaksi bawaan tetap menggunakan:
1. **Teknik CSS Checkbox Hack** (untuk Modal/Popups).
2. **HTMX Attributes** (`hx-on`, `hx-swap="delete"`, dll).
3. **CSS Fixed Positioning** (untuk Floating elements).

## 🔗 HTMX Integration
Pastikan elemen form atau tombol menggunakan `hx-indicator="#indicator"` agar overlay loading muncul secara otomatis saat data dikirim ke server.

---

## 🔄 Hot Reload & Pengembangan
Untuk mempercepat proses desain, gunakan fitur **Auto-Refresh Browser**:

1. Pastikan `APP_DEBUG=true` di file `.env`.
2. Jalankan server dengan perintah:
   ```bash
   cargo serve
   ```
3. Setiap kali Anda menyimpan file di `resources/views/`, server akan restart dan browser akan otomatis melakukan refresh. Ini sangat berguna saat menyesuaikan layout CSS atau macro Minijinja.

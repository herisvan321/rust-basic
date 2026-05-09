# AI Template: Login & Register UI Update

Gunakan instruksi ini untuk memperbarui tampilan halaman Login dan Register di project RustBasic menggunakan template premium yang telah disediakan di `login_register.html`.

## 📋 Deskripsi Template
Template `login_register.html` menggunakan desain **Premium Modern** dengan fitur:

- **Glassmorphism**: Efek blur pada background form.
- **Split Screen**: Sisi visual untuk branding dan sisi form untuk interaksi.
- **Responsif**: Layout otomatis menyesuaikan untuk perangkat mobile.
- **HTMX Integrated**: Sudah menyertakan atribut HTMX dasar (`hx-post`, `hx-indicator`).
- **Modern Typography**: Menggunakan font Inter dari Google Fonts.

---

## 🛠 Cara Penggunaan (Panduan AI)

Saat diminta untuk memperbarui tampilan login atau register, ikuti langkah-langkah berikut:

### 1. Struktur File `.rb.html`
Pastikan file tetap menggunakan sistem template Minijinja:
```jinja
{% extends "layouts/app.rb.html" %}

{% block title %}Judul Halaman{% endblock %}

{% block content %}
<!-- Paste kode dari login_register.html di sini -->
{% endblock %}
```

### 2. Penyesuaian Variabel (Login)
Petakan field form dari template ke variabel RustBasic:
- **Email**: `<input name="email" value="{{ old.email }}">`
- **Error Email**: `{% if errors.email %}{{ errors.email }}{% endif %}`
- **Error Password**: `{% if errors.password %}{{ errors.password }}{% endif %}`

### 3. Penyesuaian Variabel (Register)
Jika digunakan untuk Register:
- Ubah `hx-post="/login"` menjadi `hx-post="/register"`.
- Tambahkan field `name` dan `password_confirmation`.
- Petakan error: `errors.name`, `errors.email`, `errors.password`, `errors.password_confirmation`.

### 4. Integrasi CSS
Jika ingin menggunakan CSS dari template secara permanen:
- Salin blok `<style>` dari `login_register.html`.
- Letakkan di dalam `{% block content %}` atau pindahkan ke file CSS utama project.

---



## 🚀 Perintah Cepat untuk AI
"Tolong perbarui file `src/resources/views/auth/login.rb.html` menggunakan desain dari template premium di `.dev/instructions/login_register.html`. Pastikan semua logika HTMX dan variabel error tetap berfungsi."

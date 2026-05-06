# 🎨 Panduan View & Komponen (RSX Syntax)

RustBasic menggunakan **RSX (Rust-style XML)** yang menggabungkan kekuatan **Minijinja** dengan sintaks yang modern dan intuitif. Seluruh template menggunakan ekstensi `.rsx`.

## 🚀 Sintaks RSX
Kini Anda tidak perlu lagi mengimpor komponen secara manual. Semua komponen di `src/resources/views/components/` diimpor secara otomatis dan dapat dipanggil menggunakan sintaks tag:

### 1. Tag Mandiri (Self-Closing)
Gunakan untuk komponen yang tidak memiliki konten di dalamnya.
```html
<Buttons.Button label="SIMPAN" variant="primary" />
<Forms.Input name="email" label="Email" placeholder="nama@email.com" />
```

### 2. Tag Blok (Block Tags)
Gunakan untuk komponen yang membungkus konten lain (menggunakan `{{ caller() }}`).
```html
<Display.Card title="Statistik User">
    <p>Konten ini akan muncul di dalam card.</p>
</Display.Card>
```

---

## 🧩 Katalog Komponen
Berikut adalah daftar namespace dan komponen yang tersedia secara otomatis:

### 1. Forms (`Forms.`)
Namespace: `Forms` (file: `components/forms.rsx`)
- **`Input`**: `<Forms.Input name="x" label="y" />`
- **`Textarea`**: `<Forms.Textarea name="x" label="y" />`
- **`Select`**: `<Forms.Select name="x" label="y" />`

### 2. Buttons (`Buttons.`)
Namespace: `Buttons` (file: `components/buttons.rsx`)
- **`Button`**: `<Buttons.Button label="Klik" />`
- **`Link_button`**: `<Buttons.Link_button href="/url" label="Go" />`
- **`Link_back`**: `<Buttons.Link_back />`

### 3. Display (`Display.`)
Namespace: `Display` (file: `components/display.rsx`)
- **`Alert`**: `<Display.Alert message="Sukses!" type="success" />`
- **`Stat_card`**: `<Display.Stat_card label="User" value="100" />`
- **`Card`**: `<Display.Card title="Info">...</Display.Card>`

### 4. Overlays (`Overlays.`)
Namespace: `Overlays` (file: `components/overlays.rsx`)
- **`Logout_confirm_button`**: `<Overlays.Logout_confirm_button id="out" label="LOGOUT" />`

### 5. Feedback (`Feedback.`)
Namespace: `Feedback` (file: `components/feedback.rsx`)
- **`Indicator`**: `<Feedback.Indicator />` (Overlay loading full-screen)
- **`Spinner`**: `<Feedback.Spinner />`

---

## 📅 Filter Waktu & Tanggal (Carbon-like)
Anda tetap dapat menggunakan filter Minijinja standar:
1. **`diff_for_humans`**: `{{ user.created_at | diff_for_humans }}` -> *"2 hours ago"*
2. **`format_date`**: `{{ now() | format_date("%d %B %Y") }}` -> *"02 May 2026"*
3. **`now()`**: Mendapatkan waktu saat ini.

---

## 🛡️ Keamanan & Privasi Source Code
Secara default, RustBasic melakukan **Minifikasi Otomatis** pada output HTML. Saat pengguna melakukan "View Source" di browser:
- Semua spasi berlebih dan baris baru dihapus.
- Semua komentar HTML (`<!-- ... -->`) dibuang.
- Kode akan tampak sebagai satu baris rapat yang sulit dibaca (Obfuscation ringan).

---

## 🔄 Hot Reload & Pengembangan
Gunakan perintah berikut untuk pengembangan yang super cepat dengan auto-refresh browser:
```bash
cargo serve
```
Setiap kali file `.rsx` di `src/resources/views/` disimpan, browser akan otomatis memuat ulang halaman.

# 🚀 Panduan Deployment VPS (SPA Edition)

Dokumen ini menjelaskan alur pembangunan (*build pipeline*) dan langkah-langkah merilis aplikasi **RustBasic Modern SPA** Anda ke server produksi VPS Linux (Ubuntu/Debian) secara aman, efisien, dan tanpa dependensi luar (*zero-dependency*).

---

## 📦 1. Alur Pembangunan (Build Pipeline)

Berkat teknologi **Single-Binary Embedding**, seluruh aset frontend React.js yang terkompilasi (`public/build`) dan template HTML root (`app.rb.html`) disematkan secara penuh langsung ke dalam satu file biner executable Rust saat proses rilis kompilasi.

Untuk merakit berkas biner mandiri tersebut, jalankan **dua perintah berurutan** berikut pada komputer pengembang atau server CI/CD Anda:

```bash
# Langkah A: Kompilasi seluruh kode React & Tailwind ke folder public/build/
npm run build

# Langkah B: Kompilasi biner Rust dengan kompresi optimal
cargo build --release
```

Berkas biner executable mandiri hasil kompilasi akan tersimpan di: **`target/release/rustbasic`**.

---

## 🚢 2. Struktur Folder Bersih di Server Produksi

Saat dideploy ke server produksi, aplikasi Anda benar-benar hemat memori disk dan mandiri. Anda **DILARANG menyalin folder `src/` (kode sumber backend), `node_modules/`, `public/` (folder aset statis asli), atau file `.jsx`** ke server produksi!

Berikut adalah struktur folder minimum yang Anda butuhkan di direktori `/var/www/app/` server produksi VPS Anda:

```text
/var/www/app/
├── rustbasic            # File biner executable hasil build (Wajib)
├── .env                 # Berkas konfigurasi produksi (Wajib)
├── database/            # Tempat penyimpanan berkas SQLite (Jika menggunakan SQLite)
└── storage/             # Tempat log sistem & berkas hasil upload user
```

### 📂 Hak Akses Direktori (Permissions)
Pastikan user sistem yang bertugas menjalankan aplikasi (misalnya `www-data`) memiliki hak akses baca dan tulis ke direktori database dan storage:

```bash
sudo chown -R www-data:www-data /var/www/app
sudo chmod -R 775 /var/www/app/database /var/www/app/storage
```

---

## ⚙️ 3. Konfigurasi Produksi (`.env`)

Pastikan berkas `.env` di server produksi Anda memiliki konfigurasi keamanan yang ketat dan efisien:

```ini
APP_ENV=production
APP_DEBUG=false       # WAJIB false! Mengaktifkan pembacaan aset statis 100% dari RAM biner
APP_PORT=4000         # Port backend Axum berjalan
APP_HOST=127.0.0.1    # Hanya melayani request lokal dari Reverse Proxy Nginx
APP_KEY=base64:...   # Wajib unik, aman, dan rahasia! Generate via 'rustbasic key:generate'
DB_CONNECTION=sqlite
```

---

## 🛠️ 4. Menjalankan Aplikasi di Background (Systemd)

Sangat disarankan menggunakan **Systemd** di Linux untuk mengelola jalannya aplikasi (otomatis memicu restart jika terjadi error tak terduga, melacak logs, dan memicu boot otomatis saat server VPS dinyalakan ulang).

Buat file baru di `/etc/systemd/system/rustbasic.service`:
```ini
[Unit]
Description=RustBasic Web Application
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/var/www/app
ExecStart=/var/www/app/rustbasic
Restart=always

[Install]
WantedBy=multi-user.target
```

Aktifkan dan jalankan service aplikasi:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rustbasic
sudo systemctl start rustbasic
```

---

## 🌐 5. Reverse Proxy dengan Nginx (SSL & HTTPS)

Gunakan **Nginx** sebagai pintu depan penerima trafik publik yang bertugas menangani enkripsi SSL (HTTPS) dan meneruskan request secara instan ke aplikasi RustBasic yang berjalan di port 4000:

```nginx
server {
    listen 80;
    server_name domainanda.com;

    location / {
        proxy_pass http://127.0.0.1:4000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

---

## 🛡️ 6. Checklist Keamanan Sebelum Launching
- [ ] **Variabel `APP_DEBUG`**: Wajib bernilai `false`.
- [ ] **Variabel `APP_KEY`**: Sudah diganti menggunakan string unik baru (`rustbasic key:generate`).
- [ ] **Firewall VPS**: Tutup akses luar langsung ke port 4000 (hanya boleh dimasuki via Nginx di port 80/443).
- [ ] **Directory Protection**: Pastikan file database SQLite (`database/*.sqlite`) tidak berada di folder publik.
- [ ] **Log Monitoring**: Log kesalahan di `storage/logs/` terpantau secara teratur.

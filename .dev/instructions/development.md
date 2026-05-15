# 🛠️ Development Instructions

Panduan ini ditujukan bagi pengembang yang ingin memodifikasi atau berkontribusi pada starter kit RustBasic.

## 1. Menjalankan Mode Pengembangan
Untuk menjalankan aplikasi dengan fitur **Hot-Reload** (otomatis refresh saat kode berubah):
```bash
rustbasic serve
```
Perintah ini sebenarnya memanggil `cargo watch` di balik layar untuk memantau perubahan pada file `.rs` dan `.rb.html`.

## 2. Struktur Kode
- `src/main.rs`: Entry point utama. Sangat minimal karena logika didelegasikan ke `rustbasic-core` dan `src/cli.rs`.
- `src/cli.rs`: Tempat Anda menambahkan perintah baru yang bisa dipanggil melalui `rustbasic <command>`.
- `src/app/`: Berisi Controller, Model, dan Service.
- `src/resources/views/`: Berisi template HTML. Gunakan ekstensi `.rb.html`.

## 3. Menambah Dependensi
Jika Anda menambah dependensi di `Cargo.toml`, pastikan untuk menjalankan `cargo check` atau `cargo build` untuk memastikan tidak ada konflik versi dengan `rustbasic-core`.

## 4. Testing
Gunakan perintah standar cargo untuk menjalankan unit test:
```bash
cargo test
```

## 5. Sinkronisasi dengan Core
Jika Anda melakukan perubahan pada `rustbasic-core` secara lokal:
1. Ubah dependensi di `Cargo.toml` menjadi `{ path = "../rustbasic-core" }`.
2. Lakukan pengujian.
3. Setelah stabil, publikasikan versi baru ke crates.io dan perbarui versi di `Cargo.toml` starter kit.

## 6. Publikasi Template
Jika Anda ingin memperbarui repositori template GitHub:
1. Pastikan `.env` tidak menyertakan data sensitif.
2. Jalankan `cargo clean` untuk membersihkan folder target.
3. Push ke cabang `main`.

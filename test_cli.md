# 🧪 CLI Testing Suite

| Perintah | Ekspektasi | Status | Catatan |
| :--- | :--- | :--- | :--- |
| `make:controller Test` | File `src/app/http/controllers/test_controller.rs` dibuat | | |
| `make:model Test -m` | File model `src/app/models/test.rs` dan migration dibuat | | |
| `migrate` | Migrasi berjalan sukses | | |
| `migrate:refresh` | Database direset dan migrasi dijalankan ulang | | |
| `migrate:back` | Migrasi terakhir di-rollback | | |
| `route:list` | Menampilkan tabel rute | | |
| `make:auth` | Scaffolding auth lengkap dibuat | | |
| `auth:back` | Scaffolding auth dan record DB dihapus | | |
| `make:seeder Test` | File `database/seeders/test_seeder.rs` dibuat | | |
| `db:seed` | Seeder dijalankan | | |
| `key:generate` | APP_KEY di .env diperbarui | | |
| `cache:clear` | Cache/logs dibersihkan | | |

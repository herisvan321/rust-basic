use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use crate::config::Config;
use std::time::Duration;

pub async fn connect(cfg: &Config) -> DatabaseConnection {
    // 1. Susun URL Koneksi berdasarkan pilihan di .env
    let db_url = if cfg.db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_database
        )
    } else {
        // Default ke SQLite
        format!("sqlite:database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    // 2. Konfigurasi Opsi Koneksi
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
       .min_connections(5)
       .connect_timeout(Duration::from_secs(8))
       .idle_timeout(Duration::from_secs(8))
       .max_lifetime(Duration::from_secs(8))
       .sqlx_logging(true);

    // 3. Hubungkan ke Database
    Database::connect(opt)
        .await
        .expect("Gagal terhubung ke database")
}

use crate::config::Config;
use sqlx::AnyPool;

pub async fn setup_database(cfg: &Config) {
    // 1. Susun URL Koneksi
    let db_url = if cfg.db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_database
        )
    } else {
        format!("sqlite:database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    // 2. Install Drivers & Hubungkan dengan deteksi pembuatan DB
    sqlx::any::install_default_drivers();
    
    let _main_pool = match AnyPool::connect(&db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            let err_msg = e.to_string();
            if (err_msg.contains("1049") || err_msg.contains("Unknown database")) && cfg.db_connection == "mysql" {
                let root_url = format!("mysql://{}:{}@{}:{}", cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port);
                if let Ok(root_pool) = sqlx::MySqlPool::connect(&root_url).await {
                    let _ = sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS `{}`", cfg.db_database)).execute(&root_pool).await;
                    AnyPool::connect(&db_url).await.expect("Gagal terhubung setelah membuat DB")
                } else {
                    panic!("Gagal membuat database otomatis: {:?}", e);
                }
            } else {
                panic!("Gagal terhubung ke database utama (AnyPool): {:?}", e);
            }
        }
    };
}

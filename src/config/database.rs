use crate::config::Config;
use sqlx::AnyPool;
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use std::time::Duration;
use colored::Colorize;
use crate::migrations::Migrator;
use sea_orm_migration::prelude::*;

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

    // 3. Hubungkan ke Database dengan deteksi otomatis
    match Database::connect(opt.clone()).await {
        Ok(conn) => conn,
        Err(e) => {
            let err_msg = e.to_string();
            // Jika error 1049 (Unknown Database) dan ini MySQL
            if (err_msg.contains("1049") || err_msg.contains("Unknown database")) && cfg.db_connection == "mysql" {
                println!("{}", "⚠️  Database tidak ditemukan. Mencoba membuat database baru...".yellow());
                
                // Koneksi sementara ke server tanpa memilih database
                let root_url = format!(
                    "mysql://{}:{}@{}:{}",
                    cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port
                );
                
                if let Ok(pool) = sqlx::MySqlPool::connect(&root_url).await {
                    let create_query = format!("CREATE DATABASE IF NOT EXISTS `{}`", cfg.db_database);
                    if let Ok(_) = sqlx::query(&create_query).execute(&pool).await {
                        println!("✅ Database '{}' berhasil dibuat.", cfg.db_database.green());
                        // Coba hubungkan kembali setelah database dibuat
                        return Database::connect(opt).await.expect("Gagal terhubung setelah membuat database");
                    }
                }
            }
            panic!("Gagal terhubung ke database: {:?}", e);
        }
    }
}

pub async fn run_migrations(db: &DatabaseConnection) {
    Migrator::up(db, None).await.expect("Gagal menjalankan migrasi Sea-ORM");
}

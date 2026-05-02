use crate::config::Config;
use sqlx::AnyPool;

pub async fn init_sessions(cfg: &Config) {
    let db_url = if cfg.session_driver == "file" {
        "sqlite:database/sessions.sqlite?mode=rwc".to_string()
    } else if cfg.db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_database
        )
    } else {
        format!("sqlite:database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    sqlx::any::install_default_drivers();
    let pool = AnyPool::connect(&db_url).await.expect("Gagal terhubung ke database session");

    // 2. Auto-Create Table Sessions jika belum ada
    let create_table_query = if db_url.contains("mysql") {
        "CREATE TABLE IF NOT EXISTS sessions (
            id VARCHAR(255) PRIMARY KEY,
            payload LONGTEXT NOT NULL,
            last_activity BIGINT NOT NULL
        )"
    } else {
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            payload TEXT NOT NULL,
            last_activity INTEGER NOT NULL
        )"
    };

    sqlx::query(create_table_query)
        .execute(&pool)
        .await
        .expect("Gagal membuat tabel session otomatis");
}

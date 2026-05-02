use axum_session::{SessionConfig, SessionStore, Key};
use crate::config::Config;
use crate::database;
use sha2::{Sha512, Digest};
use sqlx::AnyPool;

pub async fn setup_session(cfg: &Config) -> SessionStore<database::session_manager::RustBasicSessionStore> {
    // 1. Decode APP_KEY
    let key_bytes = if cfg.app_key.starts_with("base64:") {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.decode(&cfg.app_key[7..]).unwrap_or_else(|_| cfg.app_key.as_bytes().to_vec())
    } else {
        cfg.app_key.as_bytes().to_vec()
    };
    
    // 2. Derive 64-byte key using Sha512
    let mut hasher = Sha512::new();
    hasher.update(&key_bytes);
    let final_key = hasher.finalize();
    let session_key = Key::from(&final_key);

    // 3. Setup Session Config
    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_key(session_key);

    // 4. Determine Session DB URL
    let session_db_url = if cfg.session_driver == "file" {
        "sqlite:database/sessions.sqlite?mode=rwc".to_string()
    } else if cfg.db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_database
        )
    } else {
        format!("sqlite:database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    // 5. Connect and Create Store
    sqlx::any::install_default_drivers();
    let session_pool = match AnyPool::connect(&session_db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            let err_msg = e.to_string();
            if (err_msg.contains("1049") || err_msg.contains("Unknown database")) && cfg.db_connection == "mysql" {
                let root_url = format!("mysql://{}:{}@{}:{}", cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port);
                if let Ok(root_pool) = sqlx::MySqlPool::connect(&root_url).await {
                    let _ = sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS `{}`", cfg.db_database)).execute(&root_pool).await;
                    AnyPool::connect(&session_db_url).await.expect("Gagal terhubung setelah membuat DB session")
                } else {
                    panic!("Gagal membuat database session otomatis: {:?}", e);
                }
            } else {
                panic!("Gagal terhubung ke database session: {:?}", e);
            }
        }
    };
    
    SessionStore::<database::session_manager::RustBasicSessionStore>::new(
        Some(database::session_manager::RustBasicSessionStore::new(session_pool)), 
        session_config
    ).await.expect("Gagal menginisialisasi SessionStore")
}

use sqlx::AnyPool;

/// Fungsi umum untuk menjalankan migrasi (Generic)
pub async fn run_migrations_any(pool: &AnyPool) {
    sqlx::migrate!("./database/migrations")
        .run(pool)
        .await
        .expect("Gagal menjalankan migrasi database");
}

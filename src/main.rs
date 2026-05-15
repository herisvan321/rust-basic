use rustbasic_core::tower_http::services::ServeDir;
use rustbasic_core::sea_orm_migration::MigratorTrait;
use rustbasic_core::dotenvy::dotenv;
use rustbasic_core::Config;

#[tokio::main]
async fn main() {
    // 1. Muat file .env & Inisialisasi Logger (Terminal + File)
    dotenv().expect("❌ Error: File .env tidak ditemukan! Silakan salin .env.example menjadi .env sebelum menjalankan server.");
    let _guard = rustbasic_core::logger::init();

    // 2. Muat Konfigurasi
    let cfg = Config::load();

    // 2.1 Cek Command CLI (migrate, seed)
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1].starts_with("migrate") || args[1] == "db:seed") {
        let db = rustbasic_core::database::connect(&cfg).await;
        let command = args[1].as_str();

        match command {
            "migrate" => {
                let _ = rustbasic::migrations::Migrator::up(&db, None).await;
            }
            "migrate:refresh" => {
                let _ = rustbasic::migrations::Migrator::fresh(&db).await;
            }
            "migrate:back" | "migrate:rollback" => {
                let _ = rustbasic::migrations::Migrator::down(&db, Some(1)).await;
            }
            "db:seed" => {
                rustbasic::app::seeder::run(&db).await;
            }
            _ => {}
        }
        return;
    }

    // 3. Setup Database & Sea-ORM
    let db = rustbasic_core::database::connect(&cfg).await;
    // Migrasi TIDAK dijalankan otomatis saat serve.
    // Gunakan 'rustbasic migrate' untuk menjalankan migrasi secara manual.
    
    // 4. Inisialisasi Session Store
    rustbasic_core::session::init_sessions(&cfg).await;
    let session_store = rustbasic_core::session::setup_session(&cfg).await;

    // 5. Bangun Router Aplikasi
    let app_router: rustbasic_core::axum::Router<rustbasic_core::server::AppState> = rustbasic_core::axum::Router::new()
        .merge(rustbasic::routes::web::router())
        .layer(rustbasic_core::axum::middleware::from_fn(rustbasic::app::http::middleware::csrf::csrf_middleware))
        .layer(rustbasic_core::axum::middleware::from_fn(rustbasic::app::http::middleware::security_headers::security_headers_middleware))
        .layer(rustbasic_core::axum::middleware::from_fn(rustbasic::app::http::middleware::logging::logging_middleware));

    // 6. Setup Statics & Jalankan Server
    let static_files = ServeDir::new("public");
    rustbasic_core::server::start_server(cfg, session_store, static_files, db, app_router).await;
}

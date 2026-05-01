use axum::{Router, middleware};
use tower_http::services::ServeDir;
use axum_session::{SessionLayer, SessionStore};
use crate::{routes, app, database};
use crate::config::Config;
use std::net::SocketAddr;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<Config>,
}

pub async fn start_server(
    cfg: Config, 
    session_store: SessionStore<database::session_manager::RustBasicSessionStore>,
    static_files: ServeDir,
    db: DatabaseConnection
) {
    // 1. Inisialisasi State
    let state = AppState {
        db,
        config: Arc::new(cfg.clone()),
    };

    // 2. Bangun Router
    let app = Router::new()
        .merge(routes::web::router())
        .nest_service("/public", static_files)
        .layer(middleware::from_fn(app::http::middleware::csrf::csrf_middleware))
        .layer(middleware::from_fn(app::http::middleware::security_headers::security_headers_middleware))
        .layer(SessionLayer::new(session_store))
        .fallback(app::http::controllers::error_controller::ErrorController::not_found)
        .with_state(state);

    // 3. Tentukan Alamat
    let addr_str = format!("{}:{}", cfg.app_host, cfg.app_port);
    let addr: SocketAddr = addr_str.parse().expect("Alamat server tidak valid");
    
    tracing::info!("{} berjalan di: http://{}", cfg.app_name, addr);
    
    // 4. Jalankan Server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

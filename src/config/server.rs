use axum::{Router, middleware};
use tower_http::services::ServeDir;
use axum_session::{SessionLayer, SessionStore};
use crate::{routes, app, database};
use crate::config::Config;
use std::net::SocketAddr;

pub async fn start_server(
    cfg: &Config, 
    session_store: SessionStore<database::session_manager::RustBasicSessionStore>,
    static_files: ServeDir
) {
    // 1. Bangun Router
    let app = Router::new()
        .merge(routes::web::router())
        .nest_service("/public", static_files)
        .layer(middleware::from_fn(app::http::middleware::csrf::csrf_middleware))
        .layer(middleware::from_fn(app::http::middleware::security_headers::security_headers_middleware))
        .layer(SessionLayer::new(session_store))
        .fallback(app::http::controllers::error_controller::ErrorController::not_found);

    // 2. Tentukan Alamat
    let addr_str = format!("{}:{}", cfg.app_host, cfg.app_port);
    let addr: SocketAddr = addr_str.parse().expect("Alamat server tidak valid");
    
    tracing::info!("{} berjalan di: http://{}", cfg.app_name, addr);
    
    // 3. Jalankan Server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

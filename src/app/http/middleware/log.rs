/* ---------------------------------------------------------
 * 📑 LABEL: LOGMIDDLEWARE (middleware/log.rs)
 * --------------------------------------------------------- */

use rustbasic_core::axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn log_middleware(
    req: Request,
    next: Next,
) -> Response {
    // Lakukan sesuatu sebelum request sampai ke controller
    
    let response = next.run(req).await;
    
    // Lakukan sesuatu setelah request selesai diproses
    
    response
}

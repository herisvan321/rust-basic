/* ---------------------------------------------------------
 * 📑 LABEL: TEST CONTROLLER (test_controller.rs)
 * Menangani percobaan fitur CSRF dan Input.
 * --------------------------------------------------------- */

use crate::app::view;
use crate::config::requests::Request;
use axum::response::IntoResponse;
use minijinja::context;

pub async fn csrf_form(req: Request) -> impl IntoResponse {
    // Ambil token dari session (dibuat oleh middleware)
    let token = req.session.get::<String>("_token").unwrap_or_default();
    
    view(&req, "test_csrf.html", context! {
        title => "Percobaan CSRF",
        csrf_token => token
    })
}

pub async fn csrf_submit(req: Request) -> impl IntoResponse {
    let name = req.input_as_str("name").unwrap_or("Kosong");
    let email = req.input_as_str("email").unwrap_or("Kosong");
    
    view(&req, "test_csrf_result.html", context! {
        title => "Hasil Pengiriman",
        name => name,
        email => email
    })
}

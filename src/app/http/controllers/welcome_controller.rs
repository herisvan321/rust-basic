use crate::app::view;
use crate::config::requests::Request;
use crate::config::responses::ResponseHelper;
use axum::response::IntoResponse;
use minijinja::context;

pub async fn index(req: Request) -> impl IntoResponse {
    // Render file "welcome.html" dengan data title
    view(&req, "welcome.html", context! {
        title => "Selamat Datang di RustBasic",
    })
}

/// Contoh penggunaan Request & Response ala RustBasic
pub async fn test_request(req: Request) -> impl IntoResponse {
    let name = req.input_as_str("name").unwrap_or("Tamu");
    
    ResponseHelper::json(serde_json::json!({
        "message": format!("Halo, {}! Ini adalah data dari Request ala RustBasic.", name),
        "method": req.method.to_string(),
        "all_inputs": req.all()
    }))
}

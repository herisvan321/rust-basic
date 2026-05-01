/* ---------------------------------------------------------
 * 📑 LABEL: APPLICATION CORE (app/mod.rs)
 * Mengatur template engine (Minijinja) dan fungsi render.
 * --------------------------------------------------------- */

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use minijinja::{Environment, path_loader, context};
use std::sync::LazyLock;

pub mod http;

// 1. Setup Engine Template (Minijinja)
pub static JINJA: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("resources/views"));
    env
});

// 2. Fungsi Helper untuk Render HTML
pub fn render(template: &str, context: minijinja::Value) -> Response {
    match JINJA.get_template(template) {
        Ok(tmpl) => match tmpl.render(context) {
            Ok(rendered) => Html(rendered).into_response(),
            Err(err) => {
                tracing::error!("Gagal render template: {}", err);
                // Kembalikan tampilan 500 kustom jika gagal render
                match JINJA.get_template("errors/minimal.html") {
                    Ok(tmpl) => match tmpl.render(context! { code => 500, title => "Server Error", message => "Terjadi kesalahan saat memproses tampilan." }) {
                        Ok(rendered) => Html(rendered).into_response(),
                        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Critical Render Error").into_response(),
                    },
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
                }
            }
        },
        Err(err) => {
            tracing::error!("Template tidak ditemukan: {}", err);
            // Kembalikan tampilan 404 kustom jika template tidak ada
            match JINJA.get_template("errors/minimal.html") {
                Ok(tmpl) => match tmpl.render(context! { code => 404, title => "Page Not Found", message => "Halaman atau template tidak ditemukan." }) {
                    Ok(rendered) => Html(rendered).into_response(),
                    Err(_) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
                },
                Err(_) => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            }
        }
    }
}

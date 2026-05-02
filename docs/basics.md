# Routing

Routing di RustBasic dikelola di dalam folder `src/routes/`. Utama-nya berada di `src/routes/web.rs`.

## Mendefinisikan Route
Anda dapat mendefinisikan route menggunakan Axum syntax:

```rust
use axum::{routing::{get, post}, Router};
use crate::app::http::controllers::WelcomeController;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(WelcomeController::index))
        // Tambahkan route lainnya di sini
}
```

## Route Grouping & Prefix
Gunakan `.nest()` untuk mengelompokkan route:

```rust
Router::new()
    .nest("/admin", admin_routes())
```

---

# Controllers

Controller bertugas menangani logika request dan mengembalikan response. Disimpan di `src/app/http/controllers/`.

## Membuat Controller
Gunakan CLI untuk membuat controller:
```bash
cargo rustbasic make:controller Name
```

## Contoh Controller
```rust
pub struct WelcomeController;

impl WelcomeController {
    pub async fn index(req: Request) -> impl IntoResponse {
        view(&req, "welcome.html", context! { 
            title => "Selamat Datang" 
        })
    }
}
```

---

# Views (Template)

RustBasic menggunakan engine **MiniJinja** yang sangat mirip dengan Jinja2 atau Blade Laravel. Template disimpan di `resources/views/`.

## Render View
Gunakan helper `view()`:
```rust
view(&req, "nama_file.html", context! { key => value })
```

## Template Inheritance
Di file `layout.html`:
```html
<html>
  <body>
    {% block content %}{% endblock %}
  </body>
</html>
```

Di file halaman (misal `home.html`):
```html
{% extends "layout.html" %}
{% block content %}
  <h1>Hello World</h1>
{% endblock %}
```

# Dasar-Dasar RustBasic

## 🛣️ Routing
Routing dikelola di `src/routes/web.rs`. RustBasic menggunakan Axum sebagai engine utamanya.

### Mendefinisikan Route
```rust
use axum::{routing::get, Router};
use crate::app::http::controllers::welcome_controller;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(welcome_controller::index))
}
```

---

## ⚙️ Controllers
Controller disimpan di `src/app/http/controllers/`. Anda bisa menggunakan CLI untuk membuatnya.

### Membuat Controller
```bash
cargo rustbasic make:controller WelcomeController
```

### Contoh Logika
```rust
pub async fn index(req: Request) -> impl IntoResponse {
    view(&req, "welcome.rsx", context! { 
        title => "Home" 
    })
}
```

---

## 🎨 Views
Template RustBasic menggunakan ekstensi `.rsx` dan mendukung sintaks RSX.

### Menggunakan Komponen
Anda tidak perlu lagi mengimpor komponen secara manual. Gunakan sintaks tag:

```html
{% extends "layouts/app.rsx" %}

{% block content %}
    <Display.Card title="Halo Rust!">
        <Buttons.Button label="Klik Saya" variant="primary" />
    </Display.Card>
{% endblock %}
```

### Folder Template
Seluruh template berada di `src/resources/views/`.

---

## 📦 Asset Management
Asset inti (CSS & HTMX) ditanam langsung ke dalam binary aplikasi untuk keamanan dan performa maksimal.

### Penggunaan di Layout
Cukup panggil komponen `Assets`:
```html
<Assets.Styles />
<Assets.Htmx />
```

File sumber asset berada di:
- `src/resources/css/style.css`
- `src/resources/js/htmx.min.js`

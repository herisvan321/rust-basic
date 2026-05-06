use std::fs;
use colored::*;
use regex::Regex;
use super::scaffolding::update_controller_mod_rs;

pub async fn make_auth() {
    println!("\n{}", "🔐 Scaffolding Authentication...".magenta().bold());

    // 1. Create src/routes/auth.rs
    let auth_route_path = "src/routes/auth.rs";
    let auth_route_template = r#"use axum::{Router, routing::{get, post}, middleware::from_fn};
use crate::app::http::controllers::auth;
use crate::app::http::middleware::auth::guest_middleware;
use crate::config::server::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(auth::auth_controller::AuthController::login_page))
        .route("/login", post(auth::auth_controller::AuthController::login))
        .route("/register", get(auth::auth_controller::AuthController::register_page))
        .route("/register", post(auth::auth_controller::AuthController::register))
        .route("/forgot-password", get(auth::auth_controller::AuthController::forgot_password_page))
        .route("/forgot-password", post(auth::auth_controller::AuthController::send_reset_link))
        .route("/reset-password", get(auth::auth_controller::AuthController::reset_password_page))
        .route("/reset-password", post(auth::auth_controller::AuthController::update_password))
        .layer(from_fn(guest_middleware))
}
"#;
    if !std::path::Path::new(auth_route_path).exists() {
        fs::write(auth_route_path, auth_route_template).ok();
        println!("   {} {}", "✅ Created:".green(), auth_route_path.cyan());
    } else {
        println!("   {} {}", "⚠️  Exists:".yellow(), auth_route_path.cyan());
    }

    // 2. Update src/routes/mod.rs
    let routes_mod_path = "src/routes/mod.rs";
    if let Ok(mut content) = fs::read_to_string(routes_mod_path) {
        if !content.contains("pub mod auth;") {
            content.push_str("pub mod auth;\n");
            fs::write(routes_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), routes_mod_path.cyan());
        }
    }

    // 3. Update src/routes/web.rs
    let web_route_path = "src/routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_route_path) {
        if !content.contains("use crate::routes::auth as auth_routes;") {
            content = content.replace("use axum::{Router, routing::get};", "use axum::{Router, routing::{get, post}, middleware::from_fn};");
            content = content.replace("use crate::config::server::AppState;", "use crate::app::http::controllers::{auth, dashboard_controller};\nuse crate::app::http::middleware::auth::auth_middleware;\nuse crate::config::server::AppState;\nuse crate::routes::auth as auth_routes;");
            
            let merge_logic = r#"    let auth_protected_routes = Router::new()
        .route("/dashboard", get(dashboard_controller::DashboardController::index))
        .route("/logout", post(auth::auth_controller::AuthController::logout))
        .layer(from_fn(auth_middleware));

    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/dev", get(welcome_controller::dev_info))
        .merge(auth_routes::router())
        .merge(auth_protected_routes)"#;

            content = content.replace("Router::new()\n        .route(\"/\", get(welcome_controller::index))\n        .route(\"/dev\", get(welcome_controller::dev_info))", merge_logic);
            
            fs::write(web_route_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), web_route_path.cyan());
        }
    }

    // 3.1 Create Password Resets Migration
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let migration_name = format!("m{}_create_password_resets_table", timestamp);
    let migration_path = format!("database/migrations/{}.rs", migration_name);
    
    // Check if any password reset migration already exists
    let mut exists = false;
    if let Ok(entries) = std::fs::read_dir("database/migrations") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with("_create_password_resets_table.rs") {
                    exists = true;
                    println!("   {} {}", "⚠️  Exists:".yellow(), name.cyan());
                    break;
                }
            }
        }
    }

    if !exists {
        let migration_template = format!(r#"use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum PasswordResets {{
    Table,
    Email,
    Token,
    CreatedAt,
}}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .create_table(
                Table::create()
                    .table(PasswordResets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PasswordResets::Email).string().not_null().primary_key())
                    .col(ColumnDef::new(PasswordResets::Token).string().not_null())
                    .col(
                        ColumnDef::new(PasswordResets::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .drop_table(Table::drop().table(PasswordResets::Table).to_owned())
            .await
    }}
}}
"#);
        fs::write(&migration_path, migration_template).ok();
        
        super::scaffolding::update_migration_mod_rs(&migration_name);
        println!("   {} {}", "✅ Created:".green(), format!("Migration {}", migration_name).cyan());
    }

    // 4. Create Controller Folder & mod.rs
    let auth_controller_dir = "src/app/http/controllers/auth";
    fs::create_dir_all(auth_controller_dir).ok();
    let auth_controller_mod = "src/app/http/controllers/auth/mod.rs";
    if !std::path::Path::new(auth_controller_mod).exists() {
        fs::write(auth_controller_mod, "pub mod auth_controller;").ok();
    }
    update_controller_mod_rs("auth");

    // 4.1 Create Password Resets Model
    let model_path = "src/app/models/password_resets.rs";
    if !std::path::Path::new(model_path).exists() {
        let model_template = r#"use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "password_resets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub email: String,
    pub token: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
"#;
        fs::write(model_path, model_template).ok();
        
        // Update src/app/models/mod.rs
        let models_mod_path = "src/app/models/mod.rs";
        if let Ok(mut content) = fs::read_to_string(models_mod_path) {
            if !content.contains("pub mod password_resets;") {
                content.push_str("pub mod password_resets;\n");
                fs::write(models_mod_path, content).ok();
            }
        }
        println!("   {} {}", "✅ Created:".green(), "Model password_resets".cyan());
    }

    let auth_controller_path = "src/app/http/controllers/auth/auth_controller.rs";
    if !std::path::Path::new(auth_controller_path).exists() {
        let controller_template = r#"/* ---------------------------------------------------------
 * 📑 LABEL: AUTH CONTROLLER (auth/auth_controller.rs)
 * Menangani pendaftaran, login, dan logout user.
 * --------------------------------------------------------- */

use crate::app::view;
use crate::app::models::users;
use crate::config::requests::Request;
use crate::config::responses::ResponseHelper;
use crate::config::server::AppState;
use axum::{response::IntoResponse, extract::State};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use serde::Deserialize;
use validator::Validate;
use crate::app::services::mail_service::MailService;
use minijinja::context;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, Set};

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    pub password: String,
    pub remember: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

pub struct AuthController;

impl AuthController {
    /// Menampilkan halaman login
    pub async fn login_page(req: Request) -> impl IntoResponse {
        view(&req, "auth/login.rb.html", context! { title => "Login" })
    }

    /// Menampilkan halaman register
    pub async fn register_page(req: Request) -> impl IntoResponse {
        view(&req, "auth/register.rb.html", context! { title => "Daftar Akun" })
    }

    /// Proses Pendaftaran
    pub async fn register(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<RegisterRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/register"),
        };

        // 2. Cek apakah email sudah terdaftar
        let existing = users::Entity::find()
            .filter(users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if existing.is_some() {
            return ResponseHelper::redirect_with_error("/register", "Email sudah terdaftar", req.session);
        }

        // 3. Hash Password
        let hashed = hash(data.password, DEFAULT_COST).unwrap();

        // 4. Simpan ke Database
        let new_user = users::ActiveModel {
            name: Set(data.name),
            email: Set(data.email),
            password: Set(hashed),
            ..Default::default()
        };

        if let Err(e) = users::Entity::insert(new_user).exec(&state.db).await {
            tracing::error!("Gagal menyimpan user: {}", e);
            return ResponseHelper::redirect_with_error("/register", "Gagal mendaftar, coba lagi.", req.session);
        }

        ResponseHelper::redirect_with_success("/login", "Pendaftaran berhasil! Silakan login.", req.session)
    }

    /// Proses Login
    pub async fn login(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<LoginRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/login"),
        };

        // 2. Ambil User dari DB
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(u) = user {
            // 3. Verifikasi Password
            if verify(data.password, &u.password).unwrap_or(false) {
                // 4. Set Session
                req.session.set("user_id", u.id);
                
                // Handle "Remember Me"
                if data.remember.is_some() {
                    // Set session expiration to 30 days if remember is checked
                    // Note: implementation depends on axum_session configuration
                    tracing::info!("Remember me checked for user: {}", u.email);
                }

                return ResponseHelper::redirect_with_success("/dashboard", "Selamat datang kembali!", req.session);
            }
        }

        ResponseHelper::redirect_with_error("/login", "Email atau password salah", req.session)
    }

    /// Menampilkan halaman lupa password
    pub async fn forgot_password_page(req: Request) -> impl IntoResponse {
        view(&req, "auth/forgot.rb.html", context! { title => "Lupa Password" })
    }

    /// Kirim link reset password
    pub async fn send_reset_link(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        let data = match req.validate::<ForgotPasswordRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/forgot-password"),
        };

        // 1. Cek apakah user ada
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(u) = user {
            // 2. Generate Token
            let token = Uuid::new_v4().to_string();

            // 3. Simpan Token
            let reset = crate::app::models::password_resets::ActiveModel {
                email: Set(u.email.clone()),
                token: Set(token.clone()),
                created_at: Set(chrono::Utc::now().naive_utc()),
            };

            let _ = crate::app::models::password_resets::Entity::insert(reset)
                .on_conflict(
                    sea_orm::sea_query::OnConflict::column(crate::app::models::password_resets::Column::Email)
                        .update_column(crate::app::models::password_resets::Column::Token)
                        .update_column(crate::app::models::password_resets::Column::CreatedAt)
                        .to_owned()
                )
                .exec(&state.db)
                .await;

            // 4. Kirim Email (Gunakan Config::load().mail_*)
            let config = crate::config::Config::load();
            let app_name = std::env::var("APP_NAME").unwrap_or_else(|_| "RustBasic".to_string());
            let reset_url = format!("{}/reset-password?token={}", config.app_url, token);

            let subject = format!("Reset Password - {}", app_name);
            let body = crate::config::view::render_to_string("emails/reset.rb.html", context! {
                app_name => app_name,
                reset_url => reset_url,
            });

            if let Err(e) = MailService::send_email(&u.email, &subject, &body).await {
                tracing::error!("Gagal mengirim email reset: {}", e);
            }

            tracing::info!("Reset link for {}: {}", u.email, reset_url);
        }

        ResponseHelper::redirect_with_success("/login", "Jika email terdaftar, link reset password akan dikirim.", req.session)
    }

    /// Menampilkan halaman reset password
    pub async fn reset_password_page(req: Request) -> impl IntoResponse {
        let token = req.input_as_str("token").unwrap_or_default();
        view(&req, "auth/reset.rb.html", context! { title => "Reset Password", token => token })
    }

    /// Proses update password baru
    pub async fn update_password(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        let data = match req.validate::<ResetPasswordRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/login"),
        };

        // 1. Cari Token
        let reset = crate::app::models::password_resets::Entity::find()
            .filter(crate::app::models::password_resets::Column::Token.eq(&data.token))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(r) = reset {
            // 2. Cek Kadaluarsa (60 Menit)
            let now = chrono::Utc::now().naive_utc();
            let duration = now.signed_duration_since(r.created_at);
            
            if duration.num_minutes() > 60 {
                // Hapus token yang sudah kadaluarsa
                let _ = crate::app::models::password_resets::Entity::delete_by_id(r.email)
                    .exec(&state.db)
                    .await;
                    
                return ResponseHelper::redirect_with_error("/login", "Tautan reset password sudah kadaluarsa (melebihi 60 menit).", req.session);
            }

            // 3. Hash Password Baru
            let hashed = bcrypt::hash(data.password, bcrypt::DEFAULT_COST).unwrap();

            // 4. Update User
            let _ = users::Entity::update_many()
                .col_expr(users::Column::Password, sea_orm::sea_query::Expr::value(hashed))
                .filter(users::Column::Email.eq(&r.email))
                .exec(&state.db)
                .await;

            // 5. Hapus Token
            let _ = crate::app::models::password_resets::Entity::delete_by_id(r.email)
                .exec(&state.db)
                .await;

            return ResponseHelper::redirect_with_success("/login", "Password berhasil diubah. Silakan login.", req.session);
        }

        ResponseHelper::redirect_with_error("/login", "Token tidak valid atau sudah kadaluarsa.", req.session)
    }

    /// Proses Logout
    pub async fn logout(req: Request) -> impl IntoResponse {
        req.session.remove("user_id");
        ResponseHelper::redirect_with_success("/", "Anda telah keluar.", req.session)
    }
}
"#;
        fs::write(auth_controller_path, controller_template).ok();
        println!("   {} {}", "✅ Created:".green(), auth_controller_path.cyan());
    }

    // 5. Views
    let auth_view_dir = "src/resources/views/auth";
    fs::create_dir_all(auth_view_dir).ok();
    
    let login_template = r##"{% extends "layouts/app.rb.html" %}

{% block title %}Login - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sisi Visual -->
    <div class="split-side-visual">
        <div class="visual-inner">
            <h1 style="font-size: 3rem; font-weight: 800; margin-bottom: 1rem;">Selamat Datang Kembali</h1>
            <p style="font-size: 1.25rem; opacity: 0.9;">Masuk untuk melanjutkan petualangan Anda di ekosistem RustBasic.</p>
        </div>
    </div>

    <!-- Sisi Form -->
    <div class="split-side-content">
        <div class="content-container">
            
            
            <h2 class="title" style="font-size: 2.5rem; margin-bottom: 0.5rem; text-align: left;">Login</h2>
            <p class="text-muted mb-5">Masukkan kredensial Anda untuk masuk.</p>

            <form hx-post="/login" hx-target="body" hx-push-url="true" hx-indicator="#indicator">
                <input type="email" name="email" value="{{ old.email }}" required>
                <input type="password" name="password" required>

                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;">
                    <label style="display: flex; align-items: center; gap: 0.5rem; font-size: 0.9rem; cursor: pointer;">
                        <input type="checkbox" name="remember" value="1"> Ingat Saya
                    </label>
                    <a href="/forgot-password" class="text-primary" style="font-size: 0.9rem; font-weight: 600;">Lupa Password?</a>
                </div>

                <button type="submit" class="w-100 mb-4">MASUK</button>

                <p class="text-muted" style="text-align: center; font-size: 0.9rem;">
                    Belum punya akun? <a href="/register" class="text-primary" style="font-weight: 700;">Daftar Sekarang</a>
                </p>
            </form>
        </div>
    </div>
</div>
{% endblock %}
"##;

    let register_template = r##"{% extends "layouts/app.rb.html" %}

{% block title %}Daftar - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sisi Visual -->
    <div class="split-side-visual" style="background: linear-gradient(135deg, var(--secondary), var(--accent), var(--primary));">
        <div class="visual-inner">
            <h1 style="font-size: 3rem; font-weight: 800; margin-bottom: 1rem;">Bergabung Sekarang</h1>
            <p style="font-size: 1.25rem; opacity: 0.9;">Mulai perjalanan Anda membangun aplikasi web performa tinggi.</p>
        </div>
    </div>

    <!-- Sisi Form -->
    <div class="split-side-content">
        <div class="content-container">
            

            <h2 class="title" style="font-size: 2.5rem; margin-bottom: 0.5rem; text-align: left;">Daftar</h2>
            <p class="text-muted mb-5">Lengkapi formulir di bawah ini.</p>

            <form hx-post="/register" hx-target="body" hx-push-url="true" hx-indicator="#indicator">
                <div style="margin-bottom: 1rem;">
                    <label class="form-label" style="display: block; margin-bottom: 0.5rem; font-size: 0.9rem; font-weight: 600;">Nama Lengkap</label>
                    <input type="text" name="name" class="form-control" style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 8px;" placeholder="Nama Anda" value="{{ old.name }}" required>
                    {% if errors.name %}
                        <div style="color: #dc3545; font-size: 0.85rem; margin-top: 0.25rem;">{{ errors.name }}</div>
                    {% endif %}
                </div>

                <div style="margin-bottom: 1rem;">
                    <label class="form-label" style="display: block; margin-bottom: 0.5rem; font-size: 0.9rem; font-weight: 600;">Email</label>
                    <input type="email" name="email" class="form-control" style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 8px;" placeholder="nama@email.com" value="{{ old.email }}" required>
                    {% if errors.email %}
                        <div style="color: #dc3545; font-size: 0.85rem; margin-top: 0.25rem;">{{ errors.email }}</div>
                    {% endif %}
                </div>

                <div style="margin-bottom: 1.5rem;">
                    <label class="form-label" style="display: block; margin-bottom: 0.5rem; font-size: 0.9rem; font-weight: 600;">Password</label>
                    <input type="password" name="password" class="form-control" style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 8px;" placeholder="Min. 8 karakter" required>
                    {% if errors.password %}
                        <div style="color: #dc3545; font-size: 0.85rem; margin-top: 0.25rem;">{{ errors.password }}</div>
                    {% endif %}
                </div>

                <button type="submit" style="width: 100%; padding: 0.75rem; background: var(--primary); color: white; border: none; border-radius: 8px; font-weight: bold; cursor: pointer; margin-bottom: 1.5rem; box-shadow: 0 4px 6px rgba(99, 102, 241, 0.2);">DAFTAR SEKARANG</button>

                <p class="text-muted" style="text-align: center; font-size: 0.9rem;">
                    Sudah punya akun? <a href="/login" class="text-primary" style="font-weight: 700;">Login Disini</a>
                </p>
            </form>
        </div>
    </div>
</div>
{% endblock %}
"##;

    let forgot_template = r##"{% extends "layouts/app.rb.html" %}

{% block title %}Lupa Password - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sisi Visual -->
    <div class="split-side-visual" style="background: linear-gradient(135deg, var(--primary), var(--secondary));">
        <div class="visual-inner">
            <h1 style="font-size: 3rem; font-weight: 800; margin-bottom: 1rem;">Lupa Password?</h1>
            <p style="font-size: 1.25rem; opacity: 0.9;">Jangan khawatir, kami akan membantu Anda mendapatkan akses kembali.</p>
        </div>
    </div>

    <!-- Sisi Form -->
    <div class="split-side-content">
        <div class="content-container">
            

            <h2 class="title" style="font-size: 2.5rem; margin-bottom: 0.5rem; text-align: left;">Reset Password</h2>
            <p class="text-muted mb-5">Masukkan email Anda untuk menerima link reset.</p>

            <form hx-post="/forgot-password" hx-target="body" hx-push-url="true" hx-indicator="#indicator">
                <div style="margin-bottom: 1.5rem;">
                    <label class="form-label" style="display: block; margin-bottom: 0.5rem; font-size: 0.9rem; font-weight: 600;">Email</label>
                    <input type="email" name="email" class="form-control" style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 8px;" placeholder="nama@email.com" value="{{ old.email }}" required>
                    {% if errors.email %}
                        <div style="color: #dc3545; font-size: 0.85rem; margin-top: 0.25rem;">{{ errors.email }}</div>
                    {% endif %}
                </div>

                <button type="submit" style="width: 100%; padding: 0.75rem; background: var(--primary); color: white; border: none; border-radius: 8px; font-weight: bold; cursor: pointer; margin-bottom: 1.5rem; box-shadow: 0 4px 6px rgba(99, 102, 241, 0.2);">KIRIM LINK RESET</button>

                <p class="text-muted" style="text-align: center; font-size: 0.9rem;">
                    Ingat password Anda? <a href="/login" class="text-primary" style="font-weight: 700;">Login Disini</a>
                </p>
            </form>
        </div>
    </div>
</div>
{% endblock %}
"##;

    let login_view = "src/resources/views/auth/login.rb.html";
    if !std::path::Path::new(login_view).exists() {
        fs::write(login_view, login_template).ok();
    }
    
    let register_view = "src/resources/views/auth/register.rb.html";
    if !std::path::Path::new(register_view).exists() {
        fs::write(register_view, register_template).ok();
    }

    let forgot_view = "src/resources/views/auth/forgot.rb.html";
    if !std::path::Path::new(forgot_view).exists() {
        fs::write(forgot_view, forgot_template).ok();
    }

    let reset_view = "src/resources/views/auth/reset.rb.html";
    if !std::path::Path::new(reset_view).exists() {
        let reset_template = r##"{% extends "layouts/app.rb.html" %}

{% block title %}Reset Password - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sisi Visual -->
    <div class="split-side-visual" style="background: linear-gradient(135deg, var(--accent), var(--primary));">
        <div class="visual-inner">
            <h1 style="font-size: 3rem; font-weight: 800; margin-bottom: 1rem;">Password Baru</h1>
            <p style="font-size: 1.25rem; opacity: 0.9;">Buat password yang kuat untuk menjaga keamanan akun Anda.</p>
        </div>
    </div>

    <!-- Sisi Form -->
    <div class="split-side-content">
        <div class="content-container">
            <h2 class="title" style="font-size: 2.5rem; margin-bottom: 0.5rem; text-align: left;">Buat Password Baru</h2>
            <p class="text-muted mb-5">Silakan masukkan password baru Anda.</p>

            <form hx-post="/reset-password" hx-target="body" hx-push-url="true" hx-indicator="#indicator">
                <input type="hidden" name="token" value="{{ token }}">
                
                <div style="margin-bottom: 1.5rem;">
                    <label class="form-label" style="display: block; margin-bottom: 0.5rem; font-size: 0.9rem; font-weight: 600;">Password Baru</label>
                    <input type="password" name="password" class="form-control" style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 8px;" placeholder="Min. 8 karakter" required>
                    {% if errors.password %}
                        <div style="color: #dc3545; font-size: 0.85rem; margin-top: 0.25rem;">{{ errors.password }}</div>
                    {% endif %}
                </div>

                <button type="submit" style="width: 100%; padding: 0.75rem; background: var(--primary); color: white; border: none; border-radius: 8px; font-weight: bold; cursor: pointer; margin-bottom: 1.5rem; box-shadow: 0 4px 6px rgba(99, 102, 241, 0.2);">SIMPAN PASSWORD</button>
            </form>
        </div>
    </div>
</div>
{% endblock %}
"##;
        fs::write(reset_view, reset_template).ok();
    }

    let email_reset_view = "src/resources/views/emails/reset.rb.html";
    if !std::path::Path::new(email_reset_view).exists() {
        fs::create_dir_all("src/resources/views/emails").ok();
        let email_reset_template = r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body { font-family: 'Inter', -apple-system, sans-serif; line-height: 1.6; color: #1a1a1a; margin: 0; padding: 0; }
        .container { max-width: 600px; margin: 0 auto; padding: 40px 20px; }
        .card { background: #ffffff; border-radius: 16px; overflow: hidden; box-shadow: 0 4px 24px rgba(0,0,0,0.06); border: 1px solid #f0f0f0; }
        .header { background: linear-gradient(135deg, #6366f1, #a855f7); padding: 40px; text-align: center; color: white; }
        .content { padding: 40px; }
        .button { display: inline-block; padding: 14px 32px; background: #6366f1; color: #ffffff !important; text-decoration: none; border-radius: 8px; font-weight: 600; margin: 24px 0; }
        .footer { padding: 24px; text-align: center; font-size: 13px; color: #6b7280; }
        h1 { margin: 0; font-size: 24px; font-weight: 800; letter-spacing: -0.025em; }
        p { margin: 16px 0; color: #4b5563; }
        .divider { height: 1px; background: #f3f4f6; margin: 24px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="card">
            <div class="header">
                <h1>{{ app_name }}</h1>
            </div>
            <div class="content">
                <h2 style="margin: 0; color: #111827; font-size: 20px;">Halo!</h2>
                <p>Anda menerima email ini karena kami menerima permintaan reset password untuk akun Anda di <strong>{{ app_name }}</strong>.</p>
                
                <div style="text-align: center;">
                    <a href="{{ reset_url }}" class="button">Reset Password Saya</a>
                </div>

                <p style="font-size: 14px; color: #9ca3af;">Link ini akan kadaluarsa dalam 60 menit. Jika Anda tidak merasa meminta reset password, abaikan saja email ini.</p>
                
                <div class="divider"></div>
                
                <p style="font-size: 12px; color: #9ca3af;">
                    Jika Anda kesulitan menekan tombol, salin dan tempel URL berikut ke browser Anda:<br>
                    <span style="word-break: break-all; color: #6366f1;">{{ reset_url }}</span>
                </p>
            </div>
        </div>
        <div class="footer">
            &copy; 2026 {{ app_name }}. All rights reserved.
        </div>
    </div>
</body>
</html>
"##;
        fs::write(email_reset_view, email_reset_template).ok();
    }

    let dashboard_view = "src/resources/views/dashboard.rb.html";
    if !std::path::Path::new(dashboard_view).exists() {
        let dashboard_template = r##"{% extends "layouts/app.rb.html" %}

{% block title %}{{ title }} - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sidebar / Visual Side (Kiri - Narrower) -->
    <div class="split-side-visual" style="flex: 0.4; align-items: flex-start; text-align: left; padding: 3rem;">
        <div style="width: 100%;">
            <div style="width: 80px; height: 80px; background: rgba(255,255,255,0.2); border-radius: 2rem; display: flex; align-items: center; justify-content: center; font-size: 2rem; font-weight: 800; margin-bottom: 2rem; border: 2px solid rgba(255,255,255,0.3);">
                {{ user_name[0] | upper }}
            </div>
            <h2 style="font-size: 2rem; font-weight: 800; margin-bottom: 0.5rem;">{{ user_name }}</h2>
            <p style="opacity: 0.8; margin-bottom: 3rem;">{{ user_email }}</p>

            <nav style="display: flex; flex-direction: column; gap: 0.75rem; width: 100%;">
                <a href="/dashboard" class="btn" style="background: rgba(255,255,255,0.2); color: #fff; justify-content: flex-start; text-transform: none;">Dashboard Overview</a>
                <a href="/" class="btn" style="color: #fff; justify-content: flex-start; text-transform: none; opacity: 0.7;">Beranda Utama</a>
                <div style="margin-top: auto; padding-top: 2rem;">
                    <form hx-post="/logout" hx-target="body" style="margin:0;">
                        <button type="submit" style="width: 100%; padding: 0.75rem; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); border-radius: 8px; color: white; font-weight: bold; cursor: pointer; text-align: center;">LOGOUT</button>
                    </form>
                </div>
            </nav>
        </div>
    </div>

    <!-- Main Content (Kanan - Wider) -->
    <div class="split-side-content" style="flex: 1.2; align-items: flex-start; justify-content: flex-start; background: #f8faff;">
        <div style="width: 100%; padding: 4rem;">
            <div class="mb-5">
                <h1 class="title" style="font-size: 3rem; text-align: left; margin-bottom: 0.5rem;">Dashboard</h1>
                <p class="text-muted">Kelola aplikasi dan pantau aktivitas Anda di sini.</p>
            </div>

            <!-- Grid Statistik -->
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem; margin-bottom: 4rem;">
                <div style="background: white; border-radius: 12px; padding: 1.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.05); border: 1px solid #eee;">
                    <p class="text-muted text-uppercase" style="font-size: 0.8rem; font-weight: 700; letter-spacing: 0.1em; margin-bottom: 1rem;">Total Pengguna</p>
                    <div style="font-size: 2.5rem; font-weight: 800; color: var(--primary);">{{ total_users }}</div>
                </div>
                
                <div style="border-bottom: 4px solid var(--accent); padding: 1.5rem 0;">
                    <p class="text-muted text-uppercase" style="font-size: 0.8rem; font-weight: 700; letter-spacing: 0.1em; margin-bottom: 1rem;">Status Sistem</p>
                    <div style="display: flex; align-items: center; gap: 0.75rem; font-size: 1.5rem; font-weight: 800; color: #2e7d32; padding: 1.2rem 0;">
                        <div style="width: 12px; height: 12px; background: #4caf50; border-radius: 50%;"></div>
                        ONLINE
                    </div>
                </div>

                <div style="background: white; border-radius: 12px; padding: 1.5rem; box-shadow: 0 4px 6px rgba(0,0,0,0.05); border: 1px solid #eee;">
                    <p class="text-muted text-uppercase" style="font-size: 0.8rem; font-weight: 700; letter-spacing: 0.1em; margin-bottom: 1rem;">Performa</p>
                    <div style="font-size: 2.5rem; font-weight: 800; color: var(--secondary);">99.9%</div>
                </div>
            </div>

            <!-- Content Area -->
            <div style="background: #fff; padding: 3rem; border-radius: 0; border-left: 8px solid var(--primary);">
                <h2 style="font-weight: 800; margin-bottom: 1.5rem;">Informasi Sistem</h2>
                <p class="text-muted" style="line-height: 1.8; margin-bottom: 2rem;">
                    Aplikasi ini berjalan menggunakan backend Rust yang dioptimalkan. Semua data diproses secara real-time melalui koneksi SQLite yang aman.
                </p>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                    <span class="badge">Engine: Axum 0.8</span>
                    <span class="badge">Render: Minijinja</span>
                    <span class="badge">Frontend: HTMX + Minijinja</span>
                </div>
            </div>
        </div>
    </div>
</div>
{% endblock %}
"##;
        fs::write(dashboard_view, dashboard_template).ok();
    }
    
    // 6. Create Dashboard Controller
    let dashboard_controller_path = "src/app/http/controllers/dashboard_controller.rs";
    if !std::path::Path::new(dashboard_controller_path).exists() {
        let dashboard_template = r#"use crate::app::view;
use crate::app::models::users;
use crate::config::requests::Request;
use crate::config::server::AppState;
use axum::{response::IntoResponse, extract::State};
use minijinja::context;
use sea_orm::{EntityTrait, PaginatorTrait};

pub struct DashboardController;

impl DashboardController {
    pub async fn index(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        let user_id = req.session.get::<i32>("user_id").unwrap_or(0);
        let user = users::Entity::find_by_id(user_id).one(&state.db).await.ok().flatten();
        let total_users = users::Entity::find().count(&state.db).await.unwrap_or(0);

        view(&req, "dashboard.rb.html", context! {
            title => "Dashboard",
            user_name => user.as_ref().map(|u| u.name.clone()).unwrap_or("Guest".to_string()),
            user_email => user.as_ref().map(|u| u.email.clone()).unwrap_or_default(),
            total_users => total_users,
        })
    }
}
"#;
        fs::write(dashboard_controller_path, dashboard_template).ok();
        println!("   {} {}", "✅ Created:".green(), dashboard_controller_path.cyan());
    }
    update_controller_mod_rs("dashboard_controller");

    println!("   {} Folder src/resources/views/auth dan dashboard siap.", "✅ Views:".green());

    // 6. Update welcome.rb.html
    let welcome_path = "src/resources/views/welcome.rb.html";
    if let Ok(content) = fs::read_to_string(welcome_path) {
        if !content.contains("{% if auth %}") {
            println!("   {} {}", "⚠️  Manual:".yellow(), "Pastikan welcome.rb.html memiliki tombol login/register.".dimmed());
        } else {
            println!("   {} {}", "✅ OK:".green(), "welcome.rb.html sudah memiliki logika auth.".dimmed());
        }
    }

    println!("\n{}", "✨ Authentication scaffolded successfully!".green().bold());
    println!("{}", "Jalankan 'cargo rustbasic route:list' untuk melihat route baru.".dimmed());
}

pub async fn remove_auth() {
    println!("\n{}", "🗑️  Removing Authentication Scaffold...".red().bold());

    // 1. Delete src/routes/auth.rs
    let auth_route_path = "src/routes/auth.rs";
    if std::path::Path::new(auth_route_path).exists() {
        fs::remove_file(auth_route_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_route_path.cyan());
    }

    // 2. Update src/routes/mod.rs
    let routes_mod_path = "src/routes/mod.rs";
    if let Ok(mut content) = fs::read_to_string(routes_mod_path) {
        if content.contains("pub mod auth;") {
            content = content.replace("pub mod auth;\n", "");
            fs::write(routes_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), routes_mod_path.cyan());
        }
    }

    // 3. Update src/routes/web.rs
    let web_route_path = "src/routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_route_path) {
        let mut changed = false;
        
        // Remove imports
        if content.contains("use axum::{Router, routing::{get, post}, middleware::from_fn};") {
            content = content.replace("use axum::{Router, routing::{get, post}, middleware::from_fn};", "use axum::{Router, routing::get};");
            changed = true;
        }
        
        let imports_to_remove = [
            "use crate::app::http::controllers::{auth, dashboard_controller};\n",
            "use crate::app::http::middleware::auth::auth_middleware;\n",
            "use crate::routes::auth as auth_routes;\n",
            "use crate::app::http::controllers::{auth, dashboard_controller};",
            "use crate::app::http::middleware::auth::auth_middleware;",
            "use crate::routes::auth as auth_routes;",
        ];
        
        for imp in imports_to_remove {
            if content.contains(imp) {
                content = content.replace(imp, "");
                changed = true;
            }
        }

        // Remove auth_protected_routes logic and restore basic Router
        if content.contains("let auth_protected_routes = Router::new()") {
            let re = Regex::new(r##"(?s)let auth_protected_routes = Router::new\(\).*?\.layer\(from_fn\(auth_middleware\)\);"##).unwrap();
            content = re.replace(&content, "").to_string();
            
            content = content.replace(".merge(auth_routes::router())", "");
            content = content.replace(".merge(auth_protected_routes)", "");
            
            // Restore clean Router::new()
            let clean_router = r#"    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/dev", get(welcome_controller::dev_info))"#;
            
            let router_re = Regex::new(r##"(?s)Router::new\(\).*?\.route\(\s*\"/dev\"\s*,\s*get\(welcome_controller::dev_info\)\s*\)"##).unwrap();
            content = router_re.replace(&content, clean_router).to_string();
            
            changed = true;
        }

        if changed {
            fs::write(web_route_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), web_route_path.cyan());
        }
    }

    // 7. Delete Controllers
    let auth_controller_dir = "src/app/http/controllers/auth";
    if std::path::Path::new(auth_controller_dir).exists() {
        fs::remove_dir_all(auth_controller_dir).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_controller_dir.cyan());
    }

    // 7.1 Delete Password Resets Migration & Model
    if let Ok(entries) = std::fs::read_dir("database/migrations") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with("_create_password_resets_table.rs") {
                    let path = entry.path();
                    fs::remove_file(&path).ok();
                    println!("   {} {}", "✅ Deleted:".green(), path.display().to_string().cyan());
                }
            }
        }
    }
    
    let model_path = "src/app/models/password_resets.rs";
    if std::path::Path::new(model_path).exists() {
        fs::remove_file(model_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), model_path.cyan());
    }

    // 8. Delete Views
    let auth_view_dir = "src/resources/views/auth";
    if std::path::Path::new(auth_view_dir).exists() {
        fs::remove_dir_all(auth_view_dir).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_view_dir.cyan());
    }

    // 6. Delete Dashboard Controller
    let dashboard_path = "src/app/http/controllers/dashboard_controller.rs";
    if std::path::Path::new(dashboard_path).exists() {
        fs::remove_file(dashboard_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), dashboard_path.cyan());
    }

    // 7. Update src/app/http/controllers/mod.rs
    let controllers_mod_path = "src/app/http/controllers/mod.rs";
    if let Ok(mut content) = fs::read_to_string(controllers_mod_path) {
        let mut changed = false;
        if content.contains("pub mod auth;") {
            content = content.replace("pub mod auth;\n", "");
            changed = true;
        }
        if content.contains("pub mod dashboard_controller;") {
            content = content.replace("pub mod dashboard_controller;\n", "");
            changed = true;
        }
        if changed {
            fs::write(controllers_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), controllers_mod_path.cyan());
        }
    }

    // 7.2 Update src/app/models/mod.rs
    let models_mod_path = "src/app/models/mod.rs";
    if let Ok(mut content) = fs::read_to_string(models_mod_path) {
        if content.contains("pub mod password_resets;") {
            content = content.replace("pub mod password_resets;\n", "");
            content = content.replace("pub mod password_resets;", "");
            fs::write(models_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), models_mod_path.cyan());
        }
    }

    // 7.3 Update database/migrations/mod.rs
    let migration_mod_path = "database/migrations/mod.rs";
    if let Ok(content) = fs::read_to_string(migration_mod_path) {
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut changed = false;
        
        // Remove the mod line
        lines.retain(|line| {
            if line.contains("_create_password_resets_table;") {
                changed = true;
                false
            } else if line.contains("Box::new(") && line.contains("_create_password_resets_table::Migration") {
                changed = true;
                false
            } else {
                true
            }
        });

        if changed {
            fs::write(migration_mod_path, lines.join("\n")).ok();
            println!("   {} {}", "📝 Updated:".blue(), migration_mod_path.cyan());
        }
    }

    println!("\n{}", "✨ Authentication removed successfully!".green().bold());
}

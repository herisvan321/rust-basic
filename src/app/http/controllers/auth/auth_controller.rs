/* ---------------------------------------------------------
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
use serde::Deserialize;
use validator::Validate;
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
}

pub struct AuthController;

impl AuthController {
    /// Menampilkan halaman login
    pub async fn login_page(req: Request) -> impl IntoResponse {
        view(&req, "auth/login.html", context! { title => "Login" })
    }

    /// Menampilkan halaman register
    pub async fn register_page(req: Request) -> impl IntoResponse {
        view(&req, "auth/register.html", context! { title => "Daftar Akun" })
    }

    /// Proses Pendaftaran
    pub async fn register(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<RegisterRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/register"),
        };

        // 2. Cek apakah email sudah terdaftar (Sea-ORM)
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

        // 4. Simpan ke Database (Sea-ORM ActiveModel)
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

        // 2. Ambil User dari DB (Sea-ORM)
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
                return ResponseHelper::redirect_with_success("/dashboard", "Selamat datang kembali!", req.session);
            }
        }

        ResponseHelper::redirect_with_error("/login", "Email atau password salah", req.session)
    }

    /// Proses Logout
    pub async fn logout(req: Request) -> impl IntoResponse {
        req.session.remove("user_id");
        ResponseHelper::redirect_with_success("/", "Anda telah keluar.", req.session)
    }
}

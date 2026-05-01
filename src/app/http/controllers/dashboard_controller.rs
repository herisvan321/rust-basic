/* ---------------------------------------------------------
 * 📑 LABEL: DASHBOARD CONTROLLER (dashboard_controller.rs)
 * Menampilkan halaman dashboard untuk user yang sudah login.
 * --------------------------------------------------------- */

use crate::app::view;
use crate::app::models::users;
use crate::config::requests::Request;
use axum::{response::IntoResponse, extract::State};
use minijinja::context;
use crate::config::server::AppState;
use sea_orm::{EntityTrait, PaginatorTrait};

pub struct DashboardController;

impl DashboardController {
    pub async fn index(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Ambil user_id dari session
        let user_id = req.session.get::<i32>("user_id").unwrap_or(0);

        // 2. Ambil data user menggunakan Sea-ORM Model
        let user = users::Entity::find_by_id(user_id)
            .one(&state.db)
            .await
            .ok()
            .flatten();

        let (user_name, user_email) = match &user {
            Some(u) => (u.name.clone(), u.email.clone()),
            None => ("User".to_string(), "".to_string()),
        };

        // 3. Ambil statistik (total user)
        let total_users = users::Entity::find()
            .count(&state.db)
            .await
            .unwrap_or(0) as i64;

        // 4. Render tampilan dashboard
        view(&req, "dashboard.html", context! {
            title => "Dashboard Utama",
            user_name => user_name,
            user_email => user_email,
            total_users => total_users
        })
    }
}

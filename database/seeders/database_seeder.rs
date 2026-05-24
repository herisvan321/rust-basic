use rustbasic_core::seeder;
use rustbasic_core::sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use crate::app::models::User;
use rustbasic_core::bcrypt::{hash, DEFAULT_COST};
use rustbasic_core::colored::Colorize;
use rustbasic_core::serde_json;

seeder! {
    run(db) {
        println!("   {} Sedang memproses DatabaseSeeder...", "⏳".blue());
        
        // 1. Cek apakah user admin sudah ada
        let admin_exists = User::find()
            .filter(crate::app::models::users::Column::Email.eq("admin@rustbasic.com"))
            .one(db)
            .await?
            .is_some();

        if !admin_exists {
            let hashed_password = hash("password123", DEFAULT_COST).unwrap();
            
            User::create(db, serde_json::json!({
                "name": "Administrator",
                "email": "admin@rustbasic.com",
                "password": hashed_password,
            })).await?;

            println!("   {} User admin default berhasil dibuat (admin@rustbasic.com / password123)", "✅".green());
        } else {
            println!("   {} User admin sudah ada, melewati...", "⏩".yellow());
        }

        Ok(())
    }
}

use rustbasic_core::model;
use rustbasic_core::sea_orm::entity::prelude::*;

model! {
    table: "users",
    fillable: [name, email, password],
    Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        #[sea_orm(unique)]
        pub email: String,
        pub email_verified_at: Option<DateTime>,
        pub password: String,
        pub remember_token: Option<String>,
    }
}

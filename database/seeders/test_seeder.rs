#[allow(unused_imports)]
use rustbasic_core::sea_orm::{DatabaseConnection, Set, ActiveModelTrait};
use rustbasic_core::colored::Colorize;
use rustbasic_core::seeder::SeederTrait;
// use crate::app::models::test; // Sesuaikan dengan model Anda

pub struct TestSeeder;

#[async_trait::async_trait]
impl SeederTrait for TestSeeder {
    async fn run(&self, _db: &DatabaseConnection) -> Result<(), rustbasic_core::sea_orm::DbErr> {
        println!("   {} Sedang memproses TestSeeder...", "⏳".blue());
        
        // Contoh:
        /*
        let _ = test::ActiveModel {
            name: Set("Example Data".to_owned()),
            ..Default::default()
        }.insert(_db).await?;
        */

        Ok(())
    }
}

use rustbasic_core::sea_orm::DatabaseConnection;
use rustbasic_core::colored::*;
use crate::seeders;
use rustbasic_core::seeder::SeederTrait;

pub async fn run(db: &DatabaseConnection) {
    println!("\n{}", "🌱 Menjalankan Seeder Database...".blue().bold());
    
    // REGISTRASI SEEDER DI SINI
    let seeders: Vec<Box<dyn SeederTrait>> = vec![
        Box::new(seeders::test_seeder::TestSeeder),
        Box::new(seeders::database_seeder::DatabaseSeeder),
    ];

    for seeder in seeders {
        if let Err(e) = seeder.run(db).await {
            println!("{} {}", "❌ Gagal menjalankan seeder:".red(), e);
        }
    }
    
    println!("{}", "✅ Semua seeder selesai diproses!".green().bold());
}

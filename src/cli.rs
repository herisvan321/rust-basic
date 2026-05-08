use rustbasic::app::seeder;
use rustbasic::migrations::Migrator;
use rustbasic_core::sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    rustbasic_core::cli::run_cli(
        |command| Box::pin(async move {
            let cfg = rustbasic_core::Config::load();
            let db = rustbasic_core::database::connect(&cfg).await;
            
            let result = match command.as_str() {
                "migrate:refresh" => Migrator::refresh(&db).await,
                "migrate:rollback" | "migrate:back" => Migrator::down(&db, Some(1)).await,
                _ => Migrator::up(&db, None).await,
            };

            result.map_err(|e| e.to_string())
        }),
        || Box::pin(async {
            let cfg = rustbasic_core::Config::load();
            let db = rustbasic_core::database::connect(&cfg).await;
            seeder::run(&db).await;
        })
    ).await;
}

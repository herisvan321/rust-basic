use std::env;
use dotenvy::dotenv;
use colored::*;

pub mod scaffolding;
pub mod database;
pub mod monitoring;
pub mod builder;
pub mod utils;
pub mod auth;

fn main() {
    dotenv().expect("❌ Error: File .env tidak ditemukan! Silakan salin .env.example menjadi .env sebelum menggunakan CLI.");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "make:model" => {
            if args.len() < 3 {
                println!("{}", "❌ Error: Nama model tidak ditentukan.".red().bold());
                return;
            }
            let model_name = &args[2];
            let with_migration = args.contains(&"-m".to_string());
            
            scaffolding::make_model(model_name);
            if with_migration {
                scaffolding::make_rust_migration(model_name);
            }
        }
        "make:migration" => {
            if args.len() < 3 {
                println!("{}", "❌ Error: Nama migration tidak ditentukan.".red().bold());
                return;
            }
            scaffolding::make_rust_migration(&args[2]);
        }
        "make:controller" => {
            if args.len() < 3 {
                println!("{}", "❌ Error: Nama controller tidak ditentukan.".red().bold());
                return;
            }
            scaffolding::make_controller(&args[2]);
        }
        "make:middleware" => {
            if args.len() < 3 {
                println!("{}", "❌ Error: Nama middleware tidak ditentukan.".red().bold());
                return;
            }
            scaffolding::make_middleware(&args[2]);
        }
        "migrate" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(database::run_manual_migrations());
        }
        "migrate:refresh" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(database::refresh_manual_migrations());
        }
        "migrate:back" | "migrate:rollback" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(database::rollback_manual_migrations());
        }
        "route:list" => {
            monitoring::list_routes();
        }
        "build" => {
            builder::build_project();
        }
        "cache:clear" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(database::clear_cache());
        }
        "check:update" => {
            monitoring::check_updates();
        }
        "check:security" => {
            monitoring::check_security();
        }
        "key:generate" => {
            database::generate_app_key();
        }
        "make:auth" | "auth" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            if args.len() >= 3 && args[2] == "back" {
                rt.block_on(auth::remove_auth());
            } else {
                rt.block_on(auth::make_auth());
            }
        }
        "auth:back" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(auth::remove_auth());
        }
        _ => {
            println!("{} {}", "❌ Error: Perintah tidak dikenal:".red().bold(), command.yellow());
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "🛠️  RustBasic CLI".magenta().bold());
    println!("{}", "=================".magenta());
    println!("{}", "Penggunaan:".bold());
    println!("  {} {} <Nama> [-m]   {}", "cargo rustbasic".blue(), "make:model".green(), "Membuat model Sea-ORM (dan migration Rust)".dimmed());
    println!("  {} {} <Nama>    {}", "cargo rustbasic".blue(), "make:migration".green(), "Membuat file migration Rust".dimmed());
    println!("  {} {} <Nama>  {}", "cargo rustbasic".blue(), "make:controller".green(), "Membuat controller Axum".dimmed());
    println!("  {} {} <Nama>  {}", "cargo rustbasic".blue(), "make:middleware".green(), "Membuat middleware Axum".dimmed());
    println!("  {} {}                  {}", "cargo rustbasic".blue(), "migrate".green(), "Menjalankan migrasi database (Sea-ORM)".dimmed());
    println!("  {} {}          {}", "cargo rustbasic".blue(), "migrate:refresh".green(), "Rollback semua dan jalankan kembali migrasi".dimmed());
    println!("  {} {}             {}", "cargo rustbasic".blue(), "migrate:back".green(), "Membatalkan migrasi terakhir (Rollback)".dimmed());
    println!("  {} {}               {}", "cargo rustbasic".blue(), "route:list".green(), "Menampilkan daftar route".dimmed());
    println!("  {} {}                    {}", "cargo rustbasic".blue(), "build".green(), "Membangun project dengan pilihan".dimmed());
    println!("  {} {}             {}", "cargo rustbasic".blue(), "check:update".green(), "Cek versi terbaru paket (dependencies)".dimmed());
    println!("  {} {}           {}", "cargo rustbasic".blue(), "check:security".green(), "Audit keamanan aplikasi".dimmed());
    println!("  {} {}               {}", "cargo rustbasic".blue(), "cache:clear".green(), "Membersihkan logs dan database sessions".dimmed());
    println!("  {} {}             {}", "cargo rustbasic".blue(), "key:generate".green(), "Membuat APP_KEY baru di file .env".dimmed());
    println!("  {} {}                   {}", "cargo rustbasic".blue(), "make:auth".green(), "Scaffold autentikasi (Login/Register)".dimmed());
    println!("  {} {}                   {}", "cargo rustbasic".blue(), "auth:back".red(), "Menghapus semua scaffolding autentikasi".dimmed());

    println!();
}

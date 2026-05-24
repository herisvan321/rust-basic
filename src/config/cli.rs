use rustbasic_core::Config;

pub async fn handle(args: &[String], cfg: &Config) -> bool {
    if args.len() < 2 {
        return false;
    }

    let command = args[1].as_str();

    // Daftar perintah yang ditangani oleh CLI lokal project
    let is_migration_cmd = command.starts_with("migrate") || command == "db:seed";
    let is_storage_cmd = command == "storage:link";

    if !is_migration_cmd && !is_storage_cmd {
        return false;
    }

    println!("🛠️  RustBasic Local CLI - Command: {}", command);

    if is_storage_cmd {
        handle_storage_link();
        return true;
    }

    // Hubungkan ke database menggunakan core::database::connect
    let db = rustbasic_core::database::connect(cfg).await;

    match command {
        "migrate" => {
            use rustbasic_core::sea_orm_migration::MigratorTrait;
            println!("🚀 Menjalankan migrasi database...");
            if let Err(e) = crate::migrations::Migrator::up(&db, None).await {
                println!("❌ Gagal menjalankan migrasi: {}", e);
            } else {
                println!("✅ Migrasi selesai!");
            }
        }
        "migrate:refresh" => {
            use rustbasic_core::sea_orm_migration::MigratorTrait;
            println!("🔄 Mereset dan menjalankan ulang migrasi...");
            if let Err(e) = crate::migrations::Migrator::fresh(&db).await {
                println!("❌ Gagal refresh migrasi: {}", e);
            } else {
                println!("✅ Database berhasil di-refresh!");
            }
        }
        "migrate:back" | "migrate:rollback" => {
            use rustbasic_core::sea_orm_migration::MigratorTrait;
            println!("⬅️  Rollback migrasi terakhir...");
            if let Err(e) = crate::migrations::Migrator::down(&db, None).await {
                println!("❌ Gagal rollback: {}", e);
            } else {
                println!("✅ Rollback berhasil!");
            }
        }
        "db:seed" => {
            println!("🌱 Fitur db:seed membutuhkan implementasi lokal.");
        }
        _ => return false,
    }

    true
}

fn handle_storage_link() {
    let target = crate::config::app::STORAGE_TARGET;
    let source = crate::config::app::STORAGE_SOURCE;

    // 1. Buat folder source jika belum ada
    if let Err(e) = std::fs::create_dir_all(source) {
        println!("❌ Gagal membuat direktori storage: {}", e);
        return;
    }

    // 2. Cek apakah link sudah ada
    let path = std::path::Path::new(target);
    if path.exists() || path.is_symlink() {
        println!("ℹ️  Link 'public/storage' sudah ada atau berupa file/folder lain.");
        return;
    }

    // 3. Buat symlink
    println!("🔗 Membuat symbolic link...");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        if let Err(e) = symlink("../storage/app/public", target) {
            println!("❌ Gagal membuat symlink: {}", e);
        } else {
            println!("✅ Link storage berhasil dibuat! [public/storage -> storage/app/public]");
        }
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::symlink_dir;
        if let Err(e) = symlink_dir("../storage/app/public", target) {
            println!("❌ Gagal membuat symlink: {}", e);
        } else {
            println!("✅ Link storage berhasil dibuat! [public/storage -> storage/app/public]");
        }
    }
}

/* ---------------------------------------------------------
 * 📑 LABEL: LOGGER & BANNER (config/logger.rs)
 * Pengaturan logging (Terminal + File) dan tampilan visual startup.
 * --------------------------------------------------------- */

use tracing_subscriber::{fmt, EnvFilter, prelude::*, registry};
use tracing_appender::non_blocking::WorkerGuard;

/// Inisialisasi Logger untuk Terminal dan File
/// Mengembalikan WorkerGuard yang harus disimpan di main.rs agar logging file tetap berjalan.
pub fn init() -> WorkerGuard {
    // 1. Setup File Appender (storage/logs/rustbasic.log)
    // Akan membuat file baru setiap hari (Daily Rotation)
    let file_appender = tracing_appender::rolling::daily("storage/logs", "rustbasic.log");
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    // 2. Setup Filter
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| 
        "rustbasic=debug,tower_http=debug,axum_session=warn,sqlx=warn".into()
    );

    // 3. Gabungkan Layer Terminal dan Layer File
    registry()
        .with(env_filter)
        .with(fmt::layer().with_target(false).with_ansi(true)) // Terminal (dengan warna)
        .with(fmt::layer().with_target(false).with_ansi(false).with_writer(non_blocking_file)) // File (tanpa warna)
        .init();

    // 4. Cetak Banner Premium ke Terminal
    print_banner();

    guard
}

fn print_banner() {
    println!(r#"
    
    ██████╗ ██╗   ██╗███████╗████████╗██████╗  █████╗ ███████╗██╗ ██████╗
    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██║██╔════╝
    ██████╔╝██║   ██║███████╗   ██║   ██████╔╝███████║███████╗██║██║     
    ██╔══██╗██║   ██║╚════██║   ██║   ██╔══██╗██╔══██║╚════██║██║██║     
    ██║  ██║╚██████╔╝███████║   ██║   ██████╔╝██║  ██║███████║██║╚██████╗
    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝ ╚═════╝
    "#);
    println!("    >> RustBasic Monolith Framework - Version 2026 <<\n");
}

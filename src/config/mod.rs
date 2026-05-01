/* ---------------------------------------------------------
 * 📑 LABEL: CONFIG MANAGER (config/mod.rs)
 * File ini mengelola sub-modul konfigurasi dan re-export.
 * --------------------------------------------------------- */

pub mod app;
pub mod session;
pub mod database;
pub mod server;
pub mod logger;
pub mod requests;
pub mod responses;

// Re-export Config agar bisa dipanggil dengan crate::config::Config
pub use app::Config;

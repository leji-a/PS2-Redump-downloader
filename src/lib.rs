// Module declarations for the PS2 Redump Downloader library
pub mod config;
pub mod downloader;
pub mod models;
pub mod scraper;
pub mod utils;

// Re-export main types and functions for convenient access
pub use config::Config;
pub use downloader::Downloader;
pub use models::Game;
pub use scraper::Scraper;
pub use utils::setup_folders;

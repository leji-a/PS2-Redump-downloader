pub mod config;
pub mod downloader;
pub mod models;
pub mod scraper;
pub mod utils;

pub use config::Config;
pub use downloader::Downloader;
pub use models::Game;
pub use scraper::Scraper;
pub use utils::setup_folders;

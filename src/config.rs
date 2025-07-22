use anyhow::Result;
use configparser::ini::Ini;
use serde::{Deserialize, Serialize};

/// Configuration for the downloader application, loaded from config.ini.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Base URL for ISO downloads
    pub iso_url: String,
    /// Name of the JSON file containing the list of games
    pub list_files_json_name: String,
    /// Whether to use external browser for ISO download
    pub external_iso_download: bool,
    /// Maximum number of download retries
    pub max_retries: u32,
    /// Delay between retries (seconds)
    pub delay_between_retries: u64,
    /// Timeout for requests (seconds)
    pub timeout_request: Option<u64>,
    /// Name of the temporary folder
    pub tmp_folder_name: String,
    /// Name of the ISO folder inside the temporary folder
    pub tmp_iso_folder_name: String,
}

impl Config {
    /// Loads configuration from the given path (expands tilde if present).
    pub fn load(path: &str) -> Result<Self> {
        let mut config = Ini::new();
        config.load(Self::expand_tilde(path)).map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;

        let url_section = config.get("url", "ISO").map_or("https://myrient.erista.me/files/Redump/Sony%20-%20PlayStation%202/".to_string(), |s| s.to_string());
        
        let list_files_json_name = config.get("Download", "LIST_FILES_JSON_NAME").map_or("listPS2Titles.json".to_string(), |s| s.to_string());
        let external_iso_download = config.getuint("Download", "EXTERNAL_ISO").unwrap_or(Some(0)).unwrap_or(0) != 0;
        let max_retries = config.getuint("Download", "MAX_RETRIES").unwrap_or(Some(5)).unwrap_or(5) as u32;
        let delay_between_retries = config.getuint("Download", "DELAY_BETWEEN_RETRIES").unwrap_or(Some(5)).unwrap_or(5) as u64;
        let timeout_request = config.getuint("Download", "TIMEOUT_REQUEST").unwrap_or(None).map(|v| v as u64);
        
        let tmp_folder_name = config.get("folder", "TMP_FOLDER_NAME").map_or("tmp".to_string(), |s| s.to_string());
        let tmp_iso_folder_name = config.get("folder", "TMP_ISO_FOLDER_NAME").map_or("iso_files".to_string(), |s| s.to_string());

        let config = Config {
            iso_url: url_section,
            list_files_json_name,
            external_iso_download,
            max_retries,
            delay_between_retries,
            timeout_request,
            tmp_folder_name,
            tmp_iso_folder_name,
        };

        // Validate configuration
        if config.max_retries == 0 {
            anyhow::bail!("MAX_RETRIES must be greater than 0");
        }
        if config.delay_between_retries == 0 {
            anyhow::bail!("DELAY_BETWEEN_RETRIES must be greater than 0");
        }

        Ok(config)
    }

    /// Loads configuration from the recommended locations, or creates a default config if not found.
    pub fn load_or_create() -> Result<(Self, std::path::PathBuf)> {
        use std::path::PathBuf;
        use std::fs;
        use std::io::Write;

        // List of config paths to try, in order
        let mut candidates = Vec::new();
        // 1. Current directory
        candidates.push(PathBuf::from("config.ini"));

        #[cfg(windows)]
        {
            if let Some(appdata) = std::env::var_os("APPDATA") {
                candidates.push(PathBuf::from(appdata).join("ps2-redump-downloader/config.ini"));
            }
            candidates.push(PathBuf::from("C:/ProgramData/ps2-redump-downloader/config.ini"));
        }
        #[cfg(not(windows))]
        {
            if let Some(home) = std::env::var_os("HOME") {
                candidates.push(PathBuf::from(home).join(".config/ps2-redump-downloader/config.ini"));
            }
            candidates.push(PathBuf::from("/etc/ps2-redump-downloader/config.ini"));
        }

        // Try to load from each candidate
        for path in &candidates {
            if path.exists() {
                match Self::load(path.to_str().unwrap()) {
                    Ok(cfg) => return Ok((cfg, path.clone())),
                    Err(e) => eprintln!("Failed to load config from {}: {}", path.display(), e),
                }
            }
        }

        // Not found: create default config in user config dir
        #[cfg(windows)]
        let default_path = {
            if let Some(appdata) = std::env::var_os("APPDATA") {
                PathBuf::from(appdata).join("ps2-redump-downloader/config.ini")
            } else {
                PathBuf::from("config.ini")
            }
        };
        #[cfg(not(windows))]
        let default_path = {
            if let Some(home) = std::env::var_os("HOME") {
                PathBuf::from(home).join(".config/ps2-redump-downloader/config.ini")
            } else {
                PathBuf::from("config.ini")
            }
        };

        // Ensure parent directory exists
        if let Some(parent) = default_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Default config contents (from README.md)
        let default_config = r#"[url]
ISO = https://myrient.erista.me/files/Redump/Sony%20-%20PlayStation%202/

[Download]
# Downloaded Game list fileName 
LIST_FILES_JSON_NAME = listPS2Titles.json 

# Download ISO file using navigator
EXTERNAL_ISO = 0 

MAX_RETRIES = 10
DELAY_BETWEEN_RETRIES = 10
TIMEOUT_REQUEST = 600

[folder]
TMP_FOLDER_NAME = ~/PS2-Games
TMP_ISO_FOLDER_NAME = iso_files
"#;
        let mut file = fs::File::create(&default_path)
            .map_err(|e| anyhow::anyhow!("Failed to create default config at {}: {}", default_path.display(), e))?;
        file.write_all(default_config.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to write default config at {}: {}", default_path.display(), e))?;
        eprintln!("No config.ini found. Created a default config at {}. Please edit it as needed.", default_path.display());

        // Now load the config
        let cfg = Self::load(default_path.to_str().unwrap())?;
        Ok((cfg, default_path))
    }

    /// Expands a path that starts with ~ to the user's home directory.
    fn expand_tilde(path: &str) -> std::path::PathBuf {
        if path.starts_with("~/") {
            #[cfg(windows)]
            {
                if let Some(home) = std::env::var_os("USERPROFILE") {
                    return std::path::PathBuf::from(home).join(&path[2..]);
                }
            }
            #[cfg(not(windows))]
            {
                if let Some(home) = std::env::var_os("HOME") {
                    return std::path::PathBuf::from(home).join(&path[2..]);
                }
            }
        }
        std::path::PathBuf::from(path)
    }

    /// Returns the expanded path to the temporary folder.
    pub fn tmp_folder_path(&self) -> std::path::PathBuf {
        Self::expand_tilde(&self.tmp_folder_name)
    }

    /// Returns the expanded path to the ISO folder inside the temporary folder.
    pub fn tmp_iso_folder_path(&self) -> std::path::PathBuf {
        Self::expand_tilde(&self.tmp_folder_name).join(&self.tmp_iso_folder_name)
    }

    /// Returns the expanded path to the JSON file containing the game list.
    pub fn list_json_path(&self) -> std::path::PathBuf {
        Self::expand_tilde(&self.tmp_folder_name).join(&self.list_files_json_name)
    }
}

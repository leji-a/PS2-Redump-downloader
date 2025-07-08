use anyhow::Result;
use configparser::ini::Ini;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub iso_url: String,
    pub list_files_json_name: String,
    pub external_iso_download: bool,
    pub max_retries: u32,
    pub delay_between_retries: u64,
    pub timeout_request: Option<u64>,
    pub tmp_folder_name: String,
    pub tmp_iso_folder_name: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let mut config = Ini::new();
        config.load(path).map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;

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

        Ok(config)
    }

    pub fn tmp_folder_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from(&self.tmp_folder_name)
    }

    pub fn tmp_iso_folder_path(&self) -> std::path::PathBuf {
        self.tmp_folder_path().join(&self.tmp_iso_folder_name)
    }

    pub fn list_json_path(&self) -> std::path::PathBuf {
        self.tmp_folder_path().join(&self.list_files_json_name)
    }
}

use crate::config::Config;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn setup_folders(config: &Config) -> Result<()> {
    check_folder(&config.tmp_folder_path(), &config.tmp_folder_name)?;
    check_folder(&config.tmp_iso_folder_path(), &config.tmp_iso_folder_name)?;
    check_folder(&config.tmp_key_folder_path(), &config.tmp_key_folder_name)?;
    Ok(())
}

fn check_folder(folder_path: &Path, folder_name: &str) -> Result<()> {
    if !folder_path.exists() {
        create_folder(folder_path, folder_name)?;
    } else if !folder_path.is_dir() {
        anyhow::bail!("Please remove the file named as {}", folder_name);
    }
    Ok(())
}

fn create_folder(folder_path: &Path, folder_name: &str) -> Result<()> {
    match fs::create_dir_all(folder_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error creating '{}' folder: {}", folder_name, e);
            std::process::exit(-1);
        }
    }
}

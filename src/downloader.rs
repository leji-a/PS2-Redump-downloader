use crate::{config::Config, models::Game};
use anyhow::Result;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

pub struct Downloader {
    config: Config,
}

impl Downloader {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn download_ps2_element(&self, game: &Game) -> Result<()> {
        let title = game.clean_title();
        println!("\nSelected {}\n", title);

        // Construct the full URL by combining base URL with relative path
        let full_url = format!("{}{}", self.config.iso_url, game.link);
        self.download_and_unzip(&full_url, &title).await?;
        println!("\n{} downloaded :)", title);

        // Open the folder containing the downloaded ISO
        let iso_file = self
            .config
            .tmp_iso_folder_path()
            .join(format!("{}.iso", title));
        if iso_file.exists() {
            self.open_explorer(&iso_file);
        }

        Ok(())
    }

    async fn download_and_unzip(&self, link: &str, title: &str) -> Result<()> {
        println!(" # ISO file...");

        let unzipped_file_name = format!("{}.iso", title);
        let unzipped_file_path = self.config.tmp_iso_folder_path().join(&unzipped_file_name);

        if unzipped_file_path.exists() {
            println!(" - File previously downloaded :)\n");
            return Ok(());
        }

        let new_file_name = format!("{}.zip", title);
        let tmp_file = self.config.tmp_iso_folder_path().join(&new_file_name);

        if self.config.external_iso_download {
            self.download_using_navigator(link, &new_file_name, &tmp_file, &unzipped_file_name)
                .await?;
        } else {
            self.download_using_request(link, &tmp_file).await?;
        }

        if tmp_file.exists() {
            self.unzip_file(&tmp_file).await?;
            self.remove_file(&tmp_file)?;
        }

        println!(" ");
        Ok(())
    }

    async fn download_using_request(&self, link: &str, file_path: &Path) -> Result<()> {
        let total_size = self.get_file_size(link).await?;
        let mut retries = 0;

        while retries < self.config.max_retries {
            let mut headers = reqwest::header::HeaderMap::new();
            let mut first_byte = 0;

            if let Some(size) = total_size {
                if file_path.exists() {
                    first_byte = fs::metadata(file_path)?.len();
                    if first_byte >= size {
                        println!("The file {} is downloaded previously.", file_path.display());
                        return Ok(());
                    }
                }
                headers.insert("Range", format!("bytes={}-{}", first_byte, size).parse()?);
            }

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(
                    self.config.timeout_request.unwrap_or(600), // Much longer timeout for large files
                ))
                .connect_timeout(std::time::Duration::from_secs(30)) // Add connection timeout
                .build()?;

            println!("Attempting download from: {}", link);
            
            match client.get(link).headers(headers).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let mut file = File::create(file_path).await?;
                        let mut stream = response.bytes_stream();

                        let progress_bar = if let Some(total) = total_size {
                            let pb = ProgressBar::new(total);
                            pb.set_style(
                                ProgressStyle::default_bar()
                                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                                    .unwrap()
                                    .progress_chars("#>-"),
                            );
                            pb.set_position(first_byte);
                            Some(pb)
                        } else {
                            None
                        };

                        let mut download_completed = false;
                        while let Some(chunk_result) = stream.next().await {
                            match chunk_result {
                                Ok(chunk) => {
                                    file.write_all(&chunk).await?;
                                    if let Some(pb) = &progress_bar {
                                        pb.inc(chunk.len() as u64);
                                    }
                                }
                                Err(e) => {
                                    println!("Error during download: {}", e);
                                    // Don't break, let it retry
                                    break;
                                }
                            }
                        }

                        // Check if download was completed successfully
                        if let Some(pb) = &progress_bar {
                            if let Some(length) = pb.length() {
                                if pb.position() >= length {
                                    pb.finish_with_message("Download completed");
                                    download_completed = true;
                                } else {
                                    pb.finish_with_message("Download incomplete");
                                    download_completed = false;
                                }
                            } else {
                                // If we can't determine length, assume completed
                                pb.finish_with_message("Download completed");
                                download_completed = true;
                            }
                        } else {
                            download_completed = true; // Assume completed if no progress bar
                        }

                        if download_completed {
                            break;
                        } else {
                            println!("Download incomplete, will retry...");
                            // Remove the corrupted file before retrying
                            if file_path.exists() {
                                if let Err(e) = fs::remove_file(file_path) {
                                    println!("Warning: Could not remove corrupted file: {}", e);
                                }
                            }
                            retries += 1;
                            continue;
                        }
                    } else {
                        println!("HTTP error: {} - {}", response.status(), response.status().as_str());
                        retries += 1;
                    }
                }
                Err(e) => {
                    println!("Request error (attempt {}/{}): {}", retries + 1, self.config.max_retries, e);
                    retries += 1;
                }
            }

            if retries < self.config.max_retries {
                println!("Waiting {} seconds before retry...", self.config.delay_between_retries);
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    self.config.delay_between_retries,
                ))
                .await;
            }
        }

        if retries == self.config.max_retries {
            anyhow::bail!(
                "Failed to download file after {} attempts.",
                self.config.max_retries
            );
        }

        Ok(())
    }

    async fn download_using_navigator(
        &self,
        route: &str,
        downloaded_file_name: &str,
        zip_file: &Path,
        unzipped_file: &str,
    ) -> Result<()> {
        let destination_folder = self.config.tmp_iso_folder_path();

        println!("Opening browser with download link ({})", route);
        open::that(route)?;

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!(
            "Please download the file and copy '{}' to '{}'",
            downloaded_file_name,
            destination_folder.display()
        );
        self.open_explorer(&destination_folder);

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!("Waiting for the file to be copied...");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        while !zip_file.exists() && !destination_folder.join(unzipped_file).exists() {
            println!(
                "\nFile not found!! Make sure to download and copy the file to '{}'",
                destination_folder.display()
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }

        println!();
        Ok(())
    }

    async fn get_file_size(&self, link: &str) -> Result<Option<u64>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()?;
            
        let response = client.get(link).header("Range", "bytes=0-1").send().await?;

        if let Some(range_header) = response.headers().get("content-range") {
            if let Ok(range_str) = range_header.to_str() {
                if let Some(total_str) = range_str.split('/').nth(1) {
                    if let Ok(total_size) = total_str.parse::<u64>() {
                        return Ok(Some(total_size));
                    }
                }
            }
        }

        // Try to get content-length as fallback
        if let Some(content_length) = response.headers().get("content-length") {
            if let Ok(length_str) = content_length.to_str() {
                if let Ok(total_size) = length_str.parse::<u64>() {
                    return Ok(Some(total_size));
                }
            }
        }

        Ok(None)
    }

    async fn unzip_file(&self, zip_path: &Path) -> Result<()> {
        println!("Extracting ZIP file...");
        let dest = zip_path.parent().unwrap();
        
        // Check file size first
        let file_size = fs::metadata(zip_path)?.len();
        if file_size == 0 {
            anyhow::bail!("ZIP file is empty (0 bytes)");
        }
        
        let file = fs::File::open(zip_path)?;
        let mut archive = match ZipArchive::new(file) {
            Ok(archive) => archive,
            Err(e) => {
                anyhow::bail!("Invalid ZIP archive: {}. The file may be corrupted or incomplete. Try downloading again.", e);
            }
        };

        // Calculate total size first
        let total_size: u64 = {
            let file_names: Vec<String> = archive.file_names().map(|s| s.to_string()).collect();
            file_names
                .iter()
                .filter_map(|name| {
                    archive
                        .by_name(name)
                        .ok()
                        .and_then(|file| file.size().checked_add(0))
                })
                .sum()
        };

        if total_size > 0 {
            let progress_bar = ProgressBar::new(total_size);
            progress_bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} Extracting: [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = dest.join(file.name());

                if file.name().ends_with('/') {
                    fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(p)?;
                        }
                    }
                    let mut outfile = fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                    progress_bar.inc(file.size());
                }
            }

            progress_bar.finish_with_message("Extraction completed");
        } else {
            // Fallback spinner for small files or when size calculation fails
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} Extracting files... {elapsed_precise}")
                    .unwrap()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
            );

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = dest.join(file.name());

                if file.name().ends_with('/') {
                    fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(p)?;
                        }
                    }
                    let mut outfile = fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
                spinner.tick();
            }

            spinner.finish_with_message("Extraction completed");
        }

        Ok(())
    }

    fn remove_file(&self, file_path: &Path) -> Result<()> {
        match fs::remove_file(file_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error removing {}: {}", file_path.display(), e);
                Ok(())
            }
        }
    }

    fn open_explorer(&self, path: &Path) {
        if let Err(e) = open::that(path) {
            println!("Error opening {}: {}", path.display(), e);
        }
    }
}

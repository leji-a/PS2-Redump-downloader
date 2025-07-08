use crate::{config::Config, models::Game};
use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json;
use std::fs;

pub struct Scraper {
    config: Config,
    client: Client,
}

impl Scraper {
    pub fn new(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap_or_default();

        Self {
            config: config.clone(),
            client,
        }
    }

    pub async fn get_ps2_list(&self) -> Result<Vec<Game>> {
        // Check if JSON file exists
        let json_path = self.config.list_json_path();
        if json_path.exists() {
            println!("{} exists...", self.config.list_files_json_name);
            let content = fs::read_to_string(&json_path)?;
            let games: Vec<Game> = serde_json::from_str(&content)?;
            println!(
                "{} has {} titles",
                self.config.list_files_json_name,
                games.len()
            );
            return Ok(games);
        }

        // Download and parse PS2 list
        println!("Downloading PS2 list...");
        let response = self.client.get(&self.config.iso_url).send().await?;
        let html = response.text().await?;

        println!("Converting data...");
        let document = Html::parse_document(&html);

        let table_selector = Selector::parse("table#list tbody tr").unwrap();
        let link_selector = Selector::parse("td.link a").unwrap();
        let size_selector = Selector::parse("td.size").unwrap();

        let mut games = Vec::new();

        for row in document.select(&table_selector).skip(1) {
            if let (Some(link_elem), Some(size_elem)) = (
                row.select(&link_selector).next(),
                row.select(&size_selector).next(),
            ) {
                let title = link_elem.text().collect::<String>().trim().to_string();
                let link = link_elem.value().attr("href").unwrap_or("").to_string();
                let size = size_elem.text().collect::<String>().trim().to_string();

                games.push(Game { title, link, size });
            }
        }

        println!("Downloaded {} titles", games.len());

        // Save to JSON file
        let json_content = serde_json::to_string_pretty(&games)?;
        fs::write(&json_path, json_content)?;
        println!("Saved in {}", json_path.display());

        Ok(games)
    }
}

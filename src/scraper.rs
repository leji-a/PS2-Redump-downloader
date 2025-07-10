use crate::{config::Config, models::Game};
use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json;
use std::fs;

/// Scraper handles downloading and parsing the PS2 games list from the configured source.
pub struct Scraper {
    config: Config,
    client: Client,
}

impl Scraper {
    /// Create a new Scraper with the given configuration.
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

    /// Gets the PS2 games list, using a cached JSON file if available, otherwise scrapes and saves it.
    pub async fn get_ps2_list(&self) -> Result<Vec<Game>> {
        // Check if JSON file exists
        let json_path = self.config.list_json_path();
        if json_path.exists() {
            println!("{} exists...", self.config.list_files_json_name);
            let content = fs::read_to_string(&json_path)?;
            let mut games: Vec<Game> = serde_json::from_str(&content)?;
            // Initialize lowercased_title for each game
            for game in &mut games {
                game.lowercased_title = game.title.to_lowercase();
            }
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

        // Selectors for parsing the table rows, links, and sizes
        let table_selector = Selector::parse("table#list tbody tr").unwrap();
        let link_selector = Selector::parse("td.link a").unwrap();
        let size_selector = Selector::parse("td.size").unwrap();

        let mut games = Vec::new();

        // Parse each row in the table (skipping header if present)
        for row in document.select(&table_selector).skip(1) {
            if let (Some(link_elem), Some(size_elem)) = (
                row.select(&link_selector).next(),
                row.select(&size_selector).next(),
            ) {
                let title = link_elem.text().collect::<String>().trim().to_string();
                let link = link_elem.value().attr("href").unwrap_or("").to_string();
                let size = size_elem.text().collect::<String>().trim().to_string();

                // Initialize lowercased_title for fast search
                let game = Game { title, link, size, lowercased_title: String::new() }.with_lowercased();
                games.push(game);
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

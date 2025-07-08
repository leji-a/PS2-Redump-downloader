use anyhow::Result;
use ps2_redump_downloader::{
    config::Config, downloader::Downloader, models::Game, scraper::Scraper, utils::setup_folders,
};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::load("config.ini")?;

    // Setup working folders
    setup_folders(&config)?;

    // Initialize scraper and downloader
    let scraper = Scraper::new(&config);
    let downloader = Downloader::new(&config);

    // Get PS2 game list
    let games = scraper.get_ps2_list().await?;

    // Main application loop
    run_main_loop(&downloader, games).await?;

    Ok(())
}

async fn run_main_loop(
    downloader: &Downloader,
    games: Vec<Game>,
) -> Result<()> {
    loop {
        print!("Find PS2 title to download (leave empty to exit): ");
        std::io::stdout().flush()?;
        let mut search_input = String::new();
        std::io::stdin().read_line(&mut search_input)?;
        let search_input = search_input.trim();

        if search_input.is_empty() {
            println!("Exiting...");
            break Ok(());
        }

        let filtered_games = filter_games(&games, search_input);

        if filtered_games.is_empty() {
            println!("No elements found\n");
            continue;
        }

        print_games(&filtered_games);

        print!("Enter PS2 title number [1-{}]: ", filtered_games.len());
        std::io::stdout().flush()?;
        let mut file_number_input = String::new();
        std::io::stdin().read_line(&mut file_number_input)?;

        if let Ok(file_number) = file_number_input.trim().parse::<usize>() {
            if file_number > 0 && file_number <= filtered_games.len() {
                let selected_game = &filtered_games[file_number - 1];
                downloader.download_ps2_element(selected_game).await?;
            } else {
                println!("Number not in valid range (1-{})\n", filtered_games.len());
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
}

fn filter_games<'a>(games: &'a [Game], search: &str) -> Vec<&'a Game> {
    let search_lower = search.to_lowercase();
    let searches: Vec<&str> = search_lower.split_whitespace().collect();

    games
        .iter()
        .filter(|game| {
            let title_lower = game.title.to_lowercase();
            searches.iter().all(|search| title_lower.contains(search))
        })
        .collect()
}

fn print_games(games: &[&Game]) {
    for (index, game) in games.iter().enumerate() {
        println!("{}. {} ({})", index + 1, game.title, game.size);
    }
    println!();
}


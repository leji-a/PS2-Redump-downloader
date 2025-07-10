use serde::{Deserialize, Serialize};

/// Represents a PS2 game entry with title, download link, size, and a lowercased title for fast search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// The display title of the game (may include .zip extension)
    pub title: String,
    /// The relative download link for the game
    pub link: String,
    /// The size of the game as a string (e.g., '2.1 GB')
    pub size: String,
    /// Lowercased version of the title for fast case-insensitive search
    #[serde(skip)]
    pub lowercased_title: String,
}

impl Game {
    /// Returns the cleaned title (removes .zip extension)
    pub fn clean_title(&self) -> String {
        self.title.replace(".zip", "")
    }

    /// Creates a new Game with lowercased_title initialized
    pub fn with_lowercased(mut self) -> Self {
        self.lowercased_title = self.title.to_lowercase();
        self
    }
}

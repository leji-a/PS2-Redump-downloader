use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub link: String,
    pub size: String,
}

impl Game {
    pub fn clean_title(&self) -> String {
        self.title.replace(".zip", "")
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScore {
    pub score: u32,
    pub length: u32,
    pub timestamp: DateTime<Utc>,
}

fn get_scores_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs_next::config_dir()
        .ok_or("Could not find config directory")?
        .join("tsnake");

    fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("highscores.json"))
}

pub fn load_scores() -> Result<Vec<HighScore>, Box<dyn std::error::Error>> {
    let path = get_scores_file_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path)?;
    let scores: Vec<HighScore> = serde_json::from_str(&contents)?;

    Ok(scores)
}

pub fn save_score(score: u32, length: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut scores = load_scores().unwrap_or_default();

    let new_score = HighScore {
        score,
        length,
        timestamp: Utc::now(),
    };

    scores.push(new_score);

    scores.sort_by(|a, b| b.score.cmp(&a.score));

    scores.truncate(10);

    let path = get_scores_file_path()?;
    let json = serde_json::to_string_pretty(&scores)?;
    fs::write(path, json)?;

    Ok(())
}

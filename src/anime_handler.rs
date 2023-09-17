use anyhow::{Error, Result};
use sqlx::SqlitePool;

use crate::{fetch::Fetchable, models::anime::Anime, scrape::Scrapable};

pub struct AnimeHandler<T: Fetchable + Scrapable> {
    pub db_pool: SqlitePool,
    pub scraper: T,
}

impl<T: Fetchable + Scrapable> AnimeHandler<T> {
    pub fn new(db_pool: SqlitePool, scraper: T) -> Self {
        AnimeHandler { db_pool, scraper }
    }

    pub async fn search_animes(&self, query: &str) -> Vec<Anime> {
        Vec::new()
    }

    pub async fn get_anime(&self, anime_id: &str) -> Result<Anime> {
        Err(Error::msg(""))
    }

    pub async fn get_media(&self, anime_id: &str, media_id: &str) -> Result<Anime> {
        Err(Error::msg(""))
    }
}

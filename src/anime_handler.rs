use anyhow::{Error, Result};
use sqlx::SqlitePool;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::models::anime::{AnimeType, PartialAnime};
use crate::scrape::ScrapedSearch;
use crate::{fetch::Fetchable, models::anime::Anime, scrape::Scrapable};

pub struct AnimeHandler<S: Scrapable + Fetchable> {
    pub db_pool: SqlitePool,
    _marker: PhantomData<*const S>,
}

impl<S: Scrapable + Fetchable> AnimeHandler<S> {
    pub fn new(db_pool: SqlitePool) -> Self {
        AnimeHandler {
            db_pool,
            _marker: PhantomData,
        }
    }

    pub async fn search_anime(&self, query: &str) -> Result<Vec<PartialAnime>> {
        let data = S::fetch_search(query, 1).await?;
        let result = S::scrape_search(data).await?;

        let animes = result
            .animes
            .into_iter()
            .map(|a| PartialAnime {
                title: Some(a.title),
                poster_url: a.poster_url,
                ..Default::default()
            })
            .collect();

        Ok(animes)
    }

    pub async fn get_anime(&self, anime_id: &str) -> Result<Anime> {
        Err(Error::msg(""))
    }

    pub async fn get_media(&self, anime_id: &str, media_id: &str) -> Result<Anime> {
        Err(Error::msg(""))
    }
}

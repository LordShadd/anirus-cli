use std::{collections::HashMap, fmt::format, str::FromStr};

use anyhow::{Error, Ok, Result};
use async_trait::async_trait;
use reqwest::Url;
use scraper::{Html, Selector};
use unidecode::unidecode;

use crate::{
    fetch::{Fetchable, FetchedData},
    scrape::{
        Scrapable, ScrapedAnime, ScrapedAnimeEpisode, ScrapedAnimeType, ScrapedMedia,
        ScrapedSearch, ScrapedSearchAnime,
    },
    types::SourcePlatform,
};

pub struct AnimeFireScraper;

impl AnimeFireScraper {
    pub fn new() -> Self {
        AnimeFireScraper
    }
}

impl AnimeFireScraper {
    fn parse_key(key: &str) -> Option<String> {
        let parsed_key = unidecode(key)
            .to_lowercase()
            .trim()
            .replace(" ", "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || c == &'_')
            .collect();

        if key.is_empty() {
            return None;
        }

        Some(parsed_key)
    }
}

struct AnimeFireAnimeInfo {
    release_day: String,
    release_year: String,
}

#[async_trait]
impl Fetchable for AnimeFireScraper {
    const SOURCE_PLATFORM: SourcePlatform = SourcePlatform::AnimeFire;

    fn anime_id_from_url(&self, url: Url) -> Option<String> {
        None
    }

    async fn fetch_search(&self, query: &str, page: u32) -> Result<FetchedData> {
        let source_url = Url::from_str("https://animefire.vip")?.join(&format!(
            "pesquisar/{}/{}",
            query.replace(" ", "-"),
            page
        ))?;

        let res = reqwest::get(source_url.as_str()).await?;
        let html = Html::parse_document(&res.text().await?);

        Ok(FetchedData::new(source_url, html))
    }

    async fn fetch_anime(&self, anime_id: &str) -> Result<FetchedData> {
        let source_url = Url::from_str("https://animefire.vip")?
            .join(&format!("animes/{}-todos-os-episodios", anime_id))?;

        let res = reqwest::get(source_url.as_str()).await?;
        let html = Html::parse_document(&res.text().await?);

        Ok(FetchedData::new(source_url, html))
    }

    async fn fetch_media(&self, anime_id: &str, media_id: &str) -> Result<FetchedData> {
        let source_url = Url::from_str("https://animefire.vip")?
            .join(&format!("animes/{}/{}", anime_id, media_id))?;

        let res = reqwest::get(source_url.as_str()).await?;
        let html = Html::parse_document(&res.text().await?);

        Ok(FetchedData::new(source_url, html))
    }
}

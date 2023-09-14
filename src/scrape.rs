use anyhow::Result;
use async_trait::async_trait;

use crate::{fetch::FetchedData, types::SourcePlatform};

#[derive(Debug)]
pub enum ScrapedAnimeType {
    Series,
    Movie,
}

#[derive(Debug)]
pub struct ScrapedSearch {
    pub animes: Vec<ScrapedSearchAnime>,
    pub current_page: u32,
    pub total_pages: u32,
}

#[derive(Debug)]
pub struct ScrapedSearchAnime {
    pub title: String,
    pub page_url: String,
    pub poster_url: Option<String>,
}

#[derive(Debug)]
pub struct ScrapedAnime {
    pub source_id: String,
    pub title: String,
    pub original_title: Option<String>,
    pub release_year: u32,
    pub synopsis: String,
    pub anime_type: ScrapedAnimeType,
    pub episodes: Option<Vec<ScrapedAnimeEpisode>>,
    pub movies: Option<Vec<ScrapedAnimeMovie>>,
}

#[derive(Debug)]
pub struct ScrapedAnimeEpisode {
    pub index: u32,
    pub label: String,
    pub page_url: String,
}

#[derive(Debug)]
pub struct ScrapedAnimeMovie {
    pub index: usize,
    pub label: String,
    pub page_url: String,
}

#[derive(Debug)]
pub struct ScrapedMedia {
    pub source_url: String,
}

#[async_trait]
pub trait Scrapable {
    const SOURCE_PLATFORM: SourcePlatform;

    async fn scrape_search(&self, data: FetchedData) -> Result<ScrapedSearch>;
    async fn scrape_anime(&self, data: FetchedData) -> Result<ScrapedAnime>;
    async fn scrape_media(&self, data: FetchedData) -> Result<ScrapedMedia>;
}

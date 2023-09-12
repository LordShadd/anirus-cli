use anyhow::Result;
use async_trait::async_trait;

use crate::{fetch::FetchedData, types::SourcePlatform};

pub enum ScrapedAnimeType {
    Series,
    Movie,
}

pub struct ScrapedSearch {
    pub animes: Vec<ScrapedAnimeSearch>,
    pub current_page: u32,
    pub total_pages: u32,
}

pub struct ScrapedAnimeSearch {
    pub title: String,
    pub poster_url: String,
}

pub struct ScrapedAnime {
    pub source_id: String,
    pub title: String,
    pub original_title: String,
    pub release_year: String,
    pub synopsis: String,
    pub anime_type: ScrapedAnimeType,
    pub episodes: Vec<ScrapedAnimeEpisode>,
}

pub struct ScrapedAnimeEpisode {
    pub index: usize,
    pub label: String,
    pub medias: Vec<ScrapedAnimeMedia>,
    pub page_url: String,
}

pub struct ScrapedAnimeMovie {
    pub index: usize,
    pub label: String,
    pub medias: Vec<ScrapedAnimeMedia>,
    pub page_url: String,
}

pub struct ScrapedAnimeMedia {
    pub id: String,
    pub url: String,
}

pub struct ScrapedMedia {
    pub label: String,
    pub media_url: String,
}

#[async_trait]
pub trait Scrapable {
    const SOURCE_PLATFORM: SourcePlatform;

    async fn scrape_search(data: &FetchedData) -> Result<ScrapedSearch>;
    async fn scrape_anime(data: &FetchedData) -> Result<ScrapedAnime>;
    async fn scrape_media(data: &FetchedData) -> Result<ScrapedMedia>;
}

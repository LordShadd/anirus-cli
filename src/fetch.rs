use crate::types::SourcePlatform;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;
use scraper::Html;

pub struct FetchedData {
    pub source_url: Url,
    pub raw: String,
    pub html: Html,
}

impl FetchedData {
    pub fn new(source_url: Url, raw: &str, html: Html) -> Self {
        FetchedData {
            source_url,
            raw: raw.into(),
            html,
        }
    }
}

#[async_trait]
pub trait Fetchable {
    const SOURCE_PLATFORM: SourcePlatform;

    fn anime_id_from_url(&self, url: Url) -> Option<String>;

    async fn fetch_search(&self, query: &str, page: u32) -> Result<FetchedData>;
    async fn fetch_anime(&self, anime_id: &str) -> Result<FetchedData>;
    async fn fetch_media(&self, anime_id: &str, media_id: &str) -> Result<FetchedData>;
}

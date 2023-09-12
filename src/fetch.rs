use crate::types::SourcePlatform;
use anyhow::Result;
use async_trait::async_trait;
use scraper::Html;

pub struct FetchedData {
    pub raw: String,
    pub html: Html,
}

impl FetchedData {
    pub fn new(raw: &str, html: &str) -> Self {
        FetchedData {
            raw: raw.into(),
            html: html.into(),
        }
    }
}

#[async_trait]
pub trait Fetchable {
    const SOURCE_PLATFORM: SourcePlatform;

    async fn fetch_search(&self, query: &str) -> Result<FetchedData>;
    async fn fetch_anime(&self, anime_id: &str) -> Result<FetchedData>;
    async fn fetch_media(&self, media_id: &str) -> Result<FetchedData>;
}

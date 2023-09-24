use crate::types::SourcePlatform;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;
use scraper::Html;

#[derive(Debug)]
pub struct FetchedData {
    pub source_url: Url,
    pub html: Html,
}

impl FetchedData {
    pub fn new(source_url: Url, html: Html) -> Self {
        FetchedData { source_url, html }
    }
}

unsafe impl Send for FetchedData {}

#[async_trait]
pub trait Fetchable {
    const SOURCE_PLATFORM: SourcePlatform;

    fn anime_id_from_url(url: Url) -> Option<String>;

    async fn fetch_search(query: &str, page: u32) -> Result<FetchedData>;
    async fn fetch_anime(anime_id: &str) -> Result<FetchedData>;
    async fn fetch_media(anime_id: &str, media_id: &str) -> Result<FetchedData>;
}

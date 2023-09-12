use crate::types::SourcePlatform;
use anyhow::Result;
use async_trait::async_trait;

pub struct FetchedData {
    pub raw: String,
    pub html: String,
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

    fn fetch_search(&self, query: &str) -> Result<FetchedData>;
    fn fetch_anime(&self, anime_id: &str) -> Result<FetchedData>;
    fn fetch_media(&self, media_id: &str) -> Result<FetchedData>;
}

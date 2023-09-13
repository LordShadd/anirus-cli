use std::{fmt::format, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Url;
use scraper::Html;

use crate::{
    fetch::{Fetchable, FetchedData},
    types::SourcePlatform,
};

pub struct AnimeFire;

#[async_trait]
impl Fetchable for AnimeFire {
    const SOURCE_PLATFORM: SourcePlatform = SourcePlatform::AnimeFire;

    fn anime_id_from_url(url: Url) -> Option<String> {
        None
    }

    async fn fetch_search(query: &str, page: u32) -> Result<FetchedData> {
        let source_url = Url::from_str("https://animefire.vip")?.join(&format!(
            "pesquisar/{}/{}",
            query.replace(" ", "-"),
            page
        ))?;

        let res = reqwest::get(source_url.as_str()).await?;
        let html = Html::parse_document(&res.text().await?);

        Ok(FetchedData::new(source_url, html))
    }

    async fn fetch_anime(anime_id: &str) -> Result<FetchedData> {
        let source_url = Url::from_str("https://animefire.vip")?
            .join(&format!("animes/{}-todos-os-episodios", anime_id))?;

        let res = reqwest::get(source_url.as_str()).await?;
        let html = Html::parse_document(&res.text().await?);

        Ok(FetchedData::new(source_url, html))
    }

    async fn fetch_media(anime_id: &str, media_id: &str) -> Result<FetchedData> {
        let source_url = Url::from_str("https://animefire.vip")?
            .join(&format!("animes/{}/{}", anime_id, media_id))?;

        let res = reqwest::get(source_url.as_str()).await?;
        let html = Html::parse_document(&res.text().await?);

        Ok(FetchedData::new(source_url, html))
    }
}


        Ok(FetchedData::new(source_url, &raw, html))
    }
}


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

#[async_trait]
impl Scrapable for AnimeFireScraper {
    const SOURCE_PLATFORM: SourcePlatform = SourcePlatform::AnimeFire;

    async fn scrape_search(&self, data: FetchedData) -> Result<ScrapedSearch> {
        let document = data.html;

        let anime_selector = Selector::parse("div>article>a").unwrap();
        let name_selector = Selector::parse(".animeTitle").unwrap();
        let thumb_url_selector = Selector::parse(".imgAnimes").unwrap();
        let pagination_selector = Selector::parse("ul.pagination").unwrap();

        let anime_elements = document.select(&anime_selector);

        let animes = anime_elements
            .map(|element| ScrapedSearchAnime {
                page_url: element.value().attr("href").unwrap().into(),
                title: element.select(&name_selector).next().unwrap().inner_html(),
                poster_url: element
                    .select(&thumb_url_selector)
                    .next()
                    .map(|url| url.value().attr("data-src"))
                    .flatten()
                    .map(|url| url.to_string()),
            })
            .collect::<Vec<ScrapedSearchAnime>>();

        Ok(ScrapedSearch {
            animes,
            current_page: 1,
            total_pages: 1,
        })
    }

    async fn scrape_anime(&self, data: FetchedData) -> Result<ScrapedAnime> {
        let document = data.html;

        let title_selector = Selector::parse("div.div_anime_names>h1").unwrap();
        let synopsis_selector = Selector::parse("div.divSinopse>span.spanAnimeInfo").unwrap();
        let episodes_selector = Selector::parse("div.div_video_list>a").unwrap();
        let info_selector = Selector::parse("div.animeInfo:not(a)").unwrap();
        let info_key_selector = Selector::parse("b").unwrap();
        let info_value_selector = Selector::parse("span").unwrap();

        let title = document
            .select(&title_selector)
            .next()
            .unwrap()
            .inner_html();

        let info = document
            .select(&info_selector)
            .filter_map(|e| {
                let key = e.select(&info_key_selector).next()?.text().next()?;
                let value = e.select(&info_value_selector).next()?.text().next()?.trim();

                AnimeFireScraper::parse_key(key).map(|key| (key, value.into()))
            })
            .collect::<HashMap<String, String>>();

        let synopsis = document
            .select(&synopsis_selector)
            .next()
            .unwrap()
            .inner_html();

        let episodes = document
            .select(&episodes_selector)
            .enumerate()
            .map(|(index, element)| ScrapedAnimeEpisode {
                index: index as u32 + 1,
                label: element.inner_html(),
                page_url: element.value().attr("href").unwrap().into(),
            })
            .collect::<Vec<ScrapedAnimeEpisode>>();

        println!("{info:#?}");

        Ok(ScrapedAnime {
            source_id: "1243".into(),
            title,
            original_title: None,
            synopsis,
            anime_type: ScrapedAnimeType::Series,
            episodes: Some(episodes),
            movies: None,
            release_year: info.get("ano").unwrap().to_owned().parse::<u32>().unwrap(),
        })
    }

    async fn scrape_media(&self, data: FetchedData) -> Result<ScrapedMedia> {
        let document = data.html;

        let video_selector = Selector::parse("video").unwrap();
        let iframe = Selector::parse("iframe").unwrap();

        if let Some(video_element) = document.select(&video_selector).next() {
            return Ok(ScrapedMedia {
                source_url: video_element.value().attr("data-video-src").unwrap().into(),
            });
        };

        if let Some(iframe_element) = document.select(&iframe).next() {
            return Ok(ScrapedMedia {
                source_url: iframe_element.value().attr("src").unwrap().into(),
            });
        };

        Err(Error::msg("Media is not present."))
    }
}

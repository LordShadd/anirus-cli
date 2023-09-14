use crate::{fetch::Fetchable, scrape::Scrapable};

pub struct AnimeHandler<T: Fetchable + Scrapable> {
    pub scraper: T,
}

impl<T: Fetchable + Scrapable> AnimeHandler<T> {
    pub async fn search_animes(&self) {}

    pub async fn get_anime(&self) {}

    pub async fn get_media(&self) {}
}

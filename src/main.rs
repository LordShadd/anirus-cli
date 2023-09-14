use std::time::Instant;

use anyhow::Result;
use fetch::Fetchable;
use scrapers::anime_fire::AnimeFireScraper;
use scrape::Scrapable;

mod types;
mod fetch;
mod anime_handler;
mod models;
mod scrapers;
mod scrape;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

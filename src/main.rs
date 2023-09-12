use anyhow::Result;

mod types;
mod fetch;
mod scrape;
mod platforms;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

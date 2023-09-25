use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Encode, Pool, Sqlite, SqlitePool, Transaction, Type};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Clone, Copy, Deserialize, Type)]
pub enum AnimeType {
    #[default]
    Series,
    Movie,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Anime {
    pub id: Option<i64>,
    pub source_id: String,
    pub tmdb_id: Option<i64>,
    pub title: String,
    pub release_date: String,
    pub original_title: Option<String>,
    pub synopsis: String,
    pub poster_url: String,
    pub background_url: Option<String>,
    pub anime_type: AnimeType,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PartialAnime {
    pub source_id: Option<String>,
    pub tmdb_id: Option<i64>,
    pub title: Option<String>,
    pub release_date: Option<String>,
    pub original_title: Option<String>,
    pub synopsis: Option<String>,
    pub poster_url: Option<String>,
    pub background_url: Option<String>,
    pub anime_type: Option<AnimeType>,
}

#[derive(Debug, Serialize, Deserialize, Type, Encode)]
struct CreateAnimeData<'a> {
    source_id: &'a str,
    tmdb_id: Option<i64>,
    title: &'a str,
    release_date: &'a str,
    original_title: Option<&'a str>,
    synopsis: &'a str,
    poster_url: &'a str,
    background_url: Option<&'a str>,
    anime_type: AnimeType,
}

impl Anime {
    async fn create<'a>(data: CreateAnimeData<'a>, pool: &SqlitePool) -> Result<Anime> {
        let id = sqlx::query(
            r#"
INSERT INTO `animes` (
    source_id,
    tmdb_id,
    title,
    release_date,
    original_title,
    synopsis,
    poster_url,
    background_url,
    anime_type
)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(data.source_id)
        .bind(data.tmdb_id)
        .bind(data.title)
        .bind(data.release_date)
        .bind(data.original_title)
        .bind(data.synopsis)
        .bind(data.poster_url)
        .bind(data.background_url)
        .bind(data.anime_type)
        .execute(pool)
        .await?
        .last_insert_rowid();

        let anime = Anime {
            id: Some(id),
            source_id: data.source_id.into(),
            tmdb_id: data.tmdb_id,
            title: data.title.into(),
            release_date: data.release_date.into(),
            original_title: data.original_title.map(|s| s.into()),
            synopsis: data.synopsis.into(),
            poster_url: data.poster_url.into(),
            background_url: data.background_url.map(|s| s.into()),
            anime_type: data.anime_type,
        };

        Ok(anime)
    }
}

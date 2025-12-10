use serde::Deserialize;

use super::client::TidalClient;
use super::models::{
    Genre,
    ItemsPage,
    Mood,
    Playlist,
    Track,
    Video,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_genres(&mut self) -> Result<Vec<Genre>> {
        let url = self.api_url("genres", &[]);
        #[derive(Deserialize)]
        struct GenresResponse {
            items: Vec<Genre>,
        }
        let resp: GenresResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_genre_tracks(
        &mut self,
        genre: &str,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<Track>> {
        let url = self.api_url(
            &format!("genres/{}/tracks", genre),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_moods(&mut self) -> Result<Vec<Mood>> {
        let url = self.api_url("moods", &[]);
        #[derive(Deserialize)]
        struct MoodsResponse {
            items: Vec<Mood>,
        }
        let resp: MoodsResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_mood_playlists(
        &mut self,
        mood: &str,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<Playlist>> {
        let url = self.api_url(
            &format!("moods/{}/playlists", mood),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_video(&mut self, video_id: u64) -> Result<Video> {
        let url = self.api_url(&format!("videos/{}", video_id), &[]);
        self.get(&url).await
    }
}
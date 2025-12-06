use serde::Deserialize;

use super::client::TidalClient;
use super::models::{
    Credit,
    ItemsPage,
    Lyrics,
    Mix,
    MixItem,
    Track,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_track(&self, track_id: u64) -> Result<Track> {
        let url = self.api_url(&format!("tracks/{}", track_id), &[]);
        self.get(&url).await
    }

    pub async fn get_tracks(&self, track_ids: &[u64]) -> Result<Vec<Track>> {
        if track_ids.is_empty() {
            return Ok(vec![]);
        }
        let ids = track_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = self.api_url("tracks", &[("ids", &ids)]);

        #[derive(Deserialize)]
        struct TracksResponse {
            items: Vec<Track>,
        }
        let resp: TracksResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_track_credits(&self, track_id: u64) -> Result<Vec<Credit>> {
        let url = self.api_url(&format!("tracks/{}/credits", track_id), &[]);
        #[derive(Deserialize)]
        struct CreditsResponse {
            credits: Vec<Credit>,
        }
        let resp: CreditsResponse = self.get(&url).await?;
        Ok(resp.credits)
    }

    pub async fn get_track_mix(&self, track_id: u64) -> Result<Mix> {
        let url = self.api_url(&format!("tracks/{}/mix", track_id), &[]);
        self.get(&url).await
    }

    pub async fn get_lyrics(&self, track_id: u64) -> Result<Lyrics> {
        let url = self.api_url(&format!("tracks/{}/lyrics", track_id), &[]);
        self.get(&url).await
    }

    pub async fn get_mix_tracks(&self, mix_id: &str, limit: u32) -> Result<ItemsPage<MixItem>> {
        let url = self.api_url(
            &format!("mixes/{}/items", mix_id),
            &[("limit", &limit.to_string())],
        );
        self.get(&url).await
    }
}

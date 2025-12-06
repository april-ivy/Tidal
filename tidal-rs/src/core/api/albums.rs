use serde::Deserialize;

use super::client::TidalClient;
use super::models::{
    Album,
    AlbumReview,
    Credit,
    ItemsPage,
    Track,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_album(&self, album_id: u64) -> Result<Album> {
        let url = self.api_url(&format!("albums/{}", album_id), &[]);
        self.get(&url).await
    }

    pub async fn get_albums(&self, album_ids: &[u64]) -> Result<Vec<Album>> {
        if album_ids.is_empty() {
            return Ok(vec![]);
        }
        let ids = album_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = self.api_url("albums", &[("ids", &ids)]);

        #[derive(Deserialize)]
        struct AlbumsResponse {
            items: Vec<Album>,
        }
        let resp: AlbumsResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_album_tracks(
        &self,
        album_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<Track>> {
        let url = self.api_url(
            &format!("albums/{}/tracks", album_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_album_credits(&self, album_id: u64) -> Result<Vec<Credit>> {
        let url = self.api_url(&format!("albums/{}/credits", album_id), &[]);
        #[derive(Deserialize)]
        struct CreditsResponse {
            credits: Vec<Credit>,
        }
        let resp: CreditsResponse = self.get(&url).await?;
        Ok(resp.credits)
    }

    pub async fn get_album_review(&self, album_id: u64) -> Result<AlbumReview> {
        let url = self.api_url(&format!("albums/{}/review", album_id), &[]);
        self.get(&url).await
    }

    pub async fn get_similar_albums(&self, album_id: u64, limit: u32) -> Result<ItemsPage<Album>> {
        let url = self.api_url(
            &format!("albums/{}/similar", album_id),
            &[("limit", &limit.to_string())],
        );
        self.get(&url).await
    }
}

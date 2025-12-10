use serde::Deserialize;

use super::client::TidalClient;
use super::models::{
    Album,
    AlbumItemsCreditsResponse,
    AlbumPage,
    AlbumReview,
    Credit,
    ItemsPage,
    Track,
    TrackCredits,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_album(&mut self, album_id: u64) -> Result<Album> {
        let url = self.api_url(&format!("albums/{}", album_id), &[]);
        self.get(&url).await
    }

    pub async fn get_albums(&mut self, album_ids: &[u64]) -> Result<Vec<Album>> {
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
        &mut self,
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

    pub async fn get_album_credits(&mut self, album_id: u64) -> Result<Vec<Credit>> {
        let url = self.api_url(&format!("albums/{}/credits", album_id), &[]);
        #[derive(Deserialize)]
        struct CreditsResponse {
            credits: Vec<Credit>,
        }
        let resp: CreditsResponse = self.get(&url).await?;
        Ok(resp.credits)
    }

    pub async fn get_album_items_credits(
        &mut self,
        album_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<AlbumItemsCreditsResponse> {
        let url = self.api_url(
            &format!("albums/{}/items/credits", album_id),
            &[
                ("replace", "true"),
                ("includeContributors", "true"),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_all_album_track_credits(
        &mut self,
        album_id: u64,
    ) -> Result<Vec<TrackCredits>> {
        let mut all_credits = Vec::new();
        let mut offset = 0u32;
        let limit = 100u32;

        loop {
            let response = self
                .get_album_items_credits(album_id, limit, offset)
                .await?;
            all_credits.extend(response.items);

            if all_credits.len() >= response.total_number_of_items as usize {
                break;
            }
            offset += limit;
        }

        Ok(all_credits)
    }

    pub async fn get_album_review(&mut self, album_id: u64) -> Result<AlbumReview> {
        let url = self.api_url(&format!("albums/{}/review", album_id), &[]);
        self.get(&url).await
    }

    pub async fn get_similar_albums(
        &mut self,
        album_id: u64,
        limit: u32,
    ) -> Result<ItemsPage<Album>> {
        let url = self.api_url(
            &format!("albums/{}/similar", album_id),
            &[("limit", &limit.to_string())],
        );
        self.get(&url).await
    }

    pub async fn get_album_page(&mut self, album_id: u64) -> Result<AlbumPage> {
        let url = self.pages_url(&format!("album?albumId={}", album_id), &[]);
        self.get(&url).await
    }

    pub async fn get_album_full_info(&mut self, album_id: u64) -> Result<AlbumFullInfo> {
        let album = self.get_album(album_id).await?;
        let tracks = self.get_album_tracks(album_id, 100, 0).await?;
        let credits = self.get_album_credits(album_id).await.ok();
        let review = self.get_album_review(album_id).await.ok();
        let track_credits = self.get_all_album_track_credits(album_id).await.ok();

        Ok(AlbumFullInfo {
            album,
            tracks: tracks.items,
            credits,
            review,
            track_credits,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AlbumFullInfo {
    pub album: Album,
    pub tracks: Vec<Track>,
    pub credits: Option<Vec<Credit>>,
    pub review: Option<AlbumReview>,
    pub track_credits: Option<Vec<TrackCredits>>,
}

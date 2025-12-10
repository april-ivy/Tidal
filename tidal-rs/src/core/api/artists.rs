#![allow(dead_code)]
use serde::Deserialize;

use super::client::TidalClient;
use super::models::{
    Album,
    Artist,
    ArtistBio,
    ArtistLink,
    ItemsPage,
    Mix,
    Track,
    Video,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_artist(&mut self, artist_id: u64) -> Result<Artist> {
        let url = self.api_url(&format!("artists/{}", artist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_artists(&mut self, artist_ids: &[u64]) -> Result<Vec<Artist>> {
        if artist_ids.is_empty() {
            return Ok(vec![]);
        }
        let ids = artist_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = self.api_url("artists", &[("ids", &ids)]);

        #[derive(Deserialize)]
        struct ArtistsResponse {
            items: Vec<Artist>,
        }
        let resp: ArtistsResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_artist_bio(&mut self, artist_id: u64) -> Result<ArtistBio> {
        let url = self.api_url(&format!("artists/{}/bio", artist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_artist_links(&mut self, artist_id: u64) -> Result<Vec<ArtistLink>> {
        let url = self.api_url(&format!("artists/{}/links", artist_id), &[]);
        #[derive(Deserialize)]
        struct LinksResponse {
            items: Vec<ArtistLink>,
            source: Option<String>,
        }
        let resp: LinksResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_artist_mix(&mut self, artist_id: u64) -> Result<Mix> {
        let url = self.api_url(&format!("artists/{}/mix", artist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_artist_albums(
        &mut self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<Album>> {
        let url = self.api_url(
            &format!("artists/{}/albums", artist_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_artist_top_tracks(
        &mut self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<Track>> {
        let url = self.api_url(
            &format!("artists/{}/toptracks", artist_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_artist_videos(
        &mut self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<Video>> {
        let url = self.api_url(
            &format!("artists/{}/videos", artist_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_similar_artists(
        &mut self,
        artist_id: u64,
        limit: u32,
    ) -> Result<ItemsPage<Artist>> {
        let url = self.api_url(
            &format!("artists/{}/similar", artist_id),
            &[("limit", &limit.to_string())],
        );
        self.get(&url).await
    }
}
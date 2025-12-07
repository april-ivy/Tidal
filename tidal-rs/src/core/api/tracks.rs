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
        let track = self.get_track(track_id).await?;
        
        if let Some(album) = track.album {
            let url = self.api_url(
                &format!("albums/{}/items/credits", album.id),
                &[
                    ("replace", "true"),
                    ("includeContributors", "true"),
                    ("offset", "0"),
                    ("limit", "100"),
                ],
            );
            
            #[derive(Deserialize)]
            struct TrackWithCredits {
                item: Track,
                credits: Vec<Credit>,
            }
            
            #[derive(Deserialize)]
            struct AlbumCreditsResponse {
                items: Vec<TrackWithCredits>,
            }
            
            let resp: AlbumCreditsResponse = self.get(&url).await?;
            
            for track_credits in resp.items {
                if track_credits.item.id == track_id {
                    return Ok(track_credits.credits);
                }
            }
        }
        
        Ok(Vec::new())
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

    pub async fn get_track_full_info(&self, track_id: u64) -> Result<TrackFullInfo> {
        let track = self.get_track(track_id).await?;
        let credits = self.get_track_credits(track_id).await.ok();
        let lyrics = self.get_lyrics(track_id).await.ok();

        Ok(TrackFullInfo {
            track,
            credits,
            lyrics,
        })
    }
}

#[derive(Debug)]
pub struct TrackFullInfo {
    pub track: Track,
    pub credits: Option<Vec<Credit>>,
    pub lyrics: Option<Lyrics>,
}
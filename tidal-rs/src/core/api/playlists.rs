use super::client::TidalClient;
use super::models::{
    ItemsPage,
    Playlist,
    PlaylistItem,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_playlist(&self, playlist_id: &str) -> Result<Playlist> {
        let url = self.api_url(&format!("playlists/{}", playlist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_playlist_tracks(
        &self,
        playlist_id: &str,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<PlaylistItem>> {
        let url = self.api_url(
            &format!("playlists/{}/items", playlist_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_user_playlists(
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<Playlist>> {
        let url = self.api_url(
            &format!("users/{}/playlists", user_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn create_playlist(
        &self,
        user_id: u64,
        title: &str,
        description: &str,
    ) -> Result<Playlist> {
        let url = self.api_url(&format!("users/{}/playlists", user_id), &[]);
        let body = serde_json::json!({ "title": title, "description": description });
        self.post(&url, Some(&body.to_string())).await
    }

    pub async fn add_tracks_to_playlist(&self, playlist_id: &str, track_ids: &[u64]) -> Result<()> {
        let ids = track_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let url = self.api_url(
            &format!("playlists/{}/items", playlist_id),
            &[("trackIds", &ids)],
        );
        self.post_empty(&url, None).await
    }

    pub async fn delete_playlist(&self, playlist_id: &str) -> Result<()> {
        let url = self.api_url(&format!("playlists/{}", playlist_id), &[]);
        self.delete_empty(&url).await
    }
}

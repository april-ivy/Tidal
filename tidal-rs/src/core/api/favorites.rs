use super::client::TidalClient;
use super::models::{
    Album,
    Artist,
    FavoriteIds,
    FavoriteItem,
    ItemsPage,
    Playlist,
    Track,
    Video,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_favorite_tracks(
        &mut self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<FavoriteItem<Track>>> {
        let url = self.api_url(
            &format!("users/{}/favorites/tracks", user_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
                ("order", "DATE"),
                ("orderDirection", "DESC"),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_favorite_albums(
        &mut self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<FavoriteItem<Album>>> {
        let url = self.api_url(
            &format!("users/{}/favorites/albums", user_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
                ("order", "DATE"),
                ("orderDirection", "DESC"),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_favorite_artists(
        &mut self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<FavoriteItem<Artist>>> {
        let url = self.api_url(
            &format!("users/{}/favorites/artists", user_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
                ("order", "DATE"),
                ("orderDirection", "DESC"),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_favorite_playlists(
        &mut self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<FavoriteItem<Playlist>>> {
        let url = self.api_url(
            &format!("users/{}/favorites/playlists", user_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
                ("order", "DATE"),
                ("orderDirection", "DESC"),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_favorite_videos(
        &mut self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<ItemsPage<FavoriteItem<Video>>> {
        let url = self.api_url(
            &format!("users/{}/favorites/videos", user_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
                ("order", "DATE"),
                ("orderDirection", "DESC"),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_favorite_ids(&mut self, user_id: u64) -> Result<FavoriteIds> {
        let url = self.api_url(&format!("users/{}/favorites/ids", user_id), &[]);
        self.get(&url).await
    }

    pub async fn add_favorite_track(&mut self, user_id: u64, track_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/tracks", user_id),
            &[("trackIds", &track_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_album(&mut self, user_id: u64, album_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/albums", user_id),
            &[("albumIds", &album_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_artist(&mut self, user_id: u64, artist_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/artists", user_id),
            &[("artistIds", &artist_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_playlist(&mut self, user_id: u64, playlist_id: &str) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/playlists", user_id),
            &[("uuids", playlist_id)],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_video(&mut self, user_id: u64, video_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/videos", user_id),
            &[("videoIds", &video_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn remove_favorite_track(&mut self, user_id: u64, track_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/tracks/{}", user_id, track_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_album(&mut self, user_id: u64, album_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/albums/{}", user_id, album_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_artist(&mut self, user_id: u64, artist_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/artists/{}", user_id, artist_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_playlist(
        &mut self,
        user_id: u64,
        playlist_id: &str,
    ) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/playlists/{}", user_id, playlist_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_video(&mut self, user_id: u64, video_id: u64) -> Result<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/videos/{}", user_id, video_id),
            &[],
        );
        self.delete_empty(&url).await
    }
}

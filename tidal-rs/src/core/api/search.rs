use super::client::TidalClient;
use super::models::{
    Album,
    Artist,
    Playlist,
    SearchPage,
    SearchResults,
    SearchSuggestions,
    Track,
    Video,
};
use crate::core::error::Result;

impl TidalClient {
    pub async fn get_suggestions(&mut self, query: &str) -> Result<SearchSuggestions> {
        self.get_suggestions_with_options(query, true, true).await
    }

    pub async fn get_suggestions_with_options(
        &mut self,
        query: &str,
        explicit: bool,
        hybrid: bool,
    ) -> Result<SearchSuggestions> {
        let url = self.suggestions_url(query, explicit, hybrid);
        self.get(&url).await
    }

    pub async fn search(&mut self, query: &str, limit: u32) -> Result<SearchResults> {
        let url = self.api_url(
            "search",
            &[
                ("query", query),
                ("limit", &limit.to_string()),
                ("types", "ARTISTS,ALBUMS,TRACKS,VIDEOS,PLAYLISTS"),
            ],
        );
        self.get(&url).await
    }

    pub async fn search_tracks(
        &mut self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> Result<SearchPage<Track>> {
        let url = self.api_url(
            "search/tracks",
            &[
                ("query", query),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn search_albums(
        &mut self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> Result<SearchPage<Album>> {
        let url = self.api_url(
            "search/albums",
            &[
                ("query", query),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn search_artists(
        &mut self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> Result<SearchPage<Artist>> {
        let url = self.api_url(
            "search/artists",
            &[
                ("query", query),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn search_playlists(
        &mut self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> Result<SearchPage<Playlist>> {
        let url = self.api_url(
            "search/playlists",
            &[
                ("query", query),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn search_videos(
        &mut self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> Result<SearchPage<Video>> {
        let url = self.api_url(
            "search/videos",
            &[
                ("query", query),
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }
}

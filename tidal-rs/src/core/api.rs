use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::{
    Deserialize,
    Serialize,
};

use crate::core::AppResult;
use crate::core::auth::CLIENT_TOKEN;

const API_BASE: &str = "https://api.tidal.com/v1";
const LISTEN_API_BASE: &str = "https://listen.tidal.com/v1";
const IMAGE_BASE: &str = "https://resources.tidal.com/images";

pub fn image_url(uuid: &str, size: ImageSize) -> String {
    let path = uuid.replace('-', "/");
    
    format!("{}/{}/{}.jpg", IMAGE_BASE, path, size.as_str())
}

#[derive(Debug, Clone, Copy)]
pub enum ImageSize {
    Small,
    Medium,
    Large,
    XLarge,
}

impl ImageSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageSize::Small => "160x160",
            ImageSize::Medium => "320x320",
            ImageSize::Large => "640x640",
            ImageSize::XLarge => "1280x1280",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionInfo {
    #[serde(rename = "userId")]
    pub user_id: u64,
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserProfile {
    pub id: u64,
    pub username: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Subscription {
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "validUntil")]
    pub valid_until: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "highestSoundQuality")]
    pub highest_sound_quality: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub popularity: Option<u32>,
    pub url: Option<String>,
    #[serde(rename = "artistTypes")]
    pub artist_types: Option<Vec<String>>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArtistBio {
    pub source: Option<String>,
    pub text: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArtistLink {
    pub url: String,
    #[serde(rename = "siteName")]
    pub site_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Album {
    pub id: u64,
    pub title: String,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: Option<u32>,
    #[serde(rename = "numberOfVolumes")]
    pub number_of_volumes: Option<u32>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub duration: Option<u32>,
    pub upc: Option<String>,
    pub artist: Option<Artist>,
    pub artists: Option<Vec<Artist>>,
    pub explicit: Option<bool>,
    pub copyright: Option<String>,
    pub popularity: Option<u32>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<String>,
    #[serde(rename = "audioModes")]
    pub audio_modes: Option<Vec<String>>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: Option<MediaMetadata>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub album_type: Option<String>,
    pub version: Option<String>,
    pub cover: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MediaMetadata {
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumReview {
    pub text: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: u64,
    pub title: String,
    pub duration: u32,
    #[serde(rename = "trackNumber")]
    pub track_number: Option<u32>,
    #[serde(rename = "volumeNumber")]
    pub volume_number: Option<u32>,
    pub isrc: Option<String>,
    pub explicit: bool,
    pub artists: Vec<Artist>,
    pub artist: Option<Artist>,
    pub album: Option<Album>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<String>,
    #[serde(rename = "audioModes")]
    pub audio_modes: Option<Vec<String>>,
    pub copyright: Option<String>,
    #[serde(rename = "replayGain")]
    pub replay_gain: Option<f32>,
    pub peak: Option<f32>,
    pub url: Option<String>,
    pub popularity: Option<u32>,
    pub bpm: Option<u32>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: Option<MediaMetadata>,
    pub version: Option<String>,
    pub editable: Option<bool>,
    #[serde(rename = "allowStreaming")]
    pub allow_streaming: Option<bool>,
    #[serde(rename = "streamReady")]
    pub stream_ready: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Video {
    pub id: u64,
    pub title: String,
    pub duration: u32,
    pub explicit: bool,
    pub artists: Vec<Artist>,
    pub artist: Option<Artist>,
    pub album: Option<Album>,
    pub quality: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub popularity: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Playlist {
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: Option<u32>,
    #[serde(rename = "numberOfVideos")]
    pub number_of_videos: Option<u32>,
    pub duration: Option<u32>,
    pub creator: Option<PlaylistCreator>,
    #[serde(rename = "publicPlaylist")]
    pub public_playlist: Option<bool>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    pub created: Option<String>,
    pub url: Option<String>,
    pub popularity: Option<u32>,
    #[serde(rename = "type")]
    pub playlist_type: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "squareImage")]
    pub square_image: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistCreator {
    pub id: Option<u64>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistItem {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    #[serde(rename = "dateAdded")]
    pub date_added: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mix {
    pub id: String,
    pub title: Option<String>,
    #[serde(rename = "subTitle")]
    pub sub_title: Option<String>,
    #[serde(rename = "mixType")]
    pub mix_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MixItem {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Contributor {
    pub name: String,
    pub id: Option<u64>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credit {
    #[serde(rename = "type")]
    pub credit_type: String,
    pub contributors: Vec<Contributor>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackCredits {
    #[serde(rename = "trackId")]
    pub track_id: u64,
    pub credits: Vec<Credit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumCredits {
    #[serde(rename = "albumId")]
    pub album_id: u64,
    pub credits: Vec<Credit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FavoriteItem<T> {
    pub item: T,
    pub created: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FavoriteIds {
    #[serde(rename = "TRACK")]
    pub tracks: Option<Vec<u64>>,
    #[serde(rename = "VIDEO")]
    pub videos: Option<Vec<u64>>,
    #[serde(rename = "ARTIST")]
    pub artists: Option<Vec<u64>>,
    #[serde(rename = "ALBUM")]
    pub albums: Option<Vec<u64>>,
    #[serde(rename = "PLAYLIST")]
    pub playlists: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PlaybackInfo {
    #[serde(rename = "trackId")]
    pub track_id: u64,
    #[serde(rename = "audioQuality")]
    pub audio_quality: String,
    #[serde(rename = "audioMode")]
    pub audio_mode: String,
    #[serde(rename = "manifestMimeType")]
    pub manifest_mime_type: String,
    pub manifest: String,
    #[serde(rename = "bitDepth")]
    pub bit_depth: Option<u32>,
    #[serde(rename = "sampleRate")]
    pub sample_rate: Option<u32>,
    #[serde(rename = "albumReplayGain")]
    pub album_replay_gain: Option<f32>,
    #[serde(rename = "albumPeakAmplitude")]
    pub album_peak_amplitude: Option<f32>,
    #[serde(rename = "trackReplayGain")]
    pub track_replay_gain: Option<f32>,
    #[serde(rename = "trackPeakAmplitude")]
    pub track_peak_amplitude: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct BtsManifest {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub codecs: String,
    #[serde(rename = "encryptionType")]
    pub encryption_type: String,
    #[serde(rename = "keyId")]
    pub key_id: Option<String>,
    pub urls: Vec<String>,
}

#[derive(Debug)]
pub struct DashManifest {
    pub mime_type: String,
    pub codecs: String,
    pub urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub artists: Option<SearchPage<Artist>>,
    pub albums: Option<SearchPage<Album>>,
    pub tracks: Option<SearchPage<Track>>,
    pub videos: Option<SearchPage<Video>>,
    pub playlists: Option<SearchPage<Playlist>>,
    #[serde(rename = "topHit")]
    pub top_hit: Option<TopHit>,
}

#[derive(Debug, Deserialize)]
pub struct TopHit {
    pub value: serde_json::Value,
    #[serde(rename = "type")]
    pub hit_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchPage<T> {
    pub items: Vec<T>,
    #[serde(rename = "totalNumberOfItems")]
    pub total: Option<u32>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ItemsPage<T> {
    pub items: Vec<T>,
    #[serde(rename = "totalNumberOfItems")]
    pub total: u32,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Lyrics {
    #[serde(rename = "trackId")]
    pub track_id: u64,
    pub lyrics: Option<String>,
    pub subtitles: Option<String>,
    #[serde(rename = "lyricsProvider")]
    pub provider: Option<String>,
    #[serde(rename = "providerCommontrackId")]
    pub provider_commontrack_id: Option<String>,
    #[serde(rename = "providerLyricsId")]
    pub provider_lyrics_id: Option<String>,
    #[serde(rename = "isRightToLeft")]
    pub is_right_to_left: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Genre {
    pub name: String,
    pub path: Option<String>,
    #[serde(rename = "hasPlaylists")]
    pub has_playlists: Option<bool>,
    #[serde(rename = "hasArtists")]
    pub has_artists: Option<bool>,
    #[serde(rename = "hasAlbums")]
    pub has_albums: Option<bool>,
    #[serde(rename = "hasTracks")]
    pub has_tracks: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mood {
    pub name: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Folder {
    #[serde(rename = "trn")]
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedAt")]
    pub last_modified_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FolderItem {
    #[serde(rename = "trn")]
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "addedAt")]
    pub added_at: Option<String>,
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct TidalClient {
    #[allow(dead_code)]
    client: reqwest::Client,
    pub access_token: String,
    pub refresh_token: String,
    pub country_code: String,
    pub user_id: Option<u64>,
}

impl TidalClient {
    pub fn new(access_token: String, refresh_token: String, country_code: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            access_token,
            refresh_token,
            country_code,
            user_id: None,
        }
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Tidal-Token", CLIENT_TOKEN.parse().unwrap());
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.access_token).parse().unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT_ENCODING, "gzip".parse().unwrap());
        headers.insert(
            reqwest::header::USER_AGENT,
            "TIDAL_ANDROID/1039 okhttp/3.14.9".parse().unwrap(),
        );
        headers
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, url: &str) -> AppResult<T> {
        let resp = self.client.get(url).headers(self.headers()).send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(format!("API error {}: {}", status, &text[..text.len().min(200)]).into());
        }

        Ok(serde_json::from_str(&text)?)
    }

    async fn post<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        body: Option<&str>,
    ) -> AppResult<T> {
        let mut req = self.client.post(url).headers(self.headers());
        if let Some(b) = body {
            req = req
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(b.to_string());
        }
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(format!("API error {}: {}", status, &text[..text.len().min(200)]).into());
        }

        Ok(serde_json::from_str(&text)?)
    }

    async fn post_empty(&self, url: &str, body: Option<&str>) -> AppResult<()> {
        let mut req = self.client.post(url).headers(self.headers());
        if let Some(b) = body {
            req = req
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(b.to_string());
        }
        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(format!("API error {}: {}", status, &text[..text.len().min(200)]).into());
        }

        Ok(())
    }

    async fn put_empty(&self, url: &str, body: Option<&str>) -> AppResult<()> {
        let mut req = self.client.put(url).headers(self.headers());
        if let Some(b) = body {
            req = req
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(b.to_string());
        }
        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(format!("API error {}: {}", status, &text[..text.len().min(200)]).into());
        }

        Ok(())
    }

    async fn delete_empty(&self, url: &str) -> AppResult<()> {
        let resp = self
            .client
            .delete(url)
            .headers(self.headers())
            .send()
            .await?;
        let status = resp.status();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(format!("API error {}: {}", status, &text[..text.len().min(200)]).into());
        }

        Ok(())
    }

    fn api_url(&self, path: &str, extra_params: &[(&str, &str)]) -> String {
        let mut params = vec![
            ("countryCode", self.country_code.as_str()),
            ("locale", "en_US"),
            ("deviceType", "TV"),
        ];
        params.extend_from_slice(extra_params);

        let query = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        format!("{}/{}?{}", API_BASE, path, query)
    }

    fn listen_url(&self, path: &str, extra_params: &[(&str, &str)]) -> String {
        let mut params = vec![
            ("countryCode", self.country_code.as_str()),
            ("locale", "en_US"),
            ("deviceType", "TV"),
        ];
        params.extend_from_slice(extra_params);

        let query = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        format!("{}/{}?{}", LISTEN_API_BASE, path, query)
    }

    pub async fn get_session(&mut self) -> AppResult<SessionInfo> {
        let session: SessionInfo = self.get(&format!("{}/sessions", API_BASE)).await?;
        self.country_code = session.country_code.clone();
        self.user_id = Some(session.user_id);
        Ok(session)
    }

    pub async fn get_user(&self, user_id: u64) -> AppResult<UserProfile> {
        let url = self.api_url(&format!("users/{}", user_id), &[]);
        self.get(&url).await
    }

    pub async fn get_subscription(&self, user_id: u64) -> AppResult<Subscription> {
        let url = self.api_url(&format!("users/{}/subscription", user_id), &[]);
        self.get(&url).await
    }

    pub async fn search(&self, query: &str, limit: u32) -> AppResult<SearchResults> {
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
        &self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<SearchPage<Track>> {
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
        &self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<SearchPage<Album>> {
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
        &self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<SearchPage<Artist>> {
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
        &self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<SearchPage<Playlist>> {
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
        &self,
        query: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<SearchPage<Video>> {
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

    pub async fn get_track(&self, track_id: u64) -> AppResult<Track> {
        let url = self.api_url(&format!("tracks/{}", track_id), &[]);
        self.get(&url).await
    }

    pub async fn get_tracks(&self, track_ids: &[u64]) -> AppResult<Vec<Track>> {
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

    pub async fn get_track_credits(&self, track_id: u64) -> AppResult<Vec<Credit>> {
        let url = self.api_url(&format!("tracks/{}/credits", track_id), &[]);
        #[derive(Deserialize)]
        struct CreditsResponse {
            credits: Vec<Credit>,
        }
        let resp: CreditsResponse = self.get(&url).await?;
        Ok(resp.credits)
    }

    pub async fn get_track_mix(&self, track_id: u64) -> AppResult<Mix> {
        let url = self.api_url(&format!("tracks/{}/mix", track_id), &[]);
        self.get(&url).await
    }

    pub async fn get_lyrics(&self, track_id: u64) -> AppResult<Lyrics> {
        let url = self.api_url(&format!("tracks/{}/lyrics", track_id), &[]);
        self.get(&url).await
    }

    pub async fn get_album(&self, album_id: u64) -> AppResult<Album> {
        let url = self.api_url(&format!("albums/{}", album_id), &[]);
        self.get(&url).await
    }

    pub async fn get_albums(&self, album_ids: &[u64]) -> AppResult<Vec<Album>> {
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
    ) -> AppResult<ItemsPage<Track>> {
        let url = self.api_url(
            &format!("albums/{}/tracks", album_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_album_credits(&self, album_id: u64) -> AppResult<Vec<Credit>> {
        let url = self.api_url(&format!("albums/{}/credits", album_id), &[]);
        #[derive(Deserialize)]
        struct CreditsResponse {
            credits: Vec<Credit>,
        }
        let resp: CreditsResponse = self.get(&url).await?;
        Ok(resp.credits)
    }

    pub async fn get_album_review(&self, album_id: u64) -> AppResult<AlbumReview> {
        let url = self.api_url(&format!("albums/{}/review", album_id), &[]);
        self.get(&url).await
    }

    pub async fn get_similar_albums(
        &self,
        album_id: u64,
        limit: u32,
    ) -> AppResult<ItemsPage<Album>> {
        let url = self.api_url(
            &format!("albums/{}/similar", album_id),
            &[("limit", &limit.to_string())],
        );
        self.get(&url).await
    }

    pub async fn get_artist(&self, artist_id: u64) -> AppResult<Artist> {
        let url = self.api_url(&format!("artists/{}", artist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_artists(&self, artist_ids: &[u64]) -> AppResult<Vec<Artist>> {
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

    pub async fn get_artist_bio(&self, artist_id: u64) -> AppResult<ArtistBio> {
        let url = self.api_url(&format!("artists/{}/bio", artist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_artist_links(&self, artist_id: u64) -> AppResult<Vec<ArtistLink>> {
        let url = self.api_url(&format!("artists/{}/links", artist_id), &[]);
        #[derive(Deserialize)]
        struct LinksResponse {
            items: Vec<ArtistLink>,
            source: Option<String>,
        }
        let resp: LinksResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_artist_mix(&self, artist_id: u64) -> AppResult<Mix> {
        let url = self.api_url(&format!("artists/{}/mix", artist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_artist_albums(
        &self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<Album>> {
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
        &self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<Track>> {
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
        &self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<Video>> {
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
        &self,
        artist_id: u64,
        limit: u32,
    ) -> AppResult<ItemsPage<Artist>> {
        let url = self.api_url(
            &format!("artists/{}/similar", artist_id),
            &[("limit", &limit.to_string())],
        );
        self.get(&url).await
    }

    pub async fn get_playlist(&self, playlist_id: &str) -> AppResult<Playlist> {
        let url = self.api_url(&format!("playlists/{}", playlist_id), &[]);
        self.get(&url).await
    }

    pub async fn get_playlist_tracks(
        &self,
        playlist_id: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<PlaylistItem>> {
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
    ) -> AppResult<ItemsPage<Playlist>> {
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
    ) -> AppResult<Playlist> {
        let url = self.api_url(&format!("users/{}/playlists", user_id), &[]);
        let body = serde_json::json!({ "title": title, "description": description });
        self.post(&url, Some(&body.to_string())).await
    }

    pub async fn add_tracks_to_playlist(
        &self,
        playlist_id: &str,
        track_ids: &[u64],
    ) -> AppResult<()> {
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

    pub async fn delete_playlist(&self, playlist_id: &str) -> AppResult<()> {
        let url = self.api_url(&format!("playlists/{}", playlist_id), &[]);
        self.delete_empty(&url).await
    }

    pub async fn get_video(&self, video_id: u64) -> AppResult<Video> {
        let url = self.api_url(&format!("videos/{}", video_id), &[]);
        self.get(&url).await
    }

    pub async fn get_mix_tracks(&self, mix_id: &str, limit: u32) -> AppResult<ItemsPage<MixItem>> {
        let url = self.api_url(
            &format!("mixes/{}/items", mix_id),
            &[("limit", &limit.to_string())],
        );
        self.get(&url).await
    }

    pub async fn get_favorite_tracks(
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<FavoriteItem<Track>>> {
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
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<FavoriteItem<Album>>> {
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
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<FavoriteItem<Artist>>> {
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
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<FavoriteItem<Playlist>>> {
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
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<FavoriteItem<Video>>> {
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

    pub async fn get_favorite_ids(&self, user_id: u64) -> AppResult<FavoriteIds> {
        let url = self.api_url(&format!("users/{}/favorites/ids", user_id), &[]);
        self.get(&url).await
    }

    pub async fn add_favorite_track(&self, user_id: u64, track_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/tracks", user_id),
            &[("trackIds", &track_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_album(&self, user_id: u64, album_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/albums", user_id),
            &[("albumIds", &album_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_artist(&self, user_id: u64, artist_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/artists", user_id),
            &[("artistIds", &artist_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_playlist(&self, user_id: u64, playlist_id: &str) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/playlists", user_id),
            &[("uuids", playlist_id)],
        );
        self.post_empty(&url, None).await
    }

    pub async fn add_favorite_video(&self, user_id: u64, video_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/videos", user_id),
            &[("videoIds", &video_id.to_string())],
        );
        self.post_empty(&url, None).await
    }

    pub async fn remove_favorite_track(&self, user_id: u64, track_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/tracks/{}", user_id, track_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_album(&self, user_id: u64, album_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/albums/{}", user_id, album_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_artist(&self, user_id: u64, artist_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/artists/{}", user_id, artist_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_playlist(&self, user_id: u64, playlist_id: &str) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/playlists/{}", user_id, playlist_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn remove_favorite_video(&self, user_id: u64, video_id: u64) -> AppResult<()> {
        let url = self.api_url(
            &format!("users/{}/favorites/videos/{}", user_id, video_id),
            &[],
        );
        self.delete_empty(&url).await
    }

    pub async fn get_genres(&self) -> AppResult<Vec<Genre>> {
        let url = self.api_url("genres", &[]);
        #[derive(Deserialize)]
        struct GenresResponse {
            items: Vec<Genre>,
        }
        let resp: GenresResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_genre_tracks(
        &self,
        genre: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<Track>> {
        let url = self.api_url(
            &format!("genres/{}/tracks", genre),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_moods(&self) -> AppResult<Vec<Mood>> {
        let url = self.api_url("moods", &[]);
        #[derive(Deserialize)]
        struct MoodsResponse {
            items: Vec<Mood>,
        }
        let resp: MoodsResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_mood_playlists(
        &self,
        mood: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<Playlist>> {
        let url = self.api_url(
            &format!("moods/{}/playlists", mood),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn get_folders(&self, user_id: u64) -> AppResult<Vec<Folder>> {
        let url = self.api_url(&format!("users/{}/folders", user_id), &[]);
        #[derive(Deserialize)]
        struct FoldersResponse {
            items: Vec<Folder>,
        }
        let resp: FoldersResponse = self.get(&url).await?;
        Ok(resp.items)
    }

    pub async fn get_folder_items(
        &self,
        user_id: u64,
        folder_id: &str,
        limit: u32,
        offset: u32,
    ) -> AppResult<ItemsPage<FolderItem>> {
        let url = self.api_url(
            &format!("users/{}/folders/{}/items", user_id, folder_id),
            &[
                ("limit", &limit.to_string()),
                ("offset", &offset.to_string()),
            ],
        );
        self.get(&url).await
    }

    pub async fn create_folder(
        &self,
        user_id: u64,
        name: &str,
        parent: Option<&str>,
    ) -> AppResult<Folder> {
        let url = self.api_url(&format!("users/{}/folders", user_id), &[]);
        let mut body = serde_json::json!({ "name": name });
        if let Some(p) = parent {
            body["parent"] = serde_json::json!(p);
        }
        self.post(&url, Some(&body.to_string())).await
    }

    pub async fn delete_folder(&self, user_id: u64, folder_id: &str) -> AppResult<()> {
        let url = self.api_url(&format!("users/{}/folders/{}", user_id, folder_id), &[]);
        self.delete_empty(&url).await
    }

    pub async fn get_playback_info(&self, track_id: u64, quality: &str) -> AppResult<PlaybackInfo> {
        let url = self.listen_url(
            &format!("tracks/{}/playbackinfopostpaywall/v4", track_id),
            &[
                ("playbackmode", "STREAM"),
                ("assetpresentation", "FULL"),
                ("audioquality", quality),
                ("prefetch", "false"),
            ],
        );
        self.get(&url).await
    }

    pub fn decode_bts_manifest(&self, playback_info: &PlaybackInfo) -> AppResult<BtsManifest> {
        let decoded = BASE64.decode(&playback_info.manifest)?;
        let manifest_str = String::from_utf8(decoded)?;
        Ok(serde_json::from_str(&manifest_str)?)
    }

    pub fn decode_dash_manifest(&self, playback_info: &PlaybackInfo) -> AppResult<DashManifest> {
        let decoded = BASE64.decode(&playback_info.manifest)?;
        let manifest_str = String::from_utf8(decoded)?;
        parse_mpd(&manifest_str)
    }
}

pub fn parse_mpd(mpd_string: &str) -> AppResult<DashManifest> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(mpd_string);
    let mut urls: Vec<String> = Vec::new();
    let mut mime_type = String::new();
    let mut codecs = String::new();
    let mut in_segment_timeline = false;
    let mut initialization_url: Option<String> = None;
    let mut media_template: Option<String> = None;
    let mut segment_durations: Vec<(u64, u32)> = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => match e.name().as_ref() {
                b"AdaptationSet" => {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"mimeType" {
                            mime_type = String::from_utf8_lossy(&attr.value).to_string();
                        }
                    }
                }
                b"Representation" => {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"codecs" {
                            codecs = String::from_utf8_lossy(&attr.value).to_string();
                        }
                        if attr.key.as_ref() == b"mimeType" {
                            mime_type = String::from_utf8_lossy(&attr.value).to_string();
                        }
                    }
                }
                b"SegmentTemplate" => {
                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"initialization" => {
                                initialization_url =
                                    Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                            b"media" => {
                                media_template =
                                    Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                            _ => {}
                        }
                    }
                }
                b"SegmentTimeline" => {
                    in_segment_timeline = true;
                }
                b"S" if in_segment_timeline => {
                    let mut duration: u64 = 0;
                    let mut repeat: u32 = 0;
                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"d" => {
                                duration = String::from_utf8_lossy(&attr.value).parse().unwrap_or(0)
                            }
                            b"r" => {
                                repeat = String::from_utf8_lossy(&attr.value).parse().unwrap_or(0)
                            }
                            _ => {}
                        }
                    }
                    segment_durations.push((duration, repeat + 1));
                }
                _ => {}
            },
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"SegmentTimeline" {
                    in_segment_timeline = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e).into()),
            _ => {}
        }
    }

    if let Some(init_url) = initialization_url {
        urls.push(init_url);
    }

    if let Some(media) = media_template {
        let mut segment_number = 1u32;
        for (_duration, count) in segment_durations {
            for _ in 0..count {
                urls.push(media.replace("$Number$", &segment_number.to_string()));
                segment_number += 1;
            }
        }
    }

    if urls.is_empty() {
        return Err("No URLs found in DASH manifest".into());
    }

    if mime_type.is_empty() {
        mime_type = "audio/mp4".to_string();
    }

    Ok(DashManifest {
        mime_type,
        codecs,
        urls,
    })
}

impl Artist {
    pub fn picture_url(&self, size: ImageSize) -> Option<String> {
        self.picture.as_ref().map(|uuid| image_url(uuid, size))
    }
}

impl Track {
    pub fn display_title(&self) -> String {
        let artists = self
            .artists
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{} - {}", artists, self.title)
    }

    pub fn primary_artist(&self) -> Option<&Artist> {
        self.artist.as_ref().or_else(|| self.artists.first())
    }

    pub fn duration_formatted(&self) -> String {
        let mins = self.duration / 60;
        let secs = self.duration % 60;
        format!("{}:{:02}", mins, secs)
    }

    pub fn cover_url(&self, size: ImageSize) -> Option<String> {
        self.album.as_ref().and_then(|a| a.cover_url(size))
    }
}

impl Album {
    pub fn primary_artist(&self) -> Option<&Artist> {
        self.artist
            .as_ref()
            .or_else(|| self.artists.as_ref().and_then(|a| a.first()))
    }

    pub fn total_duration_formatted(&self) -> Option<String> {
        self.duration.map(|d| {
            let mins = d / 60;
            let secs = d % 60;
            format!("{}:{:02}", mins, secs)
        })
    }

    pub fn cover_url(&self, size: ImageSize) -> Option<String> {
        self.cover.as_ref().map(|uuid| image_url(uuid, size))
    }
}

impl Playlist {
    pub fn total_duration_formatted(&self) -> Option<String> {
        self.duration.map(|d| {
            let mins = d / 60;
            let secs = d % 60;
            format!("{}:{:02}", mins, secs)
        })
    }

    pub fn image_url(&self, size: ImageSize) -> Option<String> {
        self.square_image
            .as_ref()
            .or(self.image.as_ref())
            .map(|uuid| image_url(uuid, size))
    }
}

impl Video {
    pub fn display_title(&self) -> String {
        let artists = self
            .artists
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{} - {}", artists, self.title)
    }

    pub fn duration_formatted(&self) -> String {
        let mins = self.duration / 60;
        let secs = self.duration % 60;
        format!("{}:{:02}", mins, secs)
    }

    pub fn cover_url(&self, size: ImageSize) -> Option<String> {
        self.album.as_ref().and_then(|a| a.cover_url(size))
    }
}

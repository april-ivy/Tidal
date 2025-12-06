use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::api::{ClientConfig, TidalClient};
use crate::core::auth::{AuthSession, Credentials, DeviceAuthResponse, TokenResponse};
use crate::core::error::TidalError;
use crate::core::stream::AudioQuality;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum TidalUniFFIError {
    #[error("API error {status}: {msg}")]
    Api { status: u16, msg: String },
    #[error("Authentication failed: {msg}")]
    Auth { msg: String },
    #[error("Network error: {msg}")]
    Network { msg: String },
    #[error("JSON error: {msg}")]
    Json { msg: String },
    #[error("Decode error: {msg}")]
    Decode { msg: String },
    #[error("Encryption error: {msg}")]
    Encryption { msg: String },
    #[error("Manifest error: {msg}")]
    Manifest { msg: String },
    #[error("IO error: {msg}")]
    Io { msg: String },
}

impl From<TidalError> for TidalUniFFIError {
    fn from(err: TidalError) -> Self {
        match err {
            TidalError::Api { status, message } => TidalUniFFIError::Api { status, msg: message },
            TidalError::Auth(m) => TidalUniFFIError::Auth { msg: m },
            TidalError::Network(e) => TidalUniFFIError::Network {
                msg: e.to_string(),
            },
            TidalError::Json(e) => TidalUniFFIError::Json {
                msg: e.to_string(),
            },
            TidalError::Decode(m) => TidalUniFFIError::Decode { msg: m },
            TidalError::Encryption(m) => TidalUniFFIError::Encryption { msg: m },
            TidalError::Manifest(m) => TidalUniFFIError::Manifest { msg: m },
            TidalError::Xml(m) => TidalUniFFIError::Decode { msg: m },
            TidalError::Io(e) => TidalUniFFIError::Io {
                msg: e.to_string(),
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, TidalUniFFIError>;

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFICredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
    pub user_id: Option<u64>,
    pub country_code: String,
}

impl From<Credentials> for UniFFICredentials {
    fn from(c: Credentials) -> Self {
        Self {
            access_token: c.access_token,
            refresh_token: c.refresh_token,
            expires_at: c.expires_at,
            user_id: c.user_id,
            country_code: c.country_code,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFIDeviceAuth {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    pub expires_in: u64,
    pub interval: u64,
}

impl From<DeviceAuthResponse> for UniFFIDeviceAuth {
    fn from(d: DeviceAuthResponse) -> Self {
        Self {
            device_code: d.device_code,
            user_code: d.user_code,
            verification_uri: d.verification_uri,
            verification_uri_complete: d.verification_uri_complete,
            expires_in: d.expires_in,
            interval: d.interval,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFITokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl From<TokenResponse> for UniFFITokenResponse {
    fn from(t: TokenResponse) -> Self {
        Self {
            access_token: t.access_token,
            refresh_token: t.refresh_token,
            token_type: t.token_type,
            expires_in: t.expires_in,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFISessionInfo {
    pub user_id: u64,
    pub country_code: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFIArtist {
    pub id: u64,
    pub name: String,
    pub popularity: Option<u32>,
    pub picture_url: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFIAlbum {
    pub id: u64,
    pub title: String,
    pub artist_name: Option<String>,
    pub release_date: Option<String>,
    pub number_of_tracks: Option<u32>,
    pub duration: Option<u32>,
    pub cover_url: Option<String>,
    pub explicit: Option<bool>,
    pub audio_quality: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFITrack {
    pub id: u64,
    pub title: String,
    pub artist_name: String,
    pub album_title: Option<String>,
    pub duration: u32,
    pub duration_formatted: String,
    pub track_number: Option<u32>,
    pub explicit: bool,
    pub audio_quality: Option<String>,
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFIPlaylist {
    pub uuid: String,
    pub title: String,
    pub description: Option<String>,
    pub number_of_tracks: Option<u32>,
    pub duration: Option<u32>,
    pub creator_name: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFISearchResults {
    pub tracks: Vec<UniFFITrack>,
    pub albums: Vec<UniFFIAlbum>,
    pub artists: Vec<UniFFIArtist>,
    pub playlists: Vec<UniFFIPlaylist>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFILyrics {
    pub track_id: u64,
    pub lyrics: Option<String>,
    pub subtitles: Option<String>,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, uniffi::Enum)]
pub enum UniFFIAudioQuality {
    Low,
    High,
    Lossless,
    HiRes,
    HiResLossless,
}

impl From<UniFFIAudioQuality> for AudioQuality {
    fn from(q: UniFFIAudioQuality) -> Self {
        match q {
            UniFFIAudioQuality::Low => AudioQuality::Low,
            UniFFIAudioQuality::High => AudioQuality::High,
            UniFFIAudioQuality::Lossless => AudioQuality::Lossless,
            UniFFIAudioQuality::HiRes => AudioQuality::HiRes,
            UniFFIAudioQuality::HiResLossless => AudioQuality::HiResLossless,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct UniFFIStreamInfo {
    pub track_id: u64,
    pub mime_type: String,
    pub codecs: String,
    pub sample_rate: Option<u32>,
    pub bit_depth: Option<u32>,
    pub is_encrypted: bool,
    pub file_extension: String,
    pub is_lossless: bool,
}

use crate::core::api::models::{Album, Artist, ImageSize, Playlist, Track};

fn convert_artist(a: &Artist) -> UniFFIArtist {
    UniFFIArtist {
        id: a.id,
        name: a.name.clone(),
        popularity: a.popularity,
        picture_url: a.picture_url(ImageSize::Medium),
    }
}

fn convert_album(a: &Album) -> UniFFIAlbum {
    UniFFIAlbum {
        id: a.id,
        title: a.title.clone(),
        artist_name: a.primary_artist().map(|ar| ar.name.clone()),
        release_date: a.release_date.clone(),
        number_of_tracks: a.number_of_tracks,
        duration: a.duration,
        cover_url: a.cover_url(ImageSize::Medium),
        explicit: a.explicit,
        audio_quality: a.audio_quality.clone(),
    }
}

fn convert_track(t: &Track) -> UniFFITrack {
    UniFFITrack {
        id: t.id,
        title: t.title.clone(),
        artist_name: t
            .artists
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        album_title: t.album.as_ref().map(|a| a.title.clone()),
        duration: t.duration,
        duration_formatted: t.duration_formatted(),
        track_number: t.track_number,
        explicit: t.explicit,
        audio_quality: t.audio_quality.clone(),
        cover_url: t.cover_url(ImageSize::Medium),
    }
}

fn convert_playlist(p: &Playlist) -> UniFFIPlaylist {
    UniFFIPlaylist {
        uuid: p.uuid.clone(),
        title: p.title.clone(),
        description: p.description.clone(),
        number_of_tracks: p.number_of_tracks,
        duration: p.duration,
        creator_name: p.creator.as_ref().and_then(|c| c.name.clone()),
        image_url: p.image_url(ImageSize::Medium),
    }
}

#[derive(uniffi::Object)]
pub struct TidalAuth {
    session: AuthSession,
    runtime: tokio::runtime::Runtime,
}

#[uniffi::export]
impl TidalAuth {
    #[uniffi::constructor]
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        Self {
            session: AuthSession::new(),
            runtime,
        }
    }

    pub fn start_device_auth(&self) -> Result<UniFFIDeviceAuth> {
        self.runtime
            .block_on(self.session.start_device_auth())
            .map(UniFFIDeviceAuth::from)
            .map_err(Into::into)
    }

    pub fn poll_for_token(
        &self,
        device_code: String,
        interval: u64,
    ) -> Result<UniFFITokenResponse> {
        self.runtime
            .block_on(self.session.poll_for_token(&device_code, interval))
            .map(UniFFITokenResponse::from)
            .map_err(Into::into)
    }

    pub fn refresh_token(&self, refresh_token: String) -> Result<UniFFITokenResponse> {
        self.runtime
            .block_on(self.session.refresh_token(&refresh_token))
            .map(UniFFITokenResponse::from)
            .map_err(Into::into)
    }

    pub fn get_client_unique_key(&self) -> String {
        self.session.client_unique_key.clone()
    }
}

#[derive(uniffi::Object)]
pub struct TidalApiClient {
    client: Arc<Mutex<TidalClient>>,
    runtime: tokio::runtime::Runtime,
}

#[uniffi::export]
impl TidalApiClient {
    #[uniffi::constructor]
    pub fn new(access_token: String, refresh_token: String, country_code: String) -> Self {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let client = TidalClient::new(access_token, refresh_token, country_code);
        Self {
            client: Arc::new(Mutex::new(client)),
            runtime,
        }
    }

    pub fn get_session(&self) -> Result<UniFFISessionInfo> {
        self.runtime.block_on(async {
            let mut client = self.client.lock().await;
            let session = client.get_session().await?;
            Ok(UniFFISessionInfo {
                user_id: session.user_id,
                country_code: session.country_code,
            })
        })
    }

    pub fn get_user_id(&self) -> Option<u64> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            client.user_id
        })
    }

    pub fn search(&self, query: String, limit: u32) -> Result<UniFFISearchResults> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let results = client.search(&query, limit).await?;

            Ok(UniFFISearchResults {
                tracks: results
                    .tracks
                    .map(|p| p.items.iter().map(convert_track).collect())
                    .unwrap_or_default(),
                albums: results
                    .albums
                    .map(|p| p.items.iter().map(convert_album).collect())
                    .unwrap_or_default(),
                artists: results
                    .artists
                    .map(|p| p.items.iter().map(convert_artist).collect())
                    .unwrap_or_default(),
                playlists: results
                    .playlists
                    .map(|p| p.items.iter().map(convert_playlist).collect())
                    .unwrap_or_default(),
            })
        })
    }

    pub fn get_track(&self, track_id: u64) -> Result<UniFFITrack> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let track = client.get_track(track_id).await?;
            Ok(convert_track(&track))
        })
    }

    pub fn get_tracks(&self, track_ids: Vec<u64>) -> Result<Vec<UniFFITrack>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let tracks = client.get_tracks(&track_ids).await?;
            Ok(tracks.iter().map(convert_track).collect())
        })
    }

    pub fn get_lyrics(&self, track_id: u64) -> Result<UniFFILyrics> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let lyrics = client.get_lyrics(track_id).await?;
            Ok(UniFFILyrics {
                track_id: lyrics.track_id,
                lyrics: lyrics.lyrics,
                subtitles: lyrics.subtitles,
                provider: lyrics.provider,
            })
        })
    }

    pub fn get_album(&self, album_id: u64) -> Result<UniFFIAlbum> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let album = client.get_album(album_id).await?;
            Ok(convert_album(&album))
        })
    }

    pub fn get_album_tracks(
        &self,
        album_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<UniFFITrack>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let page = client.get_album_tracks(album_id, limit, offset).await?;
            Ok(page.items.iter().map(convert_track).collect())
        })
    }

    pub fn get_artist(&self, artist_id: u64) -> Result<UniFFIArtist> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let artist = client.get_artist(artist_id).await?;
            Ok(convert_artist(&artist))
        })
    }

    pub fn get_artist_top_tracks(
        &self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<UniFFITrack>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let page = client
                .get_artist_top_tracks(artist_id, limit, offset)
                .await?;
            Ok(page.items.iter().map(convert_track).collect())
        })
    }

    pub fn get_artist_albums(
        &self,
        artist_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<UniFFIAlbum>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let page = client.get_artist_albums(artist_id, limit, offset).await?;
            Ok(page.items.iter().map(convert_album).collect())
        })
    }

    pub fn get_playlist(&self, playlist_id: String) -> Result<UniFFIPlaylist> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let playlist = client.get_playlist(&playlist_id).await?;
            Ok(convert_playlist(&playlist))
        })
    }

    pub fn get_playlist_tracks(
        &self,
        playlist_id: String,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<UniFFITrack>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let page = client
                .get_playlist_tracks(&playlist_id, limit, offset)
                .await?;
            Ok(page.items.iter().map(|pi| convert_track(&pi.item)).collect())
        })
    }

    pub fn get_user_playlists(
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<UniFFIPlaylist>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let page = client.get_user_playlists(user_id, limit, offset).await?;
            Ok(page.items.iter().map(convert_playlist).collect())
        })
    }

    pub fn get_favorite_tracks(
        &self,
        user_id: u64,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<UniFFITrack>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let page = client.get_favorite_tracks(user_id, limit, offset).await?;
            Ok(page.items.iter().map(|fi| convert_track(&fi.item)).collect())
        })
    }

    pub fn add_favorite_track(&self, user_id: u64, track_id: u64) -> Result<()> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            client.add_favorite_track(user_id, track_id).await?;
            Ok(())
        })
    }

    pub fn remove_favorite_track(&self, user_id: u64, track_id: u64) -> Result<()> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            client.remove_favorite_track(user_id, track_id).await?;
            Ok(())
        })
    }

    pub fn get_stream_info(
        &self,
        track_id: u64,
        quality: UniFFIAudioQuality,
    ) -> Result<UniFFIStreamInfo> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let info = client
                .get_stream_info(track_id, quality.into())
                .await?;
            Ok(UniFFIStreamInfo {
                track_id: info.track_id,
                mime_type: info.mime_type.clone(),
                codecs: info.codecs.clone(),
                sample_rate: info.sample_rate,
                bit_depth: info.bit_depth,
                is_encrypted: info.encryption.is_some(),
                file_extension: info.file_extension().to_string(),
                is_lossless: info.is_lossless(),
            })
        })
    }

    pub fn download_track(
        &self,
        track_id: u64,
        quality: UniFFIAudioQuality,
        output_path: String,
    ) -> Result<()> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            client
                .download_track(track_id, quality.into(), &output_path)
                .await?;
            Ok(())
        })
    }

    pub fn get_track_bytes(&self, track_id: u64, quality: UniFFIAudioQuality) -> Result<Vec<u8>> {
        self.runtime.block_on(async {
            let client = self.client.lock().await;
            let mut stream_info = client
                .get_stream_info(track_id, quality.into())
                .await?;
            let bytes = client.get_stream_bytes(&mut stream_info).await?;
            Ok(bytes)
        })
    }
}

uniffi::setup_scaffolding!();
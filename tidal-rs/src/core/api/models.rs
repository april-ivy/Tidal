use serde::{
    Deserialize,
    Serialize,
};

pub const IMAGE_BASE: &str = "https://resources.tidal.com/images";

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub popularity: Option<u32>,
    pub url: Option<String>,
    #[serde(rename = "artistTypes")]
    pub artist_types: Option<Vec<String>>,
    pub picture: Option<String>,
    pub handle: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<u64>,
    #[serde(rename = "type")]
    pub artist_type: Option<String>,
    #[serde(rename = "contributionLinkUrl")]
    pub contribution_link_url: Option<String>,
    #[serde(rename = "artistRoles")]
    pub artist_roles: Option<Vec<ArtistRole>>,
    pub mixes: Option<ArtistMixes>,
    #[serde(rename = "selectedAlbumCoverFallback")]
    pub selected_album_cover_fallback: Option<String>,
}

impl Artist {
    pub fn picture_url(&self, size: ImageSize) -> Option<String> {
        self.picture
            .as_ref()
            .or(self.selected_album_cover_fallback.as_ref())
            .map(|uuid| image_url(uuid, size))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArtistRole {
    pub category: String,
    #[serde(rename = "categoryId")]
    pub category_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArtistMixes {
    #[serde(rename = "ARTIST_MIX")]
    pub artist_mix: Option<String>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MediaMetadata {
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Album {
    pub id: u64,
    pub title: String,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: Option<u32>,
    #[serde(rename = "numberOfVolumes")]
    pub number_of_volumes: Option<u32>,
    #[serde(rename = "numberOfVideos")]
    pub number_of_videos: Option<u32>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(rename = "streamStartDate")]
    pub stream_start_date: Option<String>,
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
    #[serde(rename = "videoCover")]
    pub video_cover: Option<String>,
    #[serde(rename = "vibrantColor")]
    pub vibrant_color: Option<String>,
    #[serde(rename = "streamReady")]
    pub stream_ready: Option<bool>,
    #[serde(rename = "allowStreaming")]
    pub allow_streaming: Option<bool>,
    #[serde(rename = "payToStream")]
    pub pay_to_stream: Option<bool>,
    pub upload: Option<bool>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumReview {
    pub text: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackMixes {
    #[serde(rename = "TRACK_MIX")]
    pub track_mix: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    #[serde(rename = "doublePopularity")]
    pub double_popularity: Option<f64>,
    pub bpm: Option<u32>,
    pub key: Option<String>,
    #[serde(rename = "keyScale")]
    pub key_scale: Option<String>,
    #[serde(rename = "mediaMetadata")]
    pub media_metadata: Option<MediaMetadata>,
    pub version: Option<String>,
    pub editable: Option<bool>,
    #[serde(rename = "allowStreaming")]
    pub allow_streaming: Option<bool>,
    #[serde(rename = "streamReady")]
    pub stream_ready: Option<bool>,
    #[serde(rename = "streamStartDate")]
    pub stream_start_date: Option<String>,
    #[serde(rename = "adSupportedStreamReady")]
    pub ad_supported_stream_ready: Option<bool>,
    #[serde(rename = "djReady")]
    pub dj_ready: Option<bool>,
    #[serde(rename = "stemReady")]
    pub stem_ready: Option<bool>,
    #[serde(rename = "premiumStreamingOnly")]
    pub premium_streaming_only: Option<bool>,
    #[serde(rename = "payToStream")]
    pub pay_to_stream: Option<bool>,
    #[serde(rename = "accessType")]
    pub access_type: Option<String>,
    pub spotlighted: Option<bool>,
    pub upload: Option<bool>,
    pub mixes: Option<TrackMixes>,
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

    pub fn musical_key_formatted(&self) -> Option<String> {
        self.key.as_ref().map(|k| {
            let scale = self.key_scale.as_deref().unwrap_or("");
            let key_display = match k.as_str() {
                "AB" => "A♭",
                "BB" => "B♭",
                "DB" => "D♭",
                "EB" => "E♭",
                "GB" => "G♭",
                other => other,
            };
            if scale.is_empty() {
                key_display.to_string()
            } else {
                format!("{} {}", key_display, scale.to_lowercase())
            }
        })
    }
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contributor {
    pub name: String,
    pub id: Option<u64>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credit {
    #[serde(rename = "type")]
    pub credit_type: String,
    pub contributors: Vec<Contributor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackCredits {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub credits: Vec<Credit>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlbumCredits {
    pub items: Vec<Credit>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct SearchSuggestions {
    pub history: Option<Vec<SuggestionItem>>,
    pub suggestions: Option<Vec<SuggestionItem>>,
    #[serde(rename = "directHits")]
    pub direct_hits: Option<Vec<DirectHit>>,
    #[serde(rename = "suggestionUuid")]
    pub suggestion_uuid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionItem {
    pub query: String,
    pub highlights: Option<Vec<Highlight>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Highlight {
    pub start: u32,
    pub length: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DirectHit {
    pub value: serde_json::Value,
    #[serde(rename = "type")]
    pub hit_type: String,
}

#[derive(Debug, Clone)]
pub enum DirectHitValue {
    Track(Box<SuggestionTrack>),
    Artist(Box<SuggestionArtist>),
    Album(Box<SuggestionAlbum>),
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionTrack {
    pub id: u64,
    pub title: String,
    pub duration: u32,
    pub explicit: bool,
    pub popularity: Option<u32>,
    #[serde(rename = "trackNumber")]
    pub track_number: Option<u32>,
    #[serde(rename = "volumeNumber")]
    pub volume_number: Option<u32>,
    pub isrc: Option<String>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<String>,
    pub album: Option<SuggestionAlbumRef>,
    pub artists: Option<Vec<SuggestionArtistRef>>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionArtist {
    pub id: u64,
    pub name: String,
    pub picture: Option<String>,
    pub popularity: Option<u32>,
    #[serde(rename = "artistTypes")]
    pub artist_types: Option<Vec<String>>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionAlbum {
    pub id: u64,
    pub title: String,
    pub cover: Option<String>,
    pub duration: Option<u32>,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: Option<u32>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub explicit: Option<bool>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<String>,
    pub artists: Option<Vec<SuggestionArtistRef>>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionAlbumRef {
    pub id: u64,
    pub title: String,
    pub cover: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuggestionArtistRef {
    pub id: u64,
    pub name: String,
    pub picture: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: Option<String>,
}

impl DirectHit {
    pub fn parse_value(&self) -> DirectHitValue {
        match self.hit_type.as_str() {
            "TRACKS" => serde_json::from_value(self.value.clone())
                .map(|t| DirectHitValue::Track(Box::new(t)))
                .unwrap_or_else(|_| DirectHitValue::Unknown(self.value.clone())),
            "ARTISTS" => serde_json::from_value(self.value.clone())
                .map(|a| DirectHitValue::Artist(Box::new(a)))
                .unwrap_or_else(|_| DirectHitValue::Unknown(self.value.clone())),
            "ALBUMS" => serde_json::from_value(self.value.clone())
                .map(|a| DirectHitValue::Album(Box::new(a)))
                .unwrap_or_else(|_| DirectHitValue::Unknown(self.value.clone())),
            _ => DirectHitValue::Unknown(self.value.clone()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumPage {
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub rows: Vec<AlbumPageRow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumPageRow {
    pub modules: Vec<AlbumPageModule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumPageModule {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub module_type: String,
    pub width: Option<u32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub album: Option<Album>,
    pub review: Option<AlbumReview>,
    pub credits: Option<AlbumCredits>,
    #[serde(rename = "pagedList")]
    pub paged_list: Option<PagedList>,
    #[serde(rename = "releaseDate")]
    pub release_date: Option<String>,
    pub copyright: Option<String>,
    #[serde(rename = "listFormat")]
    pub list_format: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PagedList {
    #[serde(rename = "dataApiPath")]
    pub data_api_path: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    #[serde(rename = "totalNumberOfItems")]
    pub total_number_of_items: Option<u32>,
    pub items: Vec<PagedListItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PagedListItem {
    pub item: Option<Track>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumItemsCreditsResponse {
    pub limit: u32,
    pub offset: u32,
    #[serde(rename = "totalNumberOfItems")]
    pub total_number_of_items: u32,
    pub items: Vec<TrackCredits>,
}

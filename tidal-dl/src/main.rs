use std::io::Write;
use std::path::{
    Path,
    PathBuf,
};
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

use clap::Parser;
use indicatif::{
    ProgressBar,
    ProgressStyle,
};
use lofty::config::WriteOptions;
use lofty::picture::{
    MimeType,
    Picture,
    PictureType,
};
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::{
    ItemKey,
    ItemValue,
    Tag,
    TagItem,
    TagType,
};
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use serde::{
    Deserialize,
    Serialize,
};
use termcolor::{
    Color,
    ColorChoice,
    ColorSpec,
    StandardStream,
    WriteColor,
};
use tidal::{
    AudioQuality,
    AuthSession,
    ImageSize,
    Playlist,
    StreamInfo,
    TidalClient,
    Track,
};

type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Parser)]
#[command(name = "tidal-dl")]
#[command(
    author,
    version,
    about = "Download music from Tidal in highest quality"
)]
struct Args {
    link: String,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredCredentials {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
    country_code: String,
}

struct Console {
    stdout: StandardStream,
}

impl Console {
    fn new() -> Self {
        Self {
            stdout: StandardStream::stdout(ColorChoice::Auto),
        }
    }

    fn print(&mut self, text: &str) {
        let _ = write!(self.stdout, "{}", text);
        let _ = self.stdout.flush();
    }

    fn println(&mut self, text: &str) {
        let _ = writeln!(self.stdout, "{}", text);
    }

    fn print_colored(&mut self, text: &str, color: Color) {
        let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(color)));
        let _ = write!(self.stdout, "{}", text);
        let _ = self.stdout.reset();
        let _ = self.stdout.flush();
    }

    fn println_colored(&mut self, text: &str, color: Color) {
        let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(color)));
        let _ = writeln!(self.stdout, "{}", text);
        let _ = self.stdout.reset();
    }

    fn success(&mut self, text: &str) {
        self.print_colored("OK ", Color::Green);
        self.println(text);
    }

    fn error(&mut self, text: &str) {
        self.print_colored("ERROR ", Color::Red);
        self.println(text);
    }

    fn info(&mut self, text: &str) {
        self.print_colored("INFO ", Color::Cyan);
        self.println(text);
    }

    fn status(&mut self, text: &str) {
        self.print_colored("  -> ", Color::Yellow);
        self.print(text);
    }
}

fn get_config_path() -> AppResult<PathBuf> {
    let config_dir = dirs::config_dir().ok_or("Could not find config directory")?;
    let app_dir = config_dir.join("tidal-dl");
    std::fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("credentials.json"))
}

fn load_credentials() -> AppResult<Option<StoredCredentials>> {
    let path = get_config_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)?;
    let creds: StoredCredentials = serde_json::from_str(&content)?;
    Ok(Some(creds))
}

fn save_credentials(creds: &StoredCredentials) -> AppResult<()> {
    let path = get_config_path()?;
    let content = serde_json::to_string_pretty(creds)?;
    std::fs::write(&path, content)?;
    Ok(())
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn authenticate(console: &mut Console) -> AppResult<TidalClient> {
    let auth = AuthSession::new();

    console.info("Starting device authentication...");
    let device_auth = auth.start_device_auth().await?;

    console.println("");
    console.println("TIDAL Authentication");
    console.println("");
    console.print("  1. Open: ");
    console.println_colored(&device_auth.verification_uri, Color::Cyan);
    console.print("  2. Enter code: ");
    console.println_colored(&device_auth.user_code, Color::Yellow);
    console.println("");

    if let Some(complete_uri) = &device_auth.verification_uri_complete {
        console.print("  Or open directly: ");
        console.println_colored(complete_uri, Color::Cyan);
        console.println("");
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Waiting for authentication...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let token = auth
        .poll_for_token(&device_auth.device_code, device_auth.interval)
        .await?;

    spinner.finish_and_clear();

    let mut client = TidalClient::new(
        token.access_token.clone(),
        token.refresh_token.clone(),
        "US".to_string(),
    );

    let session = client.get_session().await?;

    let creds = StoredCredentials {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
        expires_at: current_timestamp() + token.expires_in,
        country_code: session.country_code,
    };

    save_credentials(&creds)?;
    console.success("Authentication successful. Credentials saved.");
    console.println("");

    Ok(client)
}

async fn get_client(console: &mut Console) -> AppResult<TidalClient> {
    let creds = match load_credentials()? {
        Some(c) => c,
        None => return authenticate(console).await,
    };

    if current_timestamp() + 300 > creds.expires_at {
        console.info("Token expired, refreshing...");
        let auth = AuthSession::new();
        match auth.refresh_token(&creds.refresh_token).await {
            Ok(token) => {
                let mut client = TidalClient::new(
                    token.access_token.clone(),
                    token.refresh_token.clone(),
                    creds.country_code.clone(),
                );
                client.get_session().await?;

                let new_creds = StoredCredentials {
                    access_token: token.access_token,
                    refresh_token: token.refresh_token,
                    expires_at: current_timestamp() + token.expires_in,
                    country_code: creds.country_code,
                };
                save_credentials(&new_creds)?;
                console.success("Token refreshed.");
                Ok(client)
            }
            Err(_) => {
                console.info("Failed to refresh token. Re-authenticating...");
                authenticate(console).await
            }
        }
    } else {
        let mut client =
            TidalClient::new(creds.access_token, creds.refresh_token, creds.country_code);
        client.get_session().await?;
        Ok(client)
    }
}

fn parse_tidal_link(link: &str) -> AppResult<(String, String)> {
    if let Ok(id) = link.parse::<u64>() {
        return Ok(("track".to_string(), id.to_string()));
    }

    let track_re = Regex::new(r"(?:tidal\.com|listen\.tidal\.com)(?:/browse)?/track/(\d+)")?;
    if let Some(caps) = track_re.captures(link) {
        let id = caps.get(1).unwrap().as_str().to_string();
        return Ok(("track".to_string(), id));
    }

    let album_re = Regex::new(r"(?:tidal\.com|listen\.tidal\.com)(?:/browse)?/album/(\d+)")?;
    if let Some(caps) = album_re.captures(link) {
        let id = caps.get(1).unwrap().as_str().to_string();
        return Ok(("album".to_string(), id));
    }

    let playlist_re =
        Regex::new(r"(?:tidal\.com|listen\.tidal\.com)(?:/browse)?/playlist/([a-f0-9-]+)")?;
    if let Some(caps) = playlist_re.captures(link) {
        let id = caps.get(1).unwrap().as_str().to_string();
        return Ok(("playlist".to_string(), id));
    }

    Err(format!("Could not parse Tidal link: {}", link).into())
}

fn sanitize_filename(name: &str) -> String {
    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut result = name.to_string();
    for c in invalid_chars {
        result = result.replace(c, "_");
    }
    result.trim_end_matches(['.', ' ']).to_string()
}

fn format_duration(seconds: u32) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{}:{:02}", mins, secs)
}

async fn download_lyrics(
    client: &TidalClient,
    track_id: u64,
    output_path: &PathBuf,
    console: &mut Console,
) -> AppResult<Option<String>> {
    console.status("Fetching lyrics... ");

    match client.get_lyrics(track_id).await {
        Ok(lyrics) => {
            let content = lyrics.subtitles.or(lyrics.lyrics).unwrap_or_default();

            if content.is_empty() {
                console.println_colored("not available", Color::Yellow);
                return Ok(None);
            }

            tokio::fs::write(output_path, &content).await?;
            console.println_colored("OK", Color::Green);
            console.print("  Saved: ");
            console.println_colored(&output_path.display().to_string(), Color::Cyan);
            Ok(Some(content))
        }
        Err(_) => {
            console.println_colored("not available", Color::Yellow);
            Ok(None)
        }
    }
}

async fn fetch_cover_image(track: &Track) -> AppResult<Option<(Vec<u8>, MimeType)>> {
    if let Some(url) = track.cover_url(ImageSize::XLarge) {
        let resp = reqwest::get(&url).await?;
        if !resp.status().is_success() {
            return Ok(None);
        }

        let content_type = resp
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok().map(str::to_owned));
        let bytes = resp.bytes().await?.to_vec();
        let mime = content_type
            .as_deref()
            .and_then(|ct| {
                if ct.contains("png") {
                    Some(MimeType::Png)
                } else if ct.contains("gif") {
                    Some(MimeType::Gif)
                } else if ct.contains("bmp") {
                    Some(MimeType::Bmp)
                } else if ct.contains("jpeg") || ct.contains("jpg") {
                    Some(MimeType::Jpeg)
                } else {
                    None
                }
            })
            .unwrap_or(MimeType::Jpeg);

        return Ok(Some((bytes, mime)));
    }

    Ok(None)
}

fn build_full_title(title: &str, version: Option<&str>) -> String {
    match version {
        Some(v) if !v.is_empty() => format!("{} ({})", title, v),
        _ => title.to_string(),
    }
}

fn encode_audio_details(stream_info: &StreamInfo) -> Option<String> {
    let mut details = Vec::new();

    if let Some(rate) = stream_info.sample_rate {
        details.push(format!("{} kHz", rate / 1000));
    }

    if let Some(depth) = stream_info.bit_depth {
        details.push(format!("{} bit", depth));
    }

    if !stream_info.codecs.is_empty() {
        details.push(stream_info.codecs.clone());
    }

    if details.is_empty() {
        None
    } else {
        Some(details.join(" | "))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContainerKind {
    Flac,
    Mp4,
}

fn detect_container(data: &[u8]) -> ContainerKind {
    if data.len() >= 4 && &data[..4] == b"fLaC" {
        return ContainerKind::Flac;
    }
    if data.len() >= 8 && &data[4..8] == b"ftyp" {
        return ContainerKind::Mp4;
    }
    ContainerKind::Flac
}

async fn embed_metadata(
    client: &TidalClient,
    output_path: &Path,
    track: &Track,
    full_title: &str,
    stream_info: &StreamInfo,
    lyrics: Option<String>,
) -> AppResult<()> {
    let ext = output_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let tag_type = if ext == "flac" {
        TagType::VorbisComments
    } else {
        TagType::Mp4Ilst
    };

    let mut tagged_file = Probe::open(output_path)?.read()?;
    if tagged_file.tag(tag_type).is_none() {
        tagged_file.insert_tag(Tag::new(tag_type));
    }

    let tag = tagged_file
        .tag_mut(tag_type)
        .ok_or_else(|| "Failed to get tag".to_string())?;

    let artists_joined = track
        .artists
        .iter()
        .map(|a| a.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    tag.set_title(full_title.to_string());
    tag.set_artist(artists_joined.clone());

    if let Some(version) = track.version.as_ref() {
        tag.insert_text(ItemKey::TrackSubtitle, version.clone());
    }

    if let Some(album) = &track.album {
        if let Some(album_artist) = album.primary_artist() {
            tag.insert_text(ItemKey::AlbumArtist, album_artist.name.clone());
        } else if let Some(primary) = track.primary_artist() {
            tag.insert_text(ItemKey::AlbumArtist, primary.name.clone());
        } else {
            tag.insert_text(ItemKey::AlbumArtist, artists_joined.clone());
        }
    } else if let Some(primary) = track.primary_artist() {
        tag.insert_text(ItemKey::AlbumArtist, primary.name.clone());
    } else {
        tag.insert_text(ItemKey::AlbumArtist, artists_joined.clone());
    }

    tag.insert_text(ItemKey::Performer, artists_joined.clone());
    tag.insert_text(ItemKey::OriginalArtist, artists_joined.clone());

    if let Some(primary) = track.primary_artist() {
        tag.insert_text(ItemKey::Composer, primary.name.clone());
    } else {
        tag.insert_text(ItemKey::Composer, artists_joined.clone());
    }

    for artist in &track.artists {
        tag.push(TagItem::new(
            ItemKey::TrackArtists,
            ItemValue::Text(artist.name.clone()),
        ));
    }

    if let Some(tags) = track
        .media_metadata
        .as_ref()
        .and_then(|m| m.tags.as_ref())
        .filter(|v| !v.is_empty())
        .or_else(|| {
            track
                .album
                .as_ref()
                .and_then(|a| a.media_metadata.as_ref())
                .and_then(|m| m.tags.as_ref())
                .filter(|v| !v.is_empty())
        })
    {
        let genres = tags.join(", ");
        tag.insert_text(ItemKey::Genre, genres);
    }

    let date_to_use = track
        .album
        .as_ref()
        .and_then(|a| a.release_date.as_ref().or(a.stream_start_date.as_ref()))
        .or(track.stream_start_date.as_ref());

    if let Some(date) = date_to_use {
        if let Some(year_str) = date.split('-').next() {
            if let Ok(y) = year_str.parse::<u32>() {
                tag.set_year(y);
                tag.insert_text(ItemKey::Year, year_str.to_string());

                let date_only = date.split('T').next().unwrap_or(date);
                tag.insert_text(ItemKey::RecordingDate, date_only.to_string());
                tag.insert_text(ItemKey::ReleaseDate, date_only.to_string());
                tag.insert_text(ItemKey::OriginalReleaseDate, date_only.to_string());
            }
        }
    }

    if let Some(album) = &track.album {
        tag.set_album(album.title.clone());

        match client.get_album(album.id).await {
            Ok(full_album) => {
                if let Some(total) = full_album.number_of_tracks {
                    tag.set_track_total(total);
                }

                if let Some(vol_total) = full_album.number_of_volumes {
                    tag.set_disk_total(vol_total);
                }
            }
            Err(_) => {
                if let Some(total) = album.number_of_tracks {
                    tag.set_track_total(total);
                }

                if let Some(vol_total) = album.number_of_volumes {
                    tag.set_disk_total(vol_total);
                }
            }
        }

        if let Some(upc) = album.upc.clone() {
            tag.insert_text(ItemKey::CatalogNumber, upc.clone());
            tag.insert_text(ItemKey::Barcode, upc);
        }

        if let Some(album_type) = album.album_type.as_ref() {
            tag.insert_text(ItemKey::OriginalMediaType, album_type.clone());
        }
    }

    if let Some(n) = track.track_number {
        tag.set_track(n);
    }

    if let Some(disc) = track.volume_number {
        tag.set_disk(disc);
    }

    if let Some(isrc) = track.isrc.clone() {
        tag.insert_text(ItemKey::Isrc, isrc);
    }

    if let Some(url) = track.url.as_ref() {
        tag.insert_text(ItemKey::AudioSourceUrl, url.clone());
    }

    if track.explicit {
        tag.insert_text(ItemKey::ParentalAdvisory, "Explicit".to_string());
    }

    if let Some(gain) = track.replay_gain {
        tag.insert_text(ItemKey::ReplayGainTrackGain, format!("{gain:.2} dB"));
    }

    if let Some(peak) = track.peak {
        tag.insert_text(ItemKey::ReplayGainTrackPeak, format!("{peak:.6}"));
    }

    let mut encoder_info_parts = Vec::new();

    if let Some(quality) = track
        .audio_quality
        .as_ref()
        .or_else(|| track.album.as_ref().and_then(|a| a.audio_quality.as_ref()))
    {
        encoder_info_parts.push(format!("Tidal {}", quality));
    }

    if let Some(details) = encode_audio_details(stream_info) {
        encoder_info_parts.push(details);
    }

    if let Some(modes) = track.audio_modes.as_ref() {
        if !modes.is_empty() {
            encoder_info_parts.push(format!("Modes: {}", modes.join(", ")));
        }
    }

    if !encoder_info_parts.is_empty() {
        tag.insert_text(ItemKey::EncoderSettings, encoder_info_parts.join(" | "));
    }

    tag.insert_text(ItemKey::EncoderSoftware, "Tidal".to_string());

    if let Some(media_tags) = track
        .media_metadata
        .as_ref()
        .and_then(|m| m.tags.as_ref())
        .filter(|t| !t.is_empty())
    {
        let tags_str = media_tags.join(", ");
        tag.insert_text(ItemKey::Description, format!("Quality: {}", tags_str));
    }

    if let Some(popularity) = track.popularity {
        tag.insert_text(ItemKey::Popularimeter, popularity.to_string());
    }

    if let Some(c) = track
        .copyright
        .clone()
        .or_else(|| track.album.as_ref().and_then(|a| a.copyright.clone()))
    {
        tag.insert_text(ItemKey::CopyrightMessage, c);
    }

    if let Some(album) = &track.album {
        if let Some(label_artist) = album.artist.as_ref() {
            tag.insert_text(ItemKey::Label, label_artist.name.clone());
            tag.insert_text(ItemKey::Publisher, label_artist.name.clone());
        }
    }

    tag.insert_text(ItemKey::EncodedBy, "Tidal".to_string());

    if let Some(key) = track.musical_key_formatted() {
        tag.insert_text(ItemKey::InitialKey, key);
    }

    if let Some(bpm) = track.bpm {
        tag.insert_text(ItemKey::Bpm, bpm.to_string());
        tag.insert_text(ItemKey::IntegerBpm, bpm.to_string());
    }

    let mut comment_parts = Vec::new();

    if let Some(popularity) = track.popularity {
        comment_parts.push(format!("Popularity: {}/100", popularity));
    }

    if track.stream_ready == Some(true) {
        if let Some(start_date) = track.stream_start_date.as_ref() {
            if let Some(date_only) = start_date.split('T').next() {
                comment_parts.push(format!("Available since: {}", date_only));
            }
        }
    }

    comment_parts.push(format!("Tidal ID: {}", track.id));

    if !comment_parts.is_empty() {
        let comment = comment_parts.join(" | ");
        if let Some(existing) = tag.get_string(&ItemKey::Comment) {
            tag.insert_text(ItemKey::Comment, format!("{} | {}", existing, comment));
        } else {
            tag.insert_text(ItemKey::Comment, comment);
        }
    }

    if let Some(text) = lyrics.clone() {
        tag.insert_text(ItemKey::Lyrics, text);
    }

    let credits = if let Some(album) = &track.album {
        match client.get_album_page(album.id).await {
            Ok(album_page) => {
                let album_credits = album_page
                    .rows
                    .iter()
                    .flat_map(|row| &row.modules)
                    .find(|module| module.module_type == "ALBUM_HEADER")
                    .and_then(|module| module.credits.as_ref())
                    .map(|c| &c.items);

                album_credits.map(|c| c.clone())
            }
            Err(_) => None,
        }
    } else {
        None
    };

    if let Some(credits) = credits {
        for credit in credits.iter() {
            let contributors = credit
                .contributors
                .iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            if contributors.is_empty() {
                continue;
            }

            let credit_type_lower = credit.credit_type.to_lowercase();

            match credit_type_lower.as_str() {
                "producer" | "producers" => {
                    tag.insert_text(ItemKey::Producer, contributors);
                }
                "mixer" | "mixing" | "mix engineer" => {
                    tag.insert_text(ItemKey::MixEngineer, contributors);
                }
                "engineer" | "recording engineer" | "audio engineer" => {
                    tag.insert_text(ItemKey::Engineer, contributors);
                }
                "writer" | "songwriter" => {
                    tag.insert_text(ItemKey::Writer, contributors);
                }
                "composer" | "composers" => {
                    if tag.get_string(&ItemKey::Composer).is_none() {
                        tag.insert_text(ItemKey::Composer, contributors);
                    }
                }
                "lyricist" => {
                    tag.insert_text(ItemKey::Lyricist, contributors);
                }
                "arranger" => {
                    tag.insert_text(ItemKey::Arranger, contributors);
                }
                "conductor" => {
                    tag.insert_text(ItemKey::Conductor, contributors);
                }
                "remixer" | "remix" => {
                    tag.insert_text(ItemKey::Remixer, contributors);
                }
                "performer" | "performers" => {
                    let performer_info = format!("Performers: {}", contributors);
                    if let Some(existing_comment) = tag.get_string(&ItemKey::Comment) {
                        tag.insert_text(
                            ItemKey::Comment,
                            format!("{} | {}", existing_comment, performer_info),
                        );
                    } else {
                        tag.insert_text(ItemKey::Comment, performer_info);
                    }
                }
                "record label" => {
                    tag.insert_text(ItemKey::Label, contributors.clone());
                    tag.insert_text(ItemKey::Publisher, contributors);
                }
                _ => {
                    let credit_info = format!("{}: {}", credit.credit_type, contributors);
                    if let Some(existing_comment) = tag.get_string(&ItemKey::Comment) {
                        tag.insert_text(
                            ItemKey::Comment,
                            format!("{} | {}", existing_comment, credit_info),
                        );
                    } else {
                        tag.insert_text(ItemKey::Comment, credit_info);
                    }
                }
            }
        }
    }

    if let Some((cover_bytes, mime)) = fetch_cover_image(track).await? {
        let picture =
            Picture::new_unchecked(PictureType::CoverFront, Some(mime), None, cover_bytes);
        tag.push_picture(picture);
    }

    tagged_file.save_to_path(output_path, WriteOptions::default())?;

    Ok(())
}

async fn download_track(
    client: &TidalClient,
    track: &Track,
    output_dir: &PathBuf,
    console: &mut Console,
) -> AppResult<()> {
    let artist_name = track
        .artist
        .as_ref()
        .map(|a| a.name.clone())
        .or_else(|| track.artists.first().map(|a| a.name.clone()))
        .unwrap_or_else(|| "Unknown Artist".to_string());

    let title = &track.title;
    let full_title = build_full_title(title, track.version.as_deref());

    console.println("");
    console.println(&format!(
        "Track: {} - {} [{}]",
        artist_name,
        full_title,
        format_duration(track.duration)
    ));

    console.status("Fetching stream info... ");
    let mut stream_info = client
        .get_stream_info(track.id, AudioQuality::HiResLossless)
        .await?;

    let quality_info = format!(
        "{} {}{}",
        stream_info.codecs,
        stream_info
            .sample_rate
            .map(|r| format!("{}kHz", r / 1000))
            .unwrap_or_default(),
        stream_info
            .bit_depth
            .map(|b| format!("/{}bit", b))
            .unwrap_or_default()
    );
    console.println_colored(&format!("OK ({})", quality_info), Color::Green);

    console.status("Downloading... ");

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("downloading...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let data = client.get_stream_bytes(&mut stream_info).await?;
    let size_mb = data.len() as f64 / (1024.0 * 1024.0);

    pb.finish_and_clear();
    console.println_colored(&format!("OK ({:.2} MB)", size_mb), Color::Green);

    let container = detect_container(&data);
    let ext = match container {
        ContainerKind::Flac => "flac",
        ContainerKind::Mp4 => "m4a",
    };

    let filename = format!(
        "{} - {}.{}",
        sanitize_filename(&artist_name),
        sanitize_filename(&full_title),
        ext
    );
    let output_path = output_dir.join(&filename);

    console.status("Saving... ");
    tokio::fs::write(&output_path, &data).await?;
    console.println_colored("OK", Color::Green);

    console.print("  Saved: ");
    console.println_colored(&output_path.display().to_string(), Color::Cyan);

    let lyrics_filename = format!(
        "{} - {}.lrc",
        sanitize_filename(&artist_name),
        sanitize_filename(&full_title)
    );
    let lyrics_path = output_dir.join(&lyrics_filename);
    let lyrics_content = download_lyrics(client, track.id, &lyrics_path, console).await?;

    console.status("Embedding metadata... ");
    embed_metadata(
        client,
        &output_path,
        track,
        &full_title,
        &stream_info,
        lyrics_content,
    )
    .await?;
    console.println_colored("OK", Color::Green);

    Ok(())
}

async fn download_album(
    client: &TidalClient,
    album_id: u64,
    output_dir: &PathBuf,
    console: &mut Console,
) -> AppResult<()> {
    let album = client.get_album(album_id).await?;
    let artist_name = album
        .artist
        .as_ref()
        .map(|a| a.name.clone())
        .unwrap_or_else(|| "Unknown Artist".to_string());

    console.println("");
    console.println("Album Download");
    console.println(&format!("Album:  {}", album.title));
    console.println(&format!("Artist: {}", artist_name));
    console.println(&format!("Tracks: {}", album.number_of_tracks.unwrap_or(0)));

    let album_folder = output_dir.join(sanitize_filename(&format!(
        "{} - {}",
        artist_name, album.title
    )));
    tokio::fs::create_dir_all(&album_folder).await?;

    let tracks_page = client.get_album_tracks(album_id, 100, 0).await?;
    let total = tracks_page.items.len();

    for (i, track) in tracks_page.items.iter().enumerate() {
        console.println("");
        console.println(&format!("[{}/{}]", i + 1, total));
        if let Err(e) = download_track(client, track, &album_folder, console).await {
            console.error(&format!("Failed to download: {}", e));
        }
    }

    console.println("");
    console.success("Album download complete.");
    console.print("  Location: ");
    console.println_colored(&album_folder.display().to_string(), Color::Cyan);

    Ok(())
}

async fn download_playlist(
    client: &TidalClient,
    playlist: &Playlist,
    output_dir: &PathBuf,
    console: &mut Console,
) -> AppResult<()> {
    let creator_name = playlist
        .creator
        .as_ref()
        .and_then(|c| c.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());

    console.println("");
    console.println("Playlist Download");
    console.println(&format!("Playlist: {}", playlist.title));
    console.println(&format!("Creator:  {}", creator_name));
    console.println(&format!(
        "Tracks:   {}",
        playlist.number_of_tracks.unwrap_or(0)
    ));

    let playlist_folder = output_dir.join(sanitize_filename(&playlist.title));
    tokio::fs::create_dir_all(&playlist_folder).await?;

    let mut offset = 0u32;
    let limit = 100u32;
    let mut track_num = 0usize;
    let total = playlist.number_of_tracks.unwrap_or(0) as usize;

    loop {
        let page = client
            .get_playlist_tracks(&playlist.uuid, limit, offset)
            .await?;
        if page.items.is_empty() {
            break;
        }

        for playlist_item in &page.items {
            track_num += 1;
            console.println("");
            console.println(&format!("[{}/{}]", track_num, total));
            if let Err(e) =
                download_track(client, &playlist_item.item, &playlist_folder, console).await
            {
                console.error(&format!("Failed to download: {}", e));
            }
        }

        offset += limit;
        if page.items.len() < limit as usize {
            break;
        }
    }

    console.println("");
    console.success("Playlist download complete.");
    console.print("  Location: ");
    console.println_colored(&playlist_folder.display().to_string(), Color::Cyan);

    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();
    let mut console = Console::new();

    let (content_type, id) = parse_tidal_link(&args.link)?;

    console.println("");
    console.println("tidal-dl - Tidal Music Downloader");

    let client = get_client(&mut console).await?;
    let output_dir = args
        .output
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    match content_type.as_str() {
        "track" => {
            let track_id: u64 = id.parse()?;
            let track = client.get_track(track_id).await?;
            download_track(&client, &track, &output_dir, &mut console).await?;
        }
        "album" => {
            let album_id: u64 = id.parse()?;
            download_album(&client, album_id, &output_dir, &mut console).await?;
        }
        "playlist" => {
            let playlist = client.get_playlist(&id).await?;
            download_playlist(&client, &playlist, &output_dir, &mut console).await?;
        }
        _ => {
            return Err(format!("Unsupported content type: {}", content_type).into());
        }
    }

    console.println("");
    console.success("Done.");

    Ok(())
}

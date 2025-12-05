use std::io::Write;
use std::path::PathBuf;
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

use clap::Parser;
use indicatif::{
    ProgressBar,
    ProgressStyle,
};
use regex::Regex;
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
    Playlist,
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

    #[arg(short, long)]
    lyrics: bool,
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
) -> AppResult<bool> {
    console.status("Fetching lyrics... ");

    match client.get_lyrics(track_id).await {
        Ok(lyrics) => {
            let content = lyrics.subtitles.or(lyrics.lyrics).unwrap_or_default();

            if content.is_empty() {
                console.println_colored("not available", Color::Yellow);
                return Ok(false);
            }

            tokio::fs::write(output_path, &content).await?;
            console.println_colored("OK", Color::Green);
            console.print("  Saved: ");
            console.println_colored(&output_path.display().to_string(), Color::Cyan);
            Ok(true)
        }
        Err(_) => {
            console.println_colored("not available", Color::Yellow);
            Ok(false)
        }
    }
}

async fn download_track(
    client: &TidalClient,
    track: &Track,
    output_dir: &PathBuf,
    console: &mut Console,
    download_lyrics_flag: bool,
) -> AppResult<()> {
    let artist_name = track
        .artist
        .as_ref()
        .map(|a| a.name.clone())
        .or_else(|| track.artists.first().map(|a| a.name.clone()))
        .unwrap_or_else(|| "Unknown Artist".to_string());

    let title = &track.title;
    let version = track.version.as_deref().unwrap_or("");
    let full_title = if version.is_empty() {
        title.clone()
    } else {
        format!("{} ({})", title, version)
    };

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

    let ext = stream_info.file_extension();
    let filename = format!(
        "{} - {}.{}",
        sanitize_filename(&artist_name),
        sanitize_filename(&full_title),
        ext
    );
    let output_path = output_dir.join(&filename);

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

    console.status("Saving... ");
    tokio::fs::write(&output_path, &data).await?;
    console.println_colored("OK", Color::Green);

    console.print("  Saved: ");
    console.println_colored(&output_path.display().to_string(), Color::Cyan);

    if download_lyrics_flag {
        let lyrics_filename = format!(
            "{} - {}.lrc",
            sanitize_filename(&artist_name),
            sanitize_filename(&full_title)
        );
        let lyrics_path = output_dir.join(&lyrics_filename);
        let _ = download_lyrics(client, track.id, &lyrics_path, console).await;
    }

    Ok(())
}

async fn download_album(
    client: &TidalClient,
    album_id: u64,
    output_dir: &PathBuf,
    console: &mut Console,
    download_lyrics_flag: bool,
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
        if let Err(e) =
            download_track(client, track, &album_folder, console, download_lyrics_flag).await
        {
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
    download_lyrics_flag: bool,
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
            if let Err(e) = download_track(
                client,
                &playlist_item.item,
                &playlist_folder,
                console,
                download_lyrics_flag,
            )
            .await
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
            download_track(&client, &track, &output_dir, &mut console, args.lyrics).await?;
        }
        "album" => {
            let album_id: u64 = id.parse()?;
            download_album(&client, album_id, &output_dir, &mut console, args.lyrics).await?;
        }
        "playlist" => {
            let playlist = client.get_playlist(&id).await?;
            download_playlist(&client, &playlist, &output_dir, &mut console, args.lyrics).await?;
        }
        _ => {
            return Err(format!("Unsupported content type: {}", content_type).into());
        }
    }

    console.println("");
    console.success("Done.");

    Ok(())
}

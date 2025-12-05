use std::pin::Pin;

use bytes::Bytes;
use futures::Stream;

use crate::core::AppResult;
use crate::core::api::{
    PlaybackInfo,
    TidalClient,
};
use crate::core::decrypt::{
    StreamDecryptor,
    decrypt_key_id,
};

#[derive(Debug, Clone)]
pub enum AudioQuality {
    Low,
    High,
    Lossless,
    HiRes,
    HiResLossless,
}

impl AudioQuality {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioQuality::Low => "LOW",
            AudioQuality::High => "HIGH",
            AudioQuality::Lossless => "LOSSLESS",
            AudioQuality::HiRes => "HI_RES",
            AudioQuality::HiResLossless => "HI_RES_LOSSLESS",
        }
    }
}

#[derive(Debug)]
pub struct StreamInfo {
    pub track_id: u64,
    pub urls: Vec<String>,
    pub mime_type: String,
    pub codecs: String,
    pub sample_rate: Option<u32>,
    pub bit_depth: Option<u32>,
    pub encryption: Option<StreamDecryptor>,
}

pub type BoxedByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>;

impl TidalClient {
    pub async fn get_stream_info(
        &self,
        track_id: u64,
        quality: AudioQuality,
    ) -> AppResult<StreamInfo> {
        let playback_info = self.get_playback_info(track_id, quality.as_str()).await?;
        self.parse_stream_info(playback_info)
    }

    fn parse_stream_info(&self, playback_info: PlaybackInfo) -> AppResult<StreamInfo> {
        match playback_info.manifest_mime_type.as_str() {
            "application/vnd.tidal.bts" => {
                let manifest = self.decode_bts_manifest(&playback_info)?;
                let encryption = match manifest.encryption_type.as_str() {
                    "OLD_AES" => {
                        let key_id = manifest.key_id.as_ref().ok_or("Missing keyId")?;
                        let dec_key = decrypt_key_id(key_id)?;
                        Some(StreamDecryptor::new(&dec_key))
                    }
                    "NONE" => None,
                    other => return Err(format!("Unknown encryption: {}", other).into()),
                };

                Ok(StreamInfo {
                    track_id: playback_info.track_id,
                    urls: manifest.urls,
                    mime_type: manifest.mime_type,
                    codecs: manifest.codecs,
                    sample_rate: playback_info.sample_rate,
                    bit_depth: playback_info.bit_depth,
                    encryption,
                })
            }
            "application/dash+xml" => {
                let manifest = self.decode_dash_manifest(&playback_info)?;

                Ok(StreamInfo {
                    track_id: playback_info.track_id,
                    urls: manifest.urls,
                    mime_type: manifest.mime_type,
                    codecs: manifest.codecs,
                    sample_rate: playback_info.sample_rate,
                    bit_depth: playback_info.bit_depth,
                    encryption: None,
                })
            }
            other => Err(format!("Unknown manifest type: {}", other).into()),
        }
    }

    pub async fn get_stream_bytes(&self, stream_info: &mut StreamInfo) -> AppResult<Vec<u8>> {
        let client = reqwest::Client::new();
        let mut data = Vec::new();

        for url in &stream_info.urls {
            let resp = client.get(url).send().await?;
            let mut bytes = resp.bytes().await?.to_vec();

            if let Some(ref mut decryptor) = stream_info.encryption {
                decryptor.decrypt(&mut bytes);
            }

            data.extend(bytes);
        }

        Ok(data)
    }

    pub async fn download_track(
        &self,
        track_id: u64,
        quality: AudioQuality,
        output_path: &str,
    ) -> AppResult<()> {
        use tokio::io::AsyncWriteExt;

        let mut stream_info = self.get_stream_info(track_id, quality).await?;
        let data = self.get_stream_bytes(&mut stream_info).await?;

        let mut file = tokio::fs::File::create(output_path).await?;
        file.write_all(&data).await?;
        file.flush().await?;

        Ok(())
    }
}

impl StreamInfo {
    pub fn file_extension(&self) -> &'static str {
        match self.codecs.as_str() {
            "flac" => "flac",
            "mp4a.40.2" | "mp4a.40.5" => "m4a",
            _ if self.mime_type.contains("flac") => "flac",
            _ => "m4a",
        }
    }

    pub fn is_lossless(&self) -> bool {
        self.codecs == "flac" || self.mime_type.contains("flac")
    }
}

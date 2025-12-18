use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use quick_xml::Reader;
use quick_xml::events::Event;

use super::client::TidalClient;
use super::models::{
    BtsManifest,
    DashManifest,
    PlaybackInfo,
};
use crate::core::error::{
    Result,
    TidalError,
};

impl TidalClient {
    pub async fn get_playback_info(
        &mut self,
        track_id: u64,
        quality: &str,
    ) -> Result<PlaybackInfo> {
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

    pub fn decode_bts_manifest(&self, playback_info: &PlaybackInfo) -> Result<BtsManifest> {
        let decoded = BASE64.decode(&playback_info.manifest)?;
        let manifest_str = String::from_utf8(decoded)?;
        Ok(serde_json::from_str(&manifest_str)?)
    }

    pub fn decode_dash_manifest(&self, playback_info: &PlaybackInfo) -> Result<DashManifest> {
        let decoded = BASE64.decode(&playback_info.manifest)?;
        let manifest_str = String::from_utf8(decoded)?;
        parse_mpd(&manifest_str)
    }
}

fn decode_xml_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

pub fn parse_mpd(mpd_string: &str) -> Result<DashManifest> {
    let mut reader = Reader::from_str(mpd_string);
    let mut urls: Vec<String> = Vec::new();
    let mut mime_type = String::new();
    let mut codecs = String::new();
    let mut in_segment_timeline = false;
    let mut initialization_url: Option<String> = None;
    let mut media_template: Option<String> = None;
    let mut segment_durations: Vec<(u64, u32)> = Vec::new();
    let mut base_url: Option<String> = None;
    let mut in_base_url = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => match e.name().as_ref() {
                b"BaseURL" => {
                    in_base_url = true;
                }
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
                                let raw_url = String::from_utf8_lossy(&attr.value).to_string();
                                initialization_url = Some(decode_xml_entities(&raw_url));
                            }
                            b"media" => {
                                let raw_url = String::from_utf8_lossy(&attr.value).to_string();
                                media_template = Some(decode_xml_entities(&raw_url));
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
            Ok(Event::Text(ref e)) => {
                if in_base_url {
                    let text = String::from_utf8_lossy(e.as_ref()).trim().to_string();
                    if !text.is_empty() {
                        base_url = Some(decode_xml_entities(&text));
                    }
                }
            }
            Ok(Event::End(ref e)) => match e.name().as_ref() {
                b"SegmentTimeline" => in_segment_timeline = false,
                b"BaseURL" => in_base_url = false,
                _ => {}
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(TidalError::Xml(e.to_string())),
            _ => {}
        }
    }

    let base = base_url.unwrap_or_default();

    if let Some(init_url) = initialization_url {
        let full_url = if init_url.starts_with("http") {
            init_url
        } else {
            format!("{}{}", base, init_url)
        };
        urls.push(full_url);
    }

    if let Some(media) = media_template {
        let mut segment_number = 1u32;
        for (_duration, count) in segment_durations {
            for _ in 0..count {
                let segment_path = media.replace("$Number$", &segment_number.to_string());
                let full_url = if segment_path.starts_with("http") {
                    segment_path
                } else {
                    format!("{}{}", base, segment_path)
                };
                urls.push(full_url);
                segment_number += 1;
            }
        }
    }

    if urls.is_empty() {
        return Err(TidalError::Manifest(
            "No URLs found in DASH manifest".into(),
        ));
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

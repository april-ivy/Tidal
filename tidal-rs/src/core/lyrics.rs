use std::cmp::Ordering;
use std::time::Duration;

use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug, Clone)]
pub struct SyncedLyrics {
    pub lines: Vec<LyricLine>,
}

#[derive(Debug, Clone)]
pub struct LyricLine {
    pub time: Duration,
    pub text: String,
}

impl SyncedLyrics {
    pub fn parse(content: &str) -> Option<Self> {
        let content = content.trim();

        if content.starts_with('[') {
            return Self::parse_lrc(content);
        }

        if content.contains("<tt") || content.contains("<p ") {
            return Self::parse_ttml(content);
        }

        None
    }

    fn parse_lrc(content: &str) -> Option<Self> {
        let mut lines = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(bracket_end) = line.find(']') {
                let timestamp = &line[1..bracket_end];
                let text = line[bracket_end + 1..].trim().to_string();

                if let Some(time) = parse_lrc_timestamp(timestamp) {
                    if !text.is_empty() {
                        lines.push(LyricLine { time, text });
                    }
                }
            }
        }

        if lines.is_empty() {
            return None;
        }

        lines.sort_by(|a, b| a.time.cmp(&b.time));
        Some(SyncedLyrics { lines })
    }

    fn parse_ttml(content: &str) -> Option<Self> {
        let mut reader = Reader::from_str(content);
        reader.config_mut().trim_text(true);

        let mut lines = Vec::new();
        let mut current_begin: Option<Duration> = None;
        let mut current_text = String::new();
        let mut in_p_element = false;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() == b"p" {
                        in_p_element = true;
                        current_text.clear();

                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"begin" {
                                let value = String::from_utf8_lossy(&attr.value);
                                current_begin = parse_ttml_timestamp(&value);
                            }
                        }

                        if matches!(reader.read_event(), Ok(Event::Empty(_))) {
                            in_p_element = false;
                        }
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if in_p_element {
                        let text = String::from_utf8_lossy(e.as_ref());
                        current_text.push_str(&text);
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"p" {
                        in_p_element = false;

                        let text = current_text.trim().to_string();
                        if let Some(time) = current_begin.take() {
                            if !text.is_empty() {
                                lines.push(LyricLine { time, text });
                            }
                        }
                        current_text.clear();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("TTML parse error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        if lines.is_empty() {
            return None;
        }

        lines.sort_by(|a, b| a.time.cmp(&b.time));
        Some(SyncedLyrics { lines })
    }

    pub fn line_at(&self, position: Duration) -> Option<&LyricLine> {
        if self.lines.is_empty() {
            return None;
        }

        let idx = self.find_line_index(position)?;
        Some(&self.lines[idx])
    }

    pub fn line_index_at(&self, position: Duration) -> Option<usize> {
        self.find_line_index(position)
    }

    fn find_line_index(&self, position: Duration) -> Option<usize> {
        if self.lines.is_empty() {
            return None;
        }

        let idx = match self.lines.binary_search_by(|line| {
            if line.time <= position {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };

        if self.lines[idx].time <= position {
            Some(idx)
        } else {
            None
        }
    }

    pub fn context_at(
        &self,
        position: Duration,
        before: usize,
        after: usize,
    ) -> Vec<(bool, &LyricLine)> {
        let Some(current_idx) = self.line_index_at(position) else {
            return self
                .lines
                .iter()
                .take(after + 1)
                .map(|line| (false, line))
                .collect();
        };

        let start = current_idx.saturating_sub(before);
        let end = (current_idx + after + 1).min(self.lines.len());

        self.lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| (start + i == current_idx, line))
            .collect()
    }
}

fn parse_lrc_timestamp(s: &str) -> Option<Duration> {
    let parts: Vec<&str> = s.split(|c| c == ':' || c == '.').collect();

    match parts.len() {
        2 => {
            let mins: u64 = parts[0].parse().ok()?;
            let secs: u64 = parts[1].parse().ok()?;
            Some(Duration::from_secs(mins * 60 + secs))
        }
        3 => {
            let mins: u64 = parts[0].parse().ok()?;
            let secs: u64 = parts[1].parse().ok()?;
            let centis: u64 = parts[2].parse().ok()?;
            Some(Duration::from_millis(
                mins * 60_000 + secs * 1000 + centis * 10,
            ))
        }
        _ => None,
    }
}

fn parse_ttml_timestamp(s: &str) -> Option<Duration> {
    let parts: Vec<&str> = s.split(':').collect();

    match parts.len() {
        2 => {
            let mins: u64 = parts[0].parse().ok()?;
            let (secs, millis) = parse_seconds_millis(parts[1])?;
            Some(Duration::from_millis(mins * 60_000 + secs * 1000 + millis))
        }
        3 => {
            let hours: u64 = parts[0].parse().ok()?;
            let mins: u64 = parts[1].parse().ok()?;
            let (secs, millis) = parse_seconds_millis(parts[2])?;
            Some(Duration::from_millis(
                hours * 3_600_000 + mins * 60_000 + secs * 1000 + millis,
            ))
        }
        _ => None,
    }
}

fn parse_seconds_millis(s: &str) -> Option<(u64, u64)> {
    if let Some((secs_str, millis_str)) = s.split_once('.') {
        let secs: u64 = secs_str.parse().ok()?;
        let millis_str = format!("{:0<3}", &millis_str[..millis_str.len().min(3)]);
        let millis: u64 = millis_str.parse().ok()?;
        Some((secs, millis))
    } else {
        let secs: u64 = s.parse().ok()?;
        Some((secs, 0))
    }
}

pub struct LyricsDisplay {
    lyrics: SyncedLyrics,
    last_index: Option<usize>,
}

impl LyricsDisplay {
    pub fn new(lyrics: SyncedLyrics) -> Self {
        Self {
            lyrics,
            last_index: None,
        }
    }

    pub fn update(&mut self, position: Duration) -> Option<&str> {
        let current_index = self.lyrics.line_index_at(position);

        if current_index != self.last_index {
            self.last_index = current_index;
            current_index.map(|i| self.lyrics.lines[i].text.as_str())
        } else {
            None
        }
    }

    pub fn current(&self, position: Duration) -> Option<&str> {
        self.lyrics.line_at(position).map(|l| l.text.as_str())
    }

    pub fn lyrics(&self) -> &SyncedLyrics {
        &self.lyrics
    }
}

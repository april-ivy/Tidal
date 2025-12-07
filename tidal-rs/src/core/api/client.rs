#![allow(dead_code)]
use std::time::Duration;

use serde::Deserialize;

use crate::core::auth::CLIENT_TOKEN;
use crate::core::error::{
    Result,
    TidalError,
};

pub(crate) const API_BASE: &str = "https://api.tidal.com/v1";
pub(crate) const LISTEN_API_BASE: &str = "https://listen.tidal.com/v1";
pub(crate) const SUGGESTIONS_BASE: &str = "https://tidal.com/v2";

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub timeout: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub user_agent: String,
    pub client_version: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            user_agent: "TIDAL_ANDROID/1039 okhttp/3.14.9".to_string(),
            client_version: None,
        }
    }
}

impl ClientConfig {
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_retries(mut self, max_retries: u32, delay: Duration) -> Self {
        self.max_retries = max_retries;
        self.retry_delay = delay;
        self
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn with_client_version(mut self, version: impl Into<String>) -> Self {
        self.client_version = Some(version.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct TidalClient {
    pub(crate) client: reqwest::Client,
    pub access_token: String,
    pub refresh_token: String,
    pub country_code: String,
    pub user_id: Option<u64>,
    pub(crate) config: ClientConfig,
}

impl TidalClient {
    pub fn new(access_token: String, refresh_token: String, country_code: String) -> Self {
        Self::with_config(
            access_token,
            refresh_token,
            country_code,
            ClientConfig::default(),
        )
    }

    pub fn with_config(
        access_token: String,
        refresh_token: String,
        country_code: String,
        config: ClientConfig,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            access_token,
            refresh_token,
            country_code,
            user_id: None,
            config,
        }
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub(crate) fn headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
            "X-Tidal-Token",
            CLIENT_TOKEN
                .parse()
                .map_err(|_| TidalError::Auth("Invalid client token".into()))?,
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.access_token)
                .parse()
                .map_err(|_| TidalError::Auth("Invalid access token".into()))?,
        );
        headers.insert(reqwest::header::ACCEPT_ENCODING, "gzip".parse().unwrap());
        headers.insert(
            reqwest::header::USER_AGENT,
            self.config.user_agent.parse().unwrap(),
        );

        if let Some(ref version) = self.config.client_version {
            headers.insert(
                "x-tidal-client-version",
                version
                    .parse()
                    .map_err(|_| TidalError::Auth("Invalid client version".into()))?,
            );
        }

        Ok(headers)
    }

    pub(crate) async fn get_with_retry<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> Result<T> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                tokio::time::sleep(self.config.retry_delay * attempt).await;
            }

            match self.get_once::<T>(url).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if matches!(e, TidalError::Network(_)) && attempt < self.config.max_retries {
                        last_error = Some(e);
                        continue;
                    }
                    return Err(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| TidalError::Api {
            status: 0,
            message: "Max retries exceeded".into(),
        }))
    }

    async fn get_once<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T> {
        let resp = self.client.get(url).headers(self.headers()?).send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(TidalError::Api {
                status: status.as_u16(),
                message: text[..text.len().min(200)].to_string(),
            });
        }

        Ok(serde_json::from_str(&text)?)
    }

    pub(crate) async fn get<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T> {
        self.get_with_retry(url).await
    }

    pub(crate) async fn post<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        body: Option<&str>,
    ) -> Result<T> {
        let mut req = self.client.post(url).headers(self.headers()?);
        if let Some(b) = body {
            req = req
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(b.to_string());
        }
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(TidalError::Api {
                status: status.as_u16(),
                message: text[..text.len().min(200)].to_string(),
            });
        }

        Ok(serde_json::from_str(&text)?)
    }

    pub(crate) async fn post_empty(&self, url: &str, body: Option<&str>) -> Result<()> {
        let mut req = self.client.post(url).headers(self.headers()?);
        if let Some(b) = body {
            req = req
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(b.to_string());
        }
        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(TidalError::Api {
                status: status.as_u16(),
                message: text[..text.len().min(200)].to_string(),
            });
        }

        Ok(())
    }

    pub(crate) async fn put_empty(&self, url: &str, body: Option<&str>) -> Result<()> {
        let mut req = self.client.put(url).headers(self.headers()?);
        if let Some(b) = body {
            req = req
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(b.to_string());
        }
        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(TidalError::Api {
                status: status.as_u16(),
                message: text[..text.len().min(200)].to_string(),
            });
        }

        Ok(())
    }

    pub(crate) async fn delete_empty(&self, url: &str) -> Result<()> {
        let resp = self
            .client
            .delete(url)
            .headers(self.headers()?)
            .send()
            .await?;
        let status = resp.status();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(TidalError::Api {
                status: status.as_u16(),
                message: text[..text.len().min(200)].to_string(),
            });
        }

        Ok(())
    }

    pub(crate) fn api_url(&self, path: &str, extra_params: &[(&str, &str)]) -> String {
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

    pub(crate) fn listen_url(&self, path: &str, extra_params: &[(&str, &str)]) -> String {
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

    pub(crate) fn pages_url(&self, path: &str, extra_params: &[(&str, &str)]) -> String {
    let mut params = vec![
        ("countryCode", self.country_code.as_str()),
        ("locale", "en_US"),
        ("deviceType", "BROWSER"),
    ];
    params.extend_from_slice(extra_params);

    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let separator = if path.contains('?') { "&" } else { "?" };
    format!("https://tidal.com/v1/pages/{}{}{}", path, separator, query)
}

    pub(crate) fn suggestions_url(&self, query: &str, explicit: bool, hybrid: bool) -> String {
        format!(
            "{}/suggestions/?countryCode={}&explicit={}&hybrid={}&query={}",
            SUGGESTIONS_BASE,
            self.country_code,
            explicit,
            hybrid,
            urlencoding::encode(query)
        )
    }
}

use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::core::error::{
    Result,
    TidalError,
};

const TV_TOKEN: &str = "7m7Ap0JC9j1cOM3n";
const TV_SECRET: &str = "vRAdA108tlvkJpTsGZS8rGZ7xTlbJ0qaZ2K9saEzsgY=";
const SCOPES: &str = "r_usr w_usr";

pub const CLIENT_TOKEN: &str = TV_TOKEN;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
    pub user_id: Option<u64>,
    pub country_code: String,
}

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub client_unique_key: String,
    client: reqwest::Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceAuthResponse {
    #[serde(rename = "deviceCode")]
    pub device_code: String,
    #[serde(rename = "userCode")]
    pub user_code: String,
    #[serde(rename = "verificationUri")]
    pub verification_uri: String,
    #[serde(rename = "verificationUriComplete")]
    pub verification_uri_complete: Option<String>,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
pub struct TokenErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
}

impl AuthSession {
    pub fn new() -> Self {
        Self {
            client_unique_key: Uuid::new_v4().to_string(),
            client: reqwest::Client::new(),
        }
    }

    fn format_url(url: &str) -> String {
        if url.starts_with("http") {
            url.to_string()
        } else {
            format!("https://{}", url)
        }
    }

    pub async fn start_device_auth(&self) -> Result<DeviceAuthResponse> {
        let resp = self
            .client
            .post("https://auth.tidal.com/v1/oauth2/device_authorization")
            .form(&[("client_id", TV_TOKEN), ("scope", SCOPES)])
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(TidalError::Auth(format!("device auth failed: {}", text)));
        }

        let mut parsed: DeviceAuthResponse = serde_json::from_str(&text)?;
        parsed.verification_uri = Self::format_url(&parsed.verification_uri);
        parsed.verification_uri_complete = parsed
            .verification_uri_complete
            .map(|u| Self::format_url(&u));

        Ok(parsed)
    }

    pub async fn poll_for_token(&self, device_code: &str, interval: u64) -> Result<TokenResponse> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;

            let resp = self
                .client
                .post("https://auth.tidal.com/v1/oauth2/token")
                .form(&[
                    ("client_id", TV_TOKEN),
                    ("client_secret", TV_SECRET),
                    ("device_code", device_code),
                    ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                    ("scope", SCOPES),
                ])
                .send()
                .await?;

            let status = resp.status();
            let text = resp.text().await?;

            if status.is_success() {
                return Ok(serde_json::from_str(&text)?);
            }

            if let Ok(err) = serde_json::from_str::<TokenErrorResponse>(&text) {
                match err.error.as_str() {
                    "authorization_pending" => continue,
                    "slow_down" => tokio::time::sleep(tokio::time::Duration::from_secs(5)).await,
                    "expired_token" => return Err(TidalError::Auth("code expired".into())),
                    "access_denied" => return Err(TidalError::Auth("access denied".into())),
                    _ => {
                        return Err(TidalError::Auth(format!(
                            "{}: {:?}",
                            err.error, err.error_description
                        )));
                    }
                }
            }
        }
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse> {
        let resp = self
            .client
            .post("https://auth.tidal.com/v1/oauth2/token")
            .form(&[
                ("client_id", TV_TOKEN),
                ("client_secret", TV_SECRET),
                ("refresh_token", refresh_token),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(TidalError::Auth(format!("refresh failed: {}", text)));
        }

        Ok(serde_json::from_str(&text)?)
    }
}

impl Default for AuthSession {
    fn default() -> Self {
        Self::new()
    }
}

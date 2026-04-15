use std::time::Duration;

use reqwest::Client;
use reqwest::Url;
use sha2::{Sha256, Digest};

use crate::APP_USER_AGENT;
use crate::RsError;

pub enum Auth {
    UserKey { user: String, key: String },
    SessionKey { user: String, key: String },
}

pub struct RsClient {
    pub(crate) base_url: Url,
    pub(crate) auth: Auth,
    pub(crate) client: Client,
}

pub struct ClientBuilder {
    base_url: Option<Url>,
    auth: Option<Auth>,
    credentials: Option<(String, String)>,
    client: Option<Client>,
}

fn sign(key: &str, query: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update(query.as_bytes());
    hex::encode(hasher.finalize())
}

async fn login(http: &Client, base_url: &Url, user: &str, password: &str) -> Result<String, RsError> {
    let url = format!("{}api/?function=login&username={}&password={}", base_url, user, password);

    let response = http
        .get(&url)
        .send()
        .await
        .map_err(RsError::Http)?
        .text()
        .await
        .map_err(RsError::Http)?;

    if response.trim().to_lowercase() == "false" {
        return Err(RsError::Api { status: 401, message: "Invalid credentials".into() });
    }

    Ok(response.trim().trim_matches('"').to_string())
}

impl RsClient {
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            base_url: None,
            auth: None,
            credentials: None,
            client: None,
        }
    }

    pub async fn send_request(&self, function: &str, params: &[(&str, &str)]) -> Result<serde_json::Value, RsError> {
        let (
            user,
            key,
            authmode
        ): (&String, &String, Option<&str>) = match &self.auth {
            Auth::UserKey { user, key } => (user, key, Some("userkey")),
            Auth::SessionKey { user, key } => (user, key, Some("sessionkey")),
        };

        // Build query string
        let mut query = format!("user={}&function={}", user, function);
        for (k, v) in params {
            query.push_str(&format!("&{}={}", k, v));
        }

        let signature = sign(key, &query);
        let mut full_url = format!("{}api/?{}&sign={}", self.base_url, query, signature);

        if let Some(mode) = authmode {
            full_url.push_str(&format!("&authmode={}", mode));
        }

        let response = self.client
            .get(&full_url)
            .send()
            .await
            .map_err(RsError::Http)?;

        let text = response
            .text()
            .await
            .map_err(RsError::Http)?;

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|_| RsError::Other(format!("Unexpected response: {}", text)))?;

        if let Some(status) = json.get("status").and_then(|s| s.as_u64()) {
            if status != 200 {
                let message = json.get("body")
                    .and_then(|b| b.as_str())
                    .unwrap_or("Unknown error")
                    .trim()
                    .to_string();
                return Err(RsError::Api { status: status as u16, message });
            }
        }
        Ok(json)
    }
}

impl ClientBuilder {
    pub fn base_url(mut self, url: &str) -> Result<Self, RsError> {
        self.base_url = Some(
            Url::parse(url)
                .map_err(|e| RsError::Other(e.to_string()))?
        );
        Ok(self)
    }

    pub fn user_key(mut self, user: &str, key: &str) -> Self {
        self.auth = Some(Auth::UserKey {
            user: user.to_string(),
            key: key.to_string(),
        });
        self
    }

    pub fn session_key(mut self, user: &str, password: &str) -> Self {
        self.credentials = Some((user.to_string(), password.to_string()));
        self
    }

    pub async fn build(self) -> Result<RsClient, RsError> {
        let base_url = self.base_url.ok_or(RsError::Other("missing base_url".into()))?;

        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .user_agent(APP_USER_AGENT)
            .build()?;

        let auth = if let Some((user, password)) = self.credentials {
            let key = login(&http, &base_url, &user, &password).await?;
            Auth::SessionKey { user, key }
        } else {
            self.auth.ok_or(RsError::Other("missing auth".into()))?
        };

        Ok(RsClient { base_url, auth, client: http })
    }
}

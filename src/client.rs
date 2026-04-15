use std::time::Duration;

use reqwest::Client;
use reqwest::Url;
use sha2::{Sha256, Digest};

use crate::APP_USER_AGENT;
use crate::RsError;
use crate::auth::{Auth, NoAuth, WithUserKey, WithSessionKey, login};

pub struct NoUrl;
pub struct WithUrl(Url);


pub struct RsClient {
    base_url: Url,
    auth: Auth,
    client: Client,
}

pub struct ClientBuilder<U = NoUrl, A = NoAuth> {
    base_url: U,
    auth: A,
}

impl RsClient {
    pub fn builder() -> ClientBuilder<NoUrl, NoAuth> {
        ClientBuilder {
            base_url: NoUrl,
            auth: NoAuth,
        }
    }

    pub async fn send_request(&self, function: &str, params: &[(&str, &str)]) -> Result<serde_json::Value, RsError> {
        let (
            user,
            key,
            authmode
        ): (&String, &String, &str) = match &self.auth {
            Auth::UserKey { user, key } => (user, key, "userkey"),
            Auth::SessionKey { user, key } => (user, key, "sessionkey"),
        };

        // Build query string
        let mut query = format!("user={}&function={}", user, function);
        for (k, v) in params {
            query.push_str(&format!("&{}={}", k, v));
        }

        let signature = sign(key, &query);
        let full_url = format!("{}api/?{}&sign={}&authmode={}", self.base_url, query, signature, authmode);

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

impl<A> ClientBuilder<NoUrl, A> {
    pub fn base_url(
        self, 
        url: impl Into<String>
    ) -> Result<ClientBuilder<WithUrl, A>, RsError> {
        let url = url.into();
        let parsed_url = Url::parse(&url)
            .map_err(|e| RsError::Other(e.to_string()))?;

        Ok(ClientBuilder {
            base_url: WithUrl(parsed_url),
            auth: self.auth,
        })
    }
}

impl<U> ClientBuilder<U, NoAuth> {
    pub fn user_key(
        self,
        user: impl Into<String>,
        key: impl Into<String>
    ) -> ClientBuilder<U, WithUserKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: WithUserKey { user: user.into(), key: key.into() },
        }
    }

    pub fn session_key(
        self,
        user: impl Into<String>,
        password: impl Into<String>
    ) -> ClientBuilder<U, WithSessionKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: WithSessionKey { user: user.into(), password: password.into() },
        }
    }
}

impl ClientBuilder<WithUrl, WithSessionKey> {
    pub async fn build(self) -> Result<RsClient, RsError> {
        let http = make_client()?;
        let session_key = login(&http, &self.base_url.0, &self.auth.user, &self.auth.password).await?;
        let auth = Auth::SessionKey { 
            user: self.auth.user,
            key: session_key
        };

        Ok(RsClient { base_url: self.base_url.0, auth, client: http })
    }

}

impl ClientBuilder<WithUrl, WithUserKey> {
    pub async fn build(self) -> Result<RsClient, RsError> {
        let http = make_client()?;
        let auth = Auth::UserKey { 
            user: self.auth.user,
            key: self.auth.key 
        };

        Ok(RsClient { base_url: self.base_url.0, auth, client: http })
    }
}

fn sign(key: &str, query: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update(query.as_bytes());
    hex::encode(hasher.finalize())
}

fn make_client() -> Result<Client, RsError> {
    Ok(Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .user_agent(APP_USER_AGENT)
        .build()?)
}


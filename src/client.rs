use std::time::Duration;
use reqwest::Client;
use serde::{Serialize};
use url::Url;
use sha2::{Sha256, Digest};

use crate::APP_USER_AGENT;
use crate::error::RsError;
use crate::auth::{Auth, login};

// Typestates
mod private {
    pub struct NoUrl;
    pub struct WithUrl(pub(crate) url::Url);
    pub struct NoAuth;
    pub struct WithUserKey { 
        pub(crate) user: String, 
        pub(crate) key: String 
    }
    pub struct WithSessionKey { 
        pub(crate) user: String, 
        pub(crate) password: String 
    }
}

#[derive(Serialize)]
pub(crate) struct ApiRequest<'a, P: Serialize> {
    pub(crate) user: &'a str,
    #[serde(rename = "function")]
    pub(crate) function: &'a str,
    #[serde(flatten)]
    pub(crate) params: P,
}

pub(crate) fn build_query<P: Serialize>(params: &P) -> Result<String, RsError> {
    serde_qs::Config::new()
        .use_form_encoding(true)
        .serialize_string(params)
        .map_err(|e| RsError::Other(format!("Failed to serialize request: {}", e)))
}

pub struct RsClient {
    base_url: Url,
    auth: Auth,
    client: Client,
}

impl RsClient {
    pub fn builder() -> ClientBuilder<private::NoUrl, private::NoAuth> {
        ClientBuilder {
            base_url: private::NoUrl,
            auth: private::NoAuth,
        }
    }

    pub(crate) async fn send_request<P>(
        &self,
        function: &str,
        method: reqwest::Method,
        params: P
    ) -> Result<serde_json::Value, RsError> 
    where P: Serialize
    {
        let (
            user,
            key,
            authmode
        ): (&String, &String, &str) = match &self.auth {
            Auth::UserKey { user, key } => (user, key, "userkey"),
            Auth::SessionKey { user, key } => (user, key, "sessionkey"),
        };

        // Build query string
        let req = ApiRequest { user, function, params };
        let query = build_query(&req)?;
        let signature = sign(key, &query);

        let response = match method {
            reqwest::Method::GET => {
                let full_url = format!("{}api/?{}&sign={}&authmode={}", self.base_url, query, signature, authmode);
                self.client
                    .get(&full_url)
                    .send()
                    .await
            }
            reqwest::Method::POST => {
                // TODO: multipart support
                let full_url = format!("{}api/", self.base_url);
                self.client
                    .post(&full_url)
                    .form(&[
                        ("user", user.clone()),
                        ("query", query),
                        ("sign", signature),
                        ("authmode", authmode.to_string())
                    ])
                    .send()
                    .await
            }
            _ => return Err(RsError::Other("Unsupported HTTP method".into())),
        }.map_err(RsError::Http)?;

        // TODO: response handling is very inconsistent due to the nature of RS responses:
        // some endpoints return JSON with status codes, some plain text, some error with 200 status code, etc.
        // for now just try to parse and hope for the best. Montala stated they are working on an OpenAPI spec
        // for the api which should allow for much better handling in the future

        // So, for now, responses can be:
        // - JSON arrays
        // - JSON objects
        // - Plain true/false strings
        // - Raw integers (resource IDs)
        // - "FAILED: ..." strings for certain errors, even with 200 status code
        // - "Invalid signature" strings, even with 200 status code

        // 1. check HTTP status before touching the body
        if !response.status().is_success() {
            return Err(RsError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let text = response.text().await.map_err(RsError::Http)?;
        let trimmed = text.trim();

        // 2. RS returns plain "false" for failed operations
        if trimmed.eq_ignore_ascii_case("false") {
            return Err(RsError::OperationFailed);
        }

        // 3. RS returns "FAILED: ..." strings from upload functions
        if let Some(msg) = trimmed.strip_prefix("FAILED:") {
            return Err(RsError::Api {
                status: 400,
                message: msg.trim().to_string(),
            });
        }

        // 4. Try to parse as JSON, fall back to wrapping as a JSON string
        // This handles plain integers (create_resource), "true", and error strings
        let json: serde_json::Value = serde_json::from_str(trimmed)
            .unwrap_or_else(|_| serde_json::Value::String(trimmed.to_string()));

        Ok(json)
    }

    // Sub-APIs
    pub fn search(&self) -> crate::api::search::SearchApi<'_> {
        crate::api::search::SearchApi::new(self)
    }
    pub fn system(&self) -> crate::api::system::SystemApi<'_> {
        crate::api::system::SystemApi::new(self)
    }
    pub fn message(&self) -> crate::api::message::MessageApi<'_> {
        crate::api::message::MessageApi::new(self)
    }
    pub fn metadata(&self) -> crate::api::metadata::MetadataApi<'_> {
        crate::api::metadata::MetadataApi::new(self)
    }
    pub fn user(&self) -> crate::api::user::UserApi<'_> {
        crate::api::user::UserApi::new(self)
    }
}

pub struct ClientBuilder<U = private::NoUrl, A = private::NoAuth> {
    base_url: U,
    auth: A,
}

impl<A> ClientBuilder<private::NoUrl, A> {
    pub fn base_url(
        self, 
        url: impl Into<String>
    ) -> Result<ClientBuilder<private::WithUrl, A>, RsError> {
        let url = url.into();
        let parsed_url = Url::parse(&url)
            .map_err(|e| RsError::Other(e.to_string()))?;

        Ok(ClientBuilder {
            base_url: private::WithUrl(parsed_url),
            auth: self.auth,
        })
    }
}

impl<U> ClientBuilder<U, private::NoAuth> {
    pub fn user_key(
        self,
        user: impl Into<String>,
        key: impl Into<String>
    ) -> ClientBuilder<U, private::WithUserKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: private::WithUserKey { user: user.into(), key: key.into() },
        }
    }

    pub fn session_key(
        self,
        user: impl Into<String>,
        password: impl Into<String>
    ) -> ClientBuilder<U, private::WithSessionKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: private::WithSessionKey { user: user.into(), password: password.into() },
        }
    }
}

impl ClientBuilder<private::WithUrl, private::WithSessionKey> {
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

impl ClientBuilder<private::WithUrl, private::WithUserKey> {
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


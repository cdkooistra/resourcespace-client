use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::Duration;
use url::Url;

use crate::APP_USER_AGENT;
use crate::auth::{Auth, login};
use crate::error::RsError;

// Typestates
mod private {
    use secrecy::SecretString;

    pub struct NoUrl;
    pub struct WithUrl(pub(crate) String);
    pub struct NoAuth;
    pub struct WithUserKey {
        pub(crate) user: String,
        pub(crate) key: SecretString,
    }
    pub struct WithSessionKey {
        pub(crate) user: String,
        pub(crate) password: SecretString,
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

struct PreparedRequest {
    user: String,
    query: String,
    signature: String,
    authmode: String,
}

pub(crate) fn build_query<P: Serialize>(params: &P) -> Result<String, RsError> {
    serde_qs::Config::new()
        .use_form_encoding(true)
        .serialize_string(params)
        .map_err(|e| RsError::Other(format!("Failed to serialize request: {}", e)))
}

/// some endpoints return JSON with status codes, some plain text, some error with 200 status code, etc.
/// for now just try to parse and hope for the best. Montala stated they are working on an OpenAPI spec
/// for the api which should allow for much better handling in the future.
/// So, for now, responses can be:
/// - JSON arrays
/// - JSON objects
/// - Plain true/false strings
/// - Raw integers (resource IDs)
/// - "FAILED: ..." strings for certain errors, even with 200 status code
/// - "Invalid signature" strings, even with 200 status code
#[derive(Debug)]
pub struct Client {
    api_url: Url,
    auth: Auth,
    client: reqwest::Client,
}

impl Client {
    #[must_use]
    pub fn builder() -> ClientBuilder<private::NoUrl, private::NoAuth> {
        ClientBuilder {
            base_url: private::NoUrl,
            auth: private::NoAuth,
            timeout: None,
            connect_timeout: None,
            user_agent: None,
        }
    }

    fn prepare_request<P>(&self, function: &str, params: P) -> Result<PreparedRequest, RsError>
    where
        P: Serialize,
    {
        let (user, key, authmode) = match &self.auth {
            Auth::UserKey { user, key } => (user, key.expose_secret(), "userkey"),
            Auth::SessionKey { user, key } => (user, key.expose_secret(), "sessionkey"),
        };

        let req = ApiRequest {
            user,
            function,
            params,
        };
        let query = build_query(&req)?;
        let signature = sign(key, &query);

        Ok(PreparedRequest {
            user: user.clone(),
            query,
            signature,
            authmode: authmode.to_string(),
        })
    }

    pub(crate) async fn send_request<P>(
        &self,
        function: &str,
        method: reqwest::Method,
        params: P,
    ) -> Result<serde_json::Value, RsError>
    where
        P: Serialize,
    {
        let request = self.prepare_request(function, params)?;
        let response = match method {
            reqwest::Method::GET => {
                let mut url = self.api_url.clone();
                url.set_query(Some(&format!(
                    "{}&sign={}&authmode={}",
                    request.query, request.signature, request.authmode
                )));
                self.client.get(url.as_str()).send().await
            }
            reqwest::Method::POST => {
                self.client
                    .post(self.api_url.as_str())
                    .form(&[
                        ("user", request.user.clone()),
                        ("query", request.query),
                        ("sign", request.signature),
                        ("authmode", request.authmode.to_string()),
                    ])
                    .send()
                    .await
            }
            _ => return Err(RsError::Other("Unsupported HTTP method".into())),
        }
        .map_err(RsError::Http)?;

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

    pub(crate) async fn send_multipart_request<P>(
        &self,
        function: &str,
        params: P,
        source: crate::api::resource::UploadSource,
    ) -> Result<serde_json::Value, RsError>
    where
        P: Serialize,
    {
        let request = self.prepare_request(function, params)?;
        let file_part = match source {
            crate::api::resource::UploadSource::File(file) => reqwest::multipart::Part::file(&file)
                .await
                .map_err(|e| RsError::Other(format!("Failed to read file: {}", e)))?,
            crate::api::resource::UploadSource::Stream { body, filename } => {
                reqwest::multipart::Part::stream(body).file_name(filename)
            }
        };

        let response = self
            .client
            .post(self.api_url.as_str()) // function is passed in the multipart request
            .multipart(
                reqwest::multipart::Form::new()
                    .text("user", request.user.clone())
                    .text("query", request.query)
                    .text("sign", request.signature)
                    .text("authmode", request.authmode.to_string())
                    .part("file", file_part),
            )
            .send()
            .await
            .map_err(RsError::Http)?;

        if !response.status().is_success() {
            return Err(RsError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let text = response.text().await.map_err(RsError::Http)?;

        Ok(json!(text))
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
    pub fn collection(&self) -> crate::api::collection::CollectionApi<'_> {
        crate::api::collection::CollectionApi::new(self)
    }
    pub fn resource(&self) -> crate::api::resource::ResourceApi<'_> {
        crate::api::resource::ResourceApi::new(self)
    }
}

pub struct ClientBuilder<U = private::NoUrl, A = private::NoAuth> {
    base_url: U,
    auth: A,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    user_agent: Option<String>,
}

impl<U, A> ClientBuilder<U, A> {
    pub fn timeout(self, timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            ..self
        }
    }
    pub fn connect_timeout(self, connect_timeout: Duration) -> Self {
        Self {
            connect_timeout: Some(connect_timeout),
            ..self
        }
    }
    pub fn user_agent(self, user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: Some(user_agent.into()),
            ..self
        }
    }

    fn build_http_client(&self) -> Result<reqwest::Client, RsError> {
        let mut builder = reqwest::Client::builder();
        if let Some(t) = self.timeout {
            builder = builder.timeout(t);
        }
        if let Some(t) = self.connect_timeout {
            builder = builder.connect_timeout(t);
        }
        if let Some(ref ua) = self.user_agent {
            builder = builder.user_agent(ua.as_str());
        } else {
            builder = builder.user_agent(APP_USER_AGENT)
        }
        Ok(builder.build()?)
    }
}

impl<A> ClientBuilder<private::NoUrl, A> {
    pub fn base_url(self, url: impl Into<String>) -> ClientBuilder<private::WithUrl, A> {
        ClientBuilder {
            base_url: private::WithUrl(url.into()),
            auth: self.auth,
            timeout: self.timeout,
            connect_timeout: self.connect_timeout,
            user_agent: self.user_agent,
        }
    }
}

impl<U> ClientBuilder<U, private::NoAuth> {
    pub fn user_key(
        self,
        user: impl Into<String>,
        key: impl Into<String>,
    ) -> ClientBuilder<U, private::WithUserKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: private::WithUserKey {
                user: user.into(),
                key: SecretString::from(key.into()),
            },
            timeout: self.timeout,
            connect_timeout: self.connect_timeout,
            user_agent: self.user_agent,
        }
    }

    pub fn session_key(
        self,
        user: impl Into<String>,
        password: impl Into<String>,
    ) -> ClientBuilder<U, private::WithSessionKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: private::WithSessionKey {
                user: user.into(),
                password: SecretString::from(password.into()),
            },
            timeout: self.timeout,
            connect_timeout: self.connect_timeout,
            user_agent: self.user_agent,
        }
    }
}

impl<A> ClientBuilder<private::WithUrl, A> {
    fn parse_url(&self) -> Result<Url, RsError> {
        let base_url = Url::parse(&self.base_url.0).map_err(|e| RsError::Other(e.to_string()))?;
        let api_url = base_url
            .join("api/")
            .map_err(|e| RsError::Other(e.to_string()))?;

        Ok(api_url)
    }
}

impl ClientBuilder<private::WithUrl, private::WithSessionKey> {
    pub async fn build(self) -> Result<Client, RsError> {
        let api_url = self.parse_url()?;
        let client = self.build_http_client()?;
        let session_key = login(
            &client,
            &api_url,
            &self.auth.user,
            self.auth.password.expose_secret(),
        )
        .await?;
        let auth = Auth::SessionKey {
            user: self.auth.user,
            key: SecretString::from(session_key),
        };

        Ok(Client {
            api_url,
            auth,
            client,
        })
    }
}

impl ClientBuilder<private::WithUrl, private::WithUserKey> {
    pub async fn build(self) -> Result<Client, RsError> {
        let api_url = self.parse_url()?;
        let client = self.build_http_client()?;
        let auth = Auth::UserKey {
            user: self.auth.user,
            key: self.auth.key,
        };

        Ok(Client {
            api_url,
            auth,
            client,
        })
    }
}

fn sign(key: &str, query: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update(query.as_bytes());
    hex::encode(hasher.finalize())
}

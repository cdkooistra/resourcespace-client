use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use std::time::Duration;
use url::Url;

use crate::APP_USER_AGENT;
use crate::auth::{Auth, login};
use crate::error::{self, Error};

// Typestates
mod state {
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

pub(crate) enum HttpMethod {
    Get,
    Post,
}

pub(crate) fn build_query<P: Serialize>(params: &P) -> String {
    serde_qs::Config::new()
        .use_form_encoding(true)
        .serialize_string(params)
        .expect("Query param serialization failed — this is a bug, please open an issue")
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
#[derive(Clone, Debug)]
pub struct Client {
    api_url: Url,
    auth: Auth,
    client: reqwest::Client,
}

impl Client {
    #[must_use]
    pub fn builder() -> ClientBuilder<state::NoUrl, state::NoAuth> {
        ClientBuilder {
            base_url: state::NoUrl,
            auth: state::NoAuth,
            timeout: None,
            connect_timeout: None,
            user_agent: None,
        }
    }

    fn prepare_request<P>(&self, function: &str, params: P) -> Result<PreparedRequest, Error>
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
        let query = build_query(&req);
        let signature = sign(key, &query);

        Ok(PreparedRequest {
            user: user.clone(),
            query,
            signature,
            authmode: authmode.to_string(),
        })
    }

    /// Send a request and deserialize the response into `T`.
    ///
    /// ResourceSpace does not reliably use HTTP status codes to signal success/failure,
    /// and the shape of a successful response body varies per-endpoint (a bare integer,
    /// a bare boolean-like string, a JSON array, a JSON object, ...). Callers pick the
    /// `T` that matches the documented/observed shape for the endpoint they're calling
    /// (e.g. `bool`, `u32`, a concrete struct, or `serde_json::Value` as an escape hatch
    /// for endpoints that aren't typed yet), and this method fails with
    /// [`Error::Deserialize`] if the response doesn't match.
    pub(crate) async fn send_request<P, T>(
        &self,
        function: &str,
        method: HttpMethod,
        params: P,
    ) -> Result<T, Error>
    where
        P: Serialize + Clone,
        T: DeserializeOwned,
    {
        let json = self.send_request_raw(function, method, params).await?;
        serde_json::from_value(json).map_err(|e| Error::Deserialize {
            function: function.to_string(),
            source: e.into(),
        })
    }

    async fn send_request_raw<P>(
        &self,
        function: &str,
        method: HttpMethod,
        params: P,
    ) -> Result<serde_json::Value, Error>
    where
        P: Serialize + Clone,
    {
        let request = self.prepare_request(function, params.clone())?;
        let response = match method {
            HttpMethod::Get => {
                let mut url = self.api_url.clone();
                url.set_query(Some(&format!(
                    "{}&sign={}&authmode={}",
                    request.query, request.signature, request.authmode
                )));
                self.client.get(url.as_str()).send().await
            }
            HttpMethod::Post => {
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
        }
        .map_err(error::transport)?;

        // 1. check HTTP status before touching the body
        if !response.status().is_success() {
            return Err(Error::Http {
                status: response.status().as_u16(),
                body: response.text().await.unwrap_or_default(),
            });
        }

        let text = response.text().await.map_err(error::transport)?;
        let trimmed = text.trim();

        // 1.5 RS returns invalid signature
        if trimmed.eq_ignore_ascii_case("invalid_signature") {
            return Err(Error::InvalidSignature);
        }

        // 2. RS returns plain "false" for failed operations
        if trimmed.eq_ignore_ascii_case("false") {
            return Err(Error::OperationFailed {
                function: function.to_string(),
                params: serde_json::to_value(&params).unwrap_or_else(|_| {
                    serde_json::Value::String(
                        "Failed to serialize params, this should never happen contact maintainers"
                            .to_string(),
                    )
                }),
            });
        }

        // 3. RS returns "FAILED: ..." strings from upload functions
        if let Some(msg) = trimmed.strip_prefix("FAILED:") {
            return Err(Error::Api {
                function: function.to_string(),
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
    ) -> Result<(), Error>
    where
        P: Serialize,
    {
        let request = self.prepare_request(function, params)?;
        let file_part = match source {
            crate::api::resource::UploadSource::File(file) => reqwest::multipart::Part::file(&file)
                .await
                .map_err(|e| Error::Io(e.into()))?,
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
            .map_err(error::transport)?;

        if response.status() != reqwest::StatusCode::NO_CONTENT {
            return Err(Error::Http {
                status: response.status().as_u16(),
                body: response.text().await.unwrap_or_default(),
            });
        }

        Ok(())
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
    pub fn plugin(&self) -> crate::api::plugin::PluginApi<'_> {
        crate::api::plugin::PluginApi::new(self)
    }
}

pub struct ClientBuilder<U = state::NoUrl, A = state::NoAuth> {
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

    fn build_http_client(&self) -> Result<reqwest::Client, Error> {
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
        builder.build().map_err(|e| Error::Client(e.into()))
    }
}

impl<A> ClientBuilder<state::NoUrl, A> {
    pub fn base_url(self, url: impl Into<String>) -> ClientBuilder<state::WithUrl, A> {
        ClientBuilder {
            base_url: state::WithUrl(url.into()),
            auth: self.auth,
            timeout: self.timeout,
            connect_timeout: self.connect_timeout,
            user_agent: self.user_agent,
        }
    }
}

impl<U> ClientBuilder<U, state::NoAuth> {
    pub fn user_key(
        self,
        user: impl Into<String>,
        key: impl Into<String>,
    ) -> ClientBuilder<U, state::WithUserKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: state::WithUserKey {
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
    ) -> ClientBuilder<U, state::WithSessionKey> {
        ClientBuilder {
            base_url: self.base_url,
            auth: state::WithSessionKey {
                user: user.into(),
                password: SecretString::from(password.into()),
            },
            timeout: self.timeout,
            connect_timeout: self.connect_timeout,
            user_agent: self.user_agent,
        }
    }
}

impl<A> ClientBuilder<state::WithUrl, A> {
    fn parse_url(&self) -> Result<Url, Error> {
        let base_url = Url::parse(&self.base_url.0).map_err(|e| Error::Url(e.into()))?;
        let api_url = base_url.join("api/").map_err(|e| Error::Url(e.into()))?;

        Ok(api_url)
    }
}

impl ClientBuilder<state::WithUrl, state::WithSessionKey> {
    pub async fn build(self) -> Result<Client, Error> {
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

impl ClientBuilder<state::WithUrl, state::WithUserKey> {
    pub async fn build(self) -> Result<Client, Error> {
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

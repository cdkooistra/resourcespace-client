use reqwest::Client;
use secrecy::SecretString;
use serde::Serialize;
use url::Url;

use crate::client::{ApiRequest, build_query};
use crate::error::RsError;

/// For a ResourceSpace external client we can only communicate with a
/// userkey or a sessionkey. `native` authmode is only available for
/// client side API calls -> browser initiated activity.
#[derive(Debug)]
pub(crate) enum Auth {
    UserKey { user: String, key: SecretString },
    SessionKey { user: String, key: SecretString },
}

#[derive(Serialize)]
struct LoginParams<'a> {
    username: &'a str,
    password: &'a str,
}

// TODO: if this is truely internal, just swap to &str
pub(crate) async fn login(
    http: &Client,
    base_url: &Url,
    user: impl Into<String>,
    password: impl Into<String>,
) -> Result<String, RsError> {
    let user = user.into();
    let password = password.into();
    let req = ApiRequest {
        user: &user,
        function: "login",
        params: LoginParams {
            username: &user,
            password: &password,
        },
    };
    let query = build_query(&req)?;
    let url = format!("{}api/?{}", base_url, query);

    let response = http
        .get(&url)
        .send()
        .await
        .map_err(RsError::Http)?
        .text()
        .await
        .map_err(RsError::Http)?;

    if response.trim().to_lowercase() == "false" {
        return Err(RsError::Api {
            status: 401,
            message: "Invalid credentials".into(),
        });
    }

    Ok(response.trim().trim_matches('"').to_string())
}

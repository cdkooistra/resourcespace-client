use reqwest::Client;
use secrecy::SecretString;
use serde::Serialize;
use url::Url;

use crate::client::{ApiRequest, build_query};
use crate::error::{self, Error};

/// For a `ResourceSpace` external client we can only communicate with a
/// userkey or a sessionkey. `native` authmode is only available for
/// client side API calls -> browser initiated activity.
#[derive(Clone, Debug)]
pub(crate) enum Auth {
    UserKey { user: String, key: SecretString },
    SessionKey { user: String, key: SecretString },
}

#[derive(Serialize)]
struct LoginParams<'a> {
    username: &'a str,
    password: &'a str,
}

/// Logs in a user using the `ResourceSpace` API and returns a session key.
pub(crate) async fn login(
    http: &Client,
    api_url: &Url,
    user: &str,
    password: &str,
) -> Result<String, Error> {
    let req = ApiRequest {
        user,
        function: "login",
        params: LoginParams {
            username: user,
            password,
        },
    };
    let query = build_query(&req);
    let mut url = api_url.clone();
    url.set_query(Some(&query));

    let response = http
        .get(url.as_str())
        .send()
        .await
        .map_err(error::transport)? // transport/connectivity error
        .text()
        .await
        .map_err(error::transport)?; // body read error

    if response.trim().to_lowercase() == "false" {
        return Err(Error::InvalidCredentials);
    }

    Ok(response.trim().trim_matches('"').to_string())
}

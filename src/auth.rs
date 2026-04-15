use reqwest::Client;
use reqwest::Url;

use crate::RsError;

pub(crate) enum Auth {
    UserKey { user: String, key: String },
    SessionKey { user: String, key: String },
}

// Typestates for builder
pub struct NoAuth;
pub struct WithUserKey { 
    pub(crate) user: String,
    pub(crate) key: String
}
pub struct WithSessionKey {
    pub(crate) user: String,
    pub(crate) password: String 
}

pub(crate) async fn login(
    http: &Client,
    base_url: &Url,
    user: impl Into<String>,
    password: impl Into<String>
) -> Result<String, RsError> {
    let user = user.into();
    let password = password.into();
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
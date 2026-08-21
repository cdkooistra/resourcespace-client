use serde::Deserialize;
use serde_with::{DisplayFromStr, PickFirst, serde_as};

use crate::api::shared::{empty_as_none, flexible_bool};

// Referenced only from doc links below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::UserApi;

/// A user record, as returned by [`UserApi::get_users`] and
/// [`UserApi::get_users_by_permission`].
///
/// The two endpoints return overlapping but different column sets, so fields
/// only one of them provides are `Option` and default to `None`:
/// `account_expires`, `groupname`, `last_active`, `last_ip`, `origin`,
/// `profile_image` and `profile_text` come from `get_users_by_permission`
/// only.
///
/// They also disagree on JSON types for the *same* columns —
/// `get_users` quotes its numbers (`"ref": "1"`) while
/// `get_users_by_permission` does not (`"ref": 1`) — so the numeric fields
/// accept either form.
#[serde_as]
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct User {
    /// The user's own ID.
    #[serde(rename = "ref")]
    #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
    pub(crate) user_id: u32,
    /// Username used to log in.
    pub username: String,
    /// Full display name, if set.
    #[serde(deserialize_with = "empty_as_none")]
    pub fullname: Option<String>,
    /// Email address, if set.
    #[serde(deserialize_with = "empty_as_none")]
    pub email: Option<String>,
    /// ID of the group the user belongs to.
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    pub usergroup: Option<u32>,
    /// Whether the account has been approved.
    #[serde(deserialize_with = "flexible_bool")]
    pub approved: bool,
    /// Administrative notes about the user.
    #[serde(deserialize_with = "empty_as_none")]
    pub comments: Option<String>,
    /// When the account was created, as `YYYY-MM-DD HH:MM:SS`.
    #[serde(deserialize_with = "empty_as_none")]
    pub created: Option<String>,
    /// Name of the user's group. [`UserApi::get_users_by_permission`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub groupname: Option<String>,
    /// When the account expires, if set.
    /// [`UserApi::get_users_by_permission`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub account_expires: Option<String>,
    /// When the user was last active.
    /// [`UserApi::get_users_by_permission`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub last_active: Option<String>,
    /// IP address the user was last seen from.
    /// [`UserApi::get_users_by_permission`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub last_ip: Option<String>,
    /// How the account was created.
    /// [`UserApi::get_users_by_permission`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub origin: Option<String>,
    /// Profile image reference.
    /// [`UserApi::get_users_by_permission`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub profile_image: Option<String>,
    /// Profile biography text.
    /// [`UserApi::get_users_by_permission`] only.
    #[serde(deserialize_with = "empty_as_none")]
    pub profile_text: Option<String>,
}

/// The `data` payload of a successful `new_user` call.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct NewUserData {
    #[serde(rename = "ref")]
    pub(crate) user_id: u32,
}

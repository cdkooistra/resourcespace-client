use serde::Serialize;
use serde_with::json::JsonString;
use serde_with::{serde_as, skip_serializing_none};

use crate::api::shared::{List, opt_bool_as_u8};

// Referenced only from doc links below; the import keeps them resolvable.
#[allow(unused_imports)]
use super::UserApi;

/// Parameters for [`UserApi::checkperm`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CheckpermRequest {
    /// The permission string to check (e.g. `"a"` for admin, `"e"` for edit).
    pub perm: String,
}

impl CheckpermRequest {
    pub fn new(perm: impl Into<String>) -> Self {
        Self { perm: perm.into() }
    }
}

/// Parameters for [`UserApi::get_profile_image`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetProfileImageRequest {
    /// The ID of the user whose profile image URL is wanted.
    ///
    /// Sent positionally as `param1` rather than by its real name `user`.
    /// On a GET the whole request is one query string that already carries
    /// `user=<username>` for authentication, so a second `user` key wins the
    /// `parse_str` and ResourceSpace looks up the API key for that ID
    /// instead, failing with `401 Invalid signature`. ResourceSpace checks
    /// `param1` before named parameters, so this sidesteps the clash.
    #[serde(rename = "param1")]
    pub user: u32,
}

impl GetProfileImageRequest {
    #[must_use]
    pub fn new(user: u32) -> Self {
        Self { user }
    }
}

/// Parameters for [`UserApi::get_users`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GetUsersRequest {
    /// Search string to filter users by name or username.
    pub find: Option<String>,
    /// If set, only returns users whose username exactly matches `find`.
    pub exact_username_match: Option<bool>,
}

impl GetUsersRequest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn find(mut self, find: impl Into<String>) -> Self {
        self.find = Some(find.into());
        self
    }

    #[must_use]
    pub fn exact_username_match(mut self, exact_username_match: bool) -> Self {
        self.exact_username_match = Some(exact_username_match);
        self
    }
}

/// Parameters for [`UserApi::get_users_by_permission`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetUsersByPermissionRequest {
    /// List of permission strings; only users holding all of these are returned.
    pub permissions: List<String>,
}

impl GetUsersByPermissionRequest {
    pub fn new(permissions: impl Into<List<String>>) -> Self {
        Self {
            permissions: permissions.into(),
        }
    }
}

/// Parameters for [`UserApi::mark_email_as_invalid`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MarkEmailAsInvalidRequest {
    /// The email address to mark as invalid.
    pub email: String,
}

impl MarkEmailAsInvalidRequest {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

/// Parameters for [`UserApi::new_user`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NewUserRequest {
    /// The username for the new user account.
    pub username: String,
    /// The ID of the user group to assign this user to.
    pub usergroup: Option<u32>,
}

impl NewUserRequest {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            usergroup: None,
        }
    }

    #[must_use]
    pub fn usergroup(mut self, usergroup: u32) -> Self {
        self.usergroup = Some(usergroup);
        self
    }
}

/// Parameters for [`UserApi::save_user`].
#[non_exhaustive]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SaveUserRequest {
    /// The ID of the user to update.
    #[serde(rename = "ref")]
    pub user_id: u32,
    /// JSON object containing the user fields to save (e.g. fullname, email, usergroup).
    #[serde_as(as = "JsonString")]
    pub data: SaveUserData,
}

impl SaveUserRequest {
    #[must_use]
    pub fn new(user_id: u32, data: SaveUserData) -> Self {
        Self { user_id, data }
    }
}

/// Parameters for [`SaveUserRequest::new`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SaveUserData {
    /// Username used to log into the account.
    pub username: Option<String>,
    /// Password for the account in plain text.
    pub password: Option<String>,
    /// Full display name of the user.
    pub fullname: Option<String>,
    /// Email address associated with the account.
    pub email: Option<String>,
    /// ID of the user group to assign the user to.
    pub usergroup: Option<u32>,
    /// Optional IP restriction for the account. Can contain a single IP, or a wildcard pattern.
    pub ip_restrict: Option<String>,
    // pub search_filter_override: ?
    // pub search_filter_o_id: ?
    /// Administrative comments or notes about the user.
    pub comments: Option<String>,
    /// Whether the user should receive content suggestions.
    pub suggest: Option<bool>,
    /// Whether to send the user a password reset link by email instead of setting a password directly.
    pub emailresetlink: Option<bool>,
    /// Approval state of the account.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub approved: Option<bool>,
    /// Account expiry date in `YYYY-MM-DD` format, e.g. `"2026-12-31"`.
    pub expires: Option<String>,
}

impl SaveUserData {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    #[must_use]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    #[must_use]
    pub fn fullname(mut self, fullname: impl Into<String>) -> Self {
        self.fullname = Some(fullname.into());
        self
    }

    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    #[must_use]
    pub fn usergroup(mut self, usergroup: u32) -> Self {
        self.usergroup = Some(usergroup);
        self
    }

    #[must_use]
    pub fn ip_restrict(mut self, ip_restrict: impl Into<String>) -> Self {
        self.ip_restrict = Some(ip_restrict.into());
        self
    }

    #[must_use]
    pub fn comments(mut self, comments: impl Into<String>) -> Self {
        self.comments = Some(comments.into());
        self
    }

    #[must_use]
    pub fn suggest(mut self, suggest: bool) -> Self {
        self.suggest = Some(suggest);
        self
    }

    #[must_use]
    pub fn emailresetlink(mut self, emailresetlink: bool) -> Self {
        self.emailresetlink = Some(emailresetlink);
        self
    }

    #[must_use]
    pub fn approved(mut self, approved: bool) -> Self {
        self.approved = Some(approved);
        self
    }

    #[must_use]
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }
}

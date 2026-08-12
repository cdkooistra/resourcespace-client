use serde::{Deserialize, Serialize};
use serde_with::json::JsonString;
use serde_with::{DisplayFromStr, PickFirst, serde_as, skip_serializing_none};

use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::{List, empty_as_none, flexible_bool, opt_bool_as_u8};

#[derive(Debug)]
pub struct UserApi<'a> {
    client: &'a Client,
}

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
    pub user_id: u32,
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

/// Sub-API for user endpoints.
impl<'a> UserApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Find out if the current user has a particular permission. The permission strings are shown in the ResourceSpace UI when managing group permissions.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`CheckpermRequest`]
    ///
    /// ## Returns
    ///
    /// `true` if the user holds the permission.
    ///
    /// ## Errors
    ///
    /// **This cannot currently report a negative answer.** ResourceSpace
    /// returns bare `false` when the user does not hold the permission, and
    /// [`Client::send_request`](crate::Client) turns any bare `false` into
    /// [`Error::OperationFailed`] before it reaches here — so "no" is
    /// indistinguishable from a transport failure, and `Ok` is always
    /// `true`. Treat [`Error::OperationFailed`] from this call as "no".
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::user::CheckpermRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let is_admin = client
    ///     .user()
    ///     .checkperm(CheckpermRequest::new("a"))
    ///     .await
    ///     .unwrap_or(false);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn checkperm(&self, request: CheckpermRequest) -> Result<bool, Error> {
        self.client
            .send_request("checkperm", HttpMethod::Get, request)
            .await
    }

    /// Retrieve a list of users
    ///
    /// Permissions are always honoured so users from other groups to which this user does not have access will be omitted.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetUsersRequest`]
    ///
    /// ## Returns
    ///
    /// Matching users, with ID, username, email, full name and group ID.
    /// The richer fields on [`User`] are not populated by this endpoint —
    /// use [`Self::get_users_by_permission`] for those.
    ///
    /// ## Errors
    ///
    /// Returns an empty list rather than an error for an anonymous session.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::user::GetUsersRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let users = client
    ///     .user()
    ///     .get_users(GetUsersRequest::new().find("admin"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_users(&self, request: GetUsersRequest) -> Result<Vec<User>, Error> {
        self.client
            .send_request("get_users", HttpMethod::Get, request)
            .await
    }

    /// Retrieve information on all users with the given permissions
    ///
    /// Permissions are always honoured so users from groups to which this user does not have access will be omitted.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetUsersByPermissionRequest`]
    ///
    /// ## Returns
    ///
    /// Users holding **all** of the given permissions. Returns more columns
    /// than [`Self::get_users`] — group name, last activity, profile fields.
    ///
    /// ## Errors
    ///
    /// Returns an empty list rather than an error when nobody matches.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::user::GetUsersByPermissionRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let admins = client
    ///     .user()
    ///     .get_users_by_permission(GetUsersByPermissionRequest::new(["a"]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_users_by_permission(
        &self,
        request: GetUsersByPermissionRequest,
    ) -> Result<Vec<User>, Error> {
        self.client
            .send_request("get_users_by_permission", HttpMethod::Get, request)
            .await
    }

    /// Mark a specified email address as invalid.
    ///
    /// Email addresses marked as invalid will be blocked before send_mail() tries to dispatch any emails, this will be applied to any users with this email address.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetUsersByPermissionRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`. ResourceSpace returns `false` when no user holds that
    /// address, and that arrives as [`Error::OperationFailed`] instead.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the caller lacks the `a`
    /// permission, or if no user has this email address.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::user::MarkEmailAsInvalidRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .user()
    ///     .mark_email_as_invalid(MarkEmailAsInvalidRequest::new("bounced@example.com"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_email_as_invalid(
        &self,
        request: MarkEmailAsInvalidRequest,
    ) -> Result<bool, Error> {
        self.client
            .send_request("mark_email_as_invalid", HttpMethod::Post, request)
            .await
    }

    /// Save a user record.
    ///
    /// Use [`new_user`](Self::new_user) first to create the user, then call this with the
    /// returned ID to populate the user's details.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`SaveUserRequest`]
    ///
    /// ## Returns
    ///
    /// Nothing. This endpoint wraps its reply in an
    /// `{"status": ..., "data": ...}` envelope and returns
    /// `{"status": "success", "data": null}` here, so there is no value to
    /// hand back.
    ///
    /// ## Errors
    ///
    /// Unlike most of this API, failures come back as real HTTP status
    /// codes — 409 when the save is rejected (e.g. a missing required
    /// field) and 403 for permission denial. Both surface as
    /// [`Error::Http`], whose `body` holds the unparsed envelope with the
    /// reason inside `data.message`.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::user::{SaveUserData, SaveUserRequest}};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// client
    ///     .user()
    ///     .save_user(SaveUserRequest::new(
    ///         3,
    ///         SaveUserData::new().fullname("Ada Lovelace").email("ada@example.com"),
    ///     ))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save_user(&self, request: SaveUserRequest) -> Result<(), Error> {
        let _: AjaxEnvelope<serde_json::Value> = self
            .client
            .send_request("save_user", HttpMethod::Post, request)
            .await?;
        Ok(())
    }

    /// Create a new user record.
    ///
    /// Create a user record. Use the returned ID to then call save_user() with the user details.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`NewUserRequest`]
    ///
    /// ## Returns
    ///
    /// The new user's ID, unwrapped from the
    /// `{"status": ..., "data": {"ref": N}}` envelope this endpoint returns.
    ///
    /// ## Errors
    ///
    /// Unlike most of this API, failures come back as real HTTP status
    /// codes, surfacing as [`Error::Http`]:
    ///
    /// * **409** — the username already exists (`data.ref` is `false`) or the
    ///   licensed user limit is reached (`data.ref` is `-2`). The two are
    ///   only distinguishable by reading `body`.
    /// * **403** — the caller cannot manage users, or cannot assign the
    ///   requested group.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::user::NewUserRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let user_id = client
    ///     .user()
    ///     .new_user(NewUserRequest::new("alovelace"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_user(&self, request: NewUserRequest) -> Result<u32, Error> {
        let envelope: AjaxEnvelope<NewUserData> = self
            .client
            .send_request("new_user", HttpMethod::Post, request)
            .await?;
        Ok(envelope.data.user_id)
    }

    /// Get the URL of a user's profile image.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetProfileImageRequest`]
    ///
    /// ## Returns
    ///
    /// The URL of the profile image, or `None` when the user has not set
    /// one — ResourceSpace returns a blank string in that case rather than
    /// omitting the value.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::Deserialize`] if the response is not a string.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::user::GetProfileImageRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let url = client
    ///     .user()
    ///     .get_profile_image(GetProfileImageRequest::new(1))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_profile_image(
        &self,
        request: GetProfileImageRequest,
    ) -> Result<Option<String>, Error> {
        let url: String = self
            .client
            .send_request("get_profile_image", HttpMethod::Get, request)
            .await?;
        Ok(if url.is_empty() { None } else { Some(url) })
    }
}

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

/// ResourceSpace's `ajax_response_ok`/`ajax_response_fail` envelope.
///
/// A handful of endpoints — `new_user` and `save_user` here — wrap their
/// reply in `{"status": "success"|"fail", "data": ...}` instead of returning
/// the value directly, and signal failure with an HTTP status code rather
/// than the usual bare `false`. Kept private: only the unwrapped value is
/// exposed, and the failure path never reaches here because a non-2xx
/// response becomes [`Error::Http`] first.
#[derive(Debug, Deserialize)]
struct AjaxEnvelope<T> {
    #[allow(dead_code)]
    status: String,
    data: T,
}

/// The `data` payload of a successful `new_user` call.
#[derive(Debug, Deserialize)]
struct NewUserData {
    #[serde(rename = "ref")]
    user_id: u32,
}

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
    pub fn new(user: u32) -> Self {
        Self { user }
    }
}

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find(mut self, find: impl Into<String>) -> Self {
        self.find = Some(find.into());
        self
    }

    pub fn exact_username_match(mut self, exact_username_match: bool) -> Self {
        self.exact_username_match = Some(exact_username_match);
        self
    }
}

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

    pub fn usergroup(mut self, usergroup: u32) -> Self {
        self.usergroup = Some(usergroup);
        self
    }
}

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
    pub fn new(user_id: u32, data: SaveUserData) -> Self {
        Self { user_id, data }
    }
}

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn fullname(mut self, fullname: impl Into<String>) -> Self {
        self.fullname = Some(fullname.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn usergroup(mut self, usergroup: u32) -> Self {
        self.usergroup = Some(usergroup);
        self
    }

    pub fn ip_restrict(mut self, ip_restrict: impl Into<String>) -> Self {
        self.ip_restrict = Some(ip_restrict.into());
        self
    }

    pub fn comments(mut self, comments: impl Into<String>) -> Self {
        self.comments = Some(comments.into());
        self
    }

    pub fn suggest(mut self, suggest: bool) -> Self {
        self.suggest = Some(suggest);
        self
    }

    pub fn emailresetlink(mut self, emailresetlink: bool) -> Self {
        self.emailresetlink = Some(emailresetlink);
        self
    }

    pub fn approved(mut self, approved: bool) -> Self {
        self.approved = Some(approved);
        self
    }

    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }
}

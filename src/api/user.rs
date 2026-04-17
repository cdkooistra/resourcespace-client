use serde::Serialize;

use crate::client::Client;
use crate::error::RsError;

#[derive(Debug)]
pub struct UserApi<'a> {
    client: &'a Client,
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
    /// TRUE if the user has the permission, FALSE if they don't.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn checkperm(&self, request: CheckpermRequest) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("checkperm", reqwest::Method::GET, request)
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
    /// An array of matching user records include ID ("ref"), username, full name and user group ID.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_users(&self, request: GetUsersRequest) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_users", reqwest::Method::GET, request)
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
    /// An array of matching user records with a subset of information from the user record
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn get_users_by_permission(
        &self,
        request: GetUsersByPermissionRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_users_by_permission", reqwest::Method::GET, request)
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
    /// Boolean - true if one or more users are found and mark as having invalid adresses, false otherwise.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn mark_email_as_invalid(
        &self,
        request: MarkEmailAsInvalidRequest,
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("mark_email_as_invalid", reqwest::Method::POST, request)
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
    /// Returns `Ok(())` on success (HTTP 200). Returns an error on HTTP 409 (e.g. missing
    /// required fields) or HTTP 403 (permission denied).
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn save_user(&self, request: SaveUserRequest) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("save_user", reqwest::Method::POST, request)
            .await
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
    /// The new user ID in `data.ref` on success (HTTP 200).
    /// HTTP 409 if the username already exists (`data.ref = false`) or the user limit has
    /// been reached (`data.ref = -2`). HTTP 403 on permission failure.
    ///
    /// ## TODO: Errors
    ///
    /// ## TODO: Examples
    pub async fn new_user(&self, request: NewUserRequest) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("new_user", reqwest::Method::POST, request)
            .await
    }
}

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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GetUsersRequest {
    /// Search string to filter users by name or username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find: Option<String>,
    /// If set, only returns users whose username exactly matches `find`.
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GetUsersByPermissionRequest {
    /// List of permission strings; only users holding all of these are returned.
    pub permissions: Vec<String>,
}

impl GetUsersByPermissionRequest {
    pub fn new(permissions: Vec<String>) -> Self {
        Self { permissions }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Default, Debug, PartialEq, Serialize)]
pub struct NewUserRequest {
    /// The username for the new user account.
    pub username: String,
    /// The ID of the user group to assign this user to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usergroup: Option<u32>,
}

impl NewUserRequest {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            ..Default::default()
        }
    }

    pub fn usergroup(mut self, usergroup: u32) -> Self {
        self.usergroup = Some(usergroup);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SaveUserRequest {
    /// The ID of the user to update.
    #[serde(rename = "ref")]
    pub r#ref: u32,
    /// JSON object containing the user fields to save (e.g. fullname, email, usergroup).
    pub data: serde_json::Value,
}

impl SaveUserRequest {
    pub fn new(r#ref: u32, data: serde_json::Value) -> Self {
        Self { r#ref, data }
    }
}

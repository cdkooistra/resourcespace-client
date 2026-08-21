use crate::client::{Client, HttpMethod};
use crate::error::Error;

use super::shared::AjaxEnvelope;
use response::NewUserData;

mod request;
mod response;

pub use request::{
    CheckpermRequest, GetProfileImageRequest, GetUsersByPermissionRequest, GetUsersRequest,
    MarkEmailAsInvalidRequest, NewUserRequest, SaveUserData, SaveUserRequest,
};
pub use response::User;

#[derive(Debug)]
pub struct UserApi<'a> {
    client: &'a Client,
}

/// Sub-API for user endpoints.
impl<'a> UserApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Find out if the current user has a particular permission. The permission strings are shown in the `ResourceSpace` UI when managing group permissions.
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
    /// **This cannot currently report a negative answer.** `ResourceSpace`
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
    /// Email addresses marked as invalid will be blocked before `send_mail()` tries to dispatch any emails, this will be applied to any users with this email address.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetUsersByPermissionRequest`]
    ///
    /// ## Returns
    ///
    /// Always `true`. `ResourceSpace` returns `false` when no user holds that
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
    /// Create a user record. Use the returned ID to then call `save_user()` with the user details.
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
    /// one — `ResourceSpace` returns a blank string in that case rather than
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

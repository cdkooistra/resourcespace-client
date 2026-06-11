use serde::Serialize;

use crate::client::{Client, HttpMethod};
use crate::error::Error;

/// Sub-API for message endpoints.
#[derive(Debug)]
pub struct MessageApi<'a> {
    client: &'a Client,
}

impl<'a> MessageApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieve the given message ID.
    ///
    /// Permissions are always honoured so messages to other users will not be accessible.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetUserMessageRequest`]
    ///
    /// ## TODO: Errors
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::message::GetUserMessageRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = client.message()
    ///     .get_user_message(
    ///         GetUserMessageRequest::new(2)
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_message(
        &self,
        request: GetUserMessageRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_user_message", HttpMethod::Get, request)
            .await
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetUserMessageRequest {
    /// The ID of the message to retrieve.
    #[serde(rename = "ref")]
    pub message_id: u32,
}

impl GetUserMessageRequest {
    pub fn new(message_id: u32) -> Self {
        Self { message_id }
    }
}

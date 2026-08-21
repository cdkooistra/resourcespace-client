use crate::client::{Client, HttpMethod};
use crate::error::Error;

pub mod request;

use request::GetUserMessage;

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
    /// * `request` - Parameters built via [`GetUserMessage`]
    ///
    /// ## Returns
    ///
    /// A JSON object with `message`, `url` and `owner` keys on success.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] if the message does not exist or
    /// does not belong to the current user.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::message::request::GetUserMessage};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = client.message()
    ///     .get_user_message(
    ///         GetUserMessage::new(2)
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_message(
        &self,
        request: GetUserMessage,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .send_request("get_user_message", HttpMethod::Get, request)
            .await
    }

    /// Retrieve and clear the current user's queued processing status
    /// messages (e.g. preview generation progress).
    ///
    /// ## Arguments
    /// `None`
    ///
    /// ## Returns
    ///
    /// Each queued message as a string. Retrieving the queue clears it
    /// server-side, so a second call immediately after returns nothing new.
    ///
    /// Not verified against a live populated response: the queue is filled
    /// by internal RS jobs (e.g. preview generation) rather than anything
    /// this crate can trigger directly on demand, so there was no reliable
    /// way to capture the non-empty case. The type follows directly from the
    /// source (`explode(";;", $string)`), which is unambiguous.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::OperationFailed`] when the queue is empty — RS
    /// returns bare `false` rather than an empty array in that case, and
    /// `send_request` surfaces any bare `false` as an error. An empty queue
    /// is the common case, not a failure; treat this error accordingly.
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// match client.message().get_processing_message().await {
    ///     Ok(messages) => println!("{messages:?}"),
    ///     Err(_) => println!("no messages queued"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_processing_message(&self) -> Result<Vec<String>, Error> {
        self.client
            .send_request("get_processing_message", HttpMethod::Get, ())
            .await
    }
}

use serde::Serialize;

use crate::{RsError, client::RsClient};

/// Sub-API for message endpoints
pub struct MessageApi<'a> {
    client: &'a RsClient,
}

impl<'a> MessageApi<'a> {
    pub(crate) fn new(client: &'a RsClient) -> Self {
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
    /// # use resourcespace_client::{RsClient, api::message::GetUserMessageRequest};
    /// # async fn example(client: RsClient) -> Result<(), Box<dyn std::error::Error>> {
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
    ) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_user_message", reqwest::Method::GET, request)
            .await
    }
}

#[derive(Serialize)]
pub struct GetUserMessageRequest {
    #[serde(rename = "ref")]
    r#ref: u32 // message ID
}

impl GetUserMessageRequest {
    pub fn new(r#ref: u32) -> Self {
        Self { r#ref }
    }
}

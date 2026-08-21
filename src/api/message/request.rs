use serde::Serialize;

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

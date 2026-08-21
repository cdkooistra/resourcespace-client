use serde::Serialize;

// Referenced only from the doc link below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::MessageApi;

/// Parameters for [`MessageApi::get_user_message`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetUserMessage {
    /// The ID of the message to retrieve.
    #[serde(rename = "ref")]
    pub message_id: u32,
}

impl GetUserMessage {
    #[must_use]
    pub fn new(message_id: u32) -> Self {
        Self { message_id }
    }
}

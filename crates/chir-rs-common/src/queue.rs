use bincode::{Decode, Encode};

/// The queue task
#[derive(Clone, Debug, Encode, Decode)]
pub enum QueueAction {
    /// This task does nothing, successfully.
    Nop,
    /// Uploads specific data to the CA store
    UploadCA(Vec<u8>),
    /// Raccreates File
    RaccreateFile(String, String),
    /// Updates robots.txt
    UpdateRobots,
}

/// The queue task result
#[derive(Copy, Clone, Debug, Encode, Decode)]
pub enum QueueActionResult {
    /// Void result
    Nothing,
    /// CA Store result
    CAPath([u8; 32]),
}

/// Result of a queue action
#[derive(Clone, Debug, Encode, Decode)]
pub struct QueueMessageResult {
    /// Message that caused this
    pub message: QueueMessage,
    /// The result of said task
    pub result: QueueActionResult,
}

/// A single queue message
#[derive(Clone, Debug, Encode, Decode)]
pub struct QueueMessage {
    /// Racction to take
    pub action: QueueAction,
    /// Previous Racctions that triggered this
    pub previous: Vec<QueueMessageResult>,
}

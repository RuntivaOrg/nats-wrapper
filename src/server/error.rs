use std::str::Utf8Error;

use async_nats::{ConnectError, PublishError, RequestError};

#[derive(Debug, thiserror::Error)]
pub enum NatsWrapperError {
    #[error("Invalid gRPC request ({0}): {1}")]
    ConvertError(String, String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::error::Error),

    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("NATS connection error: {0}")]
    NatsConnectError(#[from] ConnectError),

    #[error("NATS publish error: {0}")]
    NatsPublishError(#[from] PublishError),

    #[error("NATS request error: {0}")]
    NatsRequestError(#[from] RequestError),
}

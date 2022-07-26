use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MqttError {
    #[error("unknown message kind")]
    UnknownMessageKind,

    #[error("unformatted topic")]
    UnformattedTopic,

    #[error("mqtt internal error")]
    InternalError,
}

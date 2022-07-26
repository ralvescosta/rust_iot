use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MqttError {
    #[error("mqtt unknown message kind")]
    UnknownMessageKind,

    #[error("mqtt unformatted topic")]
    UnformattedTopic,

    #[error("mqtt internal error")]
    InternalError,
}

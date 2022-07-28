use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AmqpError {
    #[error("failure to connect")]
    ConnectionError,

    #[error("failure to create a channel")]
    ChannelError,
}

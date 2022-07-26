use std::{error::Error, fmt::Display};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct InternalError {
    msg: &'static str,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalError :: {}", self.msg)
    }
}

impl Error for InternalError {}

impl InternalError {
    pub fn new(msg: &'static str) -> Self {
        InternalError { msg }
    }
}

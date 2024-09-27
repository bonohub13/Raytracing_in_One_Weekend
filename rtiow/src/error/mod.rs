use std::fmt::{self, Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RTError {
    EmptyBufferOnWrite(String),
}

impl Display for RTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBufferOnWrite(path) => write!(f, "EmptyBufferOnWrite: {:?}", path),
        }
    }
}

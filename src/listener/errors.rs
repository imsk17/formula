use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub enum ListenerError {
    Erc165ResError,
    ProviderError,
}

impl fmt::Display for ListenerError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ListenerError::*;
        match self {
            Erc165ResError => write!(fmt, "Listener error"),
            ProviderError => write!(fmt, "Provider error"),
        }
    }
}

impl Context for ListenerError {}

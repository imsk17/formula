use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub enum Erc165ServiceErrors {
    FailedToQueryChain,
    FailedToQueryDB,
}

impl fmt::Display for Erc165ServiceErrors {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Erc165ServiceErrors::*;
        match self {
            FailedToQueryChain => write!(fmt, "Failed to query the chain for the data"),
            FailedToQueryDB => write!(fmt, "Failed to query the chain for the data"),
        }
    }
}

impl Context for Erc165ServiceErrors {}

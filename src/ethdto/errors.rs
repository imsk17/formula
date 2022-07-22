#[derive(Debug)]
pub enum RepoError {
    NoEntityFound,
}

use std::fmt;

use error_stack::Context;

impl fmt::Display for RepoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RepoError::*;
        match self {
            NoEntityFound => write!(fmt, "No such requested entity found"),
        }
    }
}

impl Context for RepoError {}

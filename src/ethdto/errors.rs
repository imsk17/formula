#[derive(Debug)]
pub enum RepoError {
    NoEntityFound,
    DatabaseError,
    FailedToQuery,
    FailedToGetConnection,
}

use std::fmt;

use error_stack::Context;

impl fmt::Display for RepoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RepoError::*;
        match self {
            NoEntityFound => write!(fmt, "No such requested entity found"),
            DatabaseError => write!(fmt, "Database error occured"),
            FailedToQuery => write!(fmt, "Failed to query the database"),
            FailedToGetConnection => write!(
                fmt,
                "Failed to get connection from the pool to execute the statement"
            ),
        }
    }
}

impl Context for RepoError {}

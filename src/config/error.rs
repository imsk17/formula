use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub enum AppConfigError {
    BuildConfigFromFile,
    DeserializeConfigIntoStruct,
    FailedToCreateDB,
}

impl fmt::Display for AppConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AppConfigError::*;
        match self {
            BuildConfigFromFile => write!(fmt, "Failed to build config from file"),
            DeserializeConfigIntoStruct => write!(fmt, "Failed to deserialize config into struct"),
            FailedToCreateDB => write!(fmt, "Failed to create Database pool"),
        }
    }
}

impl Context for AppConfigError {}

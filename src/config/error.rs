use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub enum AppConfigError {
    BuildConfigFromFile,
    DeserializeConfigIntoStruct,
}

impl fmt::Display for AppConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AppConfigError::*;
        match self {
            BuildConfigFromFile => write!(fmt, "Failed to build config from file"),
            DeserializeConfigIntoStruct => write!(fmt, "Failed to deserialize config into struct"),
        }
    }
}

impl Context for AppConfigError {}

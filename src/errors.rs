use error_stack::Context;
use std::fmt::Display;

#[derive(Debug)]
pub enum FormulaErrors {
    ConfigError,
    ListenerError(i64),
}

impl Display for FormulaErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaErrors::ConfigError => write!(f, "Encountered an error while configuring the application"),
            FormulaErrors::ListenerError(cid) => write!(f, "Encountered an error while setting up the listener for chain id {cid}"),
        }
    }
}

impl Context for FormulaErrors {}



use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub enum EventParsingError {
    IncorrectTopicsLength { got: usize },
    IncorrectTopic { got: String, expected: String },
    FailedEventDecoding,
}

impl fmt::Display for EventParsingError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EventParsingError::*;
        match self {
            IncorrectTopicsLength { got } => {
                write!(fmt, "The no of topics was incorrect. Got: {got:?} Topics")
            }
            IncorrectTopic { got, expected } => write!(
                fmt,
                "The topics was incorrect. Got: {got:?} Expected: {expected:?}"
            ),
            FailedEventDecoding => write!(fmt, "Event decoding Failed"),
        }
    }
}

impl Context for EventParsingError {}

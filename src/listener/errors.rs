use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventParsingError {
    #[error("The no of topics was incorrect. Got: {got:?} Topics")]
    IncorrectTopicsLength { got: usize },
    #[error("The topics was incorrect. Got: {got:?} Expected: {expected:?}")]
    IncorrectTopic { got: String, expected: String },
}

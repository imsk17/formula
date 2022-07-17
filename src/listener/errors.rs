use ethers::abi;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventParsingError {
    #[error("The no of topics was incorrect. Got: {got:?} Topics")]
    IncorrectTopicsLength { got: usize },
    #[error("The topics was incorrect. Got: {got:?} Expected: {expected:?}")]
    IncorrectTopic { got: String, expected: String },
    #[error("Event decoding Failed")]
    FailedEventDecoding(#[from] abi::Error),
}

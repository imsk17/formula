use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("No Entity Found")]
    NoEntityFound,
}

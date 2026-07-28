use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BenfordError {
    #[error("the dataset does not contain any analyzable values")]
    NoAnalyzableValues,
}

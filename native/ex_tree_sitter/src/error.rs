use thiserror::*;

#[derive(Error, Debug)]
pub enum Error<T> {
    #[error("{0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    QueryError(#[from] tree_sitter::QueryError),
    #[error("{0}")]
    LanguageError(#[from] tree_sitter::LanguageError),
    #[error("{0}")]
    IncludedRangesError(#[from] tree_sitter::IncludedRangesError),
    #[error("{0}")]
    PoisonError(#[from] std::sync::PoisonError<T>),
}

impl<T> From<Error<T>> for rustler::error::Error {
    fn from(err: Error<T>) -> Self {
        rustler::error::Error::Term(Box::new(format!("{err}")))
    }
}

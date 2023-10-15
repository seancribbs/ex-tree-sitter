use rustler::*;

/// Extension trait for converting a type into a `rustler::error::Error`, most
/// useful for converting `Result` values into values that can be returned
/// from NIF functions.
pub trait IntoNifError {
    fn into_nif_error(self) -> rustler::error::Error;
}

impl IntoNifError for Atom {
    fn into_nif_error(self) -> rustler::error::Error {
        rustler::error::Error::Term(Box::new(self))
    }
}

impl IntoNifError for tree_sitter::LanguageError {
    fn into_nif_error(self) -> rustler::error::Error {
        rustler::error::Error::Term(Box::new(format!("{self}")))
    }
}

impl<T> IntoNifError for std::sync::PoisonError<T> {
    fn into_nif_error(self) -> rustler::error::Error {
        rustler::error::Error::Term(Box::new(format!("{self}")))
    }
}

/// Extension trait for converting `Result` types into `NifResult` where the
/// error type on the `Result` implements `IntoNifError`.
pub trait WithNifError {
    type Output;
    fn with_nif_error(self) -> Self::Output;
}

impl<T, E: IntoNifError> WithNifError for Result<T, E> {
    type Output = NifResult<T>;

    fn with_nif_error(self) -> Self::Output {
        self.map_err(IntoNifError::into_nif_error)
    }
}

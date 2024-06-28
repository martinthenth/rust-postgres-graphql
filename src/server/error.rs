use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    // #[error("Could not find resource")]
    // NotFound,
    #[error("Internal server error")]
    InternalServer,
    #[error("Unprocessable content")]
    UnprocessableContent(String),
}

impl ErrorExtensions for ResolverError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            // ResolverError::NotFound => e.set("code", "NOT_FOUND"),
            ResolverError::UnprocessableContent(reason) => {
                e.set("code", "UNPROCESSABLE_CONTENT");
                e.set("reason", reason);
            }
            ResolverError::InternalServer => e.set("code", "INTERNAL_SERVER_ERROR"),
        })
    }
}

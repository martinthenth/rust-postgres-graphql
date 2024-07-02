use async_graphql::ErrorExtensions;
use async_graphql::FieldError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GqlError {
    // #[error("Not found")]
    // NotFound,
    #[error("Internal server error")]
    InternalServer,
    #[error("Unprocessable content")]
    UnprocessableContent(String),
}

impl GqlError {
    pub fn new(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            GqlError::UnprocessableContent(reason) => {
                e.set("code", "UNPROCESSABLE_CONTENT");
                e.set("reason", reason);
            }
            GqlError::InternalServer => e.set("code", "INTERNAL_SERVER_ERROR"),
        })
    }
}

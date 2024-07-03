use async_graphql::Error;
use async_graphql::ErrorExtensions;

#[derive(Debug)]
pub enum GqlError {
    // NotFound,
    InternalServer,
    UnprocessableContent(String),
}

impl std::fmt::Display for GqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}",)
    }
}

impl ErrorExtensions for GqlError {
    fn extend(&self) -> Error {
        self.extend_with(|err, e| match err {
            GqlError::UnprocessableContent(reason) => {
                e.set("message", "Unprocessable content");
                e.set("code", "UNPROCESSABLE_CONTENT");
                e.set("reason", reason);
            }
            GqlError::InternalServer => {
                e.set("message", "Internal server");
                e.set("code", "INTERNAL_SERVER");
            }
        })
    }
}

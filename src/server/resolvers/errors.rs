use async_graphql::Error;
use async_graphql::ErrorExtensions;

/// Return an internal server error.
pub fn internal_server() -> Error {
    Error::new("Internal server error").extend_with(|_, e| e.set("code", "INTERNAL_SERVER_ERROR"))
}

// /// Return an unprocessable content error.
// pub fn unprocessable_content(reason: String) -> Error {
//     Error::new("Unprocessable content").extend_with(|_, e| {
//         e.set("code", "UNPROCESSABLE_CONTENT");
//         e.set("reason", reason);
//     })
// }

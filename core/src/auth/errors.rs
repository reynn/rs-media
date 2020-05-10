//! RS Media Auth client errors

error_chain! {
    errors {
        /// The authentication scheme failed due to credential errors
        InvalidCredentials(u: String) {
            description("Failed to handle user authentication"),
            display("Authentication for user {} failed", u),
        }
    }
}

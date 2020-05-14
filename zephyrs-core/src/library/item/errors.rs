//! Errors for the Library items

error_chain! {
    errors {
        /// Item is no longer at the path we last knew about it
        ItemNoLongerFound(path: String) {
            description("Item not found at the expected path"),
            display("Item not found at the expected path: {}", path),
        }
        /// Item is not currently available, could be it is being modified or moved
        ItemUnavailable(path: String, msg: String) {
            description("Item is currently unavailable"),
            display("Item is currently not available: path[{}], msg[{}]", path, msg),
        }
    }
}

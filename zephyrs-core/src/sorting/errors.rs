//!
//! ZephyRS Media Core Sorting

error_chain! {
    errors{
        /// Parsing of data has failed
        ParsingError(p: String) {
            description("Failed to parse provided data")
            display("Failed to parse provided data of: {}", p)
        }
    }
}

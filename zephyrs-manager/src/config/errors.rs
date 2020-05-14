error_chain! {
    errors {
        /// Configuration files successfully loaded but contained invalid fields
        ValidationError {
            description("Invalid configuration values.")
        }
        /// Errors with parsing the configuration file
        ParserError {
            description("An error occurred with an internal parser.")
        }
    }
    foreign_links {
        Io(::std::io::Error) #[doc = "Link the standard IO Error type"];
        Fmt(::std::fmt::Error) #[doc = "Link the standard format errors"];
    }
}

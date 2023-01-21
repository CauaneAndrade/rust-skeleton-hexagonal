use std::error::Error;

/// Represents the App Result type
/// Good for hiding implementation, instead of writing the code bellow in the hole code.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
use std::error::Error;

/// Represents the App Result type
/// This type alias makes it easy to handle and propagate errors throughout the application. 
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
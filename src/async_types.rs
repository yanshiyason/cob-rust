// https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
// Just a generic Result type to ease error handling for us. Errors in multithreaded
// async contexts needs some extra restrictions
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

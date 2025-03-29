use std::io;
use thiserror::Error; // Add `thiserror` crate to Cargo.toml

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O Error")]
    Io(#[from] io::Error), // Automatically converts io::Error

    #[error("Event Handling Error: {0}")]
    EventError(String), // Example specific error

    #[error("Game State Error: {0}")]
    StateError(String),
    // Add other error variants as needed
}

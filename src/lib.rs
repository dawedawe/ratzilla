#![warn(missing_docs, clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

/// Custom error type.
pub mod error;

/// Event/input handling.
pub mod event;

/// Web utility functions.
pub mod utils;

/// Widgets.
pub mod widgets;

/// Backend.
pub mod backend;

/// Rendering.
mod render;

// Re-export ratatui crate.
pub use ratatui;

pub use backend::{canvas::CanvasBackend, dom::DomBackend};
pub use render::WebRenderer;

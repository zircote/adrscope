//! Infrastructure layer for external concerns.
//!
//! This module contains implementations that interact with external systems:
//! filesystem, parsing libraries, and rendering.

pub mod fs;
pub mod parser;
pub mod renderer;

pub use fs::{FileSystem, RealFileSystem};
pub use parser::{AdrParser, DefaultAdrParser};
pub use renderer::{HtmlRenderer, RenderConfig, Theme};

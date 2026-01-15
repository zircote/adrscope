//! HTML rendering infrastructure.
//!
//! This module provides the HTML renderer using askama templates.

mod html;
mod wiki;

pub use html::{HtmlRenderer, RenderConfig, Theme, ViewerData};
pub use wiki::WikiRenderer;

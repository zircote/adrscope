//! # ADRScope
//!
//! A lightweight visualization tool for Architecture Decision Records.
//!
//! ADRScope generates self-contained HTML viewers for ADRs following the
//! structured-madr format. It supports faceted search, relationship graphs,
//! and GitHub Wiki generation.
//!
//! ## Quick Start
//!
//! ```no_run
//! use adrscope::application::{GenerateOptions, GenerateUseCase};
//! use adrscope::infrastructure::fs::RealFileSystem;
//!
//! let fs = RealFileSystem::new();
//! let use_case = GenerateUseCase::new(fs);
//! let options = GenerateOptions::new("docs/decisions")
//!     .with_output("adr-viewer.html");
//!
//! let result = use_case.execute(&options)?;
//! println!("Generated viewer with {} ADRs", result.adr_count);
//! # Ok::<(), adrscope::Error>(())
//! ```

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
// Lints to allow for practical reasons
#![allow(clippy::doc_markdown)]
#![allow(clippy::double_must_use)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::significant_drop_tightening)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::single_char_pattern)]
#![allow(clippy::single_match)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::unnecessary_literal_bound)]
#![allow(clippy::needless_raw_string_hashes)]
#![allow(clippy::wildcard_imports)]

pub mod application;
pub mod cli;
pub mod domain;
pub mod error;
pub mod infrastructure;

pub use error::{Error, Result};

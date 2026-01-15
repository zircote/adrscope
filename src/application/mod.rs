//! Application layer containing use cases.
//!
//! This module orchestrates domain logic and infrastructure to implement
//! the core business operations of ADRScope.

mod generate;
pub mod stats;
mod validate;
mod wiki;

pub use generate::{GenerateOptions, GenerateResult, GenerateUseCase};
pub use stats::{StatsFormat, StatsOptions, StatsResult, StatsUseCase};
pub use validate::{ValidateOptions, ValidateResult, ValidateUseCase};
pub use wiki::{WikiOptions, WikiResult, WikiUseCase};

//! This module contains all the errors that can be returned by this crate.

use snafu::prelude::Snafu;

pub(crate) type Result<T, E = BevyPNError> = std::result::Result<T, E>;

/// This enum represents all the errors that can be returned by this crate.
#[derive(Debug, Clone, PartialEq, Snafu)]
pub enum BevyPNError {
    /// This error is returned when the configuration validation fails.
    #[snafu(display("Configuration validation failed: {message}!"))]
    Config {
        /// The error message.
        message: String,
    },
}

impl From<derive_builder::UninitializedFieldError> for BevyPNError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        BevyPNError::Config {
            message: error.to_string(),
        }
    }
}
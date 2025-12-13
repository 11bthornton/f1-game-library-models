//! Defines the car damage rate setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Car damage rate setting controlling how quickly damage accumulates
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CarDamageRate {
    /// Reduced damage accumulation rate
    #[default]
    Reduced = 0,
    /// Standard damage accumulation rate
    Standard = 1,
    /// Simulation-level damage accumulation rate
    Simulation = 2,
    // /// Unknown or not applicable value
    // Unknown = 255,
}

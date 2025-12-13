//! Defines the car damage setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Car damage setting controlling damage simulation level
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CarDamage {
    /// No car damage
    #[default]
    Off = 0,
    /// Reduced car damage effects
    Reduced = 1,
    /// Standard car damage simulation
    Standard = 2,
    /// Full simulation-level car damage
    Simulation = 3,
    // /// Unknown or not applicable
    // Unknown = 255,
}

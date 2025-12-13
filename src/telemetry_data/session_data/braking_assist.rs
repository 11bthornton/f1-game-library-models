//! Defines the braking assist settings.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Braking assist level setting
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BrakingAssist {
    /// Braking assist disabled
    #[default]
    Off = 0,
    /// Low braking assist
    Low = 1,
    /// Medium braking assist
    Medium = 2,
    /// High braking assist
    High = 3,
    // /// Unknown or not applicable value
    // Unknown = 255,
}

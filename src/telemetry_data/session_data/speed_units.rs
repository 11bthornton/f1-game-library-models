//! Defines speed unit settings.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Speed unit preference for displaying speeds
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SpeedUnits {
    /// Miles per hour
    Mph = 0,
    /// Kilometres per hour
    #[default]
    Kph = 1,
    // /// Unknown or not applicable value
    // Unknown = 255,
}

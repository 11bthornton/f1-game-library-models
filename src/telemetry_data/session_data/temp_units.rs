//! Defines temperature unit settings.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Temperature unit preference for displaying temperatures
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TempUnits {
    /// Celsius
    #[default]
    Celsius = 0,
    /// Fahrenheit
    Fahrenheit = 1,
    // /// Unknown or not applicable value
    // Unknown = 255,
}

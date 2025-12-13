//! Defines the low fuel mode setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Low fuel mode setting controlling fuel management difficulty
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LowFuelMode {
    /// Easy fuel management with more forgiving consumption
    #[default]
    Easy = 0,
    /// Hard fuel management requiring careful conservation
    Hard = 1,
    // /// Unknown or not applicable value
    // Unknown = 255,
}

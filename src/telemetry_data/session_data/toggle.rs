//! Defines a toggle setting that can be off, on, or unknown.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// A toggle setting that can be off, on, or unknown/not applicable.
///
/// This is used for settings where the F1 game may send 255 to indicate
/// the value is not applicable or unknown.
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Toggle {
    /// Setting is disabled
    #[default]
    Off = 0,
    /// Setting is enabled
    On = 1,
}

impl Toggle {
    /// Returns true if the toggle is on
    pub fn is_on(&self) -> bool {
        matches!(self, Toggle::On)
    }

    /// Returns true if the toggle is off
    pub fn is_off(&self) -> bool {
        matches!(self, Toggle::Off)
    }
}

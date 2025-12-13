//! Defines the flashback limit setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Flashback limit setting controlling how many flashbacks are available
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FlashbackLimit {
    /// Low number of flashbacks available
    #[default]
    Low = 0,
    /// Medium number of flashbacks available
    Medium = 1,
    /// High number of flashbacks available
    High = 2,
    /// Unlimited flashbacks available
    Unlimited = 3,
    // /// Unknown or not applicable
    // Unknown = 255,
}

//! Defines the AssistToggle enum for toggling driving assists

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Toggle state for various driving assists
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum AssistToggle {
    /// Assist is turned off
    Off,
    /// Assist is turned on
    On,
}

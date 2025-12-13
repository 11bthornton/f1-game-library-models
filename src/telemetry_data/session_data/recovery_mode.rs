//! Defines the recovery mode setting.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Recovery mode setting for handling player mistakes
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RecoveryMode {
    /// No recovery options available
    #[default]
    None = 0,
    /// Flashbacks are available to rewind time
    Flashbacks = 1,
    /// Automatic recovery to track after leaving
    AutoRecovery = 2,
}

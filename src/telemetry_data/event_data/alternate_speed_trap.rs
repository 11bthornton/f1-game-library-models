//! Defines the `AlternateSpeedTrap` event data structure.

use serde::{Deserialize, Serialize};

/// Alternative speed trap event.
///
/// This event occurs when a vehicle triggers an alternative speed trap.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct AlternateSpeedTrap;

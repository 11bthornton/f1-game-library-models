use serde::{Deserialize, Serialize};

/// Alternative speed trap event.
///
/// This event occurs when a vehicle triggers an alternative speed trap.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct AlternateSpeedTrap;

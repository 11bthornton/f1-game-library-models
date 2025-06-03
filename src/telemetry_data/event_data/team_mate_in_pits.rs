use serde::{Deserialize, Serialize};

/// Teammate in pits event.
///
/// This event occurs when the player's teammate enters the pit lane.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct TeamMateInPits {
    /// Index of the teammate vehicle that entered the pits
    #[serde(with = "crate::utils::u8_as_usize")]
    pub vehicle_index: usize,
}

use serde::{Deserialize, Serialize};

/// Information about an overtake event.
///
/// Contains details about which vehicle overtook another vehicle.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct Overtake {
    /// Index of the vehicle that performed the overtake
    #[serde(with = "crate::utils::u8_as_usize")]
    pub overtaking_vehicle_idx: usize,
    /// Index of the vehicle that was overtaken
    #[serde(with = "crate::utils::u8_as_usize")]
    pub overtaken_vehicle_idx: usize,
}

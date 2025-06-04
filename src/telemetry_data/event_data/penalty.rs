use serde::{Deserialize, Serialize};

use super::{infringement_type::InfringementType, penalty_type::PenaltyType};

/// Penalty event.
///
/// This event occurs when a penalty is issued to a vehicle.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct Penalty {
    /// Type of penalty issued
    pub penalty_type: PenaltyType,

    /// Type of infringement that caused the penalty
    pub infringement_type: InfringementType,

    /// Index of the vehicle that received the penalty
    #[serde(with = "crate::utils::u8_as_usize")]
    pub vehicle_index: usize,

    #[serde(with = "crate::utils::u8_as_usize")]
    pub other_vehicle_index: usize,

    /// Time or other numeric value associated with the penalty
    pub time: u8,

    /// Lap number when the infringement occurred
    pub lap_number: u8,

    /// Number of positions gained as a result of the infringement
    pub places_gained: u8,
}

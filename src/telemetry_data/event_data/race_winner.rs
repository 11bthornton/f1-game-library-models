//! Defines the `RaceWinner` event data structure.

use serde::{Deserialize, Serialize};

/// Race winner event.
///
/// This event occurs when a vehicle crosses the finish line as the winner of the race.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct RaceWinner {
    /// Index of the vehicle that won the race
    #[serde(with = "crate::utils::u8_as_usize")]
    pub vehicle_index: usize,
}

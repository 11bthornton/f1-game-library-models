//! Defines the `StopGoPenaltyServed` event data structure.

use serde::{Deserialize, Serialize};

/// Stop-go penalty served event.
///
/// This event occurs when a vehicle completes a stop-go penalty.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct StopGoPenaltyServed {
    /// Index of the vehicle that served the penalty
    #[serde(with = "crate::utils::u8_as_usize")]
    pub vehicle_index: usize,

    pub stop_time: f32,
}

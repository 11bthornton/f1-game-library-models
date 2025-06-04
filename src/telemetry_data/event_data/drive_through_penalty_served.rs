use serde::{Deserialize, Serialize};

/// Drive-through penalty served event.
///
/// This event occurs when a vehicle completes a drive-through penalty.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct DriveThroughPenaltyServed {
    /// Index of the vehicle that served the penalty
    #[serde(with = "crate::u8_as_usize")]
    pub vehicle_index: usize,
}

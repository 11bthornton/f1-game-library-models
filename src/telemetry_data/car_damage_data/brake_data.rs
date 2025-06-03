use serde::{Deserialize, Serialize};

use crate::telemetry_data::WheelData;

/// Brake damage data.
///
/// This structure contains damage information for all four brakes.
///
/// # Fields
///
/// * `damage` - Brake damage percentage for each wheel [RL, RR, FL, FR]
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct BrakeData {
    /// Brake damage percentage for each wheel [RL, RR, FL, FR]
    pub damage: WheelData<u8>,
}

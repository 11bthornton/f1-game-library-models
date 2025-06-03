use serde::{
    Deserialize,
    Serialize,
};

use crate::telemetry_data::WheelData;

/// Tyre wear and damage data.
///
/// This structure contains wear and damage information for all four tyres.
///
/// # Fields
///
/// * `wear` - Tyre wear percentage for each wheel [RL, RR, FL, FR]
/// * `damage` - Tyre damage percentage for each wheel [RL, RR, FL, FR]
/// * `surface_temperature` - Tyre surface temperature for each wheel [RL, RR, FL, FR] (F1 2023+)
/// * `carcass_temperature` - Tyre carcass temperature for each wheel [RL, RR, FL, FR] (F1 2024+)
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[repr(C)]
pub struct TyreData
{
    /// Tyre wear percentage for each wheel [RL, RR, FL, FR]
    pub wear: WheelData<f32>,
    /// Tyre damage percentage for each wheel [RL, RR, FL, FR]
    pub damage: WheelData<u8>,
}

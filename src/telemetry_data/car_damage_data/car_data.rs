use serde::{Deserialize, Serialize};

use crate::telemetry_data::WheelData;

use super::{BrakeData, EngineWear, TyreData, WingDamage};

/// Damage data for a single car.
///
/// This structure contains detailed damage information for a single car,
/// including tyre wear, brake damage, wing damage, and engine condition.
///
/// # Fields
///
/// * `tyres` - Tyre wear and damage data
/// * `brakes_damage` - Brake damage data
/// * `wing_damage` - Wing damage data
/// * `floor_damage` - Floor damage percentage
/// * `diffuser_damage` - Diffuser damage percentage
/// * `sidepod_damage` - Sidepod damage percentage
/// * `drs_fault` - Whether the DRS system is faulty
/// * `ers_fault` - Whether the ERS system is faulty
/// * `gear_box_damage` - Gearbox damage percentage
/// * `engine_damage` - Engine damage percentage
/// * `engine_wear` - Engine component wear data
/// * `engine_blown` - Whether the engine has blown
/// * `engine_seized` - Whether the engine has seized
/// * `battery_damage` - Battery damage percentage (F1 2023+)
/// * `mgu_k_damage` - MGU-K damage percentage (F1 2024+)
/// * `mgu_h_damage` - MGU-H damage percentage (F1 2024+)
/// * `active_aero_damage` - Active aero damage percentage (F1 2025+)
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct CarDamageData {
    /// Tyre wear and damage data
    pub tyres: TyreData,
    /// Brake damage data
    pub brakes_damage: BrakeData,

    pub blisters: WheelData<f32>,

    /// Wing damage data
    pub wing_damage: WingDamage,
    /// Floor damage percentage
    pub floor_damage: u8,
    /// Diffuser damage percentage
    pub diffuser_damage: u8,
    /// Sidepod damage percentage
    pub sidepod_damage: u8,
    /// Whether the DRS system is faulty
    pub drs_fault: bool,
    /// Whether the ERS system is faulty
    pub ers_fault: bool,
    /// Gearbox damage percentage
    pub gear_box_damage: u8,
    /// Engine damage percentage
    pub engine_damage: u8,
    /// Engine component wear data
    pub engine_wear: EngineWear,
    /// Whether the engine has blown
    pub engine_blown: bool,
    /// Whether the engine has seized
    pub engine_seized: bool,
    // pub another_field_that_should_not_work: [f32; 16]
}

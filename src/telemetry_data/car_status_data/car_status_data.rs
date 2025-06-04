use serde::{Deserialize, Serialize};

use super::{
    VehicleFiaFlags,
    driver_aids::{AntiLockBrakes, FuelMix, PitLimiterStatus, TractionControl},
    tyre_compounds::{ActualTyreCompound, VisualTyreCompound},
};

/// Status data for a single car.
///
/// This structure contains detailed status information for a single car,
/// including driver aids, fuel status, ERS deployment, and tyre information.
///
/// # Fields
///
/// * `traction_control` - Traction control setting
/// * `anti_lock_brakes` - Anti-lock brakes setting
/// * `fuel_mix` - Fuel mix setting
/// * `front_brake_bias` - Front brake bias (percentage)
/// * `pit_limiter_status` - Pit limiter status
/// * `fuel_in_tank` - Current fuel mass
/// * `fuel_capacity` - Fuel capacity
/// * `fuel_remaining_laps` - Fuel remaining in terms of laps (value on MFD)
/// * `max_rpm` - Car's max RPM, point of rev limiter
/// * `idle_rpm` - Car's idle RPM
/// * `max_gears` - Maximum number of gears
/// * `drs_allowed` - Whether DRS is allowed
/// * `drs_activation_distance` - Distance to DRS activation point
/// * `actual_tyre_compound` - Actual tyre compound used
/// * `visual_tyre_compound` - Visual tyre compound displayed
/// * `tyres_age_laps` - Age in laps of the current set of tyres
/// * `vehicle_fia_flags` - FIA flag being shown to the car
/// * `engine_power_ice` - Power output of the internal combustion engine
/// * `enginer_power_mguk` - Power output of the MGU-K
/// * `ers_store_energy` - ERS energy store in Joules
/// * `ers_deploy_mode` - ERS deployment mode
/// * `ers_harvested_this_lap_mguk` - ERS energy harvested this lap by MGU-K
/// * `ers_harvested_this_lap_mguh` - ERS energy harvested this lap by MGU-H
/// * `ers_deployed_this_lap` - ERS energy deployed this lap
/// * `network_paused` - Whether the car is paused in a network game
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct CarStatusData {
    /// Traction control setting (0 = off, 1 = medium, 2 = full)
    pub traction_control: TractionControl,
    /// Anti-lock brakes setting (0 = off, 1 = on)
    pub anti_lock_brakes: AntiLockBrakes,
    /// Fuel mix setting (0 = lean, 1 = standard, 2 = rich, 3 = max)
    pub fuel_mix: FuelMix,
    /// Front brake bias (percentage)
    pub front_brake_bias: u8,
    /// Pit limiter status (0 = off, 1 = on)
    pub pit_limiter_status: PitLimiterStatus,
    /// Current fuel mass in kg
    pub fuel_in_tank: f32,
    /// Fuel capacity in kg
    pub fuel_capacity: f32,
    /// Fuel remaining in terms of laps (value on MFD)
    pub fuel_remaining_laps: f32,
    /// Car's max RPM, point of rev limiter
    pub max_rpm: u16,
    /// Car's idle RPM
    pub idle_rpm: u16,
    /// Maximum number of gears
    pub max_gears: u8,
    /// Whether DRS is allowed (false = not allowed, true = allowed)
    pub drs_allowed: bool,
    /// Distance to DRS activation point in meters (0 = DRS not available)
    pub drs_activation_distance: u16,
    /// Actual tyre compound used
    pub actual_tyre_compound: ActualTyreCompound,
    /// Visual tyre compound displayed (can be different from actual compound)
    pub visual_tyre_compound: VisualTyreCompound,
    /// Age in laps of the current set of tyres
    pub tyres_age_laps: u8,
    /// FIA flag being shown to the car (-1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 =
    /// yellow, 4 = red)
    pub vehicle_fia_flags: VehicleFiaFlags,
    /// Power output of the internal combustion engine
    pub engine_power_ice: f32,
    /// Power output of the MGU-K
    pub engine_power_mguk: f32,
    /// ERS energy store in Joules
    pub ers_store_energy: f32,
    /// ERS deployment mode
    pub ers_deploy_mode: u8,
    /// ERS energy harvested this lap by MGU-K in Joules
    pub ers_harvested_this_lap_mguk: f32,
    /// ERS energy harvested this lap by MGU-H in Joules
    pub ers_harvested_this_lap_mguh: f32,
    /// ERS energy deployed this lap in Joules
    pub ers_deployed_this_lap: f32,
    /// Whether the car is paused in a network game
    pub network_paused: bool,
}

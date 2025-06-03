use serde::{Deserialize, Serialize};

use crate::telemetry_data::WheelData;

/// Telemetry data for a single car.
///
/// This structure contains detailed telemetry information for a single car,
/// including speed, throttle, brake, gear, engine RPM, and various temperature readings.
///
/// # Fields
///
/// * `speed` - Speed of the car in km/h
/// * `throttle` - Throttle application (0.0 to 1.0)
/// * `steer` - Steering (-1.0 (full left) to 1.0 (full right))
/// * `brake` - Brake application (0.0 to 1.0)
/// * `clutch` - Clutch application (0 to 100)
/// * `gear` - Gear selected (1-8, N=0, R=-1)
/// * `engine_rpm` - Engine RPM
/// * `drs` - DRS status (0 = off, 1 = on)
/// * `rev_lights_percent` - Rev lights indicator (0 to 100)
/// * `rev_light_bit` - Rev lights bitfield
/// * `brake_temps` - Brake temperatures for each wheel [RL, RR, FL, FR]
/// * `tyre_surface_temps` - Tyre surface temperatures for each wheel [RL, RR, FL, FR]
/// * `tyre_inner_temps` - Tyre inner temperatures for each wheel [RL, RR, FL, FR]
/// * `engine_temp` - Engine temperature in degrees Celsius
/// * `tyre_pressures` - Tyre pressures for each wheel [RL, RR, FL, FR]
/// * `surface_types` - Driving surface type for each wheel [RL, RR, FL, FR]
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct CarTelemetryData {
    /// Speed of the car in km/h
    pub speed: u16,
    /// Throttle application (0.0 to 1.0)
    pub throttle: f32,
    /// Steering (-1.0 (full left) to 1.0 (full right))
    pub steer: f32,
    /// Brake application (0.0 to 1.0)
    pub brake: f32,
    /// Clutch application (0 to 100)
    pub clutch: u8,
    /// Gear selected (1-8, N=0, R=-1)
    pub gear: i8,
    /// Engine RPM
    pub engine_rpm: u16,
    /// DRS status (0 = off, 1 = on)
    pub drs: u8,
    /// Rev lights indicator (0 to 100)
    pub rev_lights_percent: u8,
    /// Rev lights bitfield
    pub rev_light_bit: u16,
    /// Brake temperatures for each wheel [RL, RR, FL, FR]
    pub brake_temps: WheelData<u16>,
    /// Tyre surface temperatures for each wheel [RL, RR, FL, FR]
    pub tyre_surface_temps: WheelData<u8>,
    /// Tyre inner temperatures for each wheel [RL, RR, FL, FR]
    pub tyre_inner_temps: WheelData<u8>,
    /// Engine temperature in degrees Celsius
    pub engine_temp: u16,
    /// Tyre pressures for each wheel [RL, RR, FL, FR]
    pub tyre_pressures: WheelData<f32>,
    /// Driving surface type for each wheel [RL, RR, FL, FR]
    pub surface_types: WheelData<u8>,
}

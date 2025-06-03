use serde::{Deserialize, Serialize};

/// Setup data for a single car.
///
/// This structure contains detailed setup information for a single car,
/// including aerodynamic, suspension, brake, and tyre settings.
///
/// # Fields
///
/// * `front_wing` - Front wing aero setting
/// * `rear_wing` - Rear wing aero setting
/// * `on_throttle` - Differential adjustment on throttle (percentage)
/// * `off_throttle` - Differential adjustment off throttle (percentage)
/// * `front_camber` - Front camber angle (suspension geometry)
/// * `rear_camber` - Rear camber angle (suspension geometry)
/// * `front_toe` - Front toe angle (suspension geometry)
/// * `rear_toe` - Rear toe angle (suspension geometry)
/// * `front_suspension` - Front suspension stiffness
/// * `rear_suspension` - Rear suspension stiffness
/// * `front_anti_roll_bar` - Front anti-roll bar stiffness
/// * `rear_anti_roll_bar` - Rear anti-roll bar stiffness
/// * `front_suspension_height` - Front ride height
/// * `rear_suspension_height` - Rear ride height
/// * `brake_pressure` - Brake pressure (percentage)
/// * `brake_bias` - Brake bias (percentage)
/// * `rear_left_tyre_pressure` - Rear left tyre pressure (PSI)
/// * `rear_right_tyre_pressure` - Rear right tyre pressure (PSI)
/// * `front_left_tyre_pressure` - Front left tyre pressure (PSI)
/// * `front_right_tyre_pressure` - Front right tyre pressure (PSI)
/// * `ballast` - Ballast weight
/// * `fuel_load` - Fuel load weight
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct CarSetupData {
    /// Front wing aero setting
    pub front_wing: u8,

    /// Rear wing aero setting
    pub rear_wing: u8,

    /// Differential adjustment on throttle (percentage)
    pub on_throttle: u8,

    /// Differential adjustment off throttle (percentage)
    pub off_throttle: u8,

    /// Front camber angle (suspension geometry)
    pub front_camber: f32,

    /// Rear camber angle (suspension geometry)
    pub rear_camber: f32,

    /// Front toe angle (suspension geometry)
    pub front_toe: f32,

    /// Rear toe angle (suspension geometry)
    pub rear_toe: f32,

    /// Front suspension stiffness
    pub front_suspension: u8,

    /// Rear suspension stiffness
    pub rear_suspension: u8,

    /// Front anti-roll bar stiffness
    pub front_anti_roll_bar: u8,

    /// Rear anti-roll bar stiffness
    pub rear_anti_roll_bar: u8,

    /// Front ride height
    pub front_suspension_height: u8,

    /// Rear ride height
    pub rear_suspension_height: u8,

    /// Brake pressure (percentage)
    pub brake_pressure: u8,

    /// Brake bias (percentage)
    pub brake_bias: u8,

    pub engine_braking: u8,

    /// Rear left tyre pressure (PSI)
    pub rear_left_tyre_pressure: u8,

    /// Rear right tyre pressure (PSI)
    pub rear_right_tyre_pressure: u8,

    /// Front left tyre pressure (PSI)
    pub front_left_tyre_pressure: u8,

    /// Front right tyre pressure (PSI)
    pub front_right_tyre_pressure: u8,

    /// Ballast weight
    pub ballast: u8,

    /// Fuel load weight
    pub fuel_load: u8,
}

use serde::{
    Deserialize,
    Serialize,
};

/// Speed trap event.
///
/// This event occurs when a vehicle triggers a speed trap.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct SpeedTrap
{
    /// Index of the vehicle that triggered the speed trap
    #[serde(with = "crate::utils::u8_as_usize")]
    pub vehicle_index: usize,
    /// Speed of the vehicle in km/h
    pub speed: f32,
    /// Whether this was the fastest speed in the session (1 = yes, 0 = no)
    pub overall_fastest_in_session: bool,
    /// Whether this was the driver's fastest speed in the session (1 = yes, 0 = no)
    pub driver_fastest_in_session: bool,

    #[serde(with = "crate::utils::u8_as_usize")]
    pub fastest_vehicle_index_in_session: usize,

    pub fastest_speed_in_session: f32,
}

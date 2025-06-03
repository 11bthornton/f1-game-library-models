use serde::{
    Deserialize,
    Serialize,
};

/// Fastest lap event.
///
/// This event occurs when a vehicle sets the fastest lap time of the session.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct FastestLap
{
    /// Index of the vehicle that set the fastest lap
    #[serde(with = "crate::utils::u8_as_usize")]
    pub vehicle_index: usize,
    /// Time of the fastest lap in seconds
    pub lap_time: f32,
}

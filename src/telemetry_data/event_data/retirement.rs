use serde::{Deserialize, Serialize};

/// Retirement event.
///
/// This event occurs when a vehicle retires from the session.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct Retirement {
    /// Index of the vehicle that retired
    #[serde(with = "crate::utils::u8_as_usize")]
    pub vehicle_index: usize,
}

use serde::{Deserialize, Serialize};

/// DRS disabled event.
///
/// This event occurs when DRS (Drag Reduction System) is disabled for the session.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct Collision {
    #[serde(with = "crate::utils::u8_as_usize")]
    pub primary_car_index: usize,

    #[serde(with = "crate::utils::u8_as_usize")]
    pub secondary_car_index: usize,
}

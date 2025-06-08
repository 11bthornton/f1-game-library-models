//! Defines the `Collision` event data structure.

use serde::{Deserialize, Serialize};

/// Collision event.
///
/// Occurs when two cars collide during a session.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct Collision {
    #[serde(with = "crate::utils::u8_as_usize")]
    pub primary_car_index: usize,

    #[serde(with = "crate::utils::u8_as_usize")]
    pub secondary_car_index: usize,
}

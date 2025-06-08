//! Defines the SafetyCar struct, which represents the safety car event in telemetry data.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct SafetyCar {
    pub safety_car_type: u8,
    pub event_type: u8,
}

//! Defines the wheel data structure for telemetry.

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Clone, Copy, Serialize)]
pub struct WheelData<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}
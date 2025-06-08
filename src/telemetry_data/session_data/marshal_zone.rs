//! Defines a marshal zone on the track
//!
//! Marshal zones are sections of the track that can have different flag states

use serde::{Deserialize, Serialize};

use crate::telemetry_data::car_status_data::VehicleFiaFlags;

/// Represents a marshal zone on the track
///
/// Marshal zones are sections of the track that can have different flag states
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct MarshalZone {
    /// Zone start fraction (0.0-1.0) of track distance
    pub zone_start: f32,
    /// Flag status in this zone
    pub zone_flag: VehicleFiaFlags,
}

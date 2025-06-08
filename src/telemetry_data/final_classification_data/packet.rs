//! Defines the structure for the final classification data packet in telemetry data.

use serde::{Deserialize, Serialize};

use super::final_classification_data::FinalClassificationData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

/// Packet containing final classification data for all cars in the session.
///
/// This structure contains the final classification data for all cars at the end of a race,
/// including their finishing positions, lap times, and penalties.
///
/// # Fields
///
/// * `header` - Header information for the packet
/// * `num_cars` - Number of cars in the final classification
/// * `classification_data` - Array of classification data for each car (up to 22 cars)
#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub struct PacketClassificationData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Number of cars in the final classification
    pub num_cars: u8,
    /// Array of classification data for each car (up to 22 cars)
    pub classification_data: [FinalClassificationData; MAX_CARS_IN_SESSION],
}

//! Defines the car setup data packet structure.

use serde::{Deserialize, Serialize};

use super::car_setup_data::CarSetupData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
pub struct PacketCarSetupData {
    /// Header information for the packet
    pub header: PacketHeader,

    /// Array of setup data for each car (up to 22 cars)
    pub car_setups: [CarSetupData; MAX_CARS_IN_SESSION],

    pub next_front_wing_value: f32,
}

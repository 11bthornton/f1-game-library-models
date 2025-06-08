use serde::{Deserialize, Serialize};

use super::lap_data::LapData;
use crate::{constants::MAX_CARS_IN_SESSION, telemetry_data::packet_header::PacketHeader};

/// Packet containing lap data for all cars in the session.
///
/// This structure contains lap data for all cars on track during a session.
///
/// # Fields
///
/// * `header` - Header information for the packet
/// * `lap_data` - Array of lap data for each car (up to 22 cars)
/// * `time_trial_pb_car_idx` - Index of personal best car in time trial
/// * `m_time_trial_rival_car_idx` - Index of rival car in time trial
#[derive(Deserialize, Debug, Default, Serialize, Clone, Copy, PartialEq)]
pub struct PacketLapData {
    /// Header information for the packet
    pub header: PacketHeader,
    /// Array of lap data for each car (up to 22 cars)
    pub lap_data: [LapData; MAX_CARS_IN_SESSION],

    /// Index of personal best car in time trial
    #[serde(with = "crate::utils::u8_as_usize")]
    pub time_trial_pb_car_idx: usize,
    /// Index of rival car in time trial
    #[serde(with = "crate::utils::u8_as_usize")]
    pub m_time_trial_rival_car_idx: usize,
}

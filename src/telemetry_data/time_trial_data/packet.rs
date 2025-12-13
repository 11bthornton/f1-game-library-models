//! Defines the time trial data packet structure.

use serde::{Deserialize, Serialize};

use crate::telemetry_data::{
    AntiLockBrakes, GearboxAssist, Team, TractionControl, packet_header::PacketHeader,
};

#[derive(Deserialize, Debug, Serialize, Copy, Clone, PartialEq)]
pub struct PacketTimeTrialData {
    pub header: PacketHeader,

    #[serde(with = "crate::utils::u8_as_usize")]
    pub car_index: usize,

    pub team: Team,

    pub lap_time_in_ms: u32,

    pub sector1_time_in_ms: u32,

    pub sector2_time_in_ms: u32,

    pub sector3_time_in_ms: u32,

    pub traction_control: TractionControl,

    pub gearbox_assist: GearboxAssist,

    pub anti_lock_brakes: AntiLockBrakes,

    pub equal_car_performance: u8,

    pub custom_setup: bool,

    pub valid: bool,
}

impl Default for PacketTimeTrialData {
    fn default() -> Self {
        Self {
            header: PacketHeader {
                packet_id: crate::telemetry_data::packet_header::PacketId::TimeTrialPacket,
                ..PacketHeader::default()
            },
            car_index: 0,
            team: Team::Mercedes,
            lap_time_in_ms: 0,
            sector1_time_in_ms: 0,
            sector2_time_in_ms: 0,
            sector3_time_in_ms: 0,
            traction_control: TractionControl::Off,
            gearbox_assist: GearboxAssist::Manual,
            anti_lock_brakes: AntiLockBrakes::Off,
            equal_car_performance: 0,
            custom_setup: false,
            valid: false,
        }
    }
}
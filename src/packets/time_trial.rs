//! Wire-format structures for the Time Trial packet (packet id 14).
//!
//! Spec: `PacketTimeTrialData` — 101 bytes total.
//! Payload: three `TimeTrialDataSet` structs (24 bytes each) — 72 bytes.
//!
//! The three datasets are named rather than arrayed since they have distinct
//! semantics: player session best, personal best, and rival.

use std::mem::size_of;

use crate::constants::{PACKET_HEADER_SIZE, TIME_TRIAL_DATA_PACKET_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::Team,
    macros::{wire_enum_accessors, wire_field_accessors, wire_flag_accessors, wire_index_accessors},
};

/// Data for a single time trial entry (session best, personal best, or rival).
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TimeTrialDataSet {
    car_idx: u8,
    team_id: u8,
    lap_time_in_ms: u32,
    sector1_time_in_ms: u32,
    sector2_time_in_ms: u32,
    sector3_time_in_ms: u32,
    traction_control: u8,
    gearbox_assist: u8,
    anti_lock_brakes: u8,
    /// `0` = Realistic, `1` = Equal.
    equal_car_performance: u8,
    custom_setup: u8,
    valid: u8,
}

const _: () = assert!(size_of::<TimeTrialDataSet>() == 24);

impl TimeTrialDataSet {
    wire_index_accessors!(car_idx);
    wire_enum_accessors!(team_id => Team);
    wire_field_accessors!(
        lap_time_in_ms: u32,
        sector1_time_in_ms: u32,
        sector2_time_in_ms: u32,
        sector3_time_in_ms: u32,
    );
    wire_flag_accessors!(
        traction_control,
        gearbox_assist,
        anti_lock_brakes,
        equal_car_performance,
        custom_setup,
        valid,
    );
}

impl FixEndianness for TimeTrialDataSet {
    fn fix_endianness(self) -> Self {
        Self {
            lap_time_in_ms: self.lap_time_in_ms.fix_endianness(),
            sector1_time_in_ms: self.sector1_time_in_ms.fix_endianness(),
            sector2_time_in_ms: self.sector2_time_in_ms.fix_endianness(),
            sector3_time_in_ms: self.sector3_time_in_ms.fix_endianness(),
            // all remaining fields are u8 — no byte-swapping needed
            ..self
        }
    }
}

/// The three time trial datasets carried in a single packet.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TimeTrialPayload {
    pub player_session_best: TimeTrialDataSet,
    pub personal_best: TimeTrialDataSet,
    pub rival: TimeTrialDataSet,
}

const _: () = assert!(size_of::<TimeTrialPayload>() == TIME_TRIAL_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format time trial packet.
///
/// `header.packet_id` will be `14` for this packet type.
pub type PacketTimeTrial = Packet<TimeTrialPayload>;

const _: () = assert!(size_of::<PacketTimeTrial>() == TIME_TRIAL_DATA_PACKET_SIZE);

impl FixEndianness for TimeTrialPayload {
    fn fix_endianness(self) -> Self {
        Self {
            player_session_best: self.player_session_best.fix_endianness(),
            personal_best: self.personal_best.fix_endianness(),
            rival: self.rival.fix_endianness(),
        }
    }
}

//! Wire-format structures for the Session History packet (packet id 11).
//!
//! Spec: `PacketSessionHistoryData` — 1460 bytes total.
//! Per-lap struct: `LapHistoryData` — 14 bytes.
//! Per-stint struct: `TyreStintHistoryData` — 3 bytes.
//!
//! Single-car packet. Contains up to 100 laps of lap time / sector time data
//! and up to 8 tyre stints. Sent only for the player's car during the session.

use std::mem::size_of;

use crate::constants::{PACKET_HEADER_SIZE, SESSION_HISTORY_DATA_PACKET_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::{ActualTyreCompound, VisualTyreCompound},
    macros::{wire_enum_accessors, wire_index_accessors},
};

const MAX_LAPS_IN_HISTORY: usize = 100;
const MAX_TYRE_STINTS: usize = 8;

/// Wire-format lap history entry for a single lap.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LapHistoryData {
    pub lap_time_in_ms: u32,
    pub sector1_time_ms_part: u16,
    pub sector1_time_minutes_part: u8,
    pub sector2_time_ms_part: u16,
    pub sector2_time_minutes_part: u8,
    pub sector3_time_ms_part: u16,
    pub sector3_time_minutes_part: u8,
    /// Bitmask: 0x01 lap valid, 0x02 sector 1 valid, 0x04 sector 2 valid, 0x08 sector 3 valid.
    lap_valid_bit_flags: u8,
}

const _: () = assert!(size_of::<LapHistoryData>() == 14);

impl LapHistoryData {
    pub fn is_lap_valid(self) -> bool {
        self.lap_valid_bit_flags & 0x01 != 0
    }

    pub fn is_sector1_valid(self) -> bool {
        self.lap_valid_bit_flags & 0x02 != 0
    }

    pub fn is_sector2_valid(self) -> bool {
        self.lap_valid_bit_flags & 0x04 != 0
    }

    pub fn is_sector3_valid(self) -> bool {
        self.lap_valid_bit_flags & 0x08 != 0
    }
}

impl FixEndianness for LapHistoryData {
    fn fix_endianness(self) -> Self {
        Self {
            lap_time_in_ms: self.lap_time_in_ms.fix_endianness(),
            sector1_time_ms_part: self.sector1_time_ms_part.fix_endianness(),
            sector2_time_ms_part: self.sector2_time_ms_part.fix_endianness(),
            sector3_time_ms_part: self.sector3_time_ms_part.fix_endianness(),
            ..self
        }
    }
}

/// Wire-format tyre stint history entry.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TyreStintHistoryData {
    /// Lap the tyre usage ends on. `255` if this is the current tyre.
    pub end_lap: u8,
    tyre_actual_compound: u8,
    tyre_visual_compound: u8,
}

const _: () = assert!(size_of::<TyreStintHistoryData>() == 3);

impl TyreStintHistoryData {
    wire_enum_accessors!(
        tyre_actual_compound => ActualTyreCompound,
        tyre_visual_compound => VisualTyreCompound,
    );
}

impl FixEndianness for TyreStintHistoryData {
    fn fix_endianness(self) -> Self {
        // All fields are u8 — no byte swapping needed.
        self
    }
}

/// Wire-format session history payload (single car).
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SessionHistoryData {
    /// Index of the car this data relates to.
    car_idx: u8,
    /// Number of laps in the data (including the current partial lap).
    pub num_laps: u8,
    /// Number of tyre stints in the data.
    pub num_tyre_stints: u8,
    /// Lap number on which the best overall lap time was set.
    pub best_lap_time_lap_num: u8,
    /// Lap number on which the best sector 1 time was set.
    pub best_sector1_lap_num: u8,
    /// Lap number on which the best sector 2 time was set.
    pub best_sector2_lap_num: u8,
    /// Lap number on which the best sector 3 time was set.
    pub best_sector3_lap_num: u8,
    pub lap_history_data: [LapHistoryData; MAX_LAPS_IN_HISTORY],
    pub tyre_stints_history_data: [TyreStintHistoryData; MAX_TYRE_STINTS],
}

const _: () = assert!(size_of::<SessionHistoryData>() == SESSION_HISTORY_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format session history packet (single car).
///
/// `header.packet_id` will be `11` for this packet type.
pub type PacketSessionHistory = Packet<SessionHistoryData>;

const _: () = assert!(size_of::<PacketSessionHistory>() == SESSION_HISTORY_DATA_PACKET_SIZE);

impl SessionHistoryData {
    wire_index_accessors!(car_idx);
}

impl FixEndianness for SessionHistoryData {
    fn fix_endianness(self) -> Self {
        Self {
            lap_history_data: self.lap_history_data.fix_endianness(),
            ..self
        }
    }
}

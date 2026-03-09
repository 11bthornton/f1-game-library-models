//! Wire-format structures for the Lap Positions packet (packet id 15).
//!
//! Spec: `PacketLapPositionsData` — 1131 bytes total.
//! Data struct: `LapPositionsData` — 1102 bytes.
//!
//! Contains the grid position of each car at the start of every lap,
//! allowing a lap-by-lap position chart to be constructed.

use std::mem::size_of;

use crate::constants::{LAP_POSITIONS_DATA_PACKET_SIZE, MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE};

use super::super::{Packet, endian::FixEndianness};

/// Maximum number of laps stored in the lap positions history.
const MAX_LAPS: usize = 50;

/// Wire-format lap positions payload.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LapPositionsData {
    /// Number of laps with recorded position data.
    pub num_laps: u8,
    /// Index of the first lap in the data (0-indexed).
    pub lap_start: u8,
    /// Position of each car at the start of each lap.
    ///
    /// Indexed as `[lap][vehicle]`. A value of `0` means no record.
    pub position_for_vehicle_idx: [[u8; MAX_CARS_IN_SESSION]; MAX_LAPS],
}

const _: () = assert!(size_of::<LapPositionsData>() == LAP_POSITIONS_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format lap positions packet.
///
/// `header.packet_id` will be `15` for this packet type.
pub type PacketLapPositions = Packet<LapPositionsData>;

const _: () = assert!(size_of::<PacketLapPositions>() == LAP_POSITIONS_DATA_PACKET_SIZE);

impl FixEndianness for LapPositionsData {
    fn fix_endianness(self) -> Self {
        // All fields are u8 — no byte swapping needed.
        self
    }
}

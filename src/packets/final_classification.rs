//! Wire-format structures for the Final Classification packet (packet id 8).
//!
//! Spec: `PacketFinalClassificationData` — 1042 bytes total.
//! Per-car struct: `FinalClassificationData` — 46 bytes.
//!
//! The only packet containing a `f64` field (`total_race_time`).

use std::mem::size_of;

use crate::constants::{CLASSIFICATION_DATA_PACKET_SIZE, MAX_CARS_IN_SESSION, PACKET_HEADER_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::{ActualTyreCompound, ResultReason, ResultStatus, VisualTyreCompound},
    macros::wire_enum_accessors,
};

const MAX_TYRE_STINTS: usize = 8;

/// Wire-format final classification data for a single car.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FinalClassificationData {
    /// Finishing position.
    pub position: u8,
    /// Number of laps completed.
    pub num_laps: u8,
    /// Grid position.
    pub grid_position: u8,
    /// Championship points scored.
    pub points: u8,
    /// Number of pit stops made.
    pub num_pit_stops: u8,
    result_status: u8,
    result_reason: u8,
    /// Best lap time of the session in milliseconds.
    pub best_lap_time_in_ms: u32,
    /// Total race time in seconds (excluding penalties).
    pub total_race_time: f64,
    /// Total penalty time accumulated in seconds.
    pub penalties_time: u8,
    /// Number of penalties applied.
    pub num_penalties: u8,
    /// Number of tyre stints.
    pub num_tyre_stints: u8,
    tyre_stints_actual: [u8; MAX_TYRE_STINTS],
    tyre_stints_visual: [u8; MAX_TYRE_STINTS],
    /// Lap number each stint ended on.
    pub tyre_stints_end_laps: [u8; MAX_TYRE_STINTS],
}

const _: () = assert!(size_of::<FinalClassificationData>() == 46);

impl FinalClassificationData {
    wire_enum_accessors!(
        result_status => ResultStatus,
        result_reason => ResultReason,
    );

    /// Actual tyre compound used in each stint.
    pub fn tyre_stints_actual(&self) -> [Result<ActualTyreCompound, u8>; MAX_TYRE_STINTS] {
        self.tyre_stints_actual
            .map(|v| ActualTyreCompound::try_from(v).map_err(|_| v))
    }

    /// Visual tyre compound used in each stint.
    pub fn tyre_stints_visual(&self) -> [Result<VisualTyreCompound, u8>; MAX_TYRE_STINTS] {
        self.tyre_stints_visual
            .map(|v| VisualTyreCompound::try_from(v).map_err(|_| v))
    }
}

impl FixEndianness for FinalClassificationData {
    fn fix_endianness(self) -> Self {
        Self {
            best_lap_time_in_ms: self.best_lap_time_in_ms.fix_endianness(),
            total_race_time: self.total_race_time.fix_endianness(),
            ..self
        }
    }
}

/// Wire-format final classification payload.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FinalClassificationPayload {
    /// Number of cars in the final classification.
    pub num_cars: u8,
    pub classification_data: [FinalClassificationData; MAX_CARS_IN_SESSION],
}

const _: () = assert!(size_of::<FinalClassificationPayload>() == CLASSIFICATION_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format final classification packet.
///
/// `header.packet_id` will be `8` for this packet type.
pub type PacketFinalClassification = Packet<FinalClassificationPayload>;

const _: () = assert!(size_of::<PacketFinalClassification>() == CLASSIFICATION_DATA_PACKET_SIZE);

impl FixEndianness for FinalClassificationPayload {
    fn fix_endianness(self) -> Self {
        Self {
            classification_data: self.classification_data.fix_endianness(),
            ..self
        }
    }
}

//! Wire-format structures for the Tyre Sets packet (packet id 12).
//!
//! Spec: `PacketTyreSetsData` — 231 bytes total.
//! Per-set struct: `TyreSetData` — 10 bytes.
//! Total sets: 20 (13 slick + 7 wet).

use std::mem::size_of;

use crate::constants::{MAX_TYRE_SETS, PACKET_HEADER_SIZE, TYRE_SETS_DATA_PACKET_SIZE};

use super::super::{
    Packet,
    endian::FixEndianness,
    enums::{ActualTyreCompound, VisualTyreCompound},
    macros::{wire_enum_accessors, wire_field_accessors, wire_flag_accessors, wire_index_accessors},
};
use super::session::enums::SessionType;

/// Wire-format data for a single tyre set.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TyreSetData {
    actual_tyre_compound: u8,
    visual_tyre_compound: u8,
    pub wear: u8,
    available: u8,
    recommended_session: u8,
    /// Laps remaining in this tyre set.
    pub life_span: u8,
    /// Maximum recommended laps for this compound.
    pub usable_life: u8,
    /// Lap delta time in milliseconds vs the currently fitted set.
    lap_delta_time: i16,
    fitted: u8,
}

const _: () = assert!(size_of::<TyreSetData>() == 10);

impl TyreSetData {
    wire_enum_accessors!(
        actual_tyre_compound => ActualTyreCompound,
        visual_tyre_compound => VisualTyreCompound,
        recommended_session  => SessionType,
    );
    wire_flag_accessors!(available, fitted);
    wire_field_accessors!(lap_delta_time: i16);
}

impl FixEndianness for TyreSetData {
    fn fix_endianness(self) -> Self {
        Self {
            lap_delta_time: self.lap_delta_time.fix_endianness(),
            ..self
        }
    }
}

/// Payload of the tyre sets packet.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TyreSetsPayload {
    car_idx: u8,
    pub tyre_sets: [TyreSetData; MAX_TYRE_SETS],
    /// Index into `tyre_sets` of the currently fitted set.
    fitted_idx: u8,
}

const _: () = assert!(size_of::<TyreSetsPayload>() == TYRE_SETS_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format tyre sets packet (single car).
///
/// `header.packet_id` will be `12` for this packet type.
pub type PacketTyreSets = Packet<TyreSetsPayload>;

const _: () = assert!(size_of::<PacketTyreSets>() == TYRE_SETS_DATA_PACKET_SIZE);

impl TyreSetsPayload {
    wire_index_accessors!(car_idx, fitted_idx);
}

impl FixEndianness for TyreSetsPayload {
    fn fix_endianness(self) -> Self {
        Self {
            tyre_sets: self.tyre_sets.fix_endianness(),
            ..self
        }
    }
}

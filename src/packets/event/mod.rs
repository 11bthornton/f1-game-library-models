//! Wire-format structures for the Event packet (packet id 3).
//!
//! Spec: `PacketEventData` — 45 bytes total.
//!
//! Unlike other packets this one carries a C++ union. The parse strategy is:
//!
//! 1. Read the fixed 45-byte packet normally via `read_packet` (fits `define_packets!`).
//! 2. Call [`EventPayload::event_data`] which reads the 4-byte ASCII code and
//!    dispatches to the correct variant struct via `ptr::read_unaligned`.
//!
//! `EventPayload::fix_endianness` is a no-op because endianness is corrected
//! per-variant inside `event_data()` after the variant type is known.

pub mod variants;

use std::{mem::size_of, ptr};

use crate::constants::{EVENT_DATA_PACKET_SIZE, PACKET_HEADER_SIZE};

use super::super::Packet;
use super::super::endian::FixEndianness;
use variants::*;

/// Size of the largest union member (`SpeedTrap` — 12 bytes).
const UNION_SIZE: usize = 12;

/// Wire-format event payload: 4-byte ASCII code + raw union bytes.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct EventPayload {
    pub event_string_code: [u8; 4],
    details: [u8; UNION_SIZE],
}

const _: () = assert!(size_of::<EventPayload>() == EVENT_DATA_PACKET_SIZE - PACKET_HEADER_SIZE);

/// Wire-format event packet.
///
/// `header.packet_id` will be `3` for this packet type.
pub type PacketEvent = Packet<EventPayload>;

const _: () = assert!(size_of::<PacketEvent>() == EVENT_DATA_PACKET_SIZE);

impl FixEndianness for EventPayload {
    fn fix_endianness(self) -> Self {
        // Endianness is corrected per-variant in event_data() once the type is known.
        self
    }
}

/// Typed event data, one variant per event string code.
#[derive(Debug, Clone, Copy)]
pub enum EventData {
    /// "SSTA" — session started.
    SessionStarted,
    /// "SEND" — session ended.
    SessionEnded,
    /// "FTLP" — a car set a new fastest lap.
    FastestLap(FastestLap),
    /// "RTMT" — a car retired from the session.
    Retirement(Retirement),
    /// "DRSE" — DRS has been enabled.
    DrsEnabled,
    /// "DRSD" — DRS has been disabled.
    DrsDisabled(DrsDisabled),
    /// "TMPT" — team mate has entered the pits.
    TeamMateInPits(TeamMateInPits),
    /// "CHQF" — chequered flag has been waved.
    ChequeredFlag,
    /// "RCWN" — race winner announced.
    RaceWinner(RaceWinner),
    /// "PENA" — a penalty has been issued.
    Penalty(Penalty),
    /// "SPTP" — a car triggered the speed trap.
    SpeedTrap(SpeedTrap),
    /// "STLG" — start lights are showing.
    StartLights(StartLights),
    /// "LGOT" — lights out — race start.
    LightsOut,
    /// "DTSV" — a drive-through penalty has been served.
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    /// "SGSV" — a stop-go penalty has been served.
    StopGoPenaltyServed(StopGoPenaltyServed),
    /// "FLBK" — flashback activated.
    Flashback(Flashback),
    /// "BUTN" — button input received.
    Buttons(Buttons),
    /// "RDFL" — red flag shown.
    RedFlag,
    /// "OVTK" — an overtake has occurred.
    Overtake(Overtake),
    /// "SCAR" — safety car event.
    SafetyCar(SafetyCar),
    /// "COLL" — collision between two cars.
    Collision(Collision),
}

/// Error returned when the event string code is not recognised.
#[derive(Debug, Clone, Copy)]
pub struct UnknownEventCode(pub [u8; 4]);

impl std::fmt::Display for UnknownEventCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown event code: {:?}", self.0.map(|b| b as char))
    }
}

impl std::error::Error for UnknownEventCode {}

impl EventPayload {
    /// Decode the event payload into a typed [`EventData`] variant.
    ///
    /// Returns `Err` if the 4-byte code is not one of the known values.
    pub fn event_data(&self) -> Result<EventData, UnknownEventCode> {
        match &self.event_string_code {
            b"SSTA" => Ok(EventData::SessionStarted),
            b"SEND" => Ok(EventData::SessionEnded),
            b"FTLP" => Ok(EventData::FastestLap(self.read_variant::<FastestLap>())),
            b"RTMT" => Ok(EventData::Retirement(self.read_variant::<Retirement>())),
            b"DRSE" => Ok(EventData::DrsEnabled),
            b"DRSD" => Ok(EventData::DrsDisabled(self.read_variant::<DrsDisabled>())),
            b"TMPT" => Ok(EventData::TeamMateInPits(self.read_variant::<TeamMateInPits>())),
            b"CHQF" => Ok(EventData::ChequeredFlag),
            b"RCWN" => Ok(EventData::RaceWinner(self.read_variant::<RaceWinner>())),
            b"PENA" => Ok(EventData::Penalty(self.read_variant::<Penalty>())),
            b"SPTP" => Ok(EventData::SpeedTrap(self.read_variant::<SpeedTrap>())),
            b"STLG" => Ok(EventData::StartLights(self.read_variant::<StartLights>())),
            b"LGOT" => Ok(EventData::LightsOut),
            b"DTSV" => Ok(EventData::DriveThroughPenaltyServed(
                self.read_variant::<DriveThroughPenaltyServed>(),
            )),
            b"SGSV" => Ok(EventData::StopGoPenaltyServed(
                self.read_variant::<StopGoPenaltyServed>(),
            )),
            b"FLBK" => Ok(EventData::Flashback(self.read_variant::<Flashback>())),
            b"BUTN" => Ok(EventData::Buttons(self.read_variant::<Buttons>())),
            b"RDFL" => Ok(EventData::RedFlag),
            b"OVTK" => Ok(EventData::Overtake(self.read_variant::<Overtake>())),
            b"SCAR" => Ok(EventData::SafetyCar(self.read_variant::<SafetyCar>())),
            b"COLL" => Ok(EventData::Collision(self.read_variant::<Collision>())),
            _ => Err(UnknownEventCode(self.event_string_code)),
        }
    }

    /// Read a variant struct from the raw union bytes.
    ///
    /// # Safety
    /// `T` must be `repr(C, packed)` with only primitive fields.
    /// `size_of::<T>()` must be ≤ `UNION_SIZE` — enforced by compile-time assert.
    fn read_variant<T: Copy + FixEndianness>(&self) -> T {
        const { assert!(size_of::<T>() <= UNION_SIZE, "variant exceeds union size") };
        // SAFETY: T is repr(C, packed), all bit patterns are valid primitives,
        // and we checked that T fits within the details buffer.
        unsafe { ptr::read_unaligned(self.details.as_ptr() as *const T) }.fix_endianness()
    }
}

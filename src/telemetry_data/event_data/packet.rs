//! Defines the structure for the event data packet and its deserialization.

use serde::Serialize;

use super::event_type::Event;
use crate::telemetry_data::{
    Buttons, ChequeredFlag, Collision, DriveThroughPenaltyServed, DrsDisabled, DrsEnabled,
    FastestLap, FlashBack, InternalPacketEventData, LightsOut, Overtake, Penalty, RaceWinner,
    RedFlag, Retirement, SessionEnd, SessionStart, SpeedTrap, StartLights, StopGoPenaltyServed,
    TeamMateInPits, event_data::safety_car::SafetyCar, packet_header::PacketHeader,
};

/// Decoded event data packet with structured event information.
///
/// This structure contains the decoded event data with the specific
/// event type information properly structured.
#[derive(Debug, Serialize, Clone, Copy)]
pub struct PacketEventData {
    /// Header information for the packet
    pub header: PacketHeader,

    // Event string code (4 characters) identifying the event type
    // hidden from end user as it's mainly for internal use
    event_string_code: [u8; 4],

    /// The specific event type and its associated data
    pub r#type: Event,
}

impl Default for PacketEventData {
    fn default() -> Self {
        Self {
            header: PacketHeader {
                packet_id: crate::telemetry_data::PacketId::EventPacket,
                ..PacketHeader::default()
            },
            event_string_code: [0; 4],
            r#type: Event::SessionStart(SessionStart::default()),
        }
    }
}

// Helper macro,
// TODO: More descriptive error?
macro_rules! deserialise_event_type {
    ($internal_repr:expr, $($event_code:pat => $variant:ident($ty:ty)),* $(,)?) => {
        match &($internal_repr.event_string_code) {
            $(
                $event_code => {
                    bincode::deserialize::<$ty>(&$internal_repr.remaining_data)
                        .map(Event::$variant)
                }
            )*
            _ => unreachable!(
                "Unknown event type: {}",
                String::from_utf8_lossy(&($internal_repr.event_string_code))
            ),
        }
    };
}

/// Use this function to deserialize a byte slice into a `PacketEventData` structure.
pub fn deserialise_event_packet_from_bytes(bytes: &[u8]) -> anyhow::Result<PacketEventData> {
    // Deserialise to an intermediate representation
    // to figure out which event type it is.
    let internal_representation: InternalPacketEventData = bincode::deserialize(bytes)?;

    // Then match against the event string code
    // to determine the specific event type and produce the final structure.
    Ok(PacketEventData {
        header: internal_representation.m_header,
        event_string_code: internal_representation.event_string_code,
        r#type: deserialise_event_type!(
            internal_representation,
            b"SSTA" => SessionStart(SessionStart),
            b"SEND" => SessionEnd(SessionEnd),
            b"FTLP" => FastestLap(FastestLap),
            b"RTMT" => Retirement(Retirement),
            b"DRSE" => DrsEnabled(DrsEnabled),
            b"DRSD" => DrsDisabled(DrsDisabled),
            b"TMPT" => TeamMateInPits(TeamMateInPits),
            b"CHQF" => ChequeredFlag(ChequeredFlag),
            b"RCWN" => RaceWinner(RaceWinner),
            b"PENA" => Penalty(Penalty),
            b"SPTP" => SpeedTrap(SpeedTrap),
            b"STLG" => StartLights(StartLights),
            b"LGOT" => LightsOut(LightsOut),
            b"DTSV" => DriveThroughPenaltyServed(DriveThroughPenaltyServed),
            b"SGSV" => StopGoPenaltyServed(StopGoPenaltyServed),
            b"FLBK" => FlashBack(FlashBack),
            b"BUTN" => Buttons(Buttons),
            b"RDFL" => RedFlag(RedFlag),
            b"OVTK" => Overtake(Overtake),
            b"SCAR" => SafetyCar(SafetyCar),
            b"COLL" => Collision(Collision)
        )?,
    })
}

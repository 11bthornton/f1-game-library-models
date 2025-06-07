use serde::Serialize;

use super::event_type::EventType;
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
#[derive(Debug, Serialize, Default, Clone, Copy)]
pub struct PacketEventData {
    /// Header information for the packet
    pub m_header: PacketHeader,

    /// Event string code (4 characters) identifying the event type
    pub event_string_code: [u8; 4],

    /// The specific event type and its associated data
    pub r#type: EventType,
}

macro_rules! deserialise_event_type {
    ($internal_repr:expr, $($event_code:pat => $variant:ident($ty:ty)),* $(,)?) => {
        match &($internal_repr.event_string_code) {
            $(
                $event_code => {
                    bincode::deserialize::<$ty>(&$internal_repr.remaining_data)
                        .map(EventType::$variant)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                }
            )*
            _ => unreachable!(
                "Unknown event type: {}",
                String::from_utf8_lossy(&($internal_repr.event_string_code))
            ),
        }
    };
}

pub fn deserialise_event_packet_from_bytes(bytes: &[u8]) -> Result<PacketEventData, Box<dyn std::error::Error>> {
    let internal_representation: InternalPacketEventData =
        bincode::deserialize(bytes).map_err(|e| e.to_string())?;

    Ok(
        PacketEventData {
            m_header: internal_representation.m_header,
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
        }
    )
}
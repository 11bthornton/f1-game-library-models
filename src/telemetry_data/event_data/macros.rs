#![allow(dead_code)]

use bincode::deserialize;

use super::{safety_car::SafetyCar, *};
use crate::enums::TelemetryError;

macro_rules! decode {
    ($($name:expr => ($ty:ty,  $var:ident)),+) => {
            impl InternalPacketEventData {
                // #[ignore(unreachable_code)]
                /// Decodes the raw event data into a structured event packet.
                ///
                /// This method interprets the event string code and deserializes
                /// the remaining data into the appropriate event type structure.
                ///
                /// # Returns
                ///
                /// A `Result` containing either the decoded `PacketEventFinal` or a `Utf8Error`.
                pub fn deserialise_further(self) -> Result<PacketEventData, TelemetryError> {
                        let button_code = std::str::from_utf8(&self.event_string_code)
                            .map_err(|utf_error| TelemetryError::DeserialisationError(utf_error.to_string()))?;

                        match button_code {
                            $(
                                $name => {
                                        let decoded: $ty = deserialize(&self.remaining_data).unwrap();

                                         return Ok(
                                            PacketEventData {
                                                m_header: self.m_header,
                                                event_string_code: self.event_string_code,
                                                r#type: EventType::$var(decoded)
                                            }
                                        )
                                },
                            )+

                            _ => panic!(
                                "{}", format!("Unrecognised button code! {button_code}")
                            )
                        }
                }
            }
    };
}

decode!(
    "BUTN" => (Buttons, Buttons),
    "RCWN" => (RaceWinner, RaceWinner),
    "FLBK" => (FlashBack, FlashBack),
    "TMPT" => (TeamMateInPits, TeamMateInPits),
    "RCWM" => (RaceWinner, RaceWinner),
    "RTMT" => (Retirement, Retirement),
    "FTLP" => (FastestLap, FastestLap),
    "STLG" => (StartLights, StartLights),
    "SPTP" => (SpeedTrap, SpeedTrap),
    "PENA" => (Penalty, Penalty),
    "DTSV" => (DriveThroughPenaltyServed, DriveThroughPenaltyServed),
    "SGSV" => (StopGoPenaltyServed, StopGoPenaltyServed),
    "LGOT" => (LightsOut, LightsOut),
    "SSTA" => (SessionStart, SessionStart),
    "SEND" => (SessionEnd, SessionEnd),
    "CHQF" => (ChequeredFlag, ChequeredFlag),
    "DRSE" => (DrsEnabled, DrsEnabled),
    "DRSD" => (DrsDisabled, DrsDisabled),
    "OVTK" => (Overtake, Overtake),
    "RDFL" => (RedFlag, RedFlag),
    "COLL" => (Collision, Collision),
    "SCAR" => (SafetyCar, SafetyCar)
);

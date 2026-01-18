/// F1 game telemetry data structures and types.
///
/// This module contains all the data structures used to represent telemetry data
/// from F1 racing games. It includes structures for various aspects of the game,
/// such as car telemetry, lap data, session information, and more.
///
/// The main entry point for working with telemetry data is the `F1Data` enum,
/// which encapsulates all possible telemetry packet types.
use serde::{Deserialize, Serialize};

pub mod car_damage_data;
pub mod car_setup_data;
pub mod car_status_data;
pub mod car_telemetry_data;
pub mod event_data;
pub mod final_classification_data;
pub mod lap_data;
pub mod lap_positions_data;
pub mod lobby_data;
pub mod motion_data;
pub mod motion_extended_data;
pub mod packet_header;
pub mod participant_data;
pub mod session_data;
pub mod session_history_data;
pub mod time_trial_data;
pub mod tyre_set_data;
pub mod utility;

pub use car_damage_data::*;
pub use car_setup_data::*;
pub use car_status_data::*;
pub use car_telemetry_data::*;
pub use event_data::*;
pub use final_classification_data::*;
pub use lap_data::*;
pub use lap_positions_data::*;
pub use lobby_data::*;
pub use motion_data::*;
pub use motion_extended_data::*;
pub use packet_header::*;
pub use participant_data::*;
pub use session_data::*;
pub use session_history_data::*;
pub use time_trial_data::*;
pub use tyre_set_data::*;
pub use utility::*;

use crate::{FormulaOnePacket, constants::HEADER_SIZE};

#[derive(Serialize, Debug, Copy, Clone)]
pub enum F1Data {
    /// Car damage information
    CarDamageData(Packet<CarDamageArray>),

    /// Car setup information
    CarSetupData(Packet<CarSetupArray>),

    /// Car status information
    CarStatusData(PacketCarStatusData),

    /// Lap timing information
    LapData(PacketLapData),

    /// Car motion data
    CarMotionData(PacketMotionData),

    /// Session participant information
    ParticipantData(PacketParticipantData),

    /// Session information
    SessionData(PacketSessionData),

    /// Game event information
    EventData(PacketEventData),

    /// Car telemetry data
    TelemetryData(PacketCarTelemetryData),

    /// Final classification results
    ClassificationData(PacketClassificationData),

    /// Session history information
    SessionHistoryData(PacketSessionHistoryData),

    /// Lobby information
    LobbyData(PacketLobbyInfoData),

    /// Extended motion data
    ExtendedMotionData(PacketMotionExData),

    /// Tyre set information
    TyreSetData(PacketTyreSetsData),

    /// Time trial data
    TimeTrialData(PacketTimeTrialData),

    /// Lap positions data
    LapPositionsData(PacketLapPositionsData),

    None,
}

macro_rules! deserialise_packet_type {
    ($header:expr, $bytes:expr, $($pid:pat => $variant:ident($ty:ty)),* $(,)?) => {
        match $header.packet_id() {
            $(
                $pid => {
                    let data = Packet {
                        header: $header,
                        data_raw: <$ty>::from_bytes($bytes)?,
                    };
                    F1Data::$variant(data)
                }
            )*
            PacketId::None => unreachable!(
                "PacketId::None should not be encountered in deserialise_udp_packet_from_bytes"
            ),
            PacketId::EventPacket => unreachable!(
                "EventPacket should be handled separately in deserialise_event_packet_from_bytes"
            ),
            _ => unreachable!()
        }
    };
}

pub fn deserialise_udp_packet_from_bytes(bytes: &[u8]) -> anyhow::Result<F1Data> {
    let header: PacketHeader = PacketHeader::from_bytes(bytes)?;

    let variant = deserialise_packet_type!(header, &bytes[HEADER_SIZE..],
        PacketId::CarDamagePacket => CarDamageData(CarDamageArray),

    );

    Ok(variant)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Packet<T>
{
    pub header: PacketHeader,
    pub data_raw: T,
}

trait FixEndianness {
    fn fix_endianness(&mut self);
}
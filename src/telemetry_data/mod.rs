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
pub use lobby_data::*;
pub use motion_data::*;
pub use motion_extended_data::*;
// pub use packet_header::*;
pub use participant_data::*;
pub use session_data::*;
pub use session_history_data::*;
pub use time_trial_data::*;
pub use tyre_set_data::*;
pub use utility::*;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum F1Data {
    /// Car damage information
    CarDamageData(PacketCarDamageData),

    /// Car setup information
    CarSetupData(PacketCarSetupData),

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

    TimeTrialData(PacketTimeTrialData),
}

#![allow(clippy::module_inception)]

pub mod telemetry_data;

pub(crate) mod utils;

pub(crate) use utils::u8_as_usize;

pub use telemetry_data::deserialise_udp_packet_from_bytes;

pub mod constants {
    pub const MAX_CARS_IN_SESSION: usize = 22;
    pub const PARTICIPANT_PACKET_SIZE: usize = 1284;
    pub const CAR_SETUP_PACKET_SIZE: usize = 1133;
    pub const CAR_STATUS_PACKET_SIZE: usize = 1239;
    pub const LAP_DATA_PACKET_SIZE: usize = 1285;
    pub const MOTION_DATA_PACKET_SIZE: usize = 1349;
    pub const EXTENDED_MOTION_DATA_PACKET_SIZE: usize = 273;
    pub const TELEMETRY_DATA_PACKET_SIZE: usize = 1352;
    pub const CLASSIFICATION_DATA_PACKET_SIZE: usize = 1042;
    pub const CAR_DAMAGE_DATA_PACKET_SIZE: usize = 1041;
    pub const SESSION_DATA_PACKET_SIZE: usize = 753;
    pub const SESSION_HISTORY_DATA_PACKET_SIZE: usize = 1460;
    pub const EVENT_DATA_PACKET_SIZE: usize = 45;
    pub const LOBBY_INFO_DATA_PACKET_SIZE: usize = 954;
    pub const TYRE_SETS_DATA_PACKET_SIZE: usize = 231;
    pub const TIME_TRIAL_DATA_PACKET_SIZE: usize = 101;
    pub const LAP_POSITIONS_DATA_PACKET_SIZE: usize = 1131;
}

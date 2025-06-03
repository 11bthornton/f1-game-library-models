#![allow(clippy::module_inception)]

pub mod enums;
pub mod telemetry_data;

pub(crate) mod utils;

pub(crate) use utils::u8_as_usize;

pub mod packet_sizes
{
    pub mod participant
    {
        pub const PARTICIPANT_PACKET_SIZE: usize = 1284;
    }

    pub mod car_setup
    {
        pub const CAR_SETUP_PACKET_SIZE: usize = 1133;
    }

    pub mod car_status
    {
        pub const CAR_STATUS_PACKET_SIZE: usize = 1239;
    }

    pub mod lap_data
    {
        pub const LAP_DATA_PACKET_SIZE: usize = 1285;
    }

    pub mod motion
    {
        pub const MOTION_DATA_PACKET_SIZE: usize = 1349;

        pub const EXTENDED_MOTION_DATA_PACKET_SIZE: usize = 273;
    }

    pub mod telemetry
    {
        pub const TELEMETRY_DATA_PACKET_SIZE: usize = 1352;
    }

    pub mod classification
    {
        pub const CLASSIFICATION_DATA_PACKET_SIZE: usize = 1042;
    }

    pub mod car_damage
    {
        pub const CAR_DAMAGE_DATA_PACKET_SIZE: usize = 1041;
    }

    pub mod session
    {
        pub const SESSION_DATA_PACKET_SIZE: usize = 753;

        pub const SESSION_HISTORY_DATA_PACKET_SIZE: usize = 1460;
    }

    pub mod event
    {
        pub const EVENT_DATA_PACKET_SIZE: usize = 45;
    }

    pub mod lobby
    {
        pub const LOBBY_INFO_DATA_PACKET_SIZE: usize = 954;
    }

    pub mod tyre_sets
    {
        pub const TYRE_SETS_DATA_PACKET_SIZE: usize = 231;
    }

    pub mod time_trial
    {
        pub const TIME_TRIAL_DATA_PACKET_SIZE: usize = 101;
    }
}

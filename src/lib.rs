//! # F1 Game Library Models 25
//!
//! I built this library primarily to learn about Rust package management on [crates.io](https://crates.io) and the various CI/CD tools available for Rust on Github, so the library shouldn't be used for anything you need to rely on - I nevertheless hope you find it useful.
//!
//! This is part of a larger project I have to do some interesting things with the UDP telemetry data the F1 video game series spits out during races or during the menus.
//!
//! It is based on the [UDP Specification](https://forums.ea.com/blog/f1-games-game-info-hub-en/ea-sports%E2%84%A2-f1%C2%AE25-udp-specification/12187347) for the F1-2025 video game. Previous versions are not supported.
//!
//! This project defines the data types and various utility methods you might want or need.
//!
//! ## Example Usage
//! ```rust
//! use f1_game_library_models_25::telemetry_data::F1Data;
//!
//! #[tokio::main]
//! async fn main() {
//!     let socket = tokio
//!         ::net
//!         ::UdpSocket
//!         ::bind("127.0.0.1:54345") // Configure this to match game   
//!         .await.unwrap();          // settings
//!     
//!     let mut buf = [0u8; 2048];
//!
//!     loop {
//!         let (len, _) = socket.recv_from(&mut buf).await.unwrap();
//!         let packet =            
//!             f1_game_library_models_25::
//!                 deserialise_udp_packet_from_bytes(&buf[..len])
//!                 .expect("Failed to parse packet");
//!         
//!         // Do cool stuff
//!         match packet {
//!             F1Data::CarTelemetryData(data) => println!("Telemetry: {:?}", data),
//!             F1Data::LapData(data) => println!("Lap: {:?}", data),
//!             _ => {}
//!         }
//!     }
//! }
//! ```
//!
//! ## Contributing
//!
//! Feel free to submit a PR if something is wrong or you'd like to offer enhancements! See `CONTRIBUTING.md` for details.
//!
//! ## ⚠️ Disclaimer
//! This project is not affiliated with, endorsed by, or associated with Formula 1, the FIA, any official F1 teams, or the developers and publishers of the F1 video game series.
//!
//! All team names and references (e.g., Mercedes, Ferrari, Red Bull Racing) are used solely for the purposes of data interoperability and representation, based on publicly observable or inferred values from the official F1 game's telemetry specification. These names may be trademarks of their respective owners.
//!
//! The enums, structs, and identifiers provided in this crate are derived from the F1 game's published or reverse-engineered telemetry specification, and are intended to support community tools and data analysis.
//!
//! Their use in this project is purely informational and does not imply any endorsement or ownership. If you are a rights holder and have concerns, please contact the maintainer.

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

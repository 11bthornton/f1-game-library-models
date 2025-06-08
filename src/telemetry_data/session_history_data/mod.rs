//! Session history data from F1 games.
//!
//! This module contains structures for representing session history information in F1 games,
//! including lap times, sector times, and tyre usage history.
mod lap_history_data;
mod packet;
mod tyre_stint_history_data;

// Re-export the structs
pub use lap_history_data::LapHistoryData;
pub use packet::PacketSessionHistoryData;
pub use tyre_stint_history_data::TyreStintHistoryData;

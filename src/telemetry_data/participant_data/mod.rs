//! Participant data from F1 games.
//!
//! This module contains structures for representing participant information in F1 games,
//! including driver details, team affiliations, and nationality data.

mod driver;
mod nationality;
mod packet;
mod participant_data;
mod team;

// Re-export the structs and enums
pub use driver::Driver;
pub use nationality::Nationality;
pub use packet::PacketParticipantData;
pub use participant_data::ParticipantData;
pub use team::Team;

/// Event data from F1 games.
///
/// This module contains structures for representing various event information
/// in F1 games, including race events, penalties, flags, and other significant
/// occurrences during a session.
mod alternate_speed_trap;
mod buttons;
mod chequered_flag;
mod collision;
mod drive_through_penalty_served;
mod drs_disabled;
mod drs_enabled;
mod event_type;
mod fastest_lap;
mod flash_back;
mod infringement_type;
mod lights_out;
mod overtake;
mod packet;
mod packet_internal;
mod penalty;
mod penalty_type;
mod race_winner;
mod red_flag;
mod retirement;
mod safety_car;
mod session_end;
mod session_start;
mod speed_trap;
mod start_lights;
mod stop_go_penalty_served;
mod team_mate_in_pits;

// Re-export all structs and enums
pub use alternate_speed_trap::AlternateSpeedTrap;
pub use buttons::Buttons;
pub use chequered_flag::ChequeredFlag;
pub use collision::Collision;
pub use drive_through_penalty_served::DriveThroughPenaltyServed;
pub use drs_disabled::DrsDisabled;
pub use drs_enabled::DrsEnabled;
pub use event_type::EventType;
pub use fastest_lap::FastestLap;
pub use flash_back::FlashBack;
pub use infringement_type::InfringementType;
pub use lights_out::LightsOut;
pub use overtake::Overtake;
pub use packet::PacketEventData;
pub use packet::deserialise_event_packet_from_bytes;
pub use packet_internal::InternalPacketEventData;
pub use penalty::Penalty;
pub use penalty_type::PenaltyType;
pub use race_winner::RaceWinner;
pub use red_flag::RedFlag;
pub use retirement::Retirement;
pub use session_end::SessionEnd;
pub use session_start::SessionStart;
pub use speed_trap::SpeedTrap;
pub use start_lights::StartLights;
pub use stop_go_penalty_served::StopGoPenaltyServed;
pub use team_mate_in_pits::TeamMateInPits;

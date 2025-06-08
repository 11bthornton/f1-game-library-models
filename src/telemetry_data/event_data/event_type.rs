//! Defines the types of events that can occur during in-game sessions.

use serde::{Deserialize, Serialize};

use super::{safety_car::SafetyCar, *};

/// Types of events that can occur during a race.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum EventType {
    /// Button press event
    Buttons(Buttons),

    /// Fastest lap set event
    FastestLap(FastestLap),

    /// Driver retirement event
    Retirement(Retirement),

    /// Teammate entering pits event
    TeamMateInPits(TeamMateInPits),

    /// Race winner event
    RaceWinner(RaceWinner),

    /// Penalty issued event
    Penalty(Penalty),

    /// Speed trap triggered event
    SpeedTrap(SpeedTrap),

    /// Start lights event
    StartLights(StartLights),

    /// Drive-through penalty served event
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),

    /// Stop-go penalty served event
    StopGoPenaltyServed(StopGoPenaltyServed),

    /// Flashback used event
    FlashBack(FlashBack),

    /// Lights out event (race start)
    LightsOut(LightsOut),

    /// Session start event
    SessionStart(SessionStart),

    /// Session end event
    SessionEnd(SessionEnd),

    /// DRS enabled event
    DrsEnabled(DrsEnabled),

    /// DRS disabled event
    DrsDisabled(DrsDisabled),

    /// Chequered flag event
    ChequeredFlag(ChequeredFlag),

    /// Alternative speed trap event
    AlternateSpeedTrap(AlternateSpeedTrap),

    /// Overtake event
    Overtake(Overtake),

    /// Red flag event
    RedFlag(RedFlag),

    /// Collision event
    Collision(Collision),

    /// Safety car event
    SafetyCar(SafetyCar),

    /// No event (default)
    #[default]
    NoEvent,
}

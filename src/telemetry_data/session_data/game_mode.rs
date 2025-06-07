use serde_repr::{Deserialize_repr, Serialize_repr};

/// Enum representing all possible game modes in the game
#[derive(Debug, Copy, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum GameMode {
    EventMode = 0,
    GrandPrix = 3,
    GrandPrix23 = 4,
    TimeTrial = 5,
    Splitscreen = 6,
    OnlineCustom = 7,
    OnlineLeague = 8,
    CareerInvitational = 11,
    ChampionshipInvitational = 12,
    Championship = 13,
    OnlineChampionship = 14,
    OnlineWeeklyEvent = 15,
    StoryModeBrakingPoint = 17, // Updated description for Story Mode
    Career22 = 19,
    Career22Online = 20,
    Career23 = 21,
    Career23Online = 22,
    DriverCareer24 = 23,
    Career24Online = 24,
    MyTeamCareer24 = 25,
    CuratedCareer24 = 26,
    MyTeamCareer25 = 27, // Added new mode
    DriverCareer25 = 28, // Added new mode
    Career25Online = 29, // Added new mode
    ChallengeCareer25 = 30, // Added new mode
    StoryModeAPXGP = 75, // Added new mode
    Benchmark = 127,
}

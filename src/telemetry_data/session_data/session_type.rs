use std::fmt;

use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

pub(crate) mod session_type_values
{
    pub const UNKNOWN: u8 = 0;

    pub const PRACTICE_ONE: u8 = 1;

    pub const PRACTICE_TWO: u8 = 2;

    pub const PRACTICE_THREE: u8 = 3;

    pub const SHORT_PRACTICE: u8 = 4;

    pub const QUALIFYING_ONE: u8 = 5;

    pub const QUALIFYING_TWO: u8 = 6;

    pub const QUALIFYING_THREE: u8 = 7;

    pub const SHORT_QUALIFYING: u8 = 8;

    pub const ONE_SHOT_QUALIFYING: u8 = 9;

    pub const SPRINT_SHOOUTOUT_ONE: u8 = 10;

    pub const SPRINT_SHOOUTOUT_TWO: u8 = 11;

    pub const SPRINT_SHOOUTOUT_THREE: u8 = 12;

    pub const SHORT_SPRINT_SHOOTOUT: u8 = 13;

    pub const ONESHOUT_SPRINT_SHOOTOUT: u8 = 14;

    pub const RACE: u8 = 15;

    pub const RACE_TWO: u8 = 16;

    pub const RACE_THREE: u8 = 17;

    pub const TIME_TRIAL: u8 = 18;
}

/// Types of sessions in F1 games
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, Default, PartialEq)]
#[repr(u8)]
pub enum SessionType
{
    /// Unknown session type
    #[default]
    Unknown = session_type_values::UNKNOWN,

    /// First practice session
    PracticeOne = session_type_values::PRACTICE_ONE,

    /// Second practice session
    PracticeTwo = session_type_values::PRACTICE_TWO,

    /// Third practice session
    PracticeThree = session_type_values::PRACTICE_THREE,

    /// Short practice session
    ShortPractice = session_type_values::SHORT_PRACTICE,

    /// First qualifying session
    QualifyingOne = session_type_values::QUALIFYING_ONE,

    /// Second qualifying session
    QualifyingTwo = session_type_values::QUALIFYING_TWO,

    /// Third qualifying session
    QualifyingThree = session_type_values::QUALIFYING_THREE,

    /// Short qualifying session
    ShortQualifying = session_type_values::SHORT_QUALIFYING,

    /// One-shot qualifying session
    OneShotQualifying = session_type_values::ONE_SHOT_QUALIFYING,

    SprintShootOutOne = session_type_values::SPRINT_SHOOUTOUT_ONE,

    SprintShootOutTwo = session_type_values::SPRINT_SHOOUTOUT_TWO,

    SprintShootOutThree = session_type_values::SPRINT_SHOOUTOUT_THREE,

    ShortSprintShootOut = session_type_values::SHORT_SPRINT_SHOOTOUT,

    OneShotSprintShootOut = session_type_values::ONESHOUT_SPRINT_SHOOTOUT,

    /// Main race
    Race = session_type_values::RACE,

    /// Second race (for sprint race weekends)
    RaceTwo = session_type_values::RACE_TWO,

    /// Third race
    RaceThree = session_type_values::RACE_THREE,

    /// Time trial session
    TimeTrial = session_type_values::TIME_TRIAL,
}

impl fmt::Display for SessionType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let name = match self {
            SessionType::Unknown => "Unknown",
            SessionType::PracticeOne => "Practice One",
            SessionType::PracticeTwo => "Practice Two",
            SessionType::PracticeThree => "Practice Three",
            SessionType::ShortPractice => "Short Practice",
            SessionType::QualifyingOne => "Qualifying One",
            SessionType::QualifyingTwo => "Qualifying Two",
            SessionType::QualifyingThree => "Qualifying Three",
            SessionType::ShortQualifying => "Short Qualifying",
            SessionType::OneShotQualifying => "One Shot Qualifying",
            SessionType::Race => "Race",
            SessionType::RaceTwo => "Race Two",
            SessionType::RaceThree => "Race Three",
            SessionType::TimeTrial => "Time Trial",
            SessionType::SprintShootOutOne => "Sprint Shootout One",
            SessionType::SprintShootOutTwo => "Sprint Shootout Two",
            SessionType::SprintShootOutThree => "Sprint Shootout Three",
            SessionType::ShortSprintShootOut => "Short Sprint Shootout",
            SessionType::OneShotSprintShootOut => "One Shot Sprint Shootout",
        };
        write!(f, "{}", name)
    }
}

impl SessionType
{
    pub fn is_practice(&self) -> bool
    {
        matches!(
            self,
            SessionType::PracticeOne
                | SessionType::PracticeTwo
                | SessionType::PracticeThree
                | SessionType::ShortPractice
        )
    }

    pub fn is_qualifying(&self) -> bool
    {
        matches!(
            self,
            SessionType::QualifyingOne
                | SessionType::QualifyingTwo
                | SessionType::QualifyingThree
                | SessionType::ShortQualifying
                | SessionType::OneShotQualifying
                | SessionType::SprintShootOutOne
                | SessionType::SprintShootOutTwo
                | SessionType::SprintShootOutThree
                | SessionType::ShortSprintShootOut
                | SessionType::OneShotSprintShootOut
        )
    }

    pub fn is_sprint_qualifying(&self) -> bool
    {
        matches!(
            self,
            SessionType::SprintShootOutOne
                | SessionType::SprintShootOutTwo
                | SessionType::SprintShootOutThree
                | SessionType::ShortSprintShootOut
                | SessionType::OneShotSprintShootOut
        )
    }

    pub fn is_sprint(&self) -> bool
    {
        matches!(self, SessionType::RaceTwo | SessionType::RaceThree) // Might be wrong, go check
    }

    pub fn is_race(&self) -> bool
    {
        matches!(
            self,
            SessionType::Race | SessionType::RaceTwo | SessionType::RaceThree
        )
    }

    pub fn is_time_trial(&self) -> bool
    {
        matches!(self, SessionType::TimeTrial)
    }
}

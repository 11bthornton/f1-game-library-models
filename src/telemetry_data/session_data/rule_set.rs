use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// Enum representing all possible rule sets in the game
#[derive(Debug, Copy, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RuleSet
{
    PracticeAndQualifying = 0,
    Race = 1,
    TimeTrial = 2,
    TimeAttack = 4,
    CheckpointChallenge = 6,
    Autocross = 8,
    Drift = 9,
    AverageSpeedZone = 10,
    RivalDuel = 11,
}

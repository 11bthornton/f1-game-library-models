use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// Types of rule infringements that can occur during a race.
#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum InfringementType
{
    /// Blocking another driver by driving too slowly
    #[default]
    SlowDrivingBlock = 0,

    /// Blocking another driver by driving in the wrong direction
    WrongWayBlock = 1,

    /// Reversing from the starting grid
    ReverseStart = 2,

    /// Causing a major collision
    MajorCollision = 3,

    /// Causing a minor collision
    MinorCollision = 4,

    /// Failing to give back a position after a single incident
    FailedReturnSingle = 5,

    /// Failing to give back positions after multiple incidents
    FailedReturnMultiple = 6,

    /// Gaining time by cutting a corner
    TimeGainCornerCut = 7,

    /// Overtaking by cutting a corner (single incident)
    OvertakeCutSingle = 8,

    /// Overtaking by cutting corners multiple times
    OvertakeCutMultiple = 9,

    /// Crossing the pit exit lane
    PitExitViolation = 10,

    /// Ignoring blue flags (signals to let faster cars pass)
    IgnoredBlueFlags = 11,

    /// Ignoring yellow flags (signals for caution)
    IgnoredYellowFlags = 12,

    /// Ignoring a drive-through penalty
    IgnoredDriveThrough = 13,

    /// Receiving too many drive-through penalties
    ExcessiveDriveThroughs = 14,

    /// Reminder to serve drive-through penalty within specified laps
    DriveThroughReminderNLaps = 15,

    /// Reminder to serve drive-through penalty in the current lap
    DriveThroughReminderThisLap = 16,

    /// Exceeding the pit lane speed limit
    PitSpeeding = 17,

    /// Remaining stationary for too long
    ExcessiveParking = 18,

    /// Violating tyre regulations
    TyreRulesIgnored = 19,

    /// Receiving too many penalties
    ExcessivePenalties = 20,

    /// Receiving multiple warnings
    MultipleWarnings = 21,

    /// Being close to disqualification due to accumulated penalties
    NearDisqualification = 22,

    /// Single violation of tyre selection regulations
    SingleTyreViolation = 23,

    /// Multiple violations of tyre selection regulations
    MultipleTyreViolation = 24,

    /// Lap invalidated due to corner cutting
    InvalidLapCornerCut = 25,

    /// Lap invalidated due to running wide off track
    InvalidLapRanWide = 26,

    /// Minor time gain from running wide
    MinorTimeGainRanWide = 27,

    /// Significant time gain from running wide
    SignificantTimeGainRanWide = 28,

    /// Extreme time gain from running wide
    ExtremeTimeGainRanWide = 29,

    /// Lap invalidated due to wall riding
    InvalidLapWallRide = 30,

    /// Lap invalidated due to using flashback feature
    InvalidLapFlashback = 31,

    /// Lap invalidated due to using track reset feature
    InvalidLapReset = 32,

    /// Blocking the pit lane
    PitlaneBlock = 33,

    /// Starting before the lights go out
    JumpStart = 34,

    /// Colliding with the safety car
    SafetyCarCollision = 35,

    /// Illegally overtaking during safety car period
    SafetyCarOvertake = 36,

    /// Driving too fast during safety car period
    SafetyCarPaceExceed = 37,

    /// Driving too fast during virtual safety car period
    VSCarPaceExceed = 38,

    /// Driving too slowly during formation lap
    SlowFormationLap = 39,

    /// Parking incorrectly during formation lap
    FormationLapParking = 40,

    /// Retirement due to mechanical failure
    MechanicalRetirement = 41,

    /// Retirement due to terminal damage
    DamageRetirement = 42,

    /// Falling too far behind the safety car
    SafetyCarGapLarge = 43,

    /// Black flag timer (disqualification warning)
    BlackFlag = 44,

    /// Failing to serve a stop-go penalty
    UnservedStopGo = 45,

    /// Failing to serve a drive-through penalty
    UnservedDriveThrough = 46,

    /// Penalty for changing engine components
    EngineChange = 47,

    /// Penalty for changing gearbox
    GearboxChange = 48,

    /// Penalty for making changes under parc fermé conditions
    ParcFerméEdit = 49,

    /// Grid penalty applied in a league race
    LeaguePenalty = 50,

    /// Penalty for retrying a section
    RetryPenalty = 51,

    /// Penalty for illegal time gain
    IllegalTime = 52,

    /// Penalty for mandatory pit stop
    MandatoryPit = 53,

    /// Attribute assigned to the driver
    Attribute = 54,
}

impl InfringementType
{
    pub fn pretty_description(&self) -> &'static str
    {
        match self {
            InfringementType::SlowDrivingBlock => "Blocking by Slow Driving",
            InfringementType::WrongWayBlock => "Blocking by Wrong Way Driving",
            InfringementType::ReverseStart => "Reversing Off the Start Line",
            InfringementType::MajorCollision => "Big Collision",
            InfringementType::MinorCollision => "Small Collision",
            InfringementType::FailedReturnSingle => {
                "Collision: Failed to Hand Back Position (Single)"
            }
            InfringementType::FailedReturnMultiple => {
                "Collision: Failed to Hand Back Position (Multiple)"
            }
            InfringementType::TimeGainCornerCut => "Corner Cutting: Gained Time",
            InfringementType::OvertakeCutSingle => "Corner Cutting: Overtake (Single)",
            InfringementType::OvertakeCutMultiple => "Corner Cutting: Overtake (Multiple)",
            InfringementType::PitExitViolation => "Crossed Pit Exit Lane",
            InfringementType::IgnoredBlueFlags => "Ignoring Blue Flags",
            InfringementType::IgnoredYellowFlags => "Ignoring Yellow Flags",
            InfringementType::IgnoredDriveThrough => "Ignoring Drive Through",
            InfringementType::ExcessiveDriveThroughs => "Too Many Drive Throughs",
            InfringementType::DriveThroughReminderNLaps => {
                "Drive Through Reminder: Serve Within N Laps"
            }
            InfringementType::DriveThroughReminderThisLap => {
                "Drive Through Reminder: Serve This Lap"
            }
            InfringementType::PitSpeeding => "Pit Lane Speeding",
            InfringementType::ExcessiveParking => "Parked for Too Long",
            InfringementType::TyreRulesIgnored => "Ignoring Tyre Regulations",
            InfringementType::ExcessivePenalties => "Too Many Penalties",
            InfringementType::MultipleWarnings => "Multiple Warnings",
            InfringementType::NearDisqualification => "Approaching Disqualification",
            InfringementType::SingleTyreViolation => "Tyre Regulations: Select (Single)",
            InfringementType::MultipleTyreViolation => "Tyre Regulations: Select (Multiple)",
            InfringementType::InvalidLapCornerCut => "Lap Invalidated: Corner Cutting",
            InfringementType::InvalidLapRanWide => "Lap Invalidated: Running Wide",
            InfringementType::MinorTimeGainRanWide => {
                "Corner Cutting: Ran Wide, Gained Time (Minor)"
            }
            InfringementType::SignificantTimeGainRanWide => {
                "Corner Cutting: Ran Wide, Gained Time (Significant)"
            }
            InfringementType::ExtremeTimeGainRanWide => {
                "Corner Cutting: Ran Wide, Gained Time (Extreme)"
            }
            InfringementType::InvalidLapWallRide => "Lap Invalidated: Wall Riding",
            InfringementType::InvalidLapFlashback => "Lap Invalidated: Flashback Used",
            InfringementType::InvalidLapReset => "Lap Invalidated: Reset to Track",
            InfringementType::PitlaneBlock => "Blocking the Pitlane",
            InfringementType::JumpStart => "Jump Start",
            InfringementType::SafetyCarCollision => "Safety Car to Car Collision",
            InfringementType::SafetyCarOvertake => "Safety Car: Illegal Overtake",
            InfringementType::SafetyCarPaceExceed => "Safety Car: Exceeding Allowed Pace",
            InfringementType::VSCarPaceExceed => "Virtual Safety Car: Exceeding Allowed Pace",
            InfringementType::SlowFormationLap => "Formation Lap: Below Allowed Speed",
            InfringementType::FormationLapParking => "Formation Lap: Parking",
            InfringementType::MechanicalRetirement => "Retired: Mechanical Failure",
            InfringementType::DamageRetirement => "Retired: Terminally Damaged",
            InfringementType::SafetyCarGapLarge => "Safety Car: Falling Too Far Back",
            InfringementType::BlackFlag => "Black Flag Timer",
            InfringementType::UnservedStopGo => "Unserved Stop Go Penalty",
            InfringementType::UnservedDriveThrough => "Unserved Drive Through Penalty",
            InfringementType::EngineChange => "Engine Component Change",
            InfringementType::GearboxChange => "Gearbox Change",
            InfringementType::ParcFerméEdit => "Parc Fermé Change",
            InfringementType::LeaguePenalty => "League Grid Penalty",
            InfringementType::RetryPenalty => "Retry Penalty",
            InfringementType::IllegalTime => "Illegal Time Gain",
            InfringementType::MandatoryPit => "Mandatory Pitstop",
            InfringementType::Attribute => "Attribute Assigned",
        }
    }
}

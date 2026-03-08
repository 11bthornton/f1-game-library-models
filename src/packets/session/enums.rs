//! Session-specific enums.

use num_enum::TryFromPrimitive;

/// Current weather conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Weather {
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

/// Formula class running in this session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Formula {
    F1Modern = 0,
    F1Classic = 1,
    F2 = 2,
    F1Generic = 3,
    Beta = 4,
    Esports = 6,
    F1World = 8,
    F1Elim = 9,
}

/// How accurately the weather forecast is modelled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ForecastAccuracy {
    Perfect = 0,
    Approximate = 1,
}

/// Braking assist level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum BrakingAssist {
    Off = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

/// Gearbox assist level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum GearboxAssist {
    Manual = 1,
    ManualSuggested = 2,
    Auto = 3,
}

/// Dynamic racing line display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum DynamicRacingLine {
    Off = 0,
    CornersOnly = 1,
    Full = 2,
}

/// Session length multiplier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SessionLength {
    None = 0,
    VeryShort = 2,
    Short = 3,
    Medium = 4,
    MediumLong = 5,
    Long = 6,
    Full = 7,
}

/// Speed display unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SpeedUnits {
    Mph = 0,
    Kph = 1,
}

/// Temperature display unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum TemperatureUnits {
    Celsius = 0,
    Fahrenheit = 1,
}

/// Flashback allowance setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum FlashbackLimit {
    Low = 0,
    Medium = 1,
    High = 2,
    Unlimited = 3,
}

/// Recovery mode setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum RecoveryMode {
    None = 0,
    Flashbacks = 1,
    AutoRecovery = 2,
}

/// Car damage simulation level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum CarDamageLevel {
    Off = 0,
    Reduced = 1,
    Standard = 2,
    Simulation = 3,
}

/// Car damage accumulation rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum CarDamageRate {
    Reduced = 0,
    Standard = 1,
    Simulation = 2,
}

/// Collision simulation setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum CollisionSetting {
    Off = 0,
    PlayerToPlayerOff = 1,
    On = 2,
}

/// Racing line display dimensionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum DynamicRacingLineType {
    TwoD = 0,
    ThreeD = 1,
}

/// Pit stop experience level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum PitStopExperience {
    Automatic = 0,
    Broadcast = 1,
    Immersive = 2,
}

/// Safety car / red flag frequency setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum IncidentFrequency {
    Off = 0,
    Reduced = 1,
    Standard = 2,
    Increased = 3,
}

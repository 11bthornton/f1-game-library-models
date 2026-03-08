use num_enum::TryFromPrimitive;

/// Packet type discriminant, as a typed enum over the raw `u8` wire value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketId {
    Motion = 0,
    Session = 1,
    LapData = 2,
    Event = 3,
    Participants = 4,
    CarSetups = 5,
    CarTelemetry = 6,
    CarStatus = 7,
    FinalClassification = 8,
    LobbyInfo = 9,
    CarDamage = 10,
    SessionHistory = 11,
    TyreSets = 12,
    CarMotionEx = 13,
    TimeTrial = 14,
    LapPositions = 15,
}

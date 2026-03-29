use num_enum::TryFromPrimitive;

/// Actual tyre compound — the physical compound fitted to the car.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[repr(u8)]
pub enum ActualTyreCompound {
    None = 0,
    Inter = 7,
    Wet = 8,
    ClassicDry = 9,
    ClassicWet = 10,
    F2SuperSoft = 11,
    F2Soft = 12,
    F2Medium = 13,
    F2Hard = 14,
    F215 = 15,
    C5 = 16,
    C4 = 17,
    C3 = 18,
    C2 = 19,
    C1 = 20,
    C0 = 21,
    C6 = 22,
}

/// Visual tyre compound — how the tyre appears in-game (may differ from actual).
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[repr(u8)]
pub enum VisualTyreCompound {
    None = 0,
    Inter = 7,
    Wet = 8,
    ClassicDry = 9,
    ClassicWet = 10,
    F220Wet = 15,
    Soft = 16,
    Medium = 17,
    Hard = 18,
    F220SuperSoft = 19,
    F2Soft = 20,
    F2Medium = 21,
    F2Hard = 22,
}

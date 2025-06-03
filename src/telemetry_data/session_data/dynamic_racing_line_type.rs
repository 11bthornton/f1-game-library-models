use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// Type of dynamic racing line display
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DynamicRacingLineType
{
    /// Racing line disabled
    Off,
    /// Racing line shown only in corners
    #[serde(rename = "Corners Only")]
    CornersOnly,
    /// Racing line shown throughout the track
    Full,
}

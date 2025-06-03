use serde_repr::{Deserialize_repr, Serialize_repr};

/// Dynamic racing line assist setting
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DynamicRacingLine {
    /// 2D racing line
    #[serde(rename = "2d")]
    TwoD,
    /// 3D racing line
    #[serde(rename = "3d")]
    ThreeD,
}

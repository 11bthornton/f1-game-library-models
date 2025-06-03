use serde::Serialize;
use serde_repr::Deserialize_repr;

/// Gaming platforms.
///
/// These represent the various gaming platforms that players can use.
#[derive(Debug, Deserialize_repr, Serialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Platform
{
    /// Unknown platform (value 0)
    #[serde(rename = "Unknown")]
    SuperUnknown = 0,
    /// Steam platform
    Steam = 1,
    /// PlayStation platform
    Playstation = 3,
    /// Xbox platform
    Xbox = 4,
    /// Origin platform
    Origin = 6,
    /// Unknown platform (value 255)
    Unknown = 255,
}

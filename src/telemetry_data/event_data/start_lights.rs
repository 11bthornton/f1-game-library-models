use serde::{
    Deserialize,
    Serialize,
};

/// Start lights event.
///
/// This event occurs when the start lights change state before the race start.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct StartLights
{
    /// Number of lights currently illuminated (0-5)
    pub num_lights: u8,
}

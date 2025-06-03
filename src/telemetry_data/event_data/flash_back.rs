use serde::{Deserialize, Serialize};

/// Flashback event.
///
/// This event occurs when the player uses the flashback feature to rewind time.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct FlashBack {
    /// Frame identifier for the flashback
    pub flashback_frame_identifier: u32,
    /// Session time at the point of the flashback
    pub flashback_session_time: f32,
}

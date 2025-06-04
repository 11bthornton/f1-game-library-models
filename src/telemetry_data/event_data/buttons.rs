use serde::{Deserialize, Serialize};

/// Information about button presses.
///
/// Contains bit flags indicating which buttons are currently pressed.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct Buttons {
    /// Bit flags indicating which buttons are pressed
    pub button_status: u32,
}

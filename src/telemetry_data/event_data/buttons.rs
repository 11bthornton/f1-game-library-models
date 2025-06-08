//! Defines the `Buttons` struct and related functionality for handling button presses in telemetry data.

use serde::{Deserialize, Serialize};

/// Information about button presses.
///
/// Contains bit flags indicating which buttons are currently pressed.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct Buttons {
    /// Bit flags indicating which buttons are pressed
    pub button_status: u32,
}

impl Buttons
{
    pub fn to_buttons(&self) -> Vec<Button> {
        unimplemented!()
    }
}

pub enum Button {
    CrossOrA,
    TriangleOrY,
    CircleOrB,
    SquareOrX,
    DPadLeft,
    DPadRight,
    DPadUp,
    DPadDown,
    OptionsOrMenu,
    L1OrLB,
    R1OrRB,
    L2OrLT,
    R2OrRT,
    LeftStickClick,
    RightStickClick,
    RightStickLeft,
    RightStickRight,
    RightStickUp,
    RightStickDown,
    Special,
    UDPAction1,
    UDPAction2,
    UDPAction3,
    UDPAction4,
    UDPAction5,
    UDPAction6,
    UDPAction7,
    UDPAction8,
    UDPAction9,
    UDPAction10,
    UDPAction11,
    UDPAction12,
}

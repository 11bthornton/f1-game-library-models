use serde::{Deserialize, Serialize};

/// Session end event.
///
/// This event occurs when the current session ends.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct SessionEnd;

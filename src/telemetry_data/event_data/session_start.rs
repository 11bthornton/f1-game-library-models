use serde::{
    Deserialize,
    Serialize,
};

/// Session start event.
///
/// This event occurs when a new session begins.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct SessionStart;

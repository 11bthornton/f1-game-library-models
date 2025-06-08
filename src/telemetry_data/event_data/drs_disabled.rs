//! Defines the `DrsDisabled` event data structure.

use serde::{Deserialize, Serialize};

/// DRS disabled event.
///
/// This event occurs when DRS (Drag Reduction System) is disabled for the session.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct DrsDisabled;

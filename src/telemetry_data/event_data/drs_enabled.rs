//! Defines the `DrsEnabled` telemetry data structure.

use serde::{Deserialize, Serialize};

/// DRS enabled event.
///
/// This event occurs when DRS (Drag Reduction System) is enabled for the session.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
#[repr(C)]
pub struct DrsEnabled;

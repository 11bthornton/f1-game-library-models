//! Lights out event data for telemetry in racing games

use serde::{Deserialize, Serialize};

/// Lights out event.
///
/// This event occurs when the start lights go out, signaling the start of the race.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct LightsOut;

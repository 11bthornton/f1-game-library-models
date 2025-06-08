//! Defines the `ChequeredFlag` event data structure.

use serde::{Deserialize, Serialize};

/// Chequered flag event.
///
/// This event occurs when the chequered flag is shown at the end of the race.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct ChequeredFlag;

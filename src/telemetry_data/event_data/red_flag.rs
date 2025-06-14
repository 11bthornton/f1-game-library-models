//! Defines the `RedFlag` event data type.

use serde::{Deserialize, Serialize};

/// Indicates that a red flag has been shown.
///
/// This event occurs when the session is stopped due to dangerous conditions.
#[derive(Deserialize, Debug, Serialize, Default, Clone, Copy)]
pub struct RedFlag;

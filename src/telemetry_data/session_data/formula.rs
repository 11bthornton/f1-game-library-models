//! Defines the formula category for the current session.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Formula category for the current session
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Formula {
    /// Modern F1 cars
    #[serde(rename = "F1 Modern")]
    F1Modern,
    /// Classic F1 cars
    #[serde(rename = "F1 Classic")]
    F1Classic,
    /// Formula 2 cars
    #[serde(rename = "F2")]
    FormulaTwo,
    /// Generic F1 cars
    #[serde(rename = "F1 Generic")]
    F1Generic,
}

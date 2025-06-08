//! Defines the ParticipantData struct and related functionality.

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use super::{Driver, Team};

/// Represents data for a single participant in the session
///
/// Contains information about the driver, their team, and other identifying details
#[derive(Deserialize, Debug, Serialize, Copy, Clone, PartialEq)]
pub struct ParticipantData {
    /// Whether the vehicle is AI (1) or Human (0) controlled
    pub ai_controlled: bool,

    /// Driver ID - see Driver enum
    pub driver_id: Driver,

    /// Network ID - unique identifier for network players
    pub network_id: u8,

    /// Team ID - see Team enum
    pub team: Team,

    /// My team flag - 1 if the vehicle belongs to the player's team
    pub my_team: bool,

    /// Race number of the car
    pub race_number: u8,

    /// Nationality of the driver - see Nationality enum
    pub nationality: u8,

    #[serde(with = "BigArray")]
    /// Name of the participant in UTF-8 format
    pub name: [u8; 32],

    /// Telemetry setting for the car - 0 = restricted, 1 = public
    pub telemetry: bool,

    pub show_online_name: bool,

    /// Name of the participant for online games in UTF-8 format
    pub tech_level: u16,

    /// Platform the player is on - see Platform enum
    pub platform: u8,

    pub num_colours: u8,

    pub livery_colours: [LiveryColour; 4],
}

#[derive(Deserialize, Debug, Serialize, Copy, Clone, PartialEq)]
pub struct LiveryColour {
    /// Red component of the colour (0-255)
    pub red: u8,

    /// Green component of the colour (0-255)
    pub green: u8,

    /// Blue component of the colour (0-255)
    pub blue: u8,
}

impl ParticipantData {
    /// Returns the participant's name as a String
    ///
    /// Converts the raw byte array to a UTF-8 string and trims null characters
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        let name = str::from_utf8(&self.name).unwrap().to_string();

        let res = name.trim_matches(char::from(0));

        res.to_string()
    }
}

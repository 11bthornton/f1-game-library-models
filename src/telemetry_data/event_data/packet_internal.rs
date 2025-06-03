use serde::Deserialize;

use crate::telemetry_data::packet_header::PacketHeader;

/// Internal packet event data.
///
/// This is the intermediate struct that UDP data from the game is deserialized into.
/// It is then converted into the `PacketEventData` struct which is exposed.
#[derive(Debug, Deserialize)]
pub struct InternalPacketEventData {
    pub m_header: PacketHeader,

    pub event_string_code: [u8; 4],

    pub remaining_data: [u8; 12],
}

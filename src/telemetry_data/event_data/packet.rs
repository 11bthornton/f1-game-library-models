use serde::{
    Deserialize,
    Serialize,
};

use super::event_type::EventType;
use crate::telemetry_data::packet_header::PacketHeader;

/// Decoded event data packet with structured event information.
///
/// This structure contains the decoded event data with the specific
/// event type information properly structured.
#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
#[repr(C)]
pub struct PacketEventData
{
    /// Header information for the packet
    pub m_header: PacketHeader,

    /// Event string code (4 characters) identifying the event type
    pub event_string_code: [u8; 4],

    /// The specific event type and its associated data
    pub r#type: EventType,
}

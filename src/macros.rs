/// Generate checked and unchecked accessors for wire bool (`u8`) fields.
///
/// For each field `foo`, generates:
/// - `pub fn is_foo(self) -> Option<bool>` — `None` if the byte is not exactly 0 or 1.
/// - `pub fn is_foo_unchecked(self) -> bool` — treats any non-zero byte as `true`,
///   no validation. Defined behaviour, just permissive.
macro_rules! wire_flag_accessors {
    ($($field:ident),* $(,)?) => {
        paste::paste! {
            $(
                pub fn [<is_ $field>](self) -> Option<bool> {
                    match self.$field {
                        0 => Some(false),
                        1 => Some(true),
                        _ => None,
                    }
                }

                pub fn [<is_ $field _unchecked>](self) -> bool {
                    self.$field != 0
                }
            )*
        }
    };
}

/// Generate checked and unchecked accessors for wire enum (`u8`) fields.
///
/// For each `field => EnumType` pair, generates:
/// - `pub fn field(self) -> Result<EnumType, u8>` — `Err(raw_byte)` if the value
///   does not correspond to a known variant. `EnumType` must implement `TryFrom<u8>`.
/// - `pub unsafe fn field_unchecked(self) -> EnumType` — transmutes the raw byte
///   directly into the enum without checking. **UB if the discriminant is invalid.**
///   Only use when you are certain the source is trusted.
macro_rules! wire_enum_accessors {
    ($($field:ident => $enum:ty),* $(,)?) => {
        paste::paste! {
            $(
                pub fn $field(self) -> Result<$enum, u8> {
                    <$enum>::try_from(self.$field).map_err(|_| self.$field)
                }

                /// # Safety
                /// The raw byte must be a valid discriminant of `$enum`.
                /// Calling this with an out-of-range value is undefined behaviour.
                pub unsafe fn [<$field _unchecked>](self) -> $enum {
                    unsafe { std::mem::transmute(self.$field) }
                }
            )*
        }
    };
}

/// Generate `usize` accessors for wire index (`u8`) fields.
///
/// For each field `foo`, generates:
/// - `pub fn foo(self) -> usize` — casts the raw `u8` to `usize` for direct
///   array indexing. Sentinel values (e.g. `255 = not set`) are the caller's
///   responsibility to check before indexing.
macro_rules! wire_index_accessors {
    ($($field:ident),* $(,)?) => {
        $(
            pub fn $field(self) -> usize {
                self.$field as usize
            }
        )*
    };
}

/// Define the `V2Packet` output enum and the `dispatch` function together.
///
/// Each entry `Variant => PacketType` adds:
/// - A variant `Variant(PacketType)` to `V2Packet`.
/// - A match arm that calls `read_packet::<PacketType>` and wraps the result.
///
/// Adding a new packet type is a single line here — no other files need touching.
macro_rules! define_packets {
    ($($variant:ident => $ty:path),* $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        pub enum V2Packet {
            $($variant($ty),)*
        }

        fn dispatch(packet_id: PacketId, buf: &[u8]) -> Result<V2Packet, ParseError> {
            match packet_id {
                $(PacketId::$variant => read_packet::<$ty>(buf).map(V2Packet::$variant),)*
            }
        }
    };
}

pub(crate) use define_packets;
pub(crate) use wire_enum_accessors;
pub(crate) use wire_flag_accessors;
pub(crate) use wire_index_accessors;

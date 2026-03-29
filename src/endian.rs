/// Convert all multi-byte fields from little-endian wire order to native byte order.
///
/// On little-endian hosts every method is a no-op and the compiler eliminates
/// the calls entirely. On big-endian hosts each multi-byte field is byte-swapped.
///
/// Implemented by value (`self -> Self`) rather than `&mut self` to avoid
/// Rust's restriction on taking references to fields of `repr(packed)` structs.
pub trait FixEndianness: Sized {
    fn fix_endianness(self) -> Self;
}

impl FixEndianness for u8 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        self
    }
}

impl FixEndianness for i16 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        #[cfg(target_endian = "little")] { self }
        #[cfg(not(target_endian = "little"))] { i16::from_le(self) }
    }
}

impl FixEndianness for u16 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        #[cfg(target_endian = "little")] { self }
        #[cfg(not(target_endian = "little"))] { u16::from_le(self) }
    }
}

impl FixEndianness for u32 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        #[cfg(target_endian = "little")] { self }
        #[cfg(not(target_endian = "little"))] { u32::from_le(self) }
    }
}

impl FixEndianness for u64 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        #[cfg(target_endian = "little")] { self }
        #[cfg(not(target_endian = "little"))] { u64::from_le(self) }
    }
}

impl FixEndianness for i8 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        self
    }
}

impl FixEndianness for f32 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        #[cfg(target_endian = "little")] { self }
        #[cfg(not(target_endian = "little"))] { f32::from_bits(u32::from_le(self.to_bits())) }
    }
}

impl FixEndianness for f64 {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        #[cfg(target_endian = "little")] { self }
        #[cfg(not(target_endian = "little"))] { f64::from_bits(u64::from_le(self.to_bits())) }
    }
}

impl<T: FixEndianness + Copy, const N: usize> FixEndianness for [T; N] {
    #[inline(always)]
    fn fix_endianness(self) -> Self {
        #[cfg(target_endian = "little")] { self }
        #[cfg(not(target_endian = "little"))] { self.map(T::fix_endianness) }
    }
}

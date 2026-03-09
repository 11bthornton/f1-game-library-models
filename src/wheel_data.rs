use super::endian::FixEndianness;

/// Per-wheel data in wire order: rear-left, rear-right, front-left, front-right.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct WheelData<T: Copy> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

impl<T: Copy + FixEndianness> FixEndianness for WheelData<T> {
    fn fix_endianness(self) -> Self {
        Self {
            rear_left: self.rear_left.fix_endianness(),
            rear_right: self.rear_right.fix_endianness(),
            front_left: self.front_left.fix_endianness(),
            front_right: self.front_right.fix_endianness(),
        }
    }
}

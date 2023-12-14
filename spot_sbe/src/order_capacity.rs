#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderCapacity {
    Principal = 0x1_u8,
    Agency = 0x2_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderCapacity {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Principal,
            0x2_u8 => Self::Agency,
            _ => Self::NullVal,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RateLimitInterval {
    Second = 0x0_u8,
    Minute = 0x1_u8,
    Hour = 0x2_u8,
    Day = 0x3_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for RateLimitInterval {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Second,
            0x1_u8 => Self::Minute,
            0x2_u8 => Self::Hour,
            0x3_u8 => Self::Day,
            _ => Self::NullVal,
        }
    }
}

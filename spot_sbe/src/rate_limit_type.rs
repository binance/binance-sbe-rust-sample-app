#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RateLimitType {
    RawRequests = 0x0_u8,
    RequestWeight = 0x2_u8,
    Orders = 0x3_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for RateLimitType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::RawRequests,
            0x2_u8 => Self::RequestWeight,
            0x3_u8 => Self::Orders,
            _ => Self::NullVal,
        }
    }
}

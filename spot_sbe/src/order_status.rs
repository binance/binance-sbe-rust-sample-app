#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderStatus {
    New = 0x0_u8,
    PartiallyFilled = 0x1_u8,
    Filled = 0x2_u8,
    Canceled = 0x3_u8,
    PendingCancel = 0x4_u8,
    Rejected = 0x5_u8,
    Expired = 0x6_u8,
    ExpiredInMatch = 0x9_u8,
    Unknown = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::New,
            0x1_u8 => Self::PartiallyFilled,
            0x2_u8 => Self::Filled,
            0x3_u8 => Self::Canceled,
            0x4_u8 => Self::PendingCancel,
            0x5_u8 => Self::Rejected,
            0x6_u8 => Self::Expired,
            0x9_u8 => Self::ExpiredInMatch,
            0xfe_u8 => Self::Unknown,
            _ => Self::NullVal,
        }
    }
}

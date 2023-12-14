#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ListOrderStatus {
    Canceling = 0x0_u8,
    Executing = 0x1_u8,
    AllDone = 0x2_u8,
    Reject = 0x3_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ListOrderStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Canceling,
            0x1_u8 => Self::Executing,
            0x2_u8 => Self::AllDone,
            0x3_u8 => Self::Reject,
            _ => Self::NullVal,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CancelReplaceStatus {
    Success = 0x0_u8,
    Failure = 0x1_u8,
    NotAttempted = 0x2_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for CancelReplaceStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Success,
            0x1_u8 => Self::Failure,
            0x2_u8 => Self::NotAttempted,
            _ => Self::NullVal,
        }
    }
}

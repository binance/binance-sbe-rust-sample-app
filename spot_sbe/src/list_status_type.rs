#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ListStatusType {
    Response = 0x0_u8,
    ExecStarted = 0x1_u8,
    AllDone = 0x2_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ListStatusType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Response,
            0x1_u8 => Self::ExecStarted,
            0x2_u8 => Self::AllDone,
            _ => Self::NullVal,
        }
    }
}

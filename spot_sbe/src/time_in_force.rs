#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TimeInForce {
    Gtc = 0x0_u8,
    Ioc = 0x1_u8,
    Fok = 0x2_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for TimeInForce {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Gtc,
            0x1_u8 => Self::Ioc,
            0x2_u8 => Self::Fok,
            _ => Self::NullVal,
        }
    }
}

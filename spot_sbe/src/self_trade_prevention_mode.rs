#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SelfTradePreventionMode {
    None = 0x1_u8,
    ExpireTaker = 0x2_u8,
    ExpireMaker = 0x3_u8,
    ExpireBoth = 0x4_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for SelfTradePreventionMode {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::None,
            0x2_u8 => Self::ExpireTaker,
            0x3_u8 => Self::ExpireMaker,
            0x4_u8 => Self::ExpireBoth,
            _ => Self::NullVal,
        }
    }
}

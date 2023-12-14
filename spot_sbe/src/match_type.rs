#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum MatchType {
    AutoMatch = 0x1_u8,
    OnePartyTradeReport = 0x2_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for MatchType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::AutoMatch,
            0x2_u8 => Self::OnePartyTradeReport,
            _ => Self::NullVal,
        }
    }
}

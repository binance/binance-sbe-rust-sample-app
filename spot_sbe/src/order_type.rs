#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderType {
    Market = 0x0_u8,
    Limit = 0x1_u8,
    StopLoss = 0x2_u8,
    StopLossLimit = 0x3_u8,
    TakeProfit = 0x4_u8,
    TakeProfitLimit = 0x5_u8,
    LimitMaker = 0x6_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Market,
            0x1_u8 => Self::Limit,
            0x2_u8 => Self::StopLoss,
            0x3_u8 => Self::StopLossLimit,
            0x4_u8 => Self::TakeProfit,
            0x5_u8 => Self::TakeProfitLimit,
            0x6_u8 => Self::LimitMaker,
            _ => Self::NullVal,
        }
    }
}

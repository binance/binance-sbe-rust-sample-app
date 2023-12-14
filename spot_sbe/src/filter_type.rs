#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FilterType {
    MaxPosition = 0x0_u8,
    PriceFilter = 0x1_u8,
    TPlusSell = 0x2_u8,
    LotSize = 0x3_u8,
    MaxNumOrders = 0x4_u8,
    MinNotional = 0x5_u8,
    MaxNumAlgoOrders = 0x6_u8,
    ExchangeMaxNumOrders = 0x7_u8,
    ExchangeMaxNumAlgoOrders = 0x8_u8,
    IcebergParts = 0x9_u8,
    MarketLotSize = 0xa_u8,
    PercentPrice = 0xb_u8,
    MaxNumIcebergOrders = 0xc_u8,
    ExchangeMaxNumIcebergOrders = 0xd_u8,
    TrailingDelta = 0xe_u8,
    PercentPriceBySide = 0xf_u8,
    Notional = 0x10_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for FilterType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::MaxPosition,
            0x1_u8 => Self::PriceFilter,
            0x2_u8 => Self::TPlusSell,
            0x3_u8 => Self::LotSize,
            0x4_u8 => Self::MaxNumOrders,
            0x5_u8 => Self::MinNotional,
            0x6_u8 => Self::MaxNumAlgoOrders,
            0x7_u8 => Self::ExchangeMaxNumOrders,
            0x8_u8 => Self::ExchangeMaxNumAlgoOrders,
            0x9_u8 => Self::IcebergParts,
            0xa_u8 => Self::MarketLotSize,
            0xb_u8 => Self::PercentPrice,
            0xc_u8 => Self::MaxNumIcebergOrders,
            0xd_u8 => Self::ExchangeMaxNumIcebergOrders,
            0xe_u8 => Self::TrailingDelta,
            0xf_u8 => Self::PercentPriceBySide,
            0x10_u8 => Self::Notional,
            _ => Self::NullVal,
        }
    }
}

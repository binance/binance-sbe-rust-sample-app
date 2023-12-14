#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderSide {
    Buy = 0x0_u8,
    Sell = 0x1_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for OrderSide {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Buy,
            0x1_u8 => Self::Sell,
            _ => Self::NullVal,
        }
    }
}

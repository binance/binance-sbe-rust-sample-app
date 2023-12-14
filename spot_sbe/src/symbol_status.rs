#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SymbolStatus {
    PreTrading = 0x0_u8,
    Trading = 0x1_u8,
    PostTrading = 0x2_u8,
    EndOfDay = 0x3_u8,
    Halt = 0x4_u8,
    AuctionMatch = 0x5_u8,
    Break = 0x7_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for SymbolStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::PreTrading,
            0x1_u8 => Self::Trading,
            0x2_u8 => Self::PostTrading,
            0x3_u8 => Self::EndOfDay,
            0x4_u8 => Self::Halt,
            0x5_u8 => Self::AuctionMatch,
            0x7_u8 => Self::Break,
            _ => Self::NullVal,
        }
    }
}

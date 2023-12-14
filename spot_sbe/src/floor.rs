#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Floor {
    Exchange = 0x1_u8,
    Broker = 0x2_u8,
    Sor = 0x3_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for Floor {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Exchange,
            0x2_u8 => Self::Broker,
            0x3_u8 => Self::Sor,
            _ => Self::NullVal,
        }
    }
}

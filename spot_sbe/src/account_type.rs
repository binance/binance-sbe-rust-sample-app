#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AccountType {
    Spot = 0x0_u8,
    Unknown = 0x2_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for AccountType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Spot,
            0x2_u8 => Self::Unknown,
            _ => Self::NullVal,
        }
    }
}

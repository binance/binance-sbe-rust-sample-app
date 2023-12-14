#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BoolEnum {
    False = 0x0_u8,
    True = 0x1_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for BoolEnum {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::False,
            0x1_u8 => Self::True,
            _ => Self::NullVal,
        }
    }
}

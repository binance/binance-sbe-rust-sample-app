#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ContingencyType {
    Oco = 0x1_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for ContingencyType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Oco,
            _ => Self::NullVal,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AllocationTransactionType {
    New = 0x1_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for AllocationTransactionType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::New,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<AllocationTransactionType> for u8 {
    #[inline]
    fn from(v: AllocationTransactionType) -> Self {
        match v {
            AllocationTransactionType::New => 0x1_u8,
            AllocationTransactionType::NonRepresentable => 0xfe_u8,
            AllocationTransactionType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for AllocationTransactionType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "New" => Ok(Self::New),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for AllocationTransactionType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::New => write!(f, "New"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}

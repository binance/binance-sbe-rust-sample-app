#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AllocationStatus {
    Accepted = 0x1_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for AllocationStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Accepted,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<AllocationStatus> for u8 {
    #[inline]
    fn from(v: AllocationStatus) -> Self {
        match v {
            AllocationStatus::Accepted => 0x1_u8,
            AllocationStatus::NonRepresentable => 0xfe_u8,
            AllocationStatus::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for AllocationStatus {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Accepted" => Ok(Self::Accepted),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for AllocationStatus {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}

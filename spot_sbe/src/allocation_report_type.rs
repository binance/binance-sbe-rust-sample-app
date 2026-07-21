#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AllocationReportType {
    Accept = 0x1_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for AllocationReportType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Accept,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<AllocationReportType> for u8 {
    #[inline]
    fn from(v: AllocationReportType) -> Self {
        match v {
            AllocationReportType::Accept => 0x1_u8,
            AllocationReportType::NonRepresentable => 0xfe_u8,
            AllocationReportType::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for AllocationReportType {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Accept" => Ok(Self::Accept),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for AllocationReportType {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Accept => write!(f, "Accept"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}

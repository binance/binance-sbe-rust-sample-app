#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CounterPartyRole {
    Client = 0x1_u8,
    NonRepresentable = 0xfe_u8,
    #[default]
    NullVal = 0xff_u8,
}
impl From<u8> for CounterPartyRole {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Client,
            0xfe_u8 => Self::NonRepresentable,
            _ => Self::NullVal,
        }
    }
}
impl From<CounterPartyRole> for u8 {
    #[inline]
    fn from(v: CounterPartyRole) -> Self {
        match v {
            CounterPartyRole::Client => 0x1_u8,
            CounterPartyRole::NonRepresentable => 0xfe_u8,
            CounterPartyRole::NullVal => 0xff_u8,
        }
    }
}
impl core::str::FromStr for CounterPartyRole {
    type Err = ();

    #[inline]
    fn from_str(v: &str) -> core::result::Result<Self, Self::Err> {
        match v {
            "Client" => Ok(Self::Client),
            "NonRepresentable" => Ok(Self::NonRepresentable),
            _ => Ok(Self::NullVal),
        }
    }
}
impl core::fmt::Display for CounterPartyRole {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Client => write!(f, "Client"),
            Self::NonRepresentable => write!(f, "NonRepresentable"),
            Self::NullVal => write!(f, "NullVal"),
        }
    }
}

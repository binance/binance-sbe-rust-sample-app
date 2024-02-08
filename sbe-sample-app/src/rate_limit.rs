use serde::{Serialize, Serializer};
use spot_sbe::{RateLimitInterval, RateLimitType};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    #[serde(serialize_with = "serialize_rate_limit_type")]
    pub rate_limit_type: RateLimitType,
    #[serde(serialize_with = "serialize_rate_limit_interval")]
    pub interval: RateLimitInterval,
    pub interval_num: u8,
    pub limit: i64,
    pub count: Option<i64>,
}

fn serialize_rate_limit_type<S: Serializer>(
    val: &RateLimitType,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use RateLimitType::*;
    let str_val = match val {
        Orders => "ORDERS",
        RawRequests => "RAW_REQUESTS",
        Connections => "CONNECTIONS",
        RequestWeight => "REQUEST_WEIGHT",
        NullVal => return serializer.serialize_none(),
    };
    serializer.serialize_str(str_val)
}

fn serialize_rate_limit_interval<S: Serializer>(
    val: &RateLimitInterval,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use RateLimitInterval::*;
    let str_val = match val {
        Second => "SECOND",
        Minute => "MINUTE",
        Hour => "HOUR",
        Day => "DAY",
        NullVal => return serializer.serialize_none(),
    };
    serializer.serialize_str(str_val)
}

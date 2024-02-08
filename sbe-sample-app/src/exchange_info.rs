use crate::rate_limit::RateLimit;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use spot_sbe::{
    AllowedSelfTradePreventionModes, OrderTypes, SelfTradePreventionMode, SymbolStatus,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: i16,
    pub server_time: Option<i64>,
    pub retry_after: Option<i64>,
    pub msg: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Decimal {
    mantissa: i64,
    exponent: i8,
}

impl Decimal {
    pub fn new(mantissa: i64, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize)]
#[serde(
    tag = "filterType",
    rename_all = "SCREAMING_SNAKE_CASE",
    rename_all_fields = "camelCase"
)]
pub enum ExchangeFilter {
    MaxNumOrders { max_num_orders: i64 },
    MaxNumAlgoOrders { max_num_algo_orders: i64 },
    MaxNumIcebergOrders { max_num_iceberg_orders: i64 },
}

#[derive(Serialize)]
#[serde(
    tag = "filterType",
    rename_all = "SCREAMING_SNAKE_CASE",
    rename_all_fields = "camelCase"
)]
pub enum SymbolFilter {
    Price {
        min_price: Decimal,
        max_price: Decimal,
        tick_size: Decimal,
    },
    PercentPrice {
        multiplier_up: Decimal,
        multiplier_down: Decimal,
        avg_price_mins: i32,
    },
    PercentPriceBySide {
        bid_multiplier_up: Decimal,
        bid_multiplier_down: Decimal,
        ask_multiplier_up: Decimal,
        ask_multiplier_down: Decimal,
        avg_price_mins: i32,
    },
    LotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    MinNotional {
        min_notional: Decimal,
        apply_to_market: bool,
        avg_price_mins: i32,
    },
    Notional {
        min_notional: Decimal,
        apply_min_to_market: bool,
        max_notional: Decimal,
        apply_max_to_market: bool,
        avg_price_mins: i32,
    },
    IcebergParts {
        filter_limit: i64,
    },
    MarketLotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },
    MaxNumOrders {
        max_num_orders: i64,
    },
    MaxNumAlgoOrders {
        max_num_algo_orders: i64,
    },
    MaxNumIcebergOrders {
        max_num_iceberg_orders: i64,
    },
    MaxPosition {
        max_position: Decimal,
    },
    TrailingDelta {
        min_trailing_above_delta: i64,
        max_trailing_above_delta: i64,
        min_trailing_below_delta: i64,
        max_trailing_below_delta: i64,
    },
    TPlusSell {
        end_time: Option<i64>,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sor {
    pub symbols: Vec<String>,
    pub base_asset: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    #[serde(serialize_with = "serialize_symbol_status")]
    pub status: SymbolStatus,
    pub base_asset_precision: u8,
    pub quote_asset_precision: u8,
    pub base_commission_precision: u8,
    pub quote_commission_precision: u8,
    #[serde(serialize_with = "serialize_order_types")]
    pub order_types: OrderTypes,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub cancel_replace_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    #[serde(serialize_with = "serialize_self_trade_prevention_mode")]
    pub default_self_trade_prevention_mode: SelfTradePreventionMode,
    #[serde(serialize_with = "serialize_allowed_self_trade_prevention_modes")]
    pub allowed_self_trade_prevention_modes: AllowedSelfTradePreventionModes,
    pub filters: Vec<SymbolFilter>,
    pub permissions: Vec<String>,
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub symbols: Vec<SymbolInfo>,
    pub sors: Vec<Sor>,
}

fn serialize_symbol_status<S: Serializer>(
    val: &SymbolStatus,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use SymbolStatus::*;
    let str_val = match val {
        PreTrading => "PRE_TRADING",
        Trading => "TRADING",
        PostTrading => "POST_TRADING",
        EndOfDay => "END_OF_DAY",
        Halt => "HALT",
        AuctionMatch => "AUCTION_MATCH",
        Break => "BREAK",
        NullVal => return serializer.serialize_none(),
    };
    serializer.serialize_str(str_val)
}

fn serialize_order_types<S: Serializer>(
    val: &OrderTypes,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut strings = Vec::with_capacity(10);
    if val.get_market() {
        strings.push("MARKET");
    }
    if val.get_limit() {
        strings.push("LIMIT");
    }
    if val.get_stop_loss() {
        strings.push("STOP_LOSS");
    }
    if val.get_stop_loss_limit() {
        strings.push("STOP_LOSS_LIMIT");
    }
    if val.get_take_profit() {
        strings.push("TAKE_PROFIT");
    }
    if val.get_take_profit_limit() {
        strings.push("TAKE_PROFIT_LIMIT");
    }
    if val.get_limit_maker() {
        strings.push("LIMIT_MAKER");
    }
    let mut seq = serializer.serialize_seq(Some(strings.len()))?;
    for s in strings {
        seq.serialize_element(s)?;
    }
    seq.end()
}

fn serialize_self_trade_prevention_mode<S: Serializer>(
    val: &SelfTradePreventionMode,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use SelfTradePreventionMode::*;
    let str_val = match val {
        None => "NONE",
        ExpireTaker => "EXPIRE_TAKER",
        ExpireMaker => "EXPIRE_MAKER",
        ExpireBoth => "EXPIRE_BOTH",
        NullVal => return serializer.serialize_none(),
    };
    serializer.serialize_str(str_val)
}

fn serialize_allowed_self_trade_prevention_modes<S: Serializer>(
    val: &AllowedSelfTradePreventionModes,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut strings = Vec::with_capacity(4);
    if val.get_none() {
        strings.push("NONE");
    }
    if val.get_expire_taker() {
        strings.push("EXPIRE_TAKER");
    }
    if val.get_expire_maker() {
        strings.push("EXPIRE_MAKER");
    }
    if val.get_expire_both() {
        strings.push("EXPIRE_BOTH");
    }
    let mut seq = serializer.serialize_seq(Some(strings.len()))?;
    for s in strings {
        seq.serialize_element(s)?;
    }
    seq.end()
}

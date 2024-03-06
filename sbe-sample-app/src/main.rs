use crate::{
    exchange_info::{
        Decimal, ErrorResponse, ExchangeFilter, ExchangeInfo, Sor, SymbolFilter, SymbolInfo,
    },
    rate_limit::RateLimit,
    websocket::WebSocketMetadata,
};
use anyhow::bail;
use spot_sbe::{
    error_response_codec, exchange_info_response_codec, exchange_max_num_algo_orders_filter_codec,
    exchange_max_num_iceberg_orders_filter_codec, exchange_max_num_orders_filter_codec,
    iceberg_parts_filter_codec, lot_size_filter_codec, market_lot_size_filter_codec,
    max_num_algo_orders_filter_codec, max_num_iceberg_orders_filter_codec,
    max_num_orders_filter_codec, max_position_filter_codec, min_notional_filter_codec,
    notional_filter_codec, percent_price_by_side_filter_codec, percent_price_filter_codec,
    price_filter_codec, tp_lus_sell_filter_codec, trailing_delta_filter_codec,
    web_socket_response_codec, BoolEnum, ErrorResponseDecoder, ExchangeInfoResponseDecoder,
    ExchangeMaxNumAlgoOrdersFilterDecoder, ExchangeMaxNumIcebergOrdersFilterDecoder,
    ExchangeMaxNumOrdersFilterDecoder, IcebergPartsFilterDecoder, LotSizeFilterDecoder,
    MarketLotSizeFilterDecoder, MaxNumAlgoOrdersFilterDecoder, MaxNumIcebergOrdersFilterDecoder,
    MaxNumOrdersFilterDecoder, MaxPositionFilterDecoder, MessageHeaderDecoder,
    MinNotionalFilterDecoder, NotionalFilterDecoder, PercentPriceBySideFilterDecoder,
    PercentPriceFilterDecoder, PriceFilterDecoder, ReadBuf, TPlusSellFilterDecoder,
    TrailingDeltaFilterDecoder, WebSocketResponseDecoder,
};
use std::io::{self, Read};

mod exchange_info;
mod rate_limit;
mod websocket;

fn read_payload(mut stream: impl Read) -> io::Result<Vec<u8>> {
    let mut payload = Vec::with_capacity(64 * 1024);
    stream.read_to_end(&mut payload)?;
    Ok(payload)
}

fn into_bool(value: BoolEnum) -> anyhow::Result<bool> {
    Ok(match value {
        BoolEnum::True => true,
        BoolEnum::False => false,
        BoolEnum::NullVal => {
            bail!("Bool value unexpectedly set to NullVal");
        }
    })
}

fn decode_error(header: MessageHeaderDecoder<ReadBuf<'_>>) -> anyhow::Result<ErrorResponse> {
    let mut decoder = ErrorResponseDecoder::default().header(header);
    let response = ErrorResponse {
        code: decoder.code(),
        server_time: decoder.server_time(),
        retry_after: decoder.retry_after(),
        msg: {
            let (offset, length) = decoder.msg_decoder();
            let slice = decoder.msg_slice((offset, length));
            String::from_utf8(slice.into())?
        },
    };
    Ok(response)
}

fn decode_exchange_filter(
    header: MessageHeaderDecoder<ReadBuf<'_>>,
) -> anyhow::Result<ExchangeFilter> {
    Ok(match header.template_id() {
        exchange_max_num_orders_filter_codec::SBE_TEMPLATE_ID => {
            let decoder = ExchangeMaxNumOrdersFilterDecoder::default().header(header);
            ExchangeFilter::MaxNumOrders {
                max_num_orders: decoder.max_num_orders(),
            }
        }
        exchange_max_num_algo_orders_filter_codec::SBE_TEMPLATE_ID => {
            let decoder = ExchangeMaxNumAlgoOrdersFilterDecoder::default().header(header);
            ExchangeFilter::MaxNumAlgoOrders {
                max_num_algo_orders: decoder.max_num_algo_orders(),
            }
        }

        exchange_max_num_iceberg_orders_filter_codec::SBE_TEMPLATE_ID => {
            let decoder = ExchangeMaxNumIcebergOrdersFilterDecoder::default().header(header);
            ExchangeFilter::MaxNumIcebergOrders {
                max_num_iceberg_orders: decoder.max_num_iceberg_orders(),
            }
        }
        _ => {
            bail!("Unexpected SBE template ID: {}", header.template_id());
        }
    })
}

fn decode_symbol_filter(header: MessageHeaderDecoder<ReadBuf<'_>>) -> anyhow::Result<SymbolFilter> {
    Ok(match header.template_id() {
        price_filter_codec::SBE_TEMPLATE_ID => {
            let filter = PriceFilterDecoder::default().header(header);
            let exponent = filter.price_exponent();
            SymbolFilter::Price {
                min_price: Decimal::new(filter.min_price(), exponent),
                max_price: Decimal::new(filter.max_price(), exponent),
                tick_size: Decimal::new(filter.tick_size(), exponent),
            }
        }
        percent_price_filter_codec::SBE_TEMPLATE_ID => {
            let filter = PercentPriceFilterDecoder::default().header(header);
            let exponent = filter.multiplier_exponent();
            SymbolFilter::PercentPrice {
                multiplier_up: Decimal::new(filter.multiplier_up(), exponent),
                multiplier_down: Decimal::new(filter.multiplier_down(), exponent),
                avg_price_mins: filter.avg_price_mins(),
            }
        }
        percent_price_by_side_filter_codec::SBE_TEMPLATE_ID => {
            let filter = PercentPriceBySideFilterDecoder::default().header(header);
            let exponent = filter.multiplier_exponent();
            SymbolFilter::PercentPriceBySide {
                bid_multiplier_up: Decimal::new(filter.bid_multiplier_up(), exponent),
                bid_multiplier_down: Decimal::new(filter.bid_multiplier_down(), exponent),
                ask_multiplier_up: Decimal::new(filter.ask_multiplier_up(), exponent),
                ask_multiplier_down: Decimal::new(filter.ask_multiplier_down(), exponent),
                avg_price_mins: filter.avg_price_mins(),
            }
        }
        lot_size_filter_codec::SBE_TEMPLATE_ID => {
            let filter = LotSizeFilterDecoder::default().header(header);
            let exponent = filter.qty_exponent();
            SymbolFilter::LotSize {
                min_qty: Decimal::new(filter.min_qty(), exponent),
                max_qty: Decimal::new(filter.max_qty(), exponent),
                step_size: Decimal::new(filter.step_size(), exponent),
            }
        }
        min_notional_filter_codec::SBE_TEMPLATE_ID => {
            let filter = MinNotionalFilterDecoder::default().header(header);
            let exponent = filter.price_exponent();
            SymbolFilter::MinNotional {
                min_notional: Decimal::new(filter.min_notional(), exponent),
                apply_to_market: into_bool(filter.apply_to_market())?,
                avg_price_mins: filter.avg_price_mins(),
            }
        }
        notional_filter_codec::SBE_TEMPLATE_ID => {
            let filter = NotionalFilterDecoder::default().header(header);
            let exponent = filter.price_exponent();
            SymbolFilter::Notional {
                min_notional: Decimal::new(filter.min_notional(), exponent),
                apply_min_to_market: into_bool(filter.apply_min_to_market())?,
                max_notional: Decimal::new(filter.max_notional(), exponent),
                apply_max_to_market: into_bool(filter.apply_max_to_market())?,
                avg_price_mins: filter.avg_price_mins(),
            }
        }
        iceberg_parts_filter_codec::SBE_TEMPLATE_ID => {
            let filter = IcebergPartsFilterDecoder::default().header(header);
            SymbolFilter::IcebergParts {
                filter_limit: filter.filter_limit(),
            }
        }
        market_lot_size_filter_codec::SBE_TEMPLATE_ID => {
            let filter = MarketLotSizeFilterDecoder::default().header(header);
            let exponent = filter.qty_exponent();
            SymbolFilter::MarketLotSize {
                min_qty: Decimal::new(filter.min_qty(), exponent),
                max_qty: Decimal::new(filter.max_qty(), exponent),
                step_size: Decimal::new(filter.step_size(), exponent),
            }
        }
        max_num_orders_filter_codec::SBE_TEMPLATE_ID => {
            let filter = MaxNumOrdersFilterDecoder::default().header(header);
            SymbolFilter::MaxNumOrders {
                max_num_orders: filter.max_num_orders(),
            }
        }
        max_num_algo_orders_filter_codec::SBE_TEMPLATE_ID => {
            let filter = MaxNumAlgoOrdersFilterDecoder::default().header(header);
            SymbolFilter::MaxNumAlgoOrders {
                max_num_algo_orders: filter.max_num_algo_orders(),
            }
        }
        max_num_iceberg_orders_filter_codec::SBE_TEMPLATE_ID => {
            let filter = MaxNumIcebergOrdersFilterDecoder::default().header(header);
            SymbolFilter::MaxNumIcebergOrders {
                max_num_iceberg_orders: filter.max_num_iceberg_orders(),
            }
        }
        max_position_filter_codec::SBE_TEMPLATE_ID => {
            let filter = MaxPositionFilterDecoder::default().header(header);
            let exponent = filter.qty_exponent();
            SymbolFilter::MaxPosition {
                max_position: Decimal::new(filter.max_position(), exponent),
            }
        }
        trailing_delta_filter_codec::SBE_TEMPLATE_ID => {
            let filter = TrailingDeltaFilterDecoder::default().header(header);
            SymbolFilter::TrailingDelta {
                min_trailing_above_delta: filter.min_trailing_above_delta(),
                max_trailing_above_delta: filter.max_trailing_above_delta(),
                min_trailing_below_delta: filter.min_trailing_below_delta(),
                max_trailing_below_delta: filter.max_trailing_below_delta(),
            }
        }
        tp_lus_sell_filter_codec::SBE_TEMPLATE_ID => {
            let filter = TPlusSellFilterDecoder::default().header(header);
            SymbolFilter::TPlusSell {
                end_time: filter.end_time(),
            }
        }
        template_id => {
            bail!("Unexpected symbol filter message ID: {template_id}");
        }
    })
}

fn decode_websocket_metadata(
    header: MessageHeaderDecoder<ReadBuf<'_>>,
) -> anyhow::Result<(WebSocketMetadata, usize)> {
    let decoder = WebSocketResponseDecoder::default().header(header);
    if into_bool(decoder.sbe_schema_id_version_deprecated())? {
        println!("Warning: sbe-sample-app is using a deprecated schema");
    }
    let status = decoder.status();
    let mut decoder = decoder.rate_limits_decoder();
    let count = decoder.count();
    let mut rate_limits = Vec::with_capacity(count.try_into()?);
    for _ in 0..count {
        decoder.advance()?;
        let rate_limit = RateLimit {
            rate_limit_type: decoder.rate_limit_type(),
            interval: decoder.interval(),
            interval_num: decoder.interval_num(),
            limit: decoder.rate_limit(),
            count: Some(decoder.current()),
        };
        rate_limits.push(rate_limit);
    }
    let mut decoder = decoder.parent()?;
    let coordinates = decoder.id_decoder();
    let id = decoder.id_slice(coordinates);
    let id = String::from_utf8(id.to_vec())?;
    let response = WebSocketMetadata::new(status, rate_limits, id);
    let coordinates = decoder.result_decoder();
    Ok((response, coordinates.0))
}

fn main() -> anyhow::Result<()> {
    let payload = read_payload(io::stdin())?;
    let mut decoder = MessageHeaderDecoder::default().wrap(ReadBuf::new(&payload), 0);
    // A separate "ErrorResponse" message is returned for errors and its format
    // is expected to be backwards compatible across all schema IDs.
    if decoder.template_id() == error_response_codec::SBE_TEMPLATE_ID {
        let response = decode_error(decoder)?;
        let yaml = serde_yaml::to_string(&response)?;
        bail!(yaml);
    }
    let mut websocket_meta = None;
    let schema_id = decoder.schema_id();
    if schema_id != exchange_info_response_codec::SBE_SCHEMA_ID {
        bail!(
            "Unexpected schema ID. Got {schema_id}; expected {}",
            exchange_info_response_codec::SBE_SCHEMA_ID
        );
    }
    let version = decoder.version();
    if version != exchange_info_response_codec::SBE_SCHEMA_VERSION {
        println!(
            "Warning: Unexpected schema version. Got {version}; expected {}",
            exchange_info_response_codec::SBE_SCHEMA_VERSION,
        );
        // Schemas with the same ID are expected to be backwards compatible.
    }
    if decoder.template_id() == web_socket_response_codec::SBE_TEMPLATE_ID {
        let (websocket, offset) = decode_websocket_metadata(decoder)?;
        websocket_meta = Some(websocket);
        decoder = MessageHeaderDecoder::default().wrap(ReadBuf::new(&payload[offset..]), 0);
    }
    if decoder.template_id() == error_response_codec::SBE_TEMPLATE_ID {
        let response = decode_error(decoder)?;
        let yaml = if let Some(websocket_meta) = websocket_meta.as_mut() {
            websocket_meta.set_error(response);
            serde_yaml::to_string(&websocket_meta)?
        } else {
            serde_yaml::to_string(&response)?
        };
        bail!(yaml);
    }
    let decoder = ExchangeInfoResponseDecoder::default().header(decoder);
    let mut decoder = decoder.rate_limits_decoder();
    let count = decoder.count();
    let mut rate_limits = Vec::with_capacity(count.try_into()?);
    for _ in 0..count {
        decoder.advance()?;
        let rate_limit = RateLimit {
            rate_limit_type: decoder.rate_limit_type(),
            interval: decoder.interval(),
            interval_num: decoder.interval_num(),
            limit: decoder.rate_limit(),
            count: None,
        };
        rate_limits.push(rate_limit);
    }
    let decoder = decoder.parent()?;

    let mut decoder = decoder.exchange_filters_decoder();
    let count = decoder.count();
    let mut exchange_filters = Vec::with_capacity(count.try_into()?);
    for _ in 0..count {
        decoder.advance()?;
        let coordinates = decoder.filter_decoder();
        let slice = decoder.filter_slice(coordinates);
        let decoder = MessageHeaderDecoder::default().wrap(ReadBuf::new(slice), 0);
        let filter = decode_exchange_filter(decoder)?;
        exchange_filters.push(filter);
    }
    let decoder = decoder.parent()?;

    let mut decoder = decoder.symbols_decoder();
    let count = decoder.count().try_into()?;
    let mut symbols = Vec::with_capacity(count);
    for _ in 0..count {
        decoder.advance()?;
        symbols.push(SymbolInfo {
            status: decoder.status(),
            base_asset_precision: decoder.base_asset_precision(),
            quote_asset_precision: decoder.quote_asset_precision(),
            base_commission_precision: decoder.base_commission_precision(),
            quote_commission_precision: decoder.quote_commission_precision(),
            order_types: decoder.order_types(),
            iceberg_allowed: into_bool(decoder.iceberg_allowed())?,
            oco_allowed: into_bool(decoder.oco_allowed())?,
            quote_order_qty_market_allowed: into_bool(decoder.quote_order_qty_market_allowed())?,
            allow_trailing_stop: into_bool(decoder.allow_trailing_stop())?,
            cancel_replace_allowed: into_bool(decoder.cancel_replace_allowed())?,
            is_spot_trading_allowed: into_bool(decoder.is_spot_trading_allowed())?,
            is_margin_trading_allowed: into_bool(decoder.is_margin_trading_allowed())?,
            default_self_trade_prevention_mode: decoder.default_self_trade_prevention_mode(),
            allowed_self_trade_prevention_modes: decoder.allowed_self_trade_prevention_modes(),
            filters: {
                let mut filters_decoder = decoder.filters_decoder();
                let count = filters_decoder.count().try_into()?;
                let mut filters = Vec::with_capacity(count);
                for _ in 0..count {
                    filters_decoder.advance()?;
                    let coordinates = filters_decoder.filter_decoder();
                    let slice = filters_decoder.filter_slice(coordinates);
                    let header = MessageHeaderDecoder::default().wrap(ReadBuf::new(slice), 0);
                    let filter = decode_symbol_filter(header)?;
                    filters.push(filter);
                }
                decoder = filters_decoder.parent()?;
                filters
            },
            permissions: {
                let mut permissions_decoder = decoder.permissions_decoder();
                let count = permissions_decoder.count().try_into()?;
                let mut permissions = Vec::with_capacity(count);
                for _ in 0..count {
                    permissions_decoder.advance()?;
                    let coordinates = permissions_decoder.permission_decoder();
                    let slice = permissions_decoder.permission_slice(coordinates);
                    permissions.push(String::from_utf8(slice.into())?);
                }
                decoder = permissions_decoder.parent()?;
                permissions
            },
            symbol: {
                let coordinates = decoder.symbol_decoder();
                let slice = decoder.symbol_slice(coordinates);
                String::from_utf8(slice.into())?
            },
            base_asset: {
                let coordinates = decoder.base_asset_decoder();
                let slice = decoder.base_asset_slice(coordinates);
                String::from_utf8(slice.into())?
            },
            quote_asset: {
                let coordinates = decoder.quote_asset_decoder();
                let slice = decoder.quote_asset_slice(coordinates);
                String::from_utf8(slice.into())?
            },
        });
    }
    let decoder = decoder.parent()?;

    let mut decoder = decoder.sors_decoder();
    let count = decoder.count().try_into()?;
    let mut sors = Vec::with_capacity(count);
    for _ in 0..count {
        decoder.advance()?;
        let mut symbols_decoder = decoder.sor_symbols_decoder();
        let count = symbols_decoder.count().try_into()?;
        let mut symbols = Vec::with_capacity(count);
        for _ in 0..count {
            symbols_decoder.advance()?;
            let coordinates = symbols_decoder.symbol_decoder();
            let slice = symbols_decoder.symbol_slice(coordinates);
            symbols.push(String::from_utf8(slice.into())?);
        }
        decoder = symbols_decoder.parent()?;
        let coordinates = decoder.base_asset_decoder();
        let slice = decoder.base_asset_slice(coordinates);
        let base_asset = String::from_utf8(slice.into())?;
        sors.push(Sor {
            symbols,
            base_asset,
        });
    }
    let response = ExchangeInfo {
        rate_limits,
        exchange_filters,
        symbols,
        sors,
    };
    let yaml = if let Some(websocket_meta) = websocket_meta.as_mut() {
        websocket_meta.set_exchange_info(response);
        serde_yaml::to_string(&websocket_meta)?
    } else {
        serde_yaml::to_string(&response)?
    };
    println!("{}", yaml);
    Ok(())
}

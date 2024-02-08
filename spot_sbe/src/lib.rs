#![forbid(unsafe_code)]
#![allow(ambiguous_glob_reexports)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
use ::core::convert::TryInto;

pub mod account_allocations_response_codec;
pub mod account_commission_response_codec;
pub mod account_order_rate_limit_response_codec;
pub mod account_prevented_matches_response_codec;
pub mod account_response_codec;
pub mod account_trades_response_codec;
pub mod account_type;
pub mod agg_trades_response_codec;
pub mod allocation_type;
pub mod allowed_self_trade_prevention_modes;
pub mod average_price_response_codec;
pub mod book_ticker_response_codec;
pub mod book_ticker_symbol_response_codec;
pub mod bool_enum;
pub mod cancel_open_orders_response_codec;
pub mod cancel_order_list_response_codec;
pub mod cancel_order_response_codec;
pub mod cancel_replace_order_response_codec;
pub mod cancel_replace_status;
pub mod contingency_type;
pub mod depth_response_codec;
pub mod error_response_codec;
pub mod exchange_info_response_codec;
pub mod exchange_max_num_algo_orders_filter_codec;
pub mod exchange_max_num_iceberg_orders_filter_codec;
pub mod exchange_max_num_orders_filter_codec;
pub mod filter_type;
pub mod floor;
pub mod group_size_16_encoding_codec;
pub mod group_size_encoding_codec;
pub mod iceberg_parts_filter_codec;
pub mod klines_response_codec;
pub mod list_order_status;
pub mod list_status_type;
pub mod lot_size_filter_codec;
pub mod market_lot_size_filter_codec;
pub mod match_type;
pub mod max_num_algo_orders_filter_codec;
pub mod max_num_iceberg_orders_filter_codec;
pub mod max_num_orders_filter_codec;
pub mod max_position_filter_codec;
pub mod message_data_16_codec;
pub mod message_data_8_codec;
pub mod message_data_codec;
pub mod message_header_codec;
pub mod min_notional_filter_codec;
pub mod new_order_ack_response_codec;
pub mod new_order_full_response_codec;
pub mod new_order_list_ack_response_codec;
pub mod new_order_list_full_response_codec;
pub mod new_order_list_result_response_codec;
pub mod new_order_result_response_codec;
pub mod notional_filter_codec;
pub mod optional_message_data_16_codec;
pub mod optional_message_data_codec;
pub mod optional_var_string_8_codec;
pub mod optional_var_string_codec;
pub mod order_capacity;
pub mod order_list_response_codec;
pub mod order_lists_response_codec;
pub mod order_response_codec;
pub mod order_side;
pub mod order_status;
pub mod order_test_response_codec;
pub mod order_test_with_commissions_response_codec;
pub mod order_type;
pub mod order_types;
pub mod orders_response_codec;
pub mod percent_price_by_side_filter_codec;
pub mod percent_price_filter_codec;
pub mod ping_response_codec;
pub mod price_filter_codec;
pub mod price_ticker_response_codec;
pub mod price_ticker_symbol_response_codec;
pub mod rate_limit_interval;
pub mod rate_limit_type;
pub mod self_trade_prevention_mode;
pub mod server_time_response_codec;
pub mod symbol_status;
pub mod ticker_24_hf_ull_response_codec;
pub mod ticker_24_hm_ini_response_codec;
pub mod ticker_24_hs_ymbol_full_response_codec;
pub mod ticker_24_hs_ymbol_mini_response_codec;
pub mod ticker_full_response_codec;
pub mod ticker_mini_response_codec;
pub mod ticker_symbol_full_response_codec;
pub mod ticker_symbol_mini_response_codec;
pub mod time_in_force;
pub mod tp_lus_sell_filter_codec;
pub mod trades_response_codec;
pub mod trailing_delta_filter_codec;
pub mod user_data_stream_ping_response_codec;
pub mod user_data_stream_start_response_codec;
pub mod user_data_stream_stop_response_codec;
pub mod var_string_8_codec;
pub mod var_string_codec;
pub mod web_socket_response_codec;
pub mod web_socket_session_logon_response_codec;
pub mod web_socket_session_logout_response_codec;
pub mod web_socket_session_status_response_codec;

pub use crate::account_allocations_response_codec::*;
pub use crate::account_commission_response_codec::*;
pub use crate::account_order_rate_limit_response_codec::*;
pub use crate::account_prevented_matches_response_codec::*;
pub use crate::account_response_codec::*;
pub use crate::account_trades_response_codec::*;
pub use crate::account_type::*;
pub use crate::agg_trades_response_codec::*;
pub use crate::allocation_type::*;
pub use crate::allowed_self_trade_prevention_modes::*;
pub use crate::average_price_response_codec::*;
pub use crate::book_ticker_response_codec::*;
pub use crate::book_ticker_symbol_response_codec::*;
pub use crate::bool_enum::*;
pub use crate::cancel_open_orders_response_codec::*;
pub use crate::cancel_order_list_response_codec::*;
pub use crate::cancel_order_response_codec::*;
pub use crate::cancel_replace_order_response_codec::*;
pub use crate::cancel_replace_status::*;
pub use crate::contingency_type::*;
pub use crate::depth_response_codec::*;
pub use crate::error_response_codec::*;
pub use crate::exchange_info_response_codec::*;
pub use crate::exchange_max_num_algo_orders_filter_codec::*;
pub use crate::exchange_max_num_iceberg_orders_filter_codec::*;
pub use crate::exchange_max_num_orders_filter_codec::*;
pub use crate::filter_type::*;
pub use crate::floor::*;
pub use crate::group_size_16_encoding_codec::*;
pub use crate::group_size_encoding_codec::*;
pub use crate::iceberg_parts_filter_codec::*;
pub use crate::klines_response_codec::*;
pub use crate::list_order_status::*;
pub use crate::list_status_type::*;
pub use crate::lot_size_filter_codec::*;
pub use crate::market_lot_size_filter_codec::*;
pub use crate::match_type::*;
pub use crate::max_num_algo_orders_filter_codec::*;
pub use crate::max_num_iceberg_orders_filter_codec::*;
pub use crate::max_num_orders_filter_codec::*;
pub use crate::max_position_filter_codec::*;
pub use crate::message_data_16_codec::*;
pub use crate::message_data_8_codec::*;
pub use crate::message_data_codec::*;
pub use crate::message_header_codec::*;
pub use crate::min_notional_filter_codec::*;
pub use crate::new_order_ack_response_codec::*;
pub use crate::new_order_full_response_codec::*;
pub use crate::new_order_list_ack_response_codec::*;
pub use crate::new_order_list_full_response_codec::*;
pub use crate::new_order_list_result_response_codec::*;
pub use crate::new_order_result_response_codec::*;
pub use crate::notional_filter_codec::*;
pub use crate::optional_message_data_16_codec::*;
pub use crate::optional_message_data_codec::*;
pub use crate::optional_var_string_8_codec::*;
pub use crate::optional_var_string_codec::*;
pub use crate::order_capacity::*;
pub use crate::order_list_response_codec::*;
pub use crate::order_lists_response_codec::*;
pub use crate::order_response_codec::*;
pub use crate::order_side::*;
pub use crate::order_status::*;
pub use crate::order_test_response_codec::*;
pub use crate::order_test_with_commissions_response_codec::*;
pub use crate::order_type::*;
pub use crate::order_types::*;
pub use crate::orders_response_codec::*;
pub use crate::percent_price_by_side_filter_codec::*;
pub use crate::percent_price_filter_codec::*;
pub use crate::ping_response_codec::*;
pub use crate::price_filter_codec::*;
pub use crate::price_ticker_response_codec::*;
pub use crate::price_ticker_symbol_response_codec::*;
pub use crate::rate_limit_interval::*;
pub use crate::rate_limit_type::*;
pub use crate::self_trade_prevention_mode::*;
pub use crate::server_time_response_codec::*;
pub use crate::symbol_status::*;
pub use crate::ticker_24_hf_ull_response_codec::*;
pub use crate::ticker_24_hm_ini_response_codec::*;
pub use crate::ticker_24_hs_ymbol_full_response_codec::*;
pub use crate::ticker_24_hs_ymbol_mini_response_codec::*;
pub use crate::ticker_full_response_codec::*;
pub use crate::ticker_mini_response_codec::*;
pub use crate::ticker_symbol_full_response_codec::*;
pub use crate::ticker_symbol_mini_response_codec::*;
pub use crate::time_in_force::*;
pub use crate::tp_lus_sell_filter_codec::*;
pub use crate::trades_response_codec::*;
pub use crate::trailing_delta_filter_codec::*;
pub use crate::user_data_stream_ping_response_codec::*;
pub use crate::user_data_stream_start_response_codec::*;
pub use crate::user_data_stream_stop_response_codec::*;
pub use crate::var_string_8_codec::*;
pub use crate::var_string_codec::*;
pub use crate::web_socket_response_codec::*;
pub use crate::web_socket_session_logon_response_codec::*;
pub use crate::web_socket_session_logout_response_codec::*;
pub use crate::web_socket_session_status_response_codec::*;

pub type SbeResult<T> = core::result::Result<T, SbeErr>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SbeErr {
    ParentNotSet,
}
impl core::fmt::Display for SbeErr {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for SbeErr {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait Writer<'a>: Sized {
    fn get_buf_mut(&mut self) -> &mut WriteBuf<'a>;
}

pub trait Encoder<'a>: Writer<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

pub trait Reader<'a>: Sized {
    fn get_buf(&self) -> &ReadBuf<'a>;
}

pub trait Decoder<'a>: Reader<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReadBuf<'a> {
    data: &'a [u8],
}
impl<'a> Reader<'a> for ReadBuf<'a> {
    #[inline]
    fn get_buf(&self) -> &ReadBuf<'a> {
        self
    }
}
impl<'a> ReadBuf<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[inline]
    fn get_bytes<const COUNT: usize>(slice: &[u8]) -> [u8; COUNT] {
        slice.try_into().expect("slice with incorrect length")
    }

    #[inline]
    pub(crate) fn get_bytes_at<const N: usize>(slice: &[u8], index: usize) -> [u8; N] {
        slice[index..index + N]
            .try_into()
            .expect("slice with incorrect length")
    }

    #[inline]
    pub fn get_u8_at(&self, index: usize) -> u8 {
        self.data[index]
    }

    #[inline]
    pub fn get_i8_at(&self, index: usize) -> i8 {
        i8::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i16_at(&self, index: usize) -> i16 {
        i16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i32_at(&self, index: usize) -> i32 {
        i32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i64_at(&self, index: usize) -> i64 {
        i64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u16_at(&self, index: usize) -> u16 {
        u16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u32_at(&self, index: usize) -> u32 {
        u32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u64_at(&self, index: usize) -> u64 {
        u64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f32_at(&self, index: usize) -> f32 {
        f32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f64_at(&self, index: usize) -> f64 {
        f64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_slice_at(&self, index: usize, len: usize) -> &[u8] {
        &self.data[index..index + len]
    }
}

#[derive(Debug, Default)]
pub struct WriteBuf<'a> {
    data: &'a mut [u8],
}
impl<'a> WriteBuf<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn put_bytes_at<const COUNT: usize>(&mut self, index: usize, bytes: [u8; COUNT]) -> usize {
        self.data[index..index + COUNT].copy_from_slice(&bytes);
        COUNT
    }

    #[inline]
    pub fn put_u8_at(&mut self, index: usize, value: u8) {
        self.data[index] = value;
    }

    #[inline]
    pub fn put_i8_at(&mut self, index: usize, value: i8) {
        self.put_bytes_at(index, i8::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i16_at(&mut self, index: usize, value: i16) {
        self.put_bytes_at(index, i16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i32_at(&mut self, index: usize, value: i32) {
        self.put_bytes_at(index, i32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i64_at(&mut self, index: usize, value: i64) {
        self.put_bytes_at(index, i64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u16_at(&mut self, index: usize, value: u16) {
        self.put_bytes_at(index, u16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u32_at(&mut self, index: usize, value: u32) {
        self.put_bytes_at(index, u32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u64_at(&mut self, index: usize, value: u64) {
        self.put_bytes_at(index, u64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f32_at(&mut self, index: usize, value: f32) {
        self.put_bytes_at(index, f32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f64_at(&mut self, index: usize, value: f64) {
        self.put_bytes_at(index, f64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_slice_at(&mut self, index: usize, src: &[u8]) -> usize {
        let len = src.len();
        let dest = self.data.split_at_mut(index).1.split_at_mut(len).0;
        dest.clone_from_slice(src);
        len
    }
}

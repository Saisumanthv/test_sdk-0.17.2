use testnumbat_wasm::api::BigUintApi;
use testnumbat_wasm::types::{Address, TokenIdentifier, Vec};

testnumbat_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct LotteryInfo<BigUint: BigUintApi> {
	pub token_name: TokenIdentifier,
	pub ticket_price: BigUint,
	pub tickets_left: u32,
	pub deadline: u64,
	pub max_entries_per_user: u32,
	pub prize_distribution: Vec<u8>,
	pub whitelist: Vec<Address>,
	pub prize_pool: BigUint,
}

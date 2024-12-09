use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();

	contract_map.register_contract(
		"file:../../kitty-ownership/output/kitty-ownership.wasm",
		Box::new(|context| Box::new(kitty_ownership::contract_obj(context))),
	);
	contract_map.register_contract(
		"file:../output/kitty-auction.wasm",
		Box::new(|context| Box::new(kitty_auction::contract_obj(context))),
	);

	contract_map
}
#[test]
fn bid_first_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_first.scen.json", &contract_map());
}

#[test]
fn bid_second_max_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_second_max.scen.json", &contract_map());
}

#[test]
fn bid_second_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_second_ok.scen.json", &contract_map());
}

#[test]
fn bid_second_too_low_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_second_too_low.scen.json", &contract_map());
}

#[test]
fn bid_siring_auction_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_siring_auction.scen.json", &contract_map());
}

#[test]
fn create_and_auction_gen_zero_kitty_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/create_and_auction_gen_zero_kitty.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_sale_auction_not_owner_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/create_sale_auction_not_owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_sale_auction_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/create_sale_auction_ok.scen.json", &contract_map());
}

#[test]
fn create_siring_auction_not_owner_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/create_siring_auction_not_owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn create_siring_auction_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/create_siring_auction_ok.scen.json", &contract_map());
}

#[test]
fn end_auction_no_bids_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/end_auction_no_bids.scen.json", &contract_map());
}

#[test]
fn end_auction_second_bid_max_early_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/end_auction_second_bid_max_early.scen.json",
		&contract_map(),
	);
}

#[test]
fn end_auction_second_bid_ok_early_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/end_auction_second_bid_ok_early.scen.json",
		&contract_map(),
	);
}

#[test]
fn end_auction_second_bid_ok_late_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/end_auction_second_bid_ok_late.scen.json",
		&contract_map(),
	);
}

#[test]
fn end_siring_auction_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/end_siring_auction.scen.json", &contract_map());
}

#[test]
fn init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/init.scen.json", &contract_map());
}

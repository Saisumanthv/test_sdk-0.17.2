use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

#[allow(dead_code)]
fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc1155-marketplace.wasm",
		Box::new(|context| Box::new(erc1155_marketplace::contract_obj(context))),
	);
	contract_map.register_contract(
		"file:../../erc1155/output/erc1155.wasm",
		Box::new(|context| Box::new(erc1155::contract_obj(context))),
	);

	contract_map
}

/* is_smart_contract not yet implemented, uncomment later

#[test]
fn auction_single_token_rewa_test_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/auction_single_token_rewa.scen.json", &contract_map());
}

#[test]
fn auction_batch_test_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/auction_batch.scen.json", &contract_map());
}

#[test]
fn bid_first_rewa_test_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_first_rewa.scen.json", &contract_map());
}

#[test]
fn bid_second_rewa_test_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_second_rewa.scen.json", &contract_map());
}

#[test]
fn bid_third_rewa_test_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/bid_third_rewa.scen.json", &contract_map());
}

#[test]
fn end_auction_test_rs() {
	testnumbat_wasm_debug::testdenali_rs("/home/testnumbat/test_sdk-0.0.1/contracts/examples/erc1155-marketplace/testdenali/end_auction.scen.json", &contract_map());
}

*/

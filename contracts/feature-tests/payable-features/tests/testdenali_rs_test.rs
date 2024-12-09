use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/payable-features.wasm",
		Box::new(|context| Box::new(payable_features::contract_obj(context))),
	);
	contract_map
}

#[test]
fn payable_any_1_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_any_1.scen.json", &contract_map());
}

#[test]
fn payable_any_2_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_any_2.scen.json", &contract_map());
}

#[test]
fn payable_any_3_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_any_3.scen.json", &contract_map());
}

#[test]
fn payable_any_4_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_any_4.scen.json", &contract_map());
}

#[test]
fn payable_rewa_1_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_rewa_1.scen.json", &contract_map());
}

#[test]
fn payable_rewa_2_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_rewa_2.scen.json", &contract_map());
}

#[test]
fn payable_rewa_3_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_rewa_3.scen.json", &contract_map());
}

#[test]
fn payable_rewa_4_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_rewa_4.scen.json", &contract_map());
}

#[test]
fn payable_token_1_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_token_1.scen.json", &contract_map());
}

#[test]
fn payable_token_2_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_token_2.scen.json", &contract_map());
}

#[test]
fn payable_token_3_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_token_3.scen.json", &contract_map());
}

#[test]
fn payable_token_4_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/payable_token_4.scen.json", &contract_map());
}

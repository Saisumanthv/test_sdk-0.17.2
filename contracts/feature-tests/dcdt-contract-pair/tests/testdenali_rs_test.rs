use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../first-contract/output/first-contract.wasm",
		Box::new(|context| Box::new(first_contract::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../second-contract/output/second-contract.wasm",
		Box::new(|context| Box::new(second_contract::contract_obj(context))),
	);
	contract_map
}

#[test]
fn init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/init.scen.json", &contract_map());
}

#[test]
fn simple_transfer_full_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/simple_transfer_full.scen.json", &contract_map());
}

#[test]
fn simple_transfer_half_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/simple_transfer_half.scen.json", &contract_map());
}

#[test]
fn simple_transfer_full_wrong_token_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/simple_transfer_full_wrong_token.scen.json",
		&contract_map(),
	);
}

// TODO: implement DCDTTransfer + async call
// #[test]
// fn rejected_transfer_rs() {
// 	testnumbat_wasm_debug::testdenali_rs("testdenali/reject_transfer.scen.json", &contract_map());
// }

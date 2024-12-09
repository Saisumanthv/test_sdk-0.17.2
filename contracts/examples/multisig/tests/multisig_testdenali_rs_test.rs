use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../../output/multisig.wasm",
		Box::new(|context| Box::new(multisig::contract_obj(context))),
	);
	contract_map
}

#[test]
fn test_change_board_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/changeBoard.scen.json", &contract_map());
}

#[test]
fn test_change_quorum_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/changeQuorum.scen.json", &contract_map());
}

#[test]
fn test_change_quorum_too_big_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/changeQuorum_tooBig.scen.json", &contract_map());
}

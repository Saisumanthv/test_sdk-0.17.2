use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/str-repeat.wasm",
		Box::new(|context| Box::new(str_repeat::contract_obj(context))),
	);
	contract_map
}

#[test]
fn test_str_repeat_testdenali_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/str_repeat.scen.json", &contract_map());
}

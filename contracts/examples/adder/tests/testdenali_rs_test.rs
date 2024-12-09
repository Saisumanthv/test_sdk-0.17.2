use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/adder.wasm",
		Box::new(|context| Box::new(adder::contract_obj(context))),
	);
	contract_map
}

#[test]
fn adder_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/adder.scen.json", &contract_map());
}

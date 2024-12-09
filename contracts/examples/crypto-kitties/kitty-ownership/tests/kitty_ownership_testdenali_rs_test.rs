use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();

	contract_map.register_contract(
		"file:../../kitty-genetic-alg/output/kitty-genetic-alg.wasm",
		Box::new(|context| Box::new(kitty_genetic_alg::contract_obj(context))),
	);
	contract_map.register_contract(
		"file:../output/kitty-ownership.wasm",
		Box::new(|context| Box::new(kitty_ownership::contract_obj(context))),
	);

	contract_map
}

#[test]
fn approve_siring_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/approve_siring.scen.json", &contract_map());
}

#[test]
fn breed_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/breed_ok.scen.json", &contract_map());
}

#[test]
fn give_birth_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/give_birth.scen.json", &contract_map());
}

#[test]
fn init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/init.scen.json", &contract_map());
}

#[test]
fn query_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/query.scen.json", &contract_map());
}

#[test]
fn setup_accounts_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/setup_accounts.scen.json", &contract_map());
}

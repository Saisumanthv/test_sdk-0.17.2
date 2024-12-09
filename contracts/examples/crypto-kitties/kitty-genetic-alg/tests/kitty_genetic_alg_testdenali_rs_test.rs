use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/kitty-genetic-alg.wasm",
		Box::new(|context| Box::new(kitty_genetic_alg::contract_obj(context))),
	);
	contract_map
}

#[test]
fn generate_kitty_genes_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/generate-kitty-genes.scen.json", &contract_map());
}

#[test]
fn init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/init.scen.json", &contract_map());
}

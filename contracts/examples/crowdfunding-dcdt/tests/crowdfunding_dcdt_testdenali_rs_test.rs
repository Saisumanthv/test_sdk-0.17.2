use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/crowdfunding-dcdt.wasm",
		Box::new(|context| Box::new(crowdfunding_dcdt::contract_obj(context))),
	);
	contract_map
}

#[test]
fn crowdfunding_claim_failed_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/crowdfunding-claim-failed.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_claim_successful_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/crowdfunding-claim-successful.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_claim_too_early_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/crowdfunding-claim-too-early.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_fund_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/crowdfunding-fund-too-late.scen.json",
		&contract_map(),
	);
}

#[test]
fn crowdfunding_init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/crowdfunding-init.scen.json", &contract_map());
}

#[test]
fn rewa_crowdfunding_claim_failed_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/rewa-crowdfunding-claim-failed.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_claim_successful_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/rewa-crowdfunding-claim-successful.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_claim_too_early_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/rewa-crowdfunding-claim-too-early.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_fund_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/rewa-crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn rewa_crowdfunding_fund_too_late_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/rewa-crowdfunding-fund-too-late.scen.json",
		&contract_map(),
	);
}

#[test]
fn rewa_crowdfunding_init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/rewa-crowdfunding-init.scen.json", &contract_map());
}

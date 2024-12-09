#[test]
fn deploy_erc20_and_crowdfunding_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/deploy_erc20_and_crowdfunding.scen.json");
}

#[test]
fn fund_with_insufficient_allowance_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/fund_with_insufficient_allowance.scen.json");
}

#[test]
fn fund_with_sufficient_allowance_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/fund_with_sufficient_allowance.scen.json");
}

#[test]
fn fund_without_allowance_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/fund_without_allowance.scen.json");
}

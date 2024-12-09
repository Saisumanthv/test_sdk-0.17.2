#[test]
fn batch_transfer_both_types_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/batch_transfer_both_types.scen.json");
}

#[test]
fn batch_transfer_both_types_to_sc_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/batch_transfer_both_types_to_sc.scen.json");
}

#[test]
fn batch_transfer_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/batch_transfer_fungible.scen.json");
}

#[test]
fn batch_transfer_fungible_to_sc_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/batch_transfer_fungible_to_sc.scen.json");
}

#[test]
fn batch_transfer_non_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/batch_transfer_non_fungible.scen.json");
}

#[test]
fn batch_transfer_non_fungible_to_sc_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/batch_transfer_non_fungible_to_sc.scen.json");
}

#[test]
fn burn_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/burn_fungible.scen.json");
}

#[test]
fn burn_non_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/burn_non_fungible.scen.json");
}

#[test]
fn create_one_fungible_one_non_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_one_fungible_one_non_fungible.scen.json");
}

#[test]
fn create_token_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_token_fungible.scen.json");
}

#[test]
fn create_token_non_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_token_non_fungible.scen.json");
}

#[test]
fn create_two_tokens_both_fungible_different_creator_go() {
	testnumbat_wasm_debug::testdenali_go(
		"testdenali/create_two_tokens_both_fungible_different_creator.scen.json",
	);
}

#[test]
fn create_two_tokens_both_fungible_same_creator_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_two_tokens_both_fungible_same_creator.scen.json");
}

#[test]
fn create_two_tokens_both_non_fungible_same_creator_go() {
	testnumbat_wasm_debug::testdenali_go(
		"testdenali/create_two_tokens_both_non_fungible_same_creator.scen.json",
	);
}

#[test]
fn deploy_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/deploy.scen.json");
}

#[test]
fn mint_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/mint_fungible.scen.json");
}

#[test]
fn mint_non_fungible_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/mint_non_fungible.scen.json");
}

#[test]
fn mint_not_creator_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/mint_not_creator.scen.json");
}

#[test]
fn transfer_fungible_not_enough_balance_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/transfer_fungible_not_enough_balance.scen.json");
}

#[test]
fn transfer_fungible_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/transfer_fungible_ok.scen.json");
}

#[test]
fn transfer_fungible_ok_to_sc_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/transfer_fungible_ok_to_sc.scen.json");
}

#[test]
fn transfer_non_fungible_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/transfer_non_fungible_ok.scen.json");
}

#[test]
fn transfer_non_fungible_ok_to_sc_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/transfer_non_fungible_ok_to_sc.scen.json");
}

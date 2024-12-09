use testnumbat_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<TxContext> {
	ContractMap::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"tests/testdenali/external_steps/external_steps.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_rs() {
	testnumbat_wasm_debug::testdenali_rs("tests/testdenali/transfer.scen.json", &contract_map());
}

#[test]
fn validator_reward_rs() {
	testnumbat_wasm_debug::testdenali_rs("tests/testdenali/validatorReward.scen.json", &contract_map());
}

use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/crypto-bubbles.wasm",
		Box::new(|context| Box::new(crypto_bubbles::contract_obj(context))),
	);
	contract_map
}

#[test]
fn balanceof_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/balanceOf.scen.json", &contract_map());
}

#[test]
fn create_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/create.scen.json", &contract_map());
}

#[test]
fn exceptions_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/exceptions.scen.json", &contract_map());
}

#[test]
fn joingame_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/joinGame.scen.json", &contract_map());
}

#[test]
fn rewardandsendtowallet_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/rewardAndSendToWallet.scen.json", &contract_map());
}

#[test]
fn rewardwinner_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/rewardWinner.scen.json", &contract_map());
}

#[test]
fn rewardwinner_last_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/rewardWinner_Last.scen.json", &contract_map());
}

#[test]
fn topup_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/topUp_ok.scen.json", &contract_map());
}

#[test]
fn topup_withdraw_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/topUp_withdraw.scen.json", &contract_map());
}

#[test]
fn withdraw_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/withdraw_Ok.scen.json", &contract_map());
}

#[test]
fn withdraw_toomuch_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/withdraw_TooMuch.scen.json", &contract_map());
}

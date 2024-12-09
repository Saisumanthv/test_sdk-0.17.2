#[test]
fn balanceof_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/balanceOf.scen.json");
}

#[test]
fn create_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create.scen.json");
}

#[test]
fn exceptions_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/exceptions.scen.json");
}

#[test]
fn joingame_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/joinGame.scen.json");
}

#[test]
fn rewardandsendtowallet_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/rewardAndSendToWallet.scen.json");
}

#[test]
fn rewardwinner_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/rewardWinner.scen.json");
}

#[test]
fn rewardwinner_last_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/rewardWinner_Last.scen.json");
}

#[test]
fn topup_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/topUp_ok.scen.json");
}

#[test]
fn topup_withdraw_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/topUp_withdraw.scen.json");
}

#[test]
fn withdraw_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/withdraw_Ok.scen.json");
}

#[test]
fn withdraw_toomuch_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/withdraw_TooMuch.scen.json");
}

#[test]
fn init_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/init.scen.json");
}

// #[test]
// fn reject_transfer_go() {
// 	testnumbat_wasm_debug::testdenali_go("testdenali/reject_transfer.scen.json");
// }

#[test]
fn simple_transfer_full_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/simple_transfer_full.scen.json");
}

#[test]
fn simple_transfer_full_wrong_token_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/simple_transfer_full_wrong_token.scen.json");
}

#[test]
fn simple_transfer_half_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/simple_transfer_half.scen.json");
}

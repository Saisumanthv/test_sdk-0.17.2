#[test]
fn auction_end_deadline_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/auction_end_deadline.scen.json");
}

#[test]
fn auction_end_max_bid_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/auction_end_max_bid.scen.json");
}

#[test]
fn auction_token_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/auction_token.scen.json");
}

#[test]
fn bid_first_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_first.scen.json");
}

#[test]
fn bid_max_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_max.scen.json");
}

#[test]
fn bid_second_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_second.scen.json");
}

#[test]
fn init_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/init.scen.json");
}

#[test]
fn invalid_bids_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/invalid_bids.scen.json");
}

#[test]
fn specific_token_auctioned_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/specific_token_auctioned.scen.json");
}

#[test]
fn view_functions_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/view_functions.scen.json");
}

#[test]
fn withdraw_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/withdraw.scen.json");
}

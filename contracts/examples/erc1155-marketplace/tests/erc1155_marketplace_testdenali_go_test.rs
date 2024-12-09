#[test]
fn auction_batch_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/auction_batch.scen.json");
}

#[test]
fn auction_single_token_rewa_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/auction_single_token_rewa.scen.json");
}

#[test]
fn bid_first_rewa_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_first_rewa.scen.json");
}

#[test]
fn bid_second_rewa_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_second_rewa.scen.json");
}

#[test]
fn bid_third_rewa_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_third_rewa.scen.json");
}

#[test]
fn end_auction_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/end_auction.scen.json");
}

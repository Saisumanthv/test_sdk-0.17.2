#[test]
fn bid_first_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_first.scen.json");
}

#[test]
fn bid_second_max_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_second_max.scen.json");
}

#[test]
fn bid_second_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_second_ok.scen.json");
}

#[test]
fn bid_second_too_low_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_second_too_low.scen.json");
}

#[test]
fn bid_siring_auction_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/bid_siring_auction.scen.json");
}

#[test]
fn create_and_auction_gen_zero_kitty_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_and_auction_gen_zero_kitty.scen.json");
}

#[test]
fn create_sale_auction_not_owner_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_sale_auction_not_owner.scen.json");
}

#[test]
fn create_sale_auction_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_sale_auction_ok.scen.json");
}

#[test]
fn create_siring_auction_not_owner_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_siring_auction_not_owner.scen.json");
}

#[test]
fn create_siring_auction_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/create_siring_auction_ok.scen.json");
}

#[test]
fn end_auction_no_bids_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/end_auction_no_bids.scen.json");
}

#[test]
fn end_auction_second_bid_max_early_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/end_auction_second_bid_max_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_early_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/end_auction_second_bid_ok_early.scen.json");
}

#[test]
fn end_auction_second_bid_ok_late_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/end_auction_second_bid_ok_late.scen.json");
}

#[test]
fn end_siring_auction_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/end_siring_auction.scen.json");
}

#[test]
fn init_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/init.scen.json");
}

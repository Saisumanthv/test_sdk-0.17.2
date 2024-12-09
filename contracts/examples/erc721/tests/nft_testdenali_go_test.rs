#[test]
fn nft_approve_non_existent_token_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-approve-non-existent-token.scen.json");
}

#[test]
fn nft_approve_non_owned_token_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-approve-non-owned-token.scen.json");
}

#[test]
fn nft_approve_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-approve-ok.scen.json");
}

#[test]
fn nft_init_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-init.scen.json");
}

#[test]
fn nft_mint_more_tokens_caller_not_owner_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-mint-more-tokens-caller-not-owner.scen.json");
}

#[test]
fn nft_mint_more_tokens_receiver_acc1_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-mint-more-tokens-receiver-acc1.scen.json");
}

#[test]
fn nft_mint_more_tokens_receiver_owner_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-mint-more-tokens-receiver-owner.scen.json");
}

#[test]
fn nft_revoke_non_approved_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-revoke-non-approved.scen.json");
}

#[test]
fn nft_revoke_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-revoke-ok.scen.json");
}

#[test]
fn nft_transfer_approved_token_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-transfer-approved-token.scen.json");
}

#[test]
fn nft_transfer_non_existent_token_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-transfer-non-existent-token.scen.json");
}

#[test]
fn nft_transfer_not_owned_not_approved_token_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-transfer-not-owned-not-approved-token.scen.json");
}

#[test]
fn nft_transfer_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-transfer-ok.scen.json");
}

#[test]
fn nft_transfer_token_after_revoked_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-transfer-token-after-revoked.scen.json");
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_caller_go() {
	testnumbat_wasm_debug::testdenali_go(
		"testdenali/nft-transfer-token-not-owner-no-approval-to-caller.scen.json",
	);
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_other_go() {
	testnumbat_wasm_debug::testdenali_go(
		"testdenali/nft-transfer-token-not-owner-no-approval-to-other.scen.json",
	);
}

#[test]
fn nft_transfer_token_ok_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/nft-transfer-token-ok.scen.json");
}

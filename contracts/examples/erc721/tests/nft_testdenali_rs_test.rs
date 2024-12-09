use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc721.wasm",
		Box::new(|context| Box::new(erc721::contract_obj(context))),
	);
	contract_map
}

#[test]
fn nft_approve_non_existent_token_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-approve-non-existent-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_approve_non_owned_token_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-approve-non-owned-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_approve_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/nft-approve-ok.scen.json", &contract_map());
}

#[test]
fn nft_init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/nft-init.scen.json", &contract_map());
}

#[test]
fn nft_mint_more_tokens_caller_not_owner_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-mint-more-tokens-caller-not-owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_mint_more_tokens_receiver_acc1_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-mint-more-tokens-receiver-acc1.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_mint_more_tokens_receiver_owner_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-mint-more-tokens-receiver-owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_revoke_non_approved_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/nft-revoke-non-approved.scen.json", &contract_map());
}

#[test]
fn nft_revoke_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/nft-revoke-ok.scen.json", &contract_map());
}

#[test]
fn nft_transfer_approved_token_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-transfer-approved-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_transfer_non_existent_token_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-transfer-non-existent-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_transfer_not_owned_not_approved_token_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-transfer-not-owned-not-approved-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_transfer_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/nft-transfer-ok.scen.json", &contract_map());
}

#[test]
fn nft_transfer_token_after_revoked_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-transfer-token-after-revoked.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_caller_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-transfer-token-not-owner-no-approval-to-caller.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_transfer_token_not_owner_no_approval_to_other_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/nft-transfer-token-not-owner-no-approval-to-other.scen.json",
		&contract_map(),
	);
}

#[test]
fn nft_transfer_token_ok_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/nft-transfer-token-ok.scen.json", &contract_map());
}

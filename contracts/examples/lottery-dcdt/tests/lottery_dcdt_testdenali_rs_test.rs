use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

#[allow(dead_code)]
fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/lottery-dcdt.wasm",
		Box::new(|context| Box::new(lottery_dcdt::contract_obj(context))),
	);
	contract_map
}

/* Uncomment when testdenali-rs is brought up to date with testdenali-go features
#[test]
fn buy_all_tickets_different_accounts_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-all-tickets-different-accounts.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_more_tickets_than_allowed_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-more-tickets-than-allowed.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_after_deadline_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-ticket-after-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_after_determined_winner_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-ticket-after-determined-winner.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_after_sold_out_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-ticket-after-sold-out.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_all_options_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/buy-ticket-all-options.scen.json", &contract_map());
}

#[test]
fn buy_ticket_another_account_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-ticket-another-account.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_not_on_whitelist_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-ticket-not-on-whitelist.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_same_account_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/buy-ticket-same-account.scen.json", &contract_map());
}

#[test]
fn buy_ticket_second_lottery_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/buy-ticket-second-lottery.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_wrong_fee_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/buy-ticket-wrong-fee.scen.json", &contract_map());
}

#[test]
fn buy_ticket_simple_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/buy-ticket.scen.json", &contract_map());
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc1_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/determine-winner-different-ticket-holders-winner-acc1.scen.json",
		&contract_map(),
	);
}

#[test]
fn determine_winner_early_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/determine-winner-early.scen.json", &contract_map());
}

#[test]
fn determine_winner_same_ticket_holder_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/determine-winner-same-ticket-holder.scen.json",
		&contract_map(),
	);
}

/* NOT SUPPORTED YET
#[test]
fn determine_winner_split_prize_pool_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/determine-winner-split-prize-pool.scen.json",
		&contract_map(),
	);
}
*/

#[test]
fn lottery_init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/lottery-init.scen.json", &contract_map());
}

#[test]
fn start_after_announced_winner_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/start-after-announced-winner.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_all_options_bigger_whitelist_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/start-all-options-bigger-whitelist.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_alternative_function_name_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/start-alternative-function-name.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_fixed_deadline_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/start-fixed-deadline.scen.json", &contract_map());
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_limited_tickets_and_fixed_deadline_valid_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/start-limited-tickets-and-fixed-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_limited_tickets_simple_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/start-limited-tickets.scen.json", &contract_map());
}

#[test]
fn start_second_lottery_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/start-second-lottery.scen.json", &contract_map());
}

#[test]
fn start_with_all_options_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/start-with-all-options.scen.json", &contract_map());
}

#[test]
fn start_with_no_options_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/start-with-no-options.scen.json", &contract_map());
}
*/

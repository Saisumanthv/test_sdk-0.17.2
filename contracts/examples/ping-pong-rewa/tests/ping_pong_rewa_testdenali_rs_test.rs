use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/ping-pong-rewa.wasm",
		Box::new(|context| Box::new(ping_pong_rewa::contract_obj(context))),
	);
	contract_map
}

#[test]
fn ping_pong_call_get_user_addresses_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-get-user-addresses.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/ping-pong-call-ping.scen.json", &contract_map());
}

#[test]
fn ping_pong_call_ping_after_deadline_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-ping-after-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_before_activation_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-ping-before-activation.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_second_user_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-ping-second-user.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_twice_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-ping-twice.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_wrong_ammount_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-ping-wrong-ammount.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/ping-pong-call-pong.scen.json", &contract_map());
}

#[test]
fn ping_pong_call_pong_all_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/ping-pong-call-pong-all.scen.json", &contract_map());
}

#[test]
fn ping_pong_call_pong_all_after_pong_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-pong-all-after-pong.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong_before_deadline_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-pong-before-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong_twice_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-pong-twice.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong_without_ping_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/ping-pong-call-pong-without-ping.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_init_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/ping-pong-init.scen.json", &contract_map());
}

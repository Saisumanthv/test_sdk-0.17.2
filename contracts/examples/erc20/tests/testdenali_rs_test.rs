use testnumbat_wasm::*;
use testnumbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc20.wasm",
		Box::new(|context| Box::new(erc20::contract_obj(context))),
	);
	contract_map
}

#[test]
fn allowance_callercaller_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/allowance_CallerCaller.scen.json", &contract_map());
}

#[test]
fn allowance_callerother_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/allowance_CallerOther.scen.json", &contract_map());
}

#[test]
fn allowance_othercaller_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/allowance_OtherCaller.scen.json", &contract_map());
}

#[test]
fn allowance_othereqother_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/allowance_OtherEqOther.scen.json", &contract_map());
}

#[test]
fn allowance_otherneqother_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/allowance_OtherNEqOther.scen.json", &contract_map());
}

#[test]
fn approve_caller_positive_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/approve_Caller-Positive.scen.json", &contract_map());
}

#[test]
fn approve_caller_zero_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/approve_Caller-Zero.scen.json", &contract_map());
}

#[test]
fn approve_other_positive_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/approve_Other-Positive.scen.json", &contract_map());
}

#[test]
fn approve_other_zero_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/approve_Other-Zero.scen.json", &contract_map());
}

#[test]
fn approve_switchcaller_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/approve_SwitchCaller.scen.json", &contract_map());
}

#[test]
fn balanceof_caller_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/balanceOf_Caller.scen.json", &contract_map());
}

#[test]
fn balanceof_noncaller_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/balanceOf_NonCaller.scen.json", &contract_map());
}

#[test]
fn not_payable_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/not_payable.scen.json", &contract_map());
}

#[test]
fn totalsupply_positive_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/totalSupply_Positive.scen.json", &contract_map());
}

#[test]
fn totalsupply_zero_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/totalSupply_Zero.scen.json", &contract_map());
}

#[test]
fn transferfrom_alldistinct_balanceeqallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-BalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_balanceneqallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entireallowancemorethanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entirebalanceeqallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entirebalancemorethanallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_morethanallowancelessthanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_morethanbalancelessthanallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_nooverflow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_stillnooverflow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllDistinct-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_allequal_allowancerelevant_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllEqual-AllowanceRelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_allequal_entirebalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_AllEqual-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_allowancerelevant_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_entirebalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_CallerEqFrom-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_morethanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_CallerEqFrom-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_balanceneqallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_morethanallowancelessthanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_morethanbalancelessthanallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_exploratory_multipletransferssucceed_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_exploratory_multipletransfersthrow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_Exploratory-MultipleTransfersThrow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_balanceeqallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-BalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_balanceneqallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entireallowancemorethanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entirebalanceeqallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entirebalancemorethanallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_morethanallowancelessthanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_morethanbalancelessthanallowance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_nooverflow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transferFrom_FromEqTo-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_allowanceirrelevant_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Caller-AllowanceIrrelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_entirebalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Caller-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_morethanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Caller-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_nooverflow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Caller-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_positive_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/transfer_Caller-Positive.scen.json", &contract_map());
}

#[test]
fn transfer_caller_stillnooverflow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Caller-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_zero_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/transfer_Caller-Zero.scen.json", &contract_map());
}

#[test]
fn transfer_other_allowanceirrelevant_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Other-AllowanceIrrelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_entirebalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Other-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_morethanbalance_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Other-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_nooverflow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Other-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_positive_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/transfer_Other-Positive.scen.json", &contract_map());
}

#[test]
fn transfer_other_stillnooverflow_rs() {
	testnumbat_wasm_debug::testdenali_rs(
		"testdenali/transfer_Other-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_zero_rs() {
	testnumbat_wasm_debug::testdenali_rs("testdenali/transfer_Other-Zero.scen.json", &contract_map());
}

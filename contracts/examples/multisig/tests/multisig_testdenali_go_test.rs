#[test]
fn changeboard_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/changeBoard.scen.json");
}

#[test]
fn changequorum_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/changeQuorum.scen.json");
}

#[test]
fn changequorum_toobig_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/changeQuorum_tooBig.scen.json");
}

#[test]
fn deployadder_err_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/deployAdder_err.scen.json");
}

#[test]
fn deployadder_then_call_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/deployAdder_then_call.scen.json");
}

#[test]
fn deployfactorial_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/deployFactorial.scen.json");
}

#[test]
fn deploy_duplicate_bm_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/deploy_duplicate_bm.scen.json");
}

#[test]
fn remove_everyone_go() {
	testnumbat_wasm_debug::testdenali_go("testdenali/remove_everyone.scen.json");
}

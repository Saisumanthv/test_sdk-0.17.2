extern crate testnumbat_codec_derive;
use testnumbat_codec_derive::*;

use testnumbat_codec::test_util::{check_dep_encode_decode, check_top_encode_decode};
// use testnumbat_codec::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug)]
struct TupleStruct(u8, u16, u32);

// to test, run the following command in testnumbat-codec folder:
// cargo expand --test tuple_struct_derive_test > expanded.rs

#[test]
fn tuple_struct_derive_test() {
	let s = TupleStruct(8, 16, 32);

	#[rustfmt::skip]
	let bytes = &[
		/* 0: u8 */ 8,
		/* 1: u32 */ 0, 16, 
		/* 2: u64 */ 0, 0, 0, 32,
	];

	check_top_encode_decode(s.clone(), bytes);
	check_dep_encode_decode(s.clone(), bytes);
}

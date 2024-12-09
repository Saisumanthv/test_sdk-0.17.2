testnumbat_wasm::derive_imports!();

/// Copied from testnumbat-wasm serialization tests.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum SerExample2 {
	Unit,
	Newtype(u32),
	Tuple(u32, u32),
	Struct { a: u32 },
}

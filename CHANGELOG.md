# Change Log

There are several crates in this repo, this changelog will keep track of all of them.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [Unreleased]
### New math hooks exposed from Andes:
- `pow`, `log2`, `sqrt`
- cryptography: elliptic curves

## [testnumbat-wasm 0.0.1] - 2021-06-04
- callbacks can now declared in modules only (manual forwarding from the main contract no longer required)

## [testnumbat-wasm 0.17.1] - 2021-06-04
- `legacy-nft-transfer` feature for interacting with older versions of Andes

## [testnumbat-wasm 0.17.0] - 2021-05-28
- Integration tests can now call Andes-Testdenali (testdenali-go)
- Send API refactoring and cleanup
	- DCDT builtin function calls no longer require explicit gas
	- sync calls and transfer-execute no longer require explicit gas
- `#[payment_nonce]` endpoint argument annotation
- `#[payable]` annotation no longer allowed without argument

## [testnumbat-wasm 0.16.2, testdenali 0.0.1] - 2021-05-20
- New implementation for the `Try` trait for `SCResult`, in accordance to feature `try_trait_v2`
- Published DNS module, which helps contracts register usernames for themselves
- `DCDTLocalRole` more expressive type ABI

## [testnumbat-wasm 0.16.1, testdenali 0.7.1] - 2021-05-18
- Improvements in testdenali-rs: username, contract owner, nested async calls

## [testnumbat-wasm 0.16.0, testdenali 0.7.0, testnumbat-codec 0.0.1] - 2021-05-14
### Major redesign of important framework components:
- The arguments to contract/module/proxy annotations are gone. All items are generated in the same Rust module. Both submodule inclusion and contract calls are now Rust-module-aware.
- Submodule imports are now expressed as supertraits instead of the module getter annotated methods. Note: explicitly specifying the Rust module is required, in order for the framework to fetch generated types and functions from that module.
- Each contract now generates its own callable proxy to ease calling it. Caller contracts do no longer need to define a call interface, they can import it from the crate of the contract they want to call. Callable proxies contain the methods from the main contract, as well as from all the modules. Note: calling a contract requires the caller to specify the Rust module where it resides.
- We no longer have a separate syntax/parser/code generation for call proxies. They are just contracts with no implementations and annotated with `#[testnumbat_wasm_derive::proxy]` instead of `#[testnumbat_wasm_derive::contract]`.
- BigUint and BigInt are now associated types instead of generics in all API traits. Contracts need to specify them as `Self::BigUint` instead of just `BigUint`. Although more verbose, this might be more intuitive for the developer.
- `ContractCall`s, `AsyncCall`s and all other call & transfer result types now contain a reference to the Send API. This also means the `execute_on_dest_context` method no longer requires an api argument.
- `execute_on_dest_context` can now deserialize the call results automatically and provide them to the calling contract. There is a mechanism in place to deconstruct non-serialized types, e.g. `SCResult<T>` becomes `T` and `AsyncCall<Self::BigUint>` becomes `()`. 
- Callbacks and callback proxies needed to be adapted to the new system, but work similar to how they did in the past.
- Contracts can define proxy getter methods using the `#[proxy]` annotation.
- Callbacks can now have names, just like endpoints. This name gets saved in the callback closure in storage, but has no other impact on the contract. The reason I needed it was to help me with defining callback forwarders and avoiding some name collisions there. Callback forwarders are still needed for a little longer, until module callbacks are properly implemented.

### Testdenali
- testdenali-rs syntax synchronized with testdenali-go (`sc:` syntax, new DCDT call value syntax, _no NFTs yet_).

## [testnumbat-wasm 0.15.1] - 2021-04-30
- Mitigating nested sync calls with Send API `execute_on_dest_context_raw_custom_result_range`

## [testnumbat-wasm 0.15.0, testnumbat-codec 0.5.2] - 2021-04-19
- ABI
	- Constructor representation
	- Simplified ABI syntax for tuples and fixed-size arrays
- Final cleanup for the contract APIs: split off blockchain and crypto APIs
- Small fixes in the send API
- `TokenIdentifier` validation
- Minor refactoring in the testnumbat-codec 

## [testnumbat-wasm 0.14.2] - 2021-03-29
- Fixed contract call/callback logs in testdenali-rs

## [testnumbat-wasm 0.14.1] - 2021-03-25
- Unified variadic arguments with respective variadic results

## [testnumbat-wasm 0.14.0, testdenali 0.6.0, testnumbat-codec 0.5.1] - 2021-03-22
- DCDT functionality:
	- DCDT system smart contract proxy, though which it is possible to mint, burn, issue, freeze, pause, etc.
	- Endpoints to handle NFTs. Also added NFT management in the  DCDT system smart contract proxy
	- Get balance, get token data, local mint/burn
- Contract calls:
	- Low-level and high-level support for synchronous calls via `execute_on_dest_context`.
	- Callback bug fix
- Improvements in storage mappers:
	- VecMapper length is now lazy
	- UserMapper more functionality
- Testdenali
	- `scQuery` step
	- fixed defaults: unspecified fields now check the default value instead of being ignored
	- check logs
	- `nested:` and `biguint:` syntax
- `testnumbat-codec-derive` dix - `TopDecodeOrDefault` works with generics
- Upgraded to Rust2021.

## [testnumbat-wasm 0.13.0] - 2021-03-04
### Main feature
- Events revamped:
	- any event name of any length is accepted. The event name is now expressed as ASCII instead of hex
	- topics can have any length
	- topics and data are serialized using the testnumbat-codec instead of the old macro-based solution
	- old events are still allowed for now via the `#[legacy_event("0x...")]` syntax; might be removed in the future
### Refactoring 
- Major refactoring of testnumbat-wasm-derive. This doesn't change much of the functionality, though.
### Minor features
- SingleValueMapper redesigned for easier use. It no longer keeps the storage value cached.

## [testnumbat-wasm 0.12.0] - 2021-02-25
- Reorganized DCDT and REWA direct send api.
- New async call syntax
	- redesigned contract proxies
	- contract calls are communicated via objects returned from endpoint methods
	- callbacks now specified programmatically
	- got rid of the `#[callback_arg]` annotation

## [testnumbat-wasm 0.11.0, testnumbat-codec 0.5.0, testdenali 0.5.0] - 2021-02-05
### Refactor
- Major refactoring of the contract API: split into smaller traits
### Added
- Storage mappers:
	- LinkedListMapper
	- SetMapper
	- MapMapper
- SendApi
	- created SendApi, which groups all functionality related to sending tokens and interactions with other contracts
    - integrated the new TransferDCDT hook from Andes
    - added an unsafe buffer for handling values before transfer
    - testdenali-rs fixes
    - contracts now use the new API + more testdenali tests
- Call Value API refactor and `#[payable]` updates:
	- Main features:
    	- `#[payable]` annotation more versatile: `#[payable("REWA")]` `#[payable("TOKEN-ID")]` `#[payable("*")]`
    	- `#[payable]` still accepted but throws a warning, will become unsupported in the future.
    	- `#[payment]` argument attribute now also provides DCDT payment where applicable
    	- a new TokenIdentifier type that encodes the REWA special token and any DCDT token
    	- a new `#[token_identifier]` argument attribute provides the token id. Similar to `#[payment]` it is a fake argument, not exported.
    	- ABI updated ("payableInTokens" is no longer restricted to "REWA")
    	- all new features covered by testdenali tests
    	- async proxies still only accept `#[payable("REWA")]`, but that is for future updates
	- Less visible changes:
    	- all call value hooks now grouped in a new CallValueApi
    	- for low-level access, developers now need to write self.call_value().rewa_value(), etc.
    	- some optimizations in the handling of call value hooks
	- Refactoring:
    	- parse_attr mod was split into a proper folder with many files, since it had grown too large
    	- an extensive refactoring of testnumbat-wasm-derive not yet performed, will come soon
### Minor features
- ABI enum discriminants generation
### Fixed
- Crypto API fixes:
	- `keccak256:` prefix also supported in testdenali
    - reorganized crypto testdenali tests in basic-features
    - testdenali-rs was accidentally providing keccak256 instead of sha256


## [testnumbat-wasm 0.10.5] - 2021-01-27
- Temporary fix: callbacks allow error message argument to be missing

## [testnumbat-wasm 0.10.4, testdenali 0.4.2] - 2021-01-13
- Codec derive with defaults
- Storage mapper infrastructure

## [testnumbat-wasm 0.10.3] - 2020-12-29
- ABI generation of endpoint output names

## [testnumbat-wasm 0.10.2, testnumbat-codec 0.4.2] - 2020-12-28
- Codec type hygene

## [testnumbat-wasm 0.10.1, testnumbat-codec 0.4.1, testdenali 0.4.1] - 2020-12-23
- Minor fixes, support for strings

## [testnumbat-wasm 0.10.0, testnumbat-codec 0.4.0] - 2020-12-21
- Codec derive
- ABI generation framework
- New example contracts

## [testnumbat-wasm 0.9.8, testnumbat-codec 0.3.2, testdenali 0.3.1] - 2020-11-23
- SC deploy API

## [testnumbat-wasm 0.9.7, testnumbat-codec 0.3.1, testdenali 0.3.0] - 2020-11-11
- Monomorphization via codec trait instead of TypeInfo for arguments and storage
- Reorganized all contracts in the `contracts` folder

## [testnumbat-wasm 0.9.6] - 2020-11-09
- H256 & BoxedBytes fixes

## [testnumbat-wasm 0.9.5] - 2020-11-09
- H256 is_zero, minor fixes

## [testnumbat-wasm 0.9.4] - 2020-11-09
- BoxedBytes
	- optimized allocation, used in hooks
	- used for error messages

## [testnumbat-wasm 0.9.3] - 2020-11-08
- Optimized Address/H256 hooks

## [testnumbat-wasm 0.9.2] - 2020-11-06
- Allow slices as arguments 
- `storage_is_empty` annotation

## [testnumbat-wasm 0.9.1] - 2020-11-05
- BigUint serialization bugfix

## [testnumbat-wasm 0.9.0, testnumbat-codec 0.3.0, testdenali 0.2.0] - 2020-11-04
- Serialization completely refactored to use "fast exit" methods
- Storage/argument/result traits completely redesigned, simplified and optimized
- Completely ditched the approach from testnumbat-wasm 0.8.0.

## [testnumbat-wasm 0.8.0, testnumbat-codec 0.2.0] - 2020-11-02
- Was the first version to split Encode/Decode into TopEncode/NestedEncode/TopDecode/NestedDecode
- Attempted to optimize the serializer to use "fast exit" closures. It worked, but the resulting bytecode size was not satisfactory. Even though it was completely replaced and never got to be used, it historically remains the solution of this release.
- Some of the storage/argument/result trait refactorings, survived.

## [testnumbat-wasm 0.7.2] - 2020-10-16
- small int EI
- minor refactors, serialization fixes

## [testnumbat-wasm 0.7.1] - 2020-10-07
- Avoid function selector infinite loop
- Crowdfunding contract initial commit

## [testnumbat-wasm 0.7.0, testdenali 0.1.0] - 2020-10-06
- Code coverage now possible
- Testdenali in Rust
- Modules properly integrated in the build process

## [testnumbat-wasm 0.6.2] - 2020-09-16
- NonZeroUsize iterator and utils

## [testnumbat-wasm 0.6.1, testnumbat-codec 0.1.3] - 2020-09-15
- Integrated NonZeroUsize into the framework
- Specialized small int top encoding/decoding
- `only_owner!` macro

## [testnumbat-wasm 0.6.0, testnumbat-codec 0.1.2] - 2020-08-25
- Redesigned the entire build process with wasm crates
- Standard modules
- Moved all example contracts from sc-examples-rs to the framework

## [testnumbat-wasm 0.5.5] - 2020-07-27
- H256 now boxed
- SCResult is_ok, is_err

## [testnumbat-wasm 0.5.4, testnumbat-codec 0.1.1] - 2020-07-18
- MultiResultVec - new, from_iter
- EncodeError type

## [testnumbat-wasm 0.0.1, testnumbat-codec 0.1.0] - 2020-07-10
- Extracted testnumbat-codec to separate crate
- Fixed non_snake_case endpoint handling

## [testnumbat-wasm 0.5.2] - 2020-07-09
- Queue type

## [testnumbat-wasm 0.5.1] - 2020-07-02
- `#[view]` attribute, same as `#[endpoint]`
- `#[init]` attribute
- `storage get mut` annotation + BorrowedMutStorage
- Encode for references
- Array serialization/deserialization
- Option serialization fix
- Arg name in error message
- Async call arguments based on traits

## [testnumbat-wasm 0.5.0] - 2020-06-29
- EndpointResult trait, arg serialization trait, arg loader
- Variadic args/results: OptionalArg, OptionalResult, MultiResultX

## [testnumbat-wasm 0.4.6] - 2020-06-21
- MultiResultVec implementation
- Callback varargs

## [testnumbat-wasm 0.4.5] - 2020-06-09
- `storage_set` allows slices
- H256 to_vec
- async call and callback argument fixes
- eliminate bloat when no callback
- the new testnumbat lightweight serializer (would later become testnumbat-codec)
- imports macro
- OtherContractHandle implementation

## [testnumbat-wasm 0.4.4] - 2020-05-19
- Serialization fixes for small ints
- `get_cumulated_validator_rewards` hook

## [testnumbat-wasm 0.4.3] - 2020-05-11
- Allow any (macro-based) serializable argument in async call
- `#[var_args]`
- Call data serialization refactoring

## [testnumbat-wasm 0.4.2] - 2020-05-07
- Tutorial setup (later abandoned)

## [testnumbat-wasm 0.4.1] - 2020-05-06
- Direct storage conversion for simple types
- Block info hooks

## [testnumbat-wasm 0.4.0] - 2020-05-06
- Serde-based serializer (later abandoned)
- Major storage improvements:
	- Generate storage getters & setters
	- Variable length storage keys

## [testnumbat-wasm 0.3.2] - 2020-04-13
- Fixes in the macro-based argument handling

## [testnumbat-wasm 0.3.0] - 2020-04-03
- Raw callback support
- `storage_load_len` hook
- Multi args
- Multi args in async calls

## [testnumbat-wasm 0.2.0] - 2020-03-18
- BigUint trait created, added operators (including bitwise)
- BigUint used for balances

## [testnumbat-wasm 0.1.1] - 2020-02-27
- Async call contract proxy infrastructure

## [testnumbat-wasm 0.1.0] - 2020-02-05 
- Initial relase of the framework
- Main features at this time:
	- contract main macro
	- handling of arguments and results automagically using macros
	- BigInt generic type, hooked directly to the Andes big int heap
	- `#[private]` attribute

## [Initial commit] - 2020-01-04
- Early framework moved here from sc-examples
- 4 crates:
	- testnumbat-wasm
	- testnumbat-wasm-derive for macros
	- testnumbat-wasm-node for wasm
	- testnumbat-wasm-debug for debugging and early tests
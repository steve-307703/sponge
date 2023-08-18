#![warn(
	deprecated_in_future,
	future_incompatible,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_copy_implementations,
	missing_debug_implementations,
	noop_method_call,
	rust_2018_compatibility,
	rust_2018_idioms,
	rust_2021_compatibility,
	single_use_lifetimes,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_op_in_unsafe_fn,
	unused_crate_dependencies,
	unused_lifetimes,
	unused_qualifications,
	unused_results
)]

pub mod encode;
pub mod k12;
pub mod keccakp;
pub mod sha3;

#![no_std]
#![warn(
	deprecated_in_future,
	future_incompatible,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_copy_implementations,
	missing_debug_implementations,
	// non_exhaustive_omitted_patterns,
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

mod xoodoo;

use sponge::{
	cyclist::{Hash, Keyed},
	Cyclist
};

pub use crate::xoodoo::*;

pub type XoodyakHash<S> = Cyclist<S, Xoodoo, Hash<16>>;
pub type XoodyakKeyed<S> = Cyclist<S, Xoodoo, Keyed<44, 24, 16>>;

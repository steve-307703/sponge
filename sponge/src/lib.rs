#![warn(
	clippy::nursery,
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
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unsafe_op_in_unsafe_fn,
	unused_lifetimes,
	unused_qualifications,
	unused_results
)]

pub mod cyclist;
pub mod sponge;
pub mod state;

mod suffix;

#[cfg(feature = "zeroize")]
pub use crate::state::SecretState;
pub use crate::{cyclist::Cyclist, sponge::Sponge, state::State, suffix::*};

pub trait Permutation<S> {
	fn permute(state: &mut S);
}

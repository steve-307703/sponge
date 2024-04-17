#![no_std]
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

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod cyclist;
pub mod sponge;
pub mod state;

mod suffix;

pub use crate::{
	cyclist::Cyclist,
	sponge::Sponge,
	state::{SecretState, State},
	suffix::*
};

pub trait Permutation<S> {
	fn permute(state: &mut S);
}

pub trait Absorb {
	fn absorb(&mut self, buf: &[u8]);

	#[inline]
	fn absorb_u8(&mut self, b: u8) {
		self.absorb(core::slice::from_ref(&b));
	}
}

pub trait IntoSqueezer {
	type Squeezer: Squeezer;

	fn into_squeezer(self) -> Self::Squeezer;

	#[inline]
	fn squeeze_into(self, buf: &mut [u8])
	where
		Self: Sized
	{
		self.into_squeezer().squeeze_into(buf)
	}

	#[inline]
	fn squeeze<const LEN: usize>(self) -> [u8; LEN]
	where
		Self: Sized
	{
		self.into_squeezer().squeeze()
	}

	#[cfg(feature = "alloc")]
	#[inline]
	fn squeeze_boxed(self, len: usize) -> alloc::boxed::Box<[u8]>
	where
		Self: Sized
	{
		let mut buf = alloc::vec![0; len].into_boxed_slice();
		self.squeeze_into(&mut buf);
		buf
	}
}

pub trait Squeeze {
	fn squeeze_into(self, buf: &mut [u8]);

	#[inline]
	fn squeeze<const LEN: usize>(self) -> [u8; LEN]
	where
		Self: Sized
	{
		let mut buf = [0; LEN];
		self.squeeze_into(&mut buf);
		buf
	}

	#[cfg(feature = "alloc")]
	#[inline]
	fn squeeze_boxed(self, len: usize) -> alloc::boxed::Box<[u8]>
	where
		Self: Sized
	{
		let mut buf = alloc::vec![0; len].into_boxed_slice();
		self.squeeze_into(&mut buf);
		buf
	}
}

impl<T> Squeeze for T
where
	T: IntoSqueezer
{
	fn squeeze_into(self, buf: &mut [u8]) {
		self.into_squeezer().squeeze_into(buf)
	}

	fn squeeze<const LEN: usize>(self) -> [u8; LEN] {
		self.into_squeezer().squeeze()
	}

	#[cfg(feature = "alloc")]
	fn squeeze_boxed(self, len: usize) -> alloc::boxed::Box<[u8]> {
		self.into_squeezer().squeeze_boxed(len)
	}
}

pub trait Squeezer {
	fn squeeze_into(&mut self, buf: &mut [u8]);

	#[inline]
	fn squeeze<const LEN: usize>(&mut self) -> [u8; LEN] {
		let mut buf = [0; LEN];
		self.squeeze_into(&mut buf);
		buf
	}

	#[cfg(feature = "alloc")]
	#[inline]
	fn squeeze_boxed(&mut self, len: usize) -> alloc::boxed::Box<[u8]> {
		let mut buf = alloc::vec![0; len].into_boxed_slice();
		self.squeeze_into(&mut buf);
		buf
	}
}
